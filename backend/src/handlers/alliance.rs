use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppResult;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::alliance::{
    AllianceDiplomacy, AllianceInvitation, AllianceListItem, AllianceMemberResponse,
    AllianceResponse, CreateAllianceRequest, InvitePlayerRequest, RespondInvitationRequest,
    SetDiplomacyRequest, UpdateAllianceRequest, UpdateMemberRoleRequest,
};
use crate::repositories::user_repo::UserRepository;
use crate::services::alliance_service::AllianceService;
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

// ==================== Alliance CRUD ====================

/// POST /api/alliances - Create new alliance
pub async fn create_alliance(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<CreateAllianceRequest>,
) -> AppResult<Json<AllianceResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or_else(|| crate::error::AppError::Unauthorized)?;

    let alliance = AllianceService::create_alliance(&state.db, db_user.id, request).await?;
    Ok(Json(alliance))
}

/// GET /api/alliances - List all alliances
pub async fn list_alliances(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> AppResult<Json<Vec<AllianceListItem>>> {
    let alliances = AllianceService::list_alliances(&state.db, query.limit, query.offset).await?;
    Ok(Json(alliances))
}

/// GET /api/alliances/my - Get current user's alliance
pub async fn get_my_alliance(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<Option<AllianceResponse>>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or_else(|| crate::error::AppError::Unauthorized)?;

    let alliance = AllianceService::get_my_alliance(&state.db, db_user.id).await?;
    Ok(Json(alliance))
}

/// GET /api/alliances/:id - Get alliance by ID
pub async fn get_alliance(
    State(state): State<AppState>,
    Path(alliance_id): Path<Uuid>,
) -> AppResult<Json<AllianceResponse>> {
    let alliance = AllianceService::get_alliance(&state.db, alliance_id).await?;
    Ok(Json(alliance))
}

/// PUT /api/alliances/:id - Update alliance
pub async fn update_alliance(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(alliance_id): Path<Uuid>,
    Json(request): Json<UpdateAllianceRequest>,
) -> AppResult<Json<AllianceResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or_else(|| crate::error::AppError::Unauthorized)?;

    let alliance = AllianceService::update_alliance(
        &state.db,
        db_user.id,
        alliance_id,
        request.name,
        request.description,
    )
    .await?;
    Ok(Json(alliance))
}

/// DELETE /api/alliances/:id - Disband alliance
pub async fn disband_alliance(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(alliance_id): Path<Uuid>,
) -> AppResult<Json<()>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or_else(|| crate::error::AppError::Unauthorized)?;

    AllianceService::disband_alliance(&state.db, db_user.id, alliance_id).await?;
    Ok(Json(()))
}

// ==================== Members ====================

/// GET /api/alliances/:id/members - List alliance members
pub async fn list_members(
    State(state): State<AppState>,
    Path(alliance_id): Path<Uuid>,
) -> AppResult<Json<Vec<AllianceMemberResponse>>> {
    let members = AllianceService::list_members(&state.db, alliance_id).await?;
    Ok(Json(members))
}

/// POST /api/alliances/:id/invite - Invite player
pub async fn invite_player(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(alliance_id): Path<Uuid>,
    Json(request): Json<InvitePlayerRequest>,
) -> AppResult<Json<AllianceInvitation>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or_else(|| crate::error::AppError::Unauthorized)?;

    let invitation = AllianceService::invite_player(
        &state.db,
        db_user.id,
        alliance_id,
        request.player_id,
        request.message,
    )
    .await?;
    Ok(Json(invitation))
}

/// POST /api/alliances/leave - Leave current alliance
pub async fn leave_alliance(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<()>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or_else(|| crate::error::AppError::Unauthorized)?;

    AllianceService::leave_alliance(&state.db, db_user.id).await?;
    Ok(Json(()))
}

/// DELETE /api/alliances/:id/members/:user_id - Kick member
pub async fn kick_member(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((alliance_id, target_user_id)): Path<(Uuid, Uuid)>,
) -> AppResult<Json<()>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or_else(|| crate::error::AppError::Unauthorized)?;

    // Verify user is in this alliance before kicking
    let _ = AllianceService::get_alliance(&state.db, alliance_id).await?;
    AllianceService::kick_member(&state.db, db_user.id, target_user_id).await?;
    Ok(Json(()))
}

/// PUT /api/alliances/:id/members/:user_id/role - Update member role
pub async fn update_member_role(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((alliance_id, target_user_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateMemberRoleRequest>,
) -> AppResult<Json<()>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or_else(|| crate::error::AppError::Unauthorized)?;

    let _ = AllianceService::get_alliance(&state.db, alliance_id).await?;
    AllianceService::update_member_role(&state.db, db_user.id, target_user_id, request.role).await?;
    Ok(Json(()))
}

// ==================== Invitations ====================

/// GET /api/alliances/invitations - Get pending invitations for current user
pub async fn get_invitations(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<Vec<AllianceInvitation>>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or_else(|| crate::error::AppError::Unauthorized)?;

    let invitations = AllianceService::get_pending_invitations(&state.db, db_user.id).await?;
    Ok(Json(invitations))
}

/// POST /api/alliances/invitations/:id/respond - Accept or reject invitation
pub async fn respond_invitation(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(invitation_id): Path<Uuid>,
    Json(request): Json<RespondInvitationRequest>,
) -> AppResult<Json<()>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or_else(|| crate::error::AppError::Unauthorized)?;

    AllianceService::respond_invitation(&state.db, db_user.id, invitation_id, request.accept).await?;
    Ok(Json(()))
}

// ==================== Diplomacy ====================

/// GET /api/alliances/:id/diplomacy - List diplomacy relations
pub async fn list_diplomacy(
    State(state): State<AppState>,
    Path(alliance_id): Path<Uuid>,
) -> AppResult<Json<Vec<AllianceDiplomacy>>> {
    let diplomacy = AllianceService::list_diplomacy(&state.db, alliance_id).await?;
    Ok(Json(diplomacy))
}

/// POST /api/alliances/:id/diplomacy - Set diplomacy with another alliance
pub async fn set_diplomacy(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(alliance_id): Path<Uuid>,
    Json(request): Json<SetDiplomacyRequest>,
) -> AppResult<Json<AllianceDiplomacy>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or_else(|| crate::error::AppError::Unauthorized)?;

    // Verify user is in this alliance
    let _ = AllianceService::get_alliance(&state.db, alliance_id).await?;
    let diplomacy = AllianceService::set_diplomacy(
        &state.db,
        db_user.id,
        request.target_alliance_id,
        request.status,
    )
    .await?;
    Ok(Json(diplomacy))
}
