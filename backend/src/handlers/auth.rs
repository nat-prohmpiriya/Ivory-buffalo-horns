use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::error::AppResult;
use crate::middleware::AuthenticatedUser;
use crate::models::user::{CreateUser, UserResponse};
use crate::repositories::user_repo::UserRepository;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub user: Option<UserResponse>,
    pub firebase_user: FirebaseUserInfo,
}

#[derive(Debug, Serialize)]
pub struct FirebaseUserInfo {
    pub uid: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub provider: Option<String>,
}

// GET /api/auth/me - Get current user info
pub async fn me(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
) -> AppResult<Json<MeResponse>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid).await?;

    Ok(Json(MeResponse {
        user: user.map(|u| u.into()),
        firebase_user: FirebaseUserInfo {
            uid: auth_user.firebase_uid,
            email: auth_user.email,
            name: auth_user.name,
            picture: auth_user.picture,
            provider: auth_user.provider,
        },
    }))
}

#[derive(Debug, Deserialize)]
pub struct SyncUserRequest {
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SyncUserResponse {
    pub user: UserResponse,
    pub is_new: bool,
}

// POST /api/auth/sync - Sync Firebase user with database (upsert)
pub async fn sync_user(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Json(body): Json<SyncUserRequest>,
) -> AppResult<Json<SyncUserResponse>> {
    // Check if user exists
    let existing_user =
        UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid).await?;
    let is_new = existing_user.is_none();

    // Upsert user
    let create_user = CreateUser {
        firebase_uid: auth_user.firebase_uid.clone(),
        email: auth_user.email,
        display_name: body.display_name.or(auth_user.name),
        photo_url: auth_user.picture,
        provider: auth_user.provider.unwrap_or_else(|| "unknown".to_string()),
    };

    let user = UserRepository::upsert(&state.db, create_user).await?;

    if is_new {
        info!("New user registered: {}", user.firebase_uid);
    } else {
        info!("User synced: {}", user.firebase_uid);
    }

    Ok(Json(SyncUserResponse {
        user: user.into(),
        is_new,
    }))
}

// DELETE /api/auth/logout - Logout (optional: invalidate session in Redis)
pub async fn logout(
    Extension(auth_user): Extension<AuthenticatedUser>,
) -> AppResult<Json<serde_json::Value>> {
    info!("User logged out: {}", auth_user.firebase_uid);

    Ok(Json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub photo_url: Option<String>,
}

// PUT /api/auth/profile - Update user profile
pub async fn update_profile(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Json(body): Json<UpdateProfileRequest>,
) -> AppResult<Json<UserResponse>> {
    use crate::models::user::UpdateUser;

    let update_data = UpdateUser {
        email: None,
        display_name: body.display_name,
        photo_url: body.photo_url,
    };

    let user = UserRepository::update(&state.db, &auth_user.firebase_uid, update_data).await?;

    info!("User profile updated: {}", auth_user.firebase_uid);

    Ok(Json(user.into()))
}

// DELETE /api/auth/account - Soft delete user account
pub async fn delete_account(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
) -> AppResult<Json<serde_json::Value>> {
    UserRepository::soft_delete(&state.db, &auth_user.firebase_uid).await?;

    info!("User account deleted: {}", auth_user.firebase_uid);

    Ok(Json(serde_json::json!({
        "message": "Account deleted successfully"
    })))
}
