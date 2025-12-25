use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::message::{
    AllianceMessageListItem, Conversation, ConversationResponse, Message, MessageListItem,
    MessageResponse, MessageType,
};

pub struct MessageRepository;

impl MessageRepository {
    // ==================== Messages ====================

    /// Create a new private message
    pub async fn create_private_message(
        pool: &PgPool,
        sender_id: Uuid,
        recipient_id: Uuid,
        conversation_id: Uuid,
        subject: &str,
        body: &str,
    ) -> AppResult<Message> {
        let message = sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO messages (message_type, sender_id, recipient_id, conversation_id, subject, body)
            VALUES ('private', $1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(sender_id)
        .bind(recipient_id)
        .bind(conversation_id)
        .bind(subject)
        .bind(body)
        .fetch_one(pool)
        .await?;

        Ok(message)
    }

    /// Create an alliance message
    pub async fn create_alliance_message(
        pool: &PgPool,
        sender_id: Uuid,
        alliance_id: Uuid,
        subject: &str,
        body: &str,
    ) -> AppResult<Message> {
        let message = sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO messages (message_type, sender_id, alliance_id, subject, body)
            VALUES ('alliance', $1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(sender_id)
        .bind(alliance_id)
        .bind(subject)
        .bind(body)
        .fetch_one(pool)
        .await?;

        Ok(message)
    }

    /// Get message by ID with full details
    pub async fn get_message(pool: &PgPool, message_id: Uuid) -> AppResult<Option<MessageResponse>> {
        let message = sqlx::query_as::<_, MessageResponse>(
            r#"
            SELECT
                m.id,
                m.message_type,
                m.sender_id,
                sender.display_name as sender_name,
                m.recipient_id,
                recipient.display_name as recipient_name,
                m.alliance_id,
                a.name as alliance_name,
                m.subject,
                m.body,
                m.is_read,
                m.created_at
            FROM messages m
            JOIN users sender ON sender.id = m.sender_id
            LEFT JOIN users recipient ON recipient.id = m.recipient_id
            LEFT JOIN alliances a ON a.id = m.alliance_id
            WHERE m.id = $1
            "#,
        )
        .bind(message_id)
        .fetch_optional(pool)
        .await?;

        Ok(message)
    }

    /// Get inbox (received private messages)
    pub async fn get_inbox(
        pool: &PgPool,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<MessageListItem>> {
        let messages = sqlx::query_as::<_, MessageListItem>(
            r#"
            SELECT
                m.id,
                m.sender_id,
                sender.display_name as sender_name,
                m.subject,
                m.is_read,
                m.created_at
            FROM messages m
            JOIN users sender ON sender.id = m.sender_id
            WHERE m.message_type = 'private'
                AND m.recipient_id = $1
                AND m.recipient_deleted = FALSE
            ORDER BY m.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(messages)
    }

    /// Get sent messages
    pub async fn get_sent(
        pool: &PgPool,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<MessageListItem>> {
        let messages = sqlx::query_as::<_, MessageListItem>(
            r#"
            SELECT
                m.id,
                m.sender_id,
                recipient.display_name as sender_name,
                m.subject,
                m.is_read,
                m.created_at
            FROM messages m
            LEFT JOIN users recipient ON recipient.id = m.recipient_id
            WHERE m.message_type = 'private'
                AND m.sender_id = $1
                AND m.sender_deleted = FALSE
            ORDER BY m.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(messages)
    }

    /// Get alliance messages
    pub async fn get_alliance_messages(
        pool: &PgPool,
        alliance_id: Uuid,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<AllianceMessageListItem>> {
        let messages = sqlx::query_as::<_, AllianceMessageListItem>(
            r#"
            SELECT
                m.id,
                m.sender_id,
                sender.display_name as sender_name,
                m.subject,
                CASE WHEN mr.id IS NOT NULL THEN TRUE ELSE FALSE END as is_read,
                m.created_at
            FROM messages m
            JOIN users sender ON sender.id = m.sender_id
            LEFT JOIN message_reads mr ON mr.message_id = m.id AND mr.user_id = $2
            WHERE m.message_type = 'alliance'
                AND m.alliance_id = $1
            ORDER BY m.created_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(alliance_id)
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(messages)
    }

    /// Mark private message as read
    pub async fn mark_read(pool: &PgPool, message_id: Uuid, user_id: Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE messages
            SET is_read = TRUE
            WHERE id = $1 AND recipient_id = $2
            "#,
        )
        .bind(message_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Mark alliance message as read (insert into message_reads)
    pub async fn mark_alliance_message_read(
        pool: &PgPool,
        message_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO message_reads (message_id, user_id)
            VALUES ($1, $2)
            ON CONFLICT (message_id, user_id) DO NOTHING
            "#,
        )
        .bind(message_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Delete message for user (soft delete)
    pub async fn delete_for_user(
        pool: &PgPool,
        message_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<bool> {
        // Check if user is sender or recipient
        let result = sqlx::query(
            r#"
            UPDATE messages
            SET
                sender_deleted = CASE WHEN sender_id = $2 THEN TRUE ELSE sender_deleted END,
                recipient_deleted = CASE WHEN recipient_id = $2 THEN TRUE ELSE recipient_deleted END
            WHERE id = $1 AND (sender_id = $2 OR recipient_id = $2)
            "#,
        )
        .bind(message_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Get unread private message count
    pub async fn get_unread_count(pool: &PgPool, user_id: Uuid) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM messages
            WHERE message_type = 'private'
                AND recipient_id = $1
                AND recipient_deleted = FALSE
                AND is_read = FALSE
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    /// Get unread alliance message count
    pub async fn get_unread_alliance_count(
        pool: &PgPool,
        alliance_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM messages m
            LEFT JOIN message_reads mr ON mr.message_id = m.id AND mr.user_id = $2
            WHERE m.message_type = 'alliance'
                AND m.alliance_id = $1
                AND mr.id IS NULL
            "#,
        )
        .bind(alliance_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    // ==================== Conversations ====================

    /// Get or create a conversation between two users
    pub async fn get_or_create_conversation(
        pool: &PgPool,
        user_a: Uuid,
        user_b: Uuid,
    ) -> AppResult<Conversation> {
        // Ensure user_1 < user_2 for unique constraint
        let (user_1, user_2) = if user_a < user_b {
            (user_a, user_b)
        } else {
            (user_b, user_a)
        };

        let conversation = sqlx::query_as::<_, Conversation>(
            r#"
            INSERT INTO conversations (user_1_id, user_2_id)
            VALUES ($1, $2)
            ON CONFLICT (user_1_id, user_2_id) DO UPDATE
                SET user_1_deleted = CASE WHEN conversations.user_1_id = $1 THEN FALSE ELSE conversations.user_1_deleted END,
                    user_2_deleted = CASE WHEN conversations.user_2_id = $1 THEN FALSE ELSE conversations.user_2_deleted END
            RETURNING *
            "#,
        )
        .bind(user_1)
        .bind(user_2)
        .fetch_one(pool)
        .await?;

        Ok(conversation)
    }

    /// Update conversation last message
    pub async fn update_conversation_last_message(
        pool: &PgPool,
        conversation_id: Uuid,
        message_id: Uuid,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE conversations
            SET last_message_id = $2, last_message_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(conversation_id)
        .bind(message_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Get user's conversations
    pub async fn get_conversations(
        pool: &PgPool,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<ConversationResponse>> {
        let conversations = sqlx::query_as::<_, ConversationResponse>(
            r#"
            SELECT
                c.id,
                CASE WHEN c.user_1_id = $1 THEN c.user_2_id ELSE c.user_1_id END as other_user_id,
                CASE WHEN c.user_1_id = $1 THEN u2.display_name ELSE u1.display_name END as other_user_name,
                m.subject as last_message_subject,
                LEFT(m.body, 100) as last_message_preview,
                c.last_message_at,
                (
                    SELECT COUNT(*)
                    FROM messages msg
                    WHERE msg.conversation_id = c.id
                        AND msg.recipient_id = $1
                        AND msg.is_read = FALSE
                ) as unread_count
            FROM conversations c
            JOIN users u1 ON u1.id = c.user_1_id
            JOIN users u2 ON u2.id = c.user_2_id
            LEFT JOIN messages m ON m.id = c.last_message_id
            WHERE (c.user_1_id = $1 AND c.user_1_deleted = FALSE)
               OR (c.user_2_id = $1 AND c.user_2_deleted = FALSE)
            ORDER BY c.last_message_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(conversations)
    }

    /// Get messages in a conversation
    pub async fn get_conversation_messages(
        pool: &PgPool,
        conversation_id: Uuid,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<MessageResponse>> {
        let messages = sqlx::query_as::<_, MessageResponse>(
            r#"
            SELECT
                m.id,
                m.message_type,
                m.sender_id,
                sender.display_name as sender_name,
                m.recipient_id,
                recipient.display_name as recipient_name,
                m.alliance_id,
                NULL::VARCHAR as alliance_name,
                m.subject,
                m.body,
                m.is_read,
                m.created_at
            FROM messages m
            JOIN users sender ON sender.id = m.sender_id
            LEFT JOIN users recipient ON recipient.id = m.recipient_id
            WHERE m.conversation_id = $1
                AND (
                    (m.sender_id = $2 AND m.sender_deleted = FALSE)
                    OR (m.recipient_id = $2 AND m.recipient_deleted = FALSE)
                )
            ORDER BY m.created_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(conversation_id)
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(messages)
    }

    /// Delete conversation for user
    pub async fn delete_conversation(
        pool: &PgPool,
        conversation_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<bool> {
        let result = sqlx::query(
            r#"
            UPDATE conversations
            SET
                user_1_deleted = CASE WHEN user_1_id = $2 THEN TRUE ELSE user_1_deleted END,
                user_2_deleted = CASE WHEN user_2_id = $2 THEN TRUE ELSE user_2_deleted END
            WHERE id = $1 AND (user_1_id = $2 OR user_2_id = $2)
            "#,
        )
        .bind(conversation_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Check if user owns the message (sender or recipient)
    pub async fn user_can_access(pool: &PgPool, message_id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let result: Option<(Uuid, MessageType, Option<Uuid>, Option<Uuid>)> = sqlx::query_as(
            r#"
            SELECT id, message_type, sender_id, recipient_id
            FROM messages
            WHERE id = $1
            "#,
        )
        .bind(message_id)
        .fetch_optional(pool)
        .await?;

        if let Some((_id, msg_type, sender_id, recipient_id)) = result {
            match msg_type {
                MessageType::Private => {
                    Ok(sender_id == Some(user_id) || recipient_id == Some(user_id))
                }
                MessageType::Alliance => {
                    // For alliance messages, check membership via service
                    Ok(true) // Will be validated in service
                }
            }
        } else {
            Ok(false)
        }
    }
}
