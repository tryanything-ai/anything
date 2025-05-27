use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    response::{IntoResponse, Response},
    Extension,
};
use dashmap::DashMap;
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, RwLock};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::{supabase_jwt_middleware::User, types::task_types::Task, AppState};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTestingUpdate {
    pub flow_session_id: String,
    pub account_id: String,
    pub update_type: UpdateType,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateType {
    TaskCreated,
    TaskUpdated,
    TaskCompleted,
    TaskFailed,
    WorkflowCompleted,
    WorkflowFailed,
}

#[derive(Debug, Clone)]
pub struct WebSocketConnection {
    pub account_id: String,
    pub flow_session_id: String,
    pub user_id: String,
}

pub type WebSocketConnections = DashMap<String, WebSocketConnection>;
pub type WorkflowBroadcaster = broadcast::Sender<WorkflowTestingUpdate>;

#[derive(Debug, Deserialize)]
pub struct WebSocketQuery {
    token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    iss: String,
}

fn decode_jwt_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET must be set");
    let key = DecodingKey::from_secret(secret.as_ref());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["authenticated"]);
    let token_data = decode::<Claims>(&token, &key, &validation)?;
    Ok(token_data.claims)
}

/// WebSocket handler for workflow testing updates
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path((account_id, flow_session_id)): Path<(String, String)>,
    Query(query): Query<WebSocketQuery>,
    State(state): State<Arc<AppState>>,
    user: Option<Extension<User>>,
) -> Response {
    info!(
        "[WEBSOCKET] New WebSocket connection request for account: {}, session: {}, has_user: {}, has_token: {}",
        account_id, flow_session_id, user.is_some(), query.token.is_some()
    );

    // Try to get user from middleware first, then from query token
    let authenticated_user = if let Some(Extension(user)) = user {
        info!(
            "[WEBSOCKET] Using user from middleware: {}",
            user.account_id
        );
        user
    } else if let Some(token) = query.token {
        info!("[WEBSOCKET] Attempting to validate token from query parameter");
        // Validate token from query parameter
        match decode_jwt_token(&token) {
            Ok(claims) => {
                info!(
                    "[WEBSOCKET] Token validated successfully for user: {}",
                    claims.sub
                );
                User {
                    jwt: token,
                    account_id: claims.sub,
                }
            }
            Err(e) => {
                error!("[WEBSOCKET] Invalid token in query parameter: {}", e);
                return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
            }
        }
    } else {
        error!("[WEBSOCKET] No authentication provided - no user from middleware and no token in query");
        return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    ws.on_upgrade(move |socket| {
        handle_websocket_connection(
            socket,
            account_id,
            flow_session_id,
            authenticated_user,
            state,
        )
    })
}

async fn handle_websocket_connection(
    socket: WebSocket,
    account_id: String,
    flow_session_id: String,
    user: User,
    state: Arc<AppState>,
) {
    let connection_id = Uuid::new_v4().to_string();
    info!(
        "[WEBSOCKET] Handling WebSocket connection {} for account: {}, session: {}",
        connection_id, account_id, flow_session_id
    );

    // Store connection info
    let connection = WebSocketConnection {
        account_id: account_id.clone(),
        flow_session_id: flow_session_id.clone(),
        user_id: user.account_id.clone(),
    };

    state
        .websocket_connections
        .insert(connection_id.clone(), connection);

    // Subscribe to workflow updates
    let mut receiver = state.workflow_broadcaster.subscribe();

    let (mut sender, mut receiver_ws) = socket.split();

    // Send initial connection confirmation
    let confirmation = serde_json::json!({
        "type": "connection_established",
        "flow_session_id": flow_session_id,
        "account_id": account_id
    });

    if let Err(e) = sender.send(Message::Text(confirmation.to_string())).await {
        error!("[WEBSOCKET] Failed to send confirmation: {}", e);
        return;
    }

    // Send current session state if available
    if let Ok(_flow_session_uuid) = flow_session_id.parse::<Uuid>() {
        // Since we've removed the cache, we could fetch current tasks from database here if needed
        // For now, we'll let the frontend handle the initial state fetch
        let current_state = serde_json::json!({
            "type": "session_state",
            "flow_session_id": flow_session_id,
            "tasks": []
        });

        if let Err(e) = sender.send(Message::Text(current_state.to_string())).await {
            error!("[WEBSOCKET] Failed to send current state: {}", e);
        }
    }

    // Handle incoming messages and broadcast updates
    tokio::select! {
        // Handle incoming WebSocket messages
        _ = async {
            while let Some(msg) = receiver_ws.next().await {
                match msg {
                    Ok(Message::Text(_)) => {
                        // Handle ping/pong or other client messages if needed
                    }
                    Ok(Message::Close(_)) => {
                        info!("[WEBSOCKET] Client closed connection {}", connection_id);
                        break;
                    }
                    Err(e) => {
                        warn!("[WEBSOCKET] WebSocket error for connection {}: {}", connection_id, e);
                        break;
                    }
                    _ => {}
                }
            }
        } => {},

        // Handle broadcast messages
        _ = async {
            while let Ok(update) = receiver.recv().await {
                // Send updates for this specific session and account, or global updates (empty session/account)
                let should_send = (update.flow_session_id == flow_session_id && update.account_id == account_id) ||
                                  (update.flow_session_id.is_empty() && update.account_id.is_empty()) ||
                                  (update.flow_session_id == flow_session_id && update.account_id.is_empty());

                if should_send {
                    let message = serde_json::json!({
                        "type": "workflow_update",
                        "update_type": update.update_type,
                        "flow_session_id": flow_session_id, // Use the connection's session ID
                        "data": update.data
                    });

                    if let Err(e) = sender.send(Message::Text(message.to_string())).await {
                        error!("[WEBSOCKET] Failed to send update to connection {}: {}", connection_id, e);
                        break;
                    }
                }
            }
        } => {}
    }

    // Clean up connection
    state.websocket_connections.remove(&connection_id);

    info!("[WEBSOCKET] Connection {} closed", connection_id);
}

/// Broadcast a task update to all relevant WebSocket connections
pub async fn broadcast_task_update(
    broadcaster: &WorkflowBroadcaster,
    account_id: &str,
    flow_session_id: &str,
    update_type: UpdateType,
    task: &Task,
) {
    let update = WorkflowTestingUpdate {
        flow_session_id: flow_session_id.to_string(),
        account_id: account_id.to_string(),
        update_type,
        data: serde_json::to_value(task).unwrap_or_default(),
    };

    if let Err(e) = broadcaster.send(update) {
        // This is expected when no one is listening
        if broadcaster.receiver_count() > 0 {
            warn!("[WEBSOCKET] Failed to broadcast task update: {}", e);
        }
    }
}

/// Broadcast a workflow completion update
pub async fn broadcast_workflow_completion(
    broadcaster: &WorkflowBroadcaster,
    account_id: &str,
    flow_session_id: &str,
    success: bool,
    tasks: Vec<Task>,
) {
    let update_type = if success {
        UpdateType::WorkflowCompleted
    } else {
        UpdateType::WorkflowFailed
    };

    let update = WorkflowTestingUpdate {
        flow_session_id: flow_session_id.to_string(),
        account_id: account_id.to_string(),
        update_type,
        data: serde_json::json!({
            "complete": true,
            "success": success,
            "tasks": tasks
        }),
    };

    if let Err(e) = broadcaster.send(update) {
        if broadcaster.receiver_count() > 0 {
            warn!("[WEBSOCKET] Failed to broadcast workflow completion: {}", e);
        }
    }
}

/// Broadcast a simple task update (just task_id and update type)
pub async fn broadcast_task_update_simple(
    broadcaster: &WorkflowBroadcaster,
    task_id: &Uuid,
    update_type: UpdateType,
) {
    let update = WorkflowTestingUpdate {
        flow_session_id: "".to_string(), // Will be filtered by frontend
        account_id: "".to_string(),      // Will be filtered by frontend
        update_type,
        data: serde_json::json!({
            "task_id": task_id,
            "needs_refresh": true
        }),
    };

    if let Err(e) = broadcaster.send(update) {
        if broadcaster.receiver_count() > 0 {
            warn!("[WEBSOCKET] Failed to broadcast simple task update: {}", e);
        }
    }
}

/// Broadcast a simple workflow completion update
pub async fn broadcast_workflow_completion_simple(
    broadcaster: &WorkflowBroadcaster,
    flow_session_id: &Uuid,
    success: bool,
) {
    let update_type = if success {
        UpdateType::WorkflowCompleted
    } else {
        UpdateType::WorkflowFailed
    };

    let update = WorkflowTestingUpdate {
        flow_session_id: flow_session_id.to_string(),
        account_id: "".to_string(), // Will be filtered by frontend
        update_type,
        data: serde_json::json!({
            "complete": true,
            "success": success,
            "needs_refresh": true
        }),
    };

    if let Err(e) = broadcaster.send(update) {
        if broadcaster.receiver_count() > 0 {
            warn!(
                "[WEBSOCKET] Failed to broadcast simple workflow completion: {}",
                e
            );
        }
    }
}

/// Broadcast a task update with session information (for CreateTask operations)
pub async fn broadcast_task_update_with_session(
    broadcaster: &WorkflowBroadcaster,
    account_id: &str,
    flow_session_id: &str,
    task_id: &Uuid,
    update_type: UpdateType,
) {
    let update = WorkflowTestingUpdate {
        flow_session_id: flow_session_id.to_string(),
        account_id: account_id.to_string(),
        update_type,
        data: serde_json::json!({
            "task_id": task_id,
            "needs_refresh": true
        }),
    };

    if let Err(e) = broadcaster.send(update) {
        if broadcaster.receiver_count() > 0 {
            warn!(
                "[WEBSOCKET] Failed to broadcast task update with session: {}",
                e
            );
        }
    }
}
