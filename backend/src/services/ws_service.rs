use axum::extract::ws::Message;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info};
use uuid::Uuid;

/// Message types for WebSocket events
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum WsEvent {
    VillageUpdated(VillageUpdateData),
    ResourcesUpdated(ResourcesUpdateData),
    BuildingComplete(BuildingCompleteData),
    ArmyArrived(ArmyArrivedData),
    AttackIncoming(AttackIncomingData),
    TroopTrainingComplete(TroopTrainingCompleteData),
    TroopsStarved(TroopsStarvedData),
    TradeOrderExpired(TradeOrderExpiredData),
    Connected { user_id: Uuid },
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct VillageUpdateData {
    pub village_id: Uuid,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ResourcesUpdateData {
    pub village_id: Uuid,
    pub wood: i64,
    pub clay: i64,
    pub iron: i64,
    pub wheat: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BuildingCompleteData {
    pub village_id: Uuid,
    pub building_type: String,
    pub slot: i32,
    pub level: i32,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArmyArrivedData {
    pub army_id: Uuid,
    pub village_id: Uuid,
    pub mission_type: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AttackIncomingData {
    pub target_village_id: Uuid,
    pub arrival_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TroopTrainingCompleteData {
    pub village_id: Uuid,
    pub troop_type: String,
    pub quantity: i32,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TroopsStarvedData {
    pub village_id: Uuid,
    pub troop_type: String,
    pub quantity: i32,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TradeOrderExpiredData {
    pub order_id: Uuid,
    pub order_type: String,
    pub resource_type: String,
    pub quantity_remaining: i32,
    pub refunded_gold: Option<i32>,
}

/// Connection info for a single WebSocket connection
struct Connection {
    sender: mpsc::UnboundedSender<Message>,
}

/// WebSocket connection manager
/// Manages all active WebSocket connections and handles broadcasting
#[derive(Clone)]
pub struct WsManager {
    /// Map of user_id -> list of connections (user can have multiple tabs)
    connections: Arc<RwLock<HashMap<Uuid, Vec<Connection>>>>,
}

impl WsManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new connection for a user
    pub async fn register(&self, user_id: Uuid) -> mpsc::UnboundedReceiver<Message> {
        let (tx, rx) = mpsc::unbounded_channel();

        let mut connections = self.connections.write().await;
        let user_connections = connections.entry(user_id).or_insert_with(Vec::new);
        user_connections.push(Connection { sender: tx });

        info!("WebSocket connected: user_id={}, total_connections={}", user_id, user_connections.len());

        rx
    }

    /// Remove a connection for a user
    pub async fn unregister(&self, user_id: Uuid, connection_index: usize) {
        let mut connections = self.connections.write().await;

        if let Some(user_connections) = connections.get_mut(&user_id) {
            if connection_index < user_connections.len() {
                user_connections.remove(connection_index);
                info!("WebSocket disconnected: user_id={}, remaining={}", user_id, user_connections.len());
            }

            if user_connections.is_empty() {
                connections.remove(&user_id);
            }
        }
    }

    /// Send event to a specific user (all their connections)
    pub async fn send_to_user(&self, user_id: Uuid, event: &WsEvent) {
        let message = match serde_json::to_string(event) {
            Ok(json) => Message::Text(json),
            Err(e) => {
                error!("Failed to serialize WsEvent: {}", e);
                return;
            }
        };

        let connections = self.connections.read().await;

        if let Some(user_connections) = connections.get(&user_id) {
            for conn in user_connections {
                if let Err(e) = conn.sender.send(message.clone()) {
                    debug!("Failed to send message to user {}: {}", user_id, e);
                }
            }
        }
    }

    /// Send event to multiple users
    pub async fn send_to_users(&self, user_ids: &[Uuid], event: &WsEvent) {
        for user_id in user_ids {
            self.send_to_user(*user_id, event).await;
        }
    }

    /// Broadcast event to all connected users
    pub async fn broadcast(&self, event: &WsEvent) {
        let message = match serde_json::to_string(event) {
            Ok(json) => Message::Text(json),
            Err(e) => {
                error!("Failed to serialize WsEvent: {}", e);
                return;
            }
        };

        let connections = self.connections.read().await;

        for (user_id, user_connections) in connections.iter() {
            for conn in user_connections {
                if let Err(e) = conn.sender.send(message.clone()) {
                    debug!("Failed to broadcast to user {}: {}", user_id, e);
                }
            }
        }
    }

    /// Get count of connected users
    pub async fn connected_users_count(&self) -> usize {
        self.connections.read().await.len()
    }

    /// Get total connection count
    pub async fn total_connections_count(&self) -> usize {
        let connections = self.connections.read().await;
        connections.values().map(|v| v.len()).sum()
    }
}

impl Default for WsManager {
    fn default() -> Self {
        Self::new()
    }
}
