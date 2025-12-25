use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::message::{
    AllianceMessageListItem, ConversationResponse, MessageListItem, MessageResponse,
};
use crate::repositories::alliance_repo::AllianceRepository;
use crate::repositories::message_repo::MessageRepository;

pub struct MessageService;

impl MessageService {
    // ==================== Private Messages ====================

    /// Send a private message to another player
    pub async fn send_private_message(
        pool: &PgPool,
        sender_id: Uuid,
        recipient_id: Uuid,
        subject: String,
        body: String,
    ) -> AppResult<MessageResponse> {
        // Validate subject and body
        if subject.trim().is_empty() {
            return Err(AppError::BadRequest("Subject cannot be empty".into()));
        }
        if body.trim().is_empty() {
            return Err(AppError::BadRequest("Message body cannot be empty".into()));
        }
        if subject.len() > 200 {
            return Err(AppError::BadRequest(
                "Subject cannot exceed 200 characters".into(),
            ));
        }

        // Cannot send message to yourself
        if sender_id == recipient_id {
            return Err(AppError::BadRequest(
                "Cannot send message to yourself".into(),
            ));
        }

        // Get or create conversation
        let conversation =
            MessageRepository::get_or_create_conversation(pool, sender_id, recipient_id).await?;

        // Create the message
        let message = MessageRepository::create_private_message(
            pool,
            sender_id,
            recipient_id,
            conversation.id,
            &subject,
            &body,
        )
        .await?;

        // Update conversation's last message
        MessageRepository::update_conversation_last_message(pool, conversation.id, message.id)
            .await?;

        // Return full message with user names
        let response = MessageRepository::get_message(pool, message.id)
            .await?
            .ok_or_else(|| AppError::InternalError(anyhow::anyhow!("Failed to fetch created message")))?;

        Ok(response)
    }

    /// Get inbox messages
    pub async fn get_inbox(
        pool: &PgPool,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<MessageListItem>> {
        let limit = limit.min(50).max(1);
        MessageRepository::get_inbox(pool, user_id, limit, offset).await
    }

    /// Get sent messages
    pub async fn get_sent(
        pool: &PgPool,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<MessageListItem>> {
        let limit = limit.min(50).max(1);
        MessageRepository::get_sent(pool, user_id, limit, offset).await
    }

    /// Get a single message
    pub async fn get_message(
        pool: &PgPool,
        user_id: Uuid,
        message_id: Uuid,
    ) -> AppResult<MessageResponse> {
        // Check access
        if !MessageRepository::user_can_access(pool, message_id, user_id).await? {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        let message = MessageRepository::get_message(pool, message_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Message not found".into()))?;

        // Mark as read if user is recipient
        if message.recipient_id == Some(user_id) && !message.is_read {
            MessageRepository::mark_read(pool, message_id, user_id).await?;
        }

        Ok(message)
    }

    /// Delete a message for the current user
    pub async fn delete_message(pool: &PgPool, user_id: Uuid, message_id: Uuid) -> AppResult<()> {
        if !MessageRepository::delete_for_user(pool, message_id, user_id).await? {
            return Err(AppError::NotFound("Message not found".into()));
        }
        Ok(())
    }

    /// Get unread message count
    pub async fn get_unread_count(pool: &PgPool, user_id: Uuid) -> AppResult<i64> {
        MessageRepository::get_unread_count(pool, user_id).await
    }

    // ==================== Conversations ====================

    /// Get user's conversations
    pub async fn get_conversations(
        pool: &PgPool,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<ConversationResponse>> {
        let limit = limit.min(50).max(1);
        MessageRepository::get_conversations(pool, user_id, limit, offset).await
    }

    /// Get messages in a conversation
    pub async fn get_conversation_messages(
        pool: &PgPool,
        user_id: Uuid,
        conversation_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<MessageResponse>> {
        let limit = limit.min(100).max(1);
        MessageRepository::get_conversation_messages(pool, conversation_id, user_id, limit, offset)
            .await
    }

    /// Delete a conversation for the current user
    pub async fn delete_conversation(
        pool: &PgPool,
        user_id: Uuid,
        conversation_id: Uuid,
    ) -> AppResult<()> {
        if !MessageRepository::delete_conversation(pool, conversation_id, user_id).await? {
            return Err(AppError::NotFound("Conversation not found".into()));
        }
        Ok(())
    }

    // ==================== Alliance Messages ====================

    /// Send an alliance message
    pub async fn send_alliance_message(
        pool: &PgPool,
        sender_id: Uuid,
        subject: String,
        body: String,
    ) -> AppResult<MessageResponse> {
        // Validate subject and body
        if subject.trim().is_empty() {
            return Err(AppError::BadRequest("Subject cannot be empty".into()));
        }
        if body.trim().is_empty() {
            return Err(AppError::BadRequest("Message body cannot be empty".into()));
        }
        if subject.len() > 200 {
            return Err(AppError::BadRequest(
                "Subject cannot exceed 200 characters".into(),
            ));
        }

        // Check if user is in an alliance
        let member = AllianceRepository::get_user_alliance(pool, sender_id)
            .await?
            .ok_or_else(|| AppError::BadRequest("You are not in an alliance".into()))?;

        // Create the message
        let message = MessageRepository::create_alliance_message(
            pool,
            sender_id,
            member.alliance_id,
            &subject,
            &body,
        )
        .await?;

        // Return full message with user names
        let response = MessageRepository::get_message(pool, message.id)
            .await?
            .ok_or_else(|| AppError::InternalError(anyhow::anyhow!("Failed to fetch created message")))?;

        Ok(response)
    }

    /// Get alliance messages
    pub async fn get_alliance_messages(
        pool: &PgPool,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<AllianceMessageListItem>> {
        // Check if user is in an alliance
        let member = AllianceRepository::get_user_alliance(pool, user_id)
            .await?
            .ok_or_else(|| AppError::BadRequest("You are not in an alliance".into()))?;

        let limit = limit.min(50).max(1);
        MessageRepository::get_alliance_messages(pool, member.alliance_id, user_id, limit, offset)
            .await
    }

    /// Get a single alliance message
    pub async fn get_alliance_message(
        pool: &PgPool,
        user_id: Uuid,
        message_id: Uuid,
    ) -> AppResult<MessageResponse> {
        // Check if user is in an alliance
        let member = AllianceRepository::get_user_alliance(pool, user_id)
            .await?
            .ok_or_else(|| AppError::Forbidden("You are not in an alliance".into()))?;

        let message = MessageRepository::get_message(pool, message_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Message not found".into()))?;

        // Verify the message belongs to user's alliance
        if message.alliance_id != Some(member.alliance_id) {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        // Mark as read for this user
        MessageRepository::mark_alliance_message_read(pool, message_id, user_id).await?;

        Ok(message)
    }

    /// Get unread alliance message count
    pub async fn get_unread_alliance_count(pool: &PgPool, user_id: Uuid) -> AppResult<i64> {
        let member = match AllianceRepository::get_user_alliance(pool, user_id).await? {
            Some(m) => m,
            None => return Ok(0),
        };

        MessageRepository::get_unread_alliance_count(pool, member.alliance_id, user_id).await
    }

    /// Get total unread count (private + alliance)
    pub async fn get_total_unread_count(pool: &PgPool, user_id: Uuid) -> AppResult<i64> {
        let private_count = Self::get_unread_count(pool, user_id).await?;
        let alliance_count = Self::get_unread_alliance_count(pool, user_id).await?;
        Ok(private_count + alliance_count)
    }
}
