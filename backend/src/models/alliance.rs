use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ==================== Enums ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "alliance_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AllianceRole {
    Leader,
    Officer,
    Member,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "invitation_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "diplomacy_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum DiplomacyStatus {
    Neutral,
    Ally,
    Nap,
    Enemy,
}

// ==================== Database Models ====================

#[derive(Debug, Clone, FromRow)]
pub struct Alliance {
    pub id: Uuid,
    pub name: String,
    pub tag: String,
    pub description: Option<String>,
    pub founder_id: Uuid,
    pub leader_id: Uuid,
    pub max_members: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct AllianceMember {
    pub id: Uuid,
    pub alliance_id: Uuid,
    pub user_id: Uuid,
    pub role: AllianceRole,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AllianceInvitation {
    pub id: Uuid,
    pub alliance_id: Uuid,
    pub inviter_id: Uuid,
    pub invitee_id: Uuid,
    pub status: InvitationStatus,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub responded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AllianceDiplomacy {
    pub id: Uuid,
    pub alliance_id: Uuid,
    pub target_alliance_id: Uuid,
    pub status: DiplomacyStatus,
    pub proposed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ==================== Request DTOs ====================

#[derive(Debug, Deserialize)]
pub struct CreateAllianceRequest {
    pub name: String,
    pub tag: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAllianceRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InvitePlayerRequest {
    pub player_id: Uuid,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RespondInvitationRequest {
    pub accept: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRoleRequest {
    pub role: AllianceRole,
}

#[derive(Debug, Deserialize)]
pub struct SetDiplomacyRequest {
    pub target_alliance_id: Uuid,
    pub status: DiplomacyStatus,
}

// ==================== Response DTOs ====================

#[derive(Debug, Clone, Serialize)]
pub struct AllianceResponse {
    pub id: Uuid,
    pub name: String,
    pub tag: String,
    pub description: Option<String>,
    pub founder_id: Uuid,
    pub leader_id: Uuid,
    pub max_members: i32,
    pub member_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AllianceMemberResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub player_name: String,
    pub role: AllianceRole,
    pub villages_count: i32,
    pub population: i32,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AllianceInvitationResponse {
    pub id: Uuid,
    pub alliance_id: Uuid,
    pub alliance_name: String,
    pub alliance_tag: String,
    pub inviter_name: String,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AllianceDiplomacyResponse {
    pub id: Uuid,
    pub target_alliance_id: Uuid,
    pub target_alliance_name: String,
    pub target_alliance_tag: String,
    pub status: DiplomacyStatus,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AllianceListItem {
    pub id: Uuid,
    pub name: String,
    pub tag: String,
    pub member_count: i32,
    pub total_population: i64,
}

impl From<Alliance> for AllianceResponse {
    fn from(a: Alliance) -> Self {
        Self {
            id: a.id,
            name: a.name,
            tag: a.tag,
            description: a.description,
            founder_id: a.founder_id,
            leader_id: a.leader_id,
            max_members: a.max_members,
            member_count: 0, // Will be populated by service
            created_at: a.created_at,
        }
    }
}
