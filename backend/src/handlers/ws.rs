use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::middleware::auth::FirebaseAuth;
use crate::repositories::user_repo::UserRepository;
use crate::services::ws_service::{WsEvent, WsManager};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct WsQuery {
    token: Option<String>,
}

/// WebSocket upgrade handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsQuery>,
    State(state): State<AppState>,
) -> Response {
    // Authenticate user from token
    let user_id = match authenticate_ws(&query, &state).await {
        Ok(id) => id,
        Err(e) => {
            warn!("WebSocket auth failed: {}", e);
            // Return upgrade anyway but will close immediately
            return ws.on_upgrade(|socket| async move {
                let _ = socket.close().await;
            });
        }
    };

    let ws_manager = state.ws.clone();
    ws.on_upgrade(move |socket| handle_socket(socket, user_id, ws_manager))
}

/// Authenticate WebSocket connection using Firebase token
async fn authenticate_ws(query: &WsQuery, state: &AppState) -> Result<Uuid, String> {
    let token = query
        .token
        .as_ref()
        .ok_or_else(|| "Missing token".to_string())?;

    let firebase_auth = FirebaseAuth::new(state.config.firebase.project_id.clone());

    let claims = firebase_auth
        .verify_token(token)
        .await
        .map_err(|e| format!("Invalid token: {:?}", e))?;

    // Get user from database
    let user = UserRepository::find_by_firebase_uid(&state.db, &claims.sub)
        .await
        .map_err(|e| format!("Database error: {:?}", e))?
        .ok_or_else(|| "User not found".to_string())?;

    Ok(user.id)
}

/// Handle WebSocket connection
async fn handle_socket(socket: WebSocket, user_id: Uuid, ws_manager: WsManager) {
    let (mut sender, mut receiver) = socket.split();

    // Register this connection
    let mut rx = ws_manager.register(user_id).await;

    // Send connected event
    let connected_event = WsEvent::Connected { user_id };
    if let Ok(json) = serde_json::to_string(&connected_event) {
        let _ = sender.send(Message::Text(json)).await;
    }

    // Spawn task to forward messages from manager to WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages from client
    let recv_task = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    debug!("Received from user {}: {}", user_id, text);
                    // Handle client messages if needed (e.g., ping, subscribe to specific events)
                    if let Ok(msg) = serde_json::from_str::<ClientMessage>(&text) {
                        match msg {
                            ClientMessage::Ping => {
                                debug!("Ping from user {}", user_id);
                            }
                            ClientMessage::Subscribe { event_type } => {
                                debug!("User {} subscribed to {}", user_id, event_type);
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("WebSocket closed by client: user_id={}", user_id);
                    break;
                }
                Ok(Message::Ping(_)) => {
                    debug!("Ping from user {}", user_id);
                }
                Ok(Message::Pong(_)) => {}
                Ok(Message::Binary(_)) => {}
                Err(e) => {
                    error!("WebSocket error for user {}: {}", user_id, e);
                    break;
                }
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = send_task => {
            debug!("Send task finished for user {}", user_id);
        }
        _ = recv_task => {
            debug!("Recv task finished for user {}", user_id);
        }
    }

    info!("WebSocket connection closed: user_id={}", user_id);
}

/// Client message types
#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClientMessage {
    Ping,
    Subscribe { event_type: String },
}
