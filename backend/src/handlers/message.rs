use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::middleware::auth::AuthenticatedUser;
use crate::models::message::{
    AllianceMessageListItem, ConversationResponse, MessageListItem, MessageResponse,
    ReplyMessageRequest, SendAllianceMessageRequest, SendMessageRequest,
};
use crate::repositories::user_repo::UserRepository;
use crate::services::message_service::MessageService;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_limit")]
    pub limit: i32,
    #[serde(default)]
    pub offset: i32,
}

fn default_limit() -> i32 {
    20
}

// ==================== Private Messages ====================

/// POST /api/messages - Send a private message
pub async fn send_message(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<SendMessageRequest>,
) -> AppResult<Json<MessageResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let message = MessageService::send_private_message(
        &state.db,
        db_user.id,
        request.recipient_id,
        request.subject,
        request.body,
    )
    .await?;

    Ok(Json(message))
}

/// GET /api/messages/inbox - Get inbox messages
pub async fn get_inbox(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Query(query): Query<PaginationQuery>,
) -> AppResult<Json<Vec<MessageListItem>>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let messages = MessageService::get_inbox(&state.db, db_user.id, query.limit, query.offset).await?;

    Ok(Json(messages))
}

/// GET /api/messages/sent - Get sent messages
pub async fn get_sent(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Query(query): Query<PaginationQuery>,
) -> AppResult<Json<Vec<MessageListItem>>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let messages = MessageService::get_sent(&state.db, db_user.id, query.limit, query.offset).await?;

    Ok(Json(messages))
}

/// GET /api/messages/:id - Get a single message
pub async fn get_message(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(message_id): Path<Uuid>,
) -> AppResult<Json<MessageResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let message = MessageService::get_message(&state.db, db_user.id, message_id).await?;

    Ok(Json(message))
}

/// DELETE /api/messages/:id - Delete a message
pub async fn delete_message(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(message_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    MessageService::delete_message(&state.db, db_user.id, message_id).await?;

    Ok(Json(serde_json::json!({
        "message": "Message deleted"
    })))
}

/// GET /api/messages/unread-count - Get unread message count
pub async fn get_unread_count(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<serde_json::Value>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let count = MessageService::get_total_unread_count(&state.db, db_user.id).await?;

    Ok(Json(serde_json::json!({
        "unread_count": count
    })))
}

// ==================== Conversations ====================

/// GET /api/conversations - Get user's conversations
pub async fn get_conversations(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Query(query): Query<PaginationQuery>,
) -> AppResult<Json<Vec<ConversationResponse>>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let conversations =
        MessageService::get_conversations(&state.db, db_user.id, query.limit, query.offset).await?;

    Ok(Json(conversations))
}

/// GET /api/conversations/:id/messages - Get messages in a conversation
pub async fn get_conversation_messages(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(conversation_id): Path<Uuid>,
    Query(query): Query<PaginationQuery>,
) -> AppResult<Json<Vec<MessageResponse>>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let messages = MessageService::get_conversation_messages(
        &state.db,
        db_user.id,
        conversation_id,
        query.limit,
        query.offset,
    )
    .await?;

    Ok(Json(messages))
}

/// POST /api/conversations/:id/reply - Reply to a conversation
pub async fn reply_to_conversation(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(conversation_id): Path<Uuid>,
    Json(request): Json<ReplyMessageRequest>,
) -> AppResult<Json<MessageResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // Get the conversation to find the other user
    let conversations =
        MessageService::get_conversations(&state.db, db_user.id, 100, 0).await?;

    let conversation = conversations
        .into_iter()
        .find(|c| c.id == conversation_id)
        .ok_or_else(|| AppError::NotFound("Conversation not found".into()))?;

    // Send reply
    let message = MessageService::send_private_message(
        &state.db,
        db_user.id,
        conversation.other_user_id,
        format!("Re: {}", conversation.last_message_subject.unwrap_or_default()),
        request.body,
    )
    .await?;

    Ok(Json(message))
}

/// DELETE /api/conversations/:id - Delete a conversation
pub async fn delete_conversation(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(conversation_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    MessageService::delete_conversation(&state.db, db_user.id, conversation_id).await?;

    Ok(Json(serde_json::json!({
        "message": "Conversation deleted"
    })))
}

// ==================== Alliance Messages ====================

/// POST /api/alliance-messages - Send an alliance message
pub async fn send_alliance_message(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<SendAllianceMessageRequest>,
) -> AppResult<Json<MessageResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let message = MessageService::send_alliance_message(
        &state.db,
        db_user.id,
        request.subject,
        request.body,
    )
    .await?;

    Ok(Json(message))
}

/// GET /api/alliance-messages - Get alliance messages
pub async fn get_alliance_messages(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Query(query): Query<PaginationQuery>,
) -> AppResult<Json<Vec<AllianceMessageListItem>>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let messages =
        MessageService::get_alliance_messages(&state.db, db_user.id, query.limit, query.offset)
            .await?;

    Ok(Json(messages))
}

/// GET /api/alliance-messages/:id - Get a single alliance message
pub async fn get_alliance_message(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(message_id): Path<Uuid>,
) -> AppResult<Json<MessageResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let message = MessageService::get_alliance_message(&state.db, db_user.id, message_id).await?;

    Ok(Json(message))
}
