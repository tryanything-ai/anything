use chrono::{DateTime, Datelike, Duration, NaiveTime, TimeZone, Utc, Weekday};
use chrono_tz::Tz;
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::sleep;
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub campaign_id: Uuid,
    pub account_id: Uuid,
    pub agent_id: Uuid,
    pub campaign_name: String,
    pub campaign_description: String,
    pub campaign_status: String,
    pub schedule_days_of_week: Vec<String>,
    pub schedule_start_time: NaiveTime,
    pub schedule_end_time: NaiveTime,
    pub timezone: String,
    pub active: bool,
    pub archived: bool,
    pub batch_mode: bool,
    pub batch_size: i32,
    pub processed_in_batch: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignContact {
    pub campaign_contact_id: Uuid,
    pub campaign_id: Uuid,
    pub contact_id: Uuid,
    pub account_id: Uuid,
    pub status: String,
    pub active: bool,
    pub archived: bool,
}

#[derive(Debug, Clone)]
pub struct CampaignState {
    pub campaign: Campaign,
    pub last_processed: Option<DateTime<Utc>>,
    pub next_processing_time: Option<DateTime<Utc>>,
    pub contact_counts: ContactCounts,
    pub current_page: i32,
    pub page_size: i32,
    pub contacts_processed_in_batch: i32,
}

#[derive(Debug, Clone)]
pub struct ContactCounts {
    pub active: i32,
    pub pending: i32,
    pub completed: i32,
}

pub async fn campaign_engine_loop(state: Arc<AppState>) {
    // Map of campaign_id to CampaignState
    let campaign_states: Arc<RwLock<HashMap<Uuid, CampaignState>>> =
        Arc::new(RwLock::new(HashMap::new()));

    // Initial hydration of campaigns
    let client = state.anything_client.clone();
    hydrate_campaigns(state.clone(), &client, &campaign_states).await;

    // Check campaigns every minute
    let refresh_interval = tokio::time::Duration::from_secs(60);

    loop {
        // Check if shutdown signal is set
        if state
            .shutdown_signal
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            println!("[CAMPAIGN_ENGINE] Shutdown signal received, exiting loop");
            break;
        }

        println!("[CAMPAIGN_ENGINE] Starting campaign check loop");

        // Find campaigns that should be processed now
        let campaigns_to_process = {
            let campaign_states = campaign_states.read().await;
            campaign_states
                .iter()
                .filter(|(_, campaign_state)| should_process_campaign(campaign_state))
                .map(|(id, campaign_state)| (*id, campaign_state.clone()))
                .collect::<Vec<_>>()
        };

        // Process each campaign
        for (id, campaign_state) in campaigns_to_process {
            println!(
                "[CAMPAIGN_ENGINE] Processing campaign: {} ({})",
                campaign_state.campaign.campaign_name, id
            );

            if let Err(e) = process_campaign(&state, &campaign_state).await {
                println!("[CAMPAIGN_ENGINE] Error processing campaign: {:?}", e);
            } else {
                // Update campaign state with new processing time
                if let Err(e) =
                    update_campaign_processing_time(&id, &campaign_state, &campaign_states).await
                {
                    println!(
                        "[CAMPAIGN_ENGINE] Error updating campaign processing time: {:?}",
                        e
                    );
                }
            }
        }

        println!("[CAMPAIGN_ENGINE] Finished campaign check loop");

        // Refresh campaign states from database periodically
        // This ensures we have the latest campaign configurations and contacts
        let client = state.anything_client.clone();
        hydrate_campaigns(state.clone(), &client, &campaign_states).await;

        // Wait for the next check interval
        sleep(refresh_interval).await;
    }
}

async fn hydrate_campaigns(
    state: Arc<AppState>,
    client: &Postgrest,
    campaign_states: &Arc<RwLock<HashMap<Uuid, CampaignState>>>,
) {
    println!("[CAMPAIGN_ENGINE] Hydrating campaigns from the database");

    // Get API key from environment
    let supabase_service_role_api_key = match std::env::var("SUPABASE_SERVICE_ROLE_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            println!("[CAMPAIGN_ENGINE] Error getting API key: {:?}", e);
            return;
        }
    };

    // Fetch active campaigns that are not completed
    let response = match client
        .from("campaigns")
        .auth(supabase_service_role_api_key.clone())
        .select("*")
        .eq("active", "true")
        .eq("archived", "false")
        .neq("campaign_status", "completed")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[CAMPAIGN_ENGINE] Error fetching campaigns: {:?}", e);
            return;
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[CAMPAIGN_ENGINE] Error reading response body: {:?}", e);
            return;
        }
    };

    let campaigns: Vec<Campaign> = match serde_json::from_str(&body) {
        Ok(campaigns) => campaigns,
        Err(e) => {
            println!("[CAMPAIGN_ENGINE] Error parsing campaigns: {:?}", e);
            return;
        }
    };

    // Update campaign states
    let mut new_campaign_states = HashMap::new();

    for campaign in campaigns {
        let campaign_id = campaign.campaign_id;

        // Fetch contacts for this campaign
        let contacts = fetch_campaign_contacts(
            &client,
            &supabase_service_role_api_key,
            &campaign_id,
            100,
            0,
        )
        .await;

        // Categorize contacts by status
        let mut active_contacts = Vec::new();
        let mut pending_contacts = Vec::new();
        let mut completed_contacts = Vec::new();

        for contact in contacts {
            match contact.status.as_str() {
                "active" => active_contacts.push(contact),
                "pending" => pending_contacts.push(contact),
                "completed" => completed_contacts.push(contact),
                _ => pending_contacts.push(contact), // Default to pending for unknown statuses
            }
        }

        // Create or update campaign state
        let mut campaign_states_write = campaign_states.write().await;

        if let Some(existing_state) = campaign_states_write.get(&campaign_id) {
            // Update existing state
            let next_time = calculate_next_processing_time(&campaign);
            let updated_state = CampaignState {
                campaign: campaign.clone(),
                last_processed: existing_state.last_processed,
                next_processing_time: next_time,
                contact_counts: ContactCounts {
                    active: active_contacts.len() as i32,
                    pending: pending_contacts.len() as i32,
                    completed: completed_contacts.len() as i32,
                },
                current_page: 0,
                page_size: campaign.batch_size,
                contacts_processed_in_batch: campaign.processed_in_batch,
            };

            new_campaign_states.insert(campaign_id, updated_state);
        } else {
            // Create new state
            let next_time = calculate_next_processing_time(&campaign);
            let processed_in_batch = campaign.processed_in_batch;
            let batch_size = campaign.batch_size;
            let new_state = CampaignState {
                campaign,
                last_processed: None,
                next_processing_time: next_time,
                contact_counts: ContactCounts {
                    active: active_contacts.len() as i32,
                    pending: pending_contacts.len() as i32,
                    completed: completed_contacts.len() as i32,
                },
                current_page: 0,
                page_size: batch_size,
                contacts_processed_in_batch: processed_in_batch,
            };

            new_campaign_states.insert(campaign_id, new_state);
        }
    }

    // Replace the entire campaign states map with the new one
    let mut campaign_states_write = campaign_states.write().await;
    *campaign_states_write = new_campaign_states;

    println!(
        "[CAMPAIGN_ENGINE] Successfully hydrated {} campaigns",
        campaign_states_write.len()
    );
}

async fn fetch_campaign_contacts(
    client: &Postgrest,
    api_key: &str,
    campaign_id: &Uuid,
    page_size: i32,
    offset: i32,
) -> Vec<CampaignContact> {
    let response = client
        .from("campaign_contacts")
        .auth(api_key)
        .select("*")
        .eq("campaign_id", campaign_id.to_string())
        .eq("active", "true")
        .eq("archived", "false")
        .neq("status", "completed")
        .range(offset as usize, (offset + page_size - 1) as usize)
        .execute()
        .await?;

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[CAMPAIGN_ENGINE] Error reading response body: {:?}", e);
            return Vec::new();
        }
    };

    match serde_json::from_str(&body) {
        Ok(contacts) => contacts,
        Err(e) => {
            println!("[CAMPAIGN_ENGINE] Error parsing campaign contacts: {:?}", e);
            Vec::new()
        }
    }
}

fn should_process_campaign(campaign_state: &CampaignState) -> bool {
    // Check if campaign is active
    if !campaign_state.campaign.active || campaign_state.campaign.archived {
        return false;
    }

    // Check if campaign has any pending or active contacts
    if campaign_state.contact_counts.active == 0 && campaign_state.contact_counts.pending == 0 {
        return false;
    }

    // Check if we're within the scheduled time window
    if !is_within_schedule_window(&campaign_state.campaign) {
        return false;
    }

    // Check batch limits if batch mode is enabled
    if campaign_state.campaign.batch_mode {
        if campaign_state.contacts_processed_in_batch >= campaign_state.campaign.batch_size {
            // Batch limit reached, don't process more contacts
            return false;
        }
    }

    // Check if it's time to process based on last processing time
    if let Some(next_time) = campaign_state.next_processing_time {
        if Utc::now() < next_time {
            return false;
        }
    }

    true
}

fn is_within_schedule_window(campaign: &Campaign) -> bool {
    // Parse the timezone
    let timezone: Tz = match campaign.timezone.parse() {
        Ok(tz) => tz,
        Err(_) => return false, // Invalid timezone
    };

    // Get current time in the campaign's timezone
    let now = Utc::now().with_timezone(&timezone);

    // Check if today is in the scheduled days
    let today = match now.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };

    if !campaign
        .schedule_days_of_week
        .iter()
        .any(|day| day == today)
    {
        return false;
    }

    // Check if current time is within the scheduled hours
    let current_time = now.time();

    current_time >= campaign.schedule_start_time && current_time <= campaign.schedule_end_time
}

fn calculate_next_processing_time(campaign: &Campaign) -> Option<DateTime<Utc>> {
    // For simplicity, set next processing time to 5 minutes from now
    // This could be made more sophisticated based on campaign needs
    Some(Utc::now() + Duration::minutes(5))
}

async fn update_campaign_processing_time(
    campaign_id: &Uuid,
    campaign_state: &CampaignState,
    campaign_states: &Arc<RwLock<HashMap<Uuid, CampaignState>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut campaign_states = campaign_states.write().await;

    if let Some(state) = campaign_states.get_mut(campaign_id) {
        state.last_processed = Some(Utc::now());
        state.next_processing_time = calculate_next_processing_time(&campaign_state.campaign);

        // Increment the batch counter in memory
        if campaign_state.campaign.batch_mode {
            state.contacts_processed_in_batch += 1;
        }
    }

    Ok(())
}

async fn process_campaign(
    state: &Arc<AppState>,
    campaign_state: &CampaignState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[CAMPAIGN_ENGINE] Processing campaign: {} ({})",
        campaign_state.campaign.campaign_name, campaign_state.campaign.campaign_id
    );

    // Get the next contact to process
    let contact = if campaign_state.contact_counts.pending > 0 {
        // Process a pending contact first
        // This is a placeholder implementation. In a real scenario, you might want to implement a more robust contact selection logic
        // &campaign_state.contact_counts.pending as *const CampaignContact as *const CampaignContact
    } else if campaign_state.contact_counts.active > 0 {
        // Then process active contacts
        // This is a placeholder implementation. In a real scenario, you might want to implement a more robust contact selection logic
        // &campaign_state.contact_counts.active as *const CampaignContact as *const CampaignContact
    } else {
        // No contacts to process
        return Ok(());
    };

    // Run the agent for this contact
    let agent_id = campaign_state.campaign.agent_id;
    let account_id = campaign_state.campaign.account_id;
    let contact_id = contact.contact_id;

    // Create input for the agent
    let input = serde_json::json!({
        "campaign_id": campaign_state.campaign.campaign_id.to_string(),
        "campaign_name": campaign_state.campaign.campaign_name,
        "contact_id": contact_id.to_string(),
        "action": "process_campaign_contact"
    });

    // Run the agent action
    // TODO: Implement agent action execution
    // For now, we'll just log that we would run the agent
    println!(
        "[CAMPAIGN_ENGINE] Would run agent {} for account {} with input: {}",
        agent_id, account_id, input
    );

    // Update contact status in the database
    update_contact_status(
        &state.anything_client,
        &contact.campaign_contact_id,
        "active",
    )
    .await?;

    println!(
        "[CAMPAIGN_ENGINE] Successfully processed contact {} for campaign {}",
        contact_id, campaign_state.campaign.campaign_id
    );

    // After successfully processing a contact, update the batch counter in the database
    if campaign_state.campaign.batch_mode {
        update_campaign_batch_counter(&state.anything_client, &campaign_state.campaign.campaign_id)
            .await?;
    }

    Ok(())
}

async fn update_contact_status(
    client: &Arc<Postgrest>,
    campaign_contact_id: &Uuid,
    status: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let supabase_service_role_api_key = std::env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

    let body = serde_json::json!({
        "status": status
    });

    client
        .from("campaign_contacts")
        .auth(supabase_service_role_api_key)
        .eq("campaign_contact_id", campaign_contact_id.to_string())
        .update(body.to_string())
        .execute()
        .await?;

    Ok(())
}

async fn update_campaign_batch_counter(
    client: &Arc<Postgrest>,
    campaign_id: &Uuid,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let supabase_service_role_api_key = std::env::var("SUPABASE_SERVICE_ROLE_API_KEY")?;

    // Increment the processed_in_batch counter
    let body = serde_json::json!({
        "processed_in_batch": serde_json::json!({"increment": 1})
    });

    client
        .from("campaigns")
        .auth(supabase_service_role_api_key)
        .eq("campaign_id", campaign_id.to_string())
        .update(body.to_string())
        .execute()
        .await?;

    Ok(())
}
