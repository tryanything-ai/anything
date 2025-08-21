use crate::AppState;
use crate::entities::{tasks, accounts_billing};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QuerySelect, ActiveModelTrait, Set};
use tokio::time::{sleep, interval};
use uuid::Uuid;

pub async fn billing_processing_loop(state: Arc<AppState>) {
    println!("[BILLING USAGE ENGINE SEAORM] Starting billing processing engine");
    let mut interval_timer = interval(Duration::from_secs(300)); // 5 minutes

    loop {
        interval_timer.tick().await;
        
        if let Err(e) = process_billing_usage(&state).await {
            eprintln!("[BILLING USAGE ENGINE SEAORM] Error processing billing usage: {:?}", e);
        }
    }
}

async fn process_billing_usage(state: &Arc<AppState>) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("[BILLING USAGE ENGINE SEAORM] Processing billing usage");

    // Get all completed tasks that haven't been billed yet
    let unbilled_tasks = tasks::Entity::find()
        .filter(tasks::Column::TaskStatus.eq("completed"))
        // TODO: Add a billing_processed column to the tasks table to track this
        // For now, we'll use a simple time-based approach
        .filter(tasks::Column::CompletedAt.is_not_null())
        .all(&*state.db)
        .await?;

    if unbilled_tasks.is_empty() {
        println!("[BILLING USAGE ENGINE SEAORM] No unbilled tasks found");
        return Ok(());
    }

    println!("[BILLING USAGE ENGINE SEAORM] Found {} unbilled tasks", unbilled_tasks.len());

    // Group tasks by account_id for billing
    let mut account_usage: std::collections::HashMap<Uuid, Vec<&tasks::Model>> = 
        std::collections::HashMap::new();

    for task in &unbilled_tasks {
        account_usage
            .entry(task.account_id)
            .or_insert_with(Vec::new)
            .push(task);
    }

    // Process billing for each account
    for (account_id, account_tasks) in account_usage {
        if let Err(e) = process_account_billing(state, account_id, account_tasks).await {
            eprintln!("[BILLING USAGE ENGINE SEAORM] Error processing billing for account {}: {:?}", 
                     account_id, e);
        }
    }

    Ok(())
}

async fn process_account_billing(
    state: &Arc<AppState>,
    account_id: Uuid,
    tasks: Vec<&tasks::Model>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("[BILLING USAGE ENGINE SEAORM] Processing billing for account: {}", account_id);

    // Get billing information for the account
    let billing_info = accounts_billing::Entity::find()
        .filter(accounts_billing::Column::AccountId.eq(account_id))
        .one(&*state.db)
        .await?;

    let billing_record = match billing_info {
        Some(record) => record,
        None => {
            println!("[BILLING USAGE ENGINE SEAORM] No billing record found for account: {}", account_id);
            return Ok(());
        }
    };

    // Check if account is active and should be billed
    if billing_record.active != Some(true) {
        println!("[BILLING USAGE ENGINE SEAORM] Account {} is not active for billing", account_id);
        return Ok(());
    }

    // Calculate usage metrics
    let task_count = tasks.len() as i32;
    let total_execution_time: i64 = tasks
        .iter()
        .filter_map(|task| task.execution_time_ms)
        .sum();

    println!("[BILLING USAGE ENGINE SEAORM] Account {} usage: {} tasks, {}ms execution time", 
             account_id, task_count, total_execution_time);

    // Extract values before moving billing_record
    let updated_tasks_used = billing_record.tasks_used.unwrap_or(0) + task_count;
    let tasks_limit = billing_record.tasks_limit;
    let stripe_customer_id = billing_record.stripe_customer_id.clone();
    
    let mut active_model: accounts_billing::ActiveModel = billing_record.into();
    active_model.tasks_used = Set(Some(updated_tasks_used));
    active_model.updated_at = Set(Some(chrono::Utc::now()));
    
    active_model.update(&*state.db).await?;

    // Check if account has exceeded limits
    if let Some(limit) = tasks_limit {
        if updated_tasks_used > limit {
            println!("[BILLING USAGE ENGINE SEAORM] Account {} has exceeded task limit", account_id);
            // TODO: Implement limit enforcement logic
        }
    }

    // Send usage to Stripe if needed
    if let Some(customer_id) = stripe_customer_id {
        if let Err(e) = send_usage_to_stripe(&customer_id, task_count, total_execution_time).await {
            eprintln!("[BILLING USAGE ENGINE SEAORM] Error sending usage to Stripe: {:?}", e);
        }
    }

    println!("[BILLING USAGE ENGINE SEAORM] Successfully processed billing for account: {}", account_id);
    Ok(())
}

async fn send_usage_to_stripe(
    stripe_customer_id: &str,
    task_count: i32,
    execution_time_ms: i64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("[BILLING USAGE ENGINE SEAORM] Sending usage to Stripe for customer: {}", stripe_customer_id);

    // TODO: Implement Stripe usage reporting
    // This would involve:
    // 1. Creating usage records for metered billing
    // 2. Updating subscription items with usage data
    // 3. Handling any Stripe API errors

    println!("[BILLING USAGE ENGINE SEAORM] Would send {} tasks, {}ms to Stripe", 
             task_count, execution_time_ms);

    Ok(())
}

/// Reset billing usage for a new billing period
pub async fn reset_billing_period(state: Arc<AppState>) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("[BILLING USAGE ENGINE SEAORM] Resetting billing period");

    // Reset usage counters for all accounts
    let billing_records = accounts_billing::Entity::find()
        .filter(accounts_billing::Column::Active.eq(true))
        .all(&*state.db)
        .await?;

    for record in billing_records {
        let mut active_model: accounts_billing::ActiveModel = record.into();
        active_model.tasks_used = Set(Some(0));
        active_model.storage_used = Set(Some(0));
        active_model.updated_at = Set(Some(chrono::Utc::now()));
        
        active_model.update(&*state.db).await?;
    }

    println!("[BILLING USAGE ENGINE SEAORM] Billing period reset completed");
    Ok(())
}

/// Get billing summary for an account
pub async fn get_account_billing_summary(
    state: Arc<AppState>,
    account_id: Uuid,
) -> Result<Option<accounts_billing::Model>, Box<dyn Error + Send + Sync>> {
    println!("[BILLING USAGE ENGINE SEAORM] Getting billing summary for account: {}", account_id);

    let billing_record = accounts_billing::Entity::find()
        .filter(accounts_billing::Column::AccountId.eq(account_id))
        .one(&*state.db)
        .await?;

    Ok(billing_record)
}
