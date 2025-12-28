use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::middleware::AuthenticatedUser;
use crate::models::admin::{
    AdminUserResponse, AdjustResourcesRequest, BanUserRequest, PlayerDetailResponse,
    ServerStatsResponse, SetAdminRequest,
};
use crate::repositories::user_repo::UserRepository;
use crate::services::admin_service::AdminService;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    20
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

// GET /api/admin/users - List all users with pagination
pub async fn list_users(
    State(state): State<AppState>,
    Query(query): Query<ListUsersQuery>,
) -> AppResult<Json<Vec<AdminUserResponse>>> {
    let users = AdminService::list_users(&state.db, query.page, query.per_page).await?;
    Ok(Json(users))
}

// GET /api/admin/users/search - Search users
pub async fn search_users(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> AppResult<Json<Vec<AdminUserResponse>>> {
    let users = AdminService::search_users(&state.db, &query.q).await?;
    Ok(Json(users))
}

// GET /api/admin/users/:id - Get player detail
pub async fn get_player_detail(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> AppResult<Json<PlayerDetailResponse>> {
    let detail = AdminService::get_player_detail(&state.db, user_id).await?;
    Ok(Json(detail))
}

// POST /api/admin/users/:id/ban - Ban a user
pub async fn ban_user(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<BanUserRequest>,
) -> AppResult<Json<AdminUserResponse>> {
    let admin = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let user = AdminService::ban_user(&state.db, admin.id, user_id, body.reason).await?;

    info!("Admin {} banned user {}", admin.id, user_id);

    Ok(Json(user))
}

// POST /api/admin/users/:id/unban - Unban a user
pub async fn unban_user(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(user_id): Path<Uuid>,
) -> AppResult<Json<AdminUserResponse>> {
    let admin = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let user = AdminService::unban_user(&state.db, admin.id, user_id).await?;

    info!("Admin {} unbanned user {}", admin.id, user_id);

    Ok(Json(user))
}

// PUT /api/admin/users/:id/admin - Set admin status
pub async fn set_admin(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<SetAdminRequest>,
) -> AppResult<Json<AdminUserResponse>> {
    let admin = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let user = AdminService::set_admin(&state.db, admin.id, user_id, body.is_admin).await?;

    info!(
        "Admin {} {} admin status for user {}",
        admin.id,
        if body.is_admin { "granted" } else { "revoked" },
        user_id
    );

    Ok(Json(user))
}

// GET /api/admin/stats - Get server statistics
pub async fn get_server_stats(
    State(state): State<AppState>,
) -> AppResult<Json<ServerStatsResponse>> {
    let stats = AdminService::get_server_stats(&state.db).await?;
    Ok(Json(stats))
}

// POST /api/admin/villages/:id/resources - Adjust village resources
pub async fn adjust_resources(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(village_id): Path<Uuid>,
    Json(body): Json<AdjustResourcesRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let admin = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    AdminService::adjust_resources(
        &state.db,
        admin.id,
        village_id,
        body.wood,
        body.clay,
        body.iron,
        body.crop,
        &body.reason,
    )
    .await?;

    info!(
        "Admin {} adjusted resources for village {}: {:?}",
        admin.id, village_id, body
    );

    Ok(Json(serde_json::json!({
        "message": "Resources adjusted successfully"
    })))
}
