use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Admin action log
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AdminLog {
    pub id: Uuid,
    pub admin_id: Uuid,
    pub action: String,
    pub target_type: String,
    pub target_id: Option<Uuid>,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

// ==================== Request DTOs ====================

#[derive(Debug, Clone, Deserialize)]
pub struct BanUserRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AdjustResourcesRequest {
    pub wood: Option<i32>,
    pub clay: Option<i32>,
    pub iron: Option<i32>,
    pub crop: Option<i32>,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetAdminRequest {
    pub is_admin: bool,
}

// ==================== Response DTOs ====================

#[derive(Debug, Clone, Serialize)]
pub struct AdminUserResponse {
    pub id: Uuid,
    pub firebase_uid: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub photo_url: Option<String>,
    pub provider: String,
    pub is_admin: bool,
    pub banned_at: Option<DateTime<Utc>>,
    pub banned_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_login_at: DateTime<Utc>,
    pub village_count: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ServerStatsResponse {
    pub total_users: i64,
    pub active_users_24h: i64,
    pub banned_users: i64,
    pub total_villages: i64,
    pub total_alliances: i64,
    pub total_battles_today: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdminLogResponse {
    pub id: Uuid,
    pub admin_id: Uuid,
    pub admin_name: Option<String>,
    pub action: String,
    pub target_type: String,
    pub target_id: Option<Uuid>,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerDetailResponse {
    pub user: AdminUserResponse,
    pub villages: Vec<AdminVillageResponse>,
    pub heroes: Vec<AdminHeroResponse>,
    pub alliance: Option<AdminAllianceInfoResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdminVillageResponse {
    pub id: Uuid,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub is_capital: bool,
    pub wood: i32,
    pub clay: i32,
    pub iron: i32,
    pub crop: i32,
    pub population: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdminHeroResponse {
    pub id: Uuid,
    pub name: String,
    pub level: i32,
    pub health: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdminAllianceInfoResponse {
    pub id: Uuid,
    pub name: String,
    pub tag: String,
    pub role: String,
}
