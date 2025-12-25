use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ==================== Enums ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "message_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Private,
    Alliance,
}

// ==================== Database Models ====================

#[derive(Debug, Clone, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub message_type: MessageType,
    pub sender_id: Uuid,
    pub recipient_id: Option<Uuid>,
    pub alliance_id: Option<Uuid>,
    pub conversation_id: Option<Uuid>,
    pub subject: String,
    pub body: String,
    pub is_read: bool,
    pub sender_deleted: bool,
    pub recipient_deleted: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct MessageRead {
    pub id: Uuid,
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub read_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub user_1_id: Uuid,
    pub user_2_id: Uuid,
    pub last_message_id: Option<Uuid>,
    pub last_message_at: DateTime<Utc>,
    pub user_1_deleted: bool,
    pub user_2_deleted: bool,
    pub created_at: DateTime<Utc>,
}

// ==================== Request DTOs ====================

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub recipient_id: Uuid,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct SendAllianceMessageRequest {
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct ReplyMessageRequest {
    pub body: String,
}

// ==================== Response DTOs ====================

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct MessageResponse {
    pub id: Uuid,
    pub message_type: MessageType,
    pub sender_id: Uuid,
    pub sender_name: String,
    pub recipient_id: Option<Uuid>,
    pub recipient_name: Option<String>,
    pub alliance_id: Option<Uuid>,
    pub alliance_name: Option<String>,
    pub subject: String,
    pub body: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct ConversationResponse {
    pub id: Uuid,
    pub other_user_id: Uuid,
    pub other_user_name: String,
    pub last_message_subject: Option<String>,
    pub last_message_preview: Option<String>,
    pub last_message_at: DateTime<Utc>,
    pub unread_count: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct MessageListItem {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub sender_name: String,
    pub subject: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AllianceMessageListItem {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub sender_name: String,
    pub subject: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

impl From<Message> for MessageResponse {
    fn from(m: Message) -> Self {
        Self {
            id: m.id,
            message_type: m.message_type,
            sender_id: m.sender_id,
            sender_name: String::new(), // Will be populated by service
            recipient_id: m.recipient_id,
            recipient_name: None,
            alliance_id: m.alliance_id,
            alliance_name: None,
            subject: m.subject,
            body: m.body,
            is_read: m.is_read,
            created_at: m.created_at,
        }
    }
}
