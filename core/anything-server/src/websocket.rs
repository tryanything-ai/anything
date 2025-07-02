use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, Query, State, WebSocketUpgrade,
    },
    response::Response,
};
use dashmap::DashMap;
use futures_util::{sink::SinkExt, stream::StreamExt};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::AppState;

// JWT claims structure for token validation
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    iss: String,
}

fn decode_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(secret.as_ref());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["authenticated"]);
    let token_data = decode::<Claims>(&token, &key, &validation)?;
    Ok(token_data.claims)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub r#type: String,
    pub data: Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStatusUpdate {
    pub flow_session_id: Uuid,
    pub status: String,
    pub task_id: Option<Uuid>,
    pub task_status: Option<String>,
    pub result: Option<Value>,
    pub error: Option<Value>,
}

// Workflow testing specific message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTestingUpdate {
    pub r#type: String, // "workflow_update", "connection_established", "session_state"
    pub update_type: Option<String>, // "task_created", "task_updated", "task_completed", "task_failed", "workflow_completed", "workflow_failed"
    pub flow_session_id: String,
    pub data: Option<Value>,
    pub tasks: Option<Value>, // For session_state messages
    pub complete: Option<bool>,
}

pub type WebSocketSender = broadcast::Sender<WebSocketMessage>;
pub type WebSocketReceiver = broadcast::Receiver<WebSocketMessage>;

#[derive(Debug)]
pub struct WebSocketConnection {
    pub account_id: String,
    pub connection_id: String,
    pub sender: tokio::sync::mpsc::UnboundedSender<Message>,
    pub flow_session_id: Option<String>, // For workflow testing connections
}

pub struct WebSocketManager {
    pub connections: DashMap<String, WebSocketConnection>,
    pub broadcaster: WebSocketSender,
}

impl WebSocketManager {
    pub fn new() -> Self {
        let (broadcaster, _) = broadcast::channel(1000);
        Self {
            connections: DashMap::new(),
            broadcaster,
        }
    }

    pub fn add_connection(
        &self,
        account_id: String,
        connection_id: String,
        sender: tokio::sync::mpsc::UnboundedSender<Message>,
    ) {
        let connection = WebSocketConnection {
            account_id: account_id.clone(),
            connection_id: connection_id.clone(),
            sender,
            flow_session_id: None,
        };

        self.connections.insert(connection_id.clone(), connection);
        info!(
            "[WEBSOCKET] Added connection {} for account {}",
            connection_id, account_id
        );
    }

    pub fn add_workflow_testing_connection(
        &self,
        account_id: String,
        connection_id: String,
        flow_session_id: String,
        sender: tokio::sync::mpsc::UnboundedSender<Message>,
    ) {
        let connection = WebSocketConnection {
            account_id: account_id.clone(),
            connection_id: connection_id.clone(),
            sender,
            flow_session_id: Some(flow_session_id.clone()),
        };

        self.connections.insert(connection_id.clone(), connection);
        info!(
            "[WEBSOCKET] Added workflow testing connection {} for account {} and session {}",
            connection_id, account_id, flow_session_id
        );
    }

    pub fn remove_connection(&self, connection_id: &str) {
        if let Some((_, connection)) = self.connections.remove(connection_id) {
            info!(
                "[WEBSOCKET] Removed connection {} for account {}",
                connection_id, connection.account_id
            );
        }
    }

    pub fn broadcast_to_account(&self, account_id: &str, message: WebSocketMessage) {
        let connections_to_send: Vec<_> = self
            .connections
            .iter()
            .filter(|entry| entry.value().account_id == account_id)
            .map(|entry| (entry.key().clone(), entry.value().sender.clone()))
            .collect();

        for (connection_id, sender) in connections_to_send {
            let json_message = match serde_json::to_string(&message) {
                Ok(json) => json,
                Err(e) => {
                    error!("[WEBSOCKET] Failed to serialize message: {}", e);
                    continue;
                }
            };

            if let Err(e) = sender.send(Message::Text(json_message)) {
                warn!(
                    "[WEBSOCKET] Failed to send message to connection {}: {}",
                    connection_id, e
                );
                // Remove the connection if sending fails
                self.remove_connection(&connection_id);
            }
        }
    }

    pub fn broadcast_workflow_status(&self, account_id: &str, status_update: WorkflowStatusUpdate) {
        let message = WebSocketMessage {
            r#type: "workflow_status".to_string(),
            data: serde_json::to_value(status_update).unwrap_or_default(),
            timestamp: chrono::Utc::now(),
        };

        self.broadcast_to_account(account_id, message);
    }

    // Broadcast workflow testing updates to specific session connections
    pub fn broadcast_workflow_testing_update(
        &self,
        account_id: &str,
        flow_session_id: &str,
        update: WorkflowTestingUpdate,
    ) {
        let connections_to_send: Vec<_> = self
            .connections
            .iter()
            .filter(|entry| {
                entry.value().account_id == account_id
                    && entry.value().flow_session_id.as_deref() == Some(flow_session_id)
            })
            .map(|entry| (entry.key().clone(), entry.value().sender.clone()))
            .collect();

        for (connection_id, sender) in connections_to_send {
            let json_message = match serde_json::to_string(&update) {
                Ok(json) => json,
                Err(e) => {
                    error!(
                        "[WEBSOCKET] Failed to serialize workflow testing update: {}",
                        e
                    );
                    continue;
                }
            };

            if let Err(e) = sender.send(Message::Text(json_message)) {
                warn!(
                    "[WEBSOCKET] Failed to send workflow testing update to connection {}: {}",
                    connection_id, e
                );
                // Remove the connection if sending fails
                self.remove_connection(&connection_id);
            } else {
                info!(
                    "[WEBSOCKET] Sent workflow testing update to connection {} for session {}",
                    connection_id, flow_session_id
                );
            }
        }
    }
}

#[derive(Deserialize)]
pub struct WebSocketQuery {
    account_id: String,
}

#[derive(Deserialize)]
pub struct WorkflowTestingWebSocketQuery {
    token: String,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path(connection_id): Path<String>,
    Query(query): Query<WebSocketQuery>,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_websocket(socket, connection_id, query.account_id, state))
}

pub async fn workflow_testing_websocket_handler(
    ws: WebSocketUpgrade,
    Path((account_id, flow_session_id)): Path<(String, String)>,
    Query(query): Query<WorkflowTestingWebSocketQuery>,
    State(state): State<Arc<AppState>>,
) -> Response {
    // Validate the JWT token
    let secret = match env::var("SUPABASE_JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => {
            error!("[WEBSOCKET] SUPABASE_JWT_SECRET not set");
            return axum::http::Response::builder()
                .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                .body("Server configuration error".into())
                .unwrap();
        }
    };

    let claims = match decode_jwt(&query.token, &secret) {
        Ok(claims) => claims,
        Err(e) => {
            error!("[WEBSOCKET] Invalid JWT token: {}", e);
            return axum::http::Response::builder()
                .status(axum::http::StatusCode::UNAUTHORIZED)
                .body("Invalid token".into())
                .unwrap();
        }
    };

    // Verify the account_id matches the token's subject
    if claims.sub != account_id {
        error!(
            "[WEBSOCKET] Account ID mismatch: token={}, path={}",
            claims.sub, account_id
        );
        return axum::http::Response::builder()
            .status(axum::http::StatusCode::FORBIDDEN)
            .body("Account mismatch".into())
            .unwrap();
    }

    let connection_id = format!("testing_{}_{}", account_id, flow_session_id);

    ws.on_upgrade(move |socket| {
        handle_workflow_testing_websocket(socket, connection_id, account_id, flow_session_id, state)
    })
}

async fn handle_websocket(
    socket: WebSocket,
    connection_id: String,
    account_id: String,
    state: Arc<AppState>,
) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // Add connection to manager
    state
        .websocket_manager
        .add_connection(account_id.clone(), connection_id.clone(), tx);

    // Spawn task to handle outgoing messages
    let connection_id_clone = connection_id.clone();
    let websocket_manager_clone = state.websocket_manager.clone();
    let outgoing_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if sender.send(message).await.is_err() {
                break;
            }
        }
        // Clean up connection when task ends
        websocket_manager_clone.remove_connection(&connection_id_clone);
    });

    // Handle incoming messages (mostly for keepalive)
    let connection_id_clone = connection_id.clone();
    let websocket_manager_clone = state.websocket_manager.clone();
    let incoming_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // Handle ping/pong or other client messages
                    if text == "ping" {
                        // Connection is alive, no action needed
                        continue;
                    }
                }
                Ok(Message::Close(_)) => {
                    info!(
                        "[WEBSOCKET] Connection {} closed by client",
                        connection_id_clone
                    );
                    break;
                }
                Err(e) => {
                    error!(
                        "[WEBSOCKET] WebSocket error for connection {}: {}",
                        connection_id_clone, e
                    );
                    break;
                }
                _ => {
                    // Ignore other message types
                }
            }
        }
        // Clean up connection when task ends
        websocket_manager_clone.remove_connection(&connection_id_clone);
    });

    // Wait for either task to complete
    tokio::select! {
        _ = outgoing_task => {},
        _ = incoming_task => {},
    }

    info!("[WEBSOCKET] WebSocket connection {} closed", connection_id);
}

async fn handle_workflow_testing_websocket(
    socket: WebSocket,
    connection_id: String,
    account_id: String,
    flow_session_id: String,
    state: Arc<AppState>,
) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // Add workflow testing connection to manager
    state.websocket_manager.add_workflow_testing_connection(
        account_id.clone(),
        connection_id.clone(),
        flow_session_id.clone(),
        tx,
    );

    // Send connection established message
    let connection_msg = WorkflowTestingUpdate {
        r#type: "connection_established".to_string(),
        update_type: None,
        flow_session_id: flow_session_id.clone(),
        data: Some(serde_json::json!({"message": "Connected to workflow testing session"})),
        tasks: None,
        complete: None,
    };

    if let Ok(json_msg) = serde_json::to_string(&connection_msg) {
        if let Err(e) = sender.send(Message::Text(json_msg)).await {
            error!(
                "[WEBSOCKET] Failed to send connection established message: {}",
                e
            );
            return;
        }
    }

    // TODO: Send initial session state with current tasks
    // This would require querying the database for existing tasks for this flow_session_id

    // Spawn task to handle outgoing messages
    let connection_id_clone = connection_id.clone();
    let websocket_manager_clone = state.websocket_manager.clone();
    let outgoing_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if sender.send(message).await.is_err() {
                break;
            }
        }
        // Clean up connection when task ends
        websocket_manager_clone.remove_connection(&connection_id_clone);
    });

    // Handle incoming messages (mostly for keepalive)
    let connection_id_clone = connection_id.clone();
    let websocket_manager_clone = state.websocket_manager.clone();
    let incoming_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // Handle ping/pong or other client messages
                    if text == "ping" {
                        // Connection is alive, no action needed
                        continue;
                    }
                }
                Ok(Message::Close(_)) => {
                    info!(
                        "[WEBSOCKET] Workflow testing connection {} closed by client",
                        connection_id_clone
                    );
                    break;
                }
                Err(e) => {
                    error!(
                        "[WEBSOCKET] WebSocket error for workflow testing connection {}: {}",
                        connection_id_clone, e
                    );
                    break;
                }
                _ => {
                    // Ignore other message types
                }
            }
        }
        // Clean up connection when task ends
        websocket_manager_clone.remove_connection(&connection_id_clone);
    });

    // Wait for either task to complete
    tokio::select! {
        _ = outgoing_task => {},
        _ = incoming_task => {},
    }

    info!(
        "[WEBSOCKET] Workflow testing WebSocket connection {} closed",
        connection_id
    );
}
