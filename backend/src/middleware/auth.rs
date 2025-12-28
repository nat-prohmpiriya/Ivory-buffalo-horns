use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error};

use crate::error::AppError;
use crate::AppState;

// Firebase public keys cache
static FIREBASE_KEYS_URL: &str =
    "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";

#[derive(Clone)]
pub struct FirebaseAuth {
    project_id: String,
    http_client: Client,
    keys_cache: Arc<RwLock<HashMap<String, DecodingKey>>>,
}

impl std::fmt::Debug for FirebaseAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FirebaseAuth")
            .field("project_id", &self.project_id)
            .finish()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirebaseClaims {
    pub sub: String,           // Firebase UID
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub iss: String,
    pub aud: String,
    pub auth_time: i64,
    pub iat: i64,
    pub exp: i64,
    pub firebase: Option<FirebaseIdentity>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirebaseIdentity {
    pub sign_in_provider: Option<String>,
    pub identities: Option<serde_json::Value>,
}

impl FirebaseAuth {
    pub fn new(project_id: String) -> Self {
        Self {
            project_id,
            http_client: Client::new(),
            keys_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn fetch_public_keys(&self) -> Result<HashMap<String, String>, AppError> {
        let response = self
            .http_client
            .get(FIREBASE_KEYS_URL)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to fetch Firebase public keys: {}", e);
                AppError::InternalError(anyhow::anyhow!("Failed to fetch Firebase keys"))
            })?;

        let keys: HashMap<String, String> = response.json().await.map_err(|e| {
            error!("Failed to parse Firebase public keys: {}", e);
            AppError::InternalError(anyhow::anyhow!("Failed to parse Firebase keys"))
        })?;

        Ok(keys)
    }

    async fn get_decoding_key(&self, kid: &str) -> Result<DecodingKey, AppError> {
        // Check cache first
        {
            let cache = self.keys_cache.read().await;
            if let Some(key) = cache.get(kid) {
                return Ok(key.clone());
            }
        }

        // Fetch new keys
        let keys = self.fetch_public_keys().await?;

        // Update cache
        let mut cache = self.keys_cache.write().await;
        for (key_id, pem) in &keys {
            if let Ok(decoding_key) = DecodingKey::from_rsa_pem(pem.as_bytes()) {
                cache.insert(key_id.clone(), decoding_key);
            }
        }

        cache
            .get(kid)
            .cloned()
            .ok_or_else(|| AppError::Unauthorized)
    }

    pub async fn verify_token(&self, token: &str) -> Result<FirebaseClaims, AppError> {
        // Decode header to get kid
        let header = decode_header(token).map_err(|e| {
            debug!("Failed to decode token header: {}", e);
            AppError::Unauthorized
        })?;

        let kid = header.kid.ok_or(AppError::Unauthorized)?;

        // Get decoding key
        let decoding_key = self.get_decoding_key(&kid).await?;

        // Set up validation
        let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_issuer(&[format!(
            "https://securetoken.google.com/{}",
            self.project_id
        )]);
        validation.set_audience(&[self.project_id.clone()]);

        // Decode and verify token
        let token_data = decode::<FirebaseClaims>(token, &decoding_key, &validation).map_err(|e| {
            debug!("Token validation failed: {}", e);
            AppError::Unauthorized
        })?;

        Ok(token_data.claims)
    }
}

// Extension to store authenticated user info in request
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub firebase_uid: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub provider: Option<String>,
}

impl From<FirebaseClaims> for AuthenticatedUser {
    fn from(claims: FirebaseClaims) -> Self {
        let provider = claims
            .firebase
            .as_ref()
            .and_then(|f| f.sign_in_provider.clone());

        Self {
            firebase_uid: claims.sub,
            email: claims.email,
            name: claims.name,
            picture: claims.picture,
            provider,
        }
    }
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let firebase_auth = FirebaseAuth::new(state.config.firebase.project_id.clone());
    let claims = firebase_auth.verify_token(token).await?;

    let user: AuthenticatedUser = claims.into();
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}

/// Admin middleware - must be used after auth_middleware
/// Checks if the authenticated user has admin privileges
pub async fn admin_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    use crate::repositories::user_repo::UserRepository;

    // Get the authenticated user from request extensions
    let auth_user = request
        .extensions()
        .get::<AuthenticatedUser>()
        .ok_or(AppError::Unauthorized)?
        .clone();

    // Check if user exists and is admin
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // Check if user is banned
    if db_user.banned_at.is_some() {
        return Err(AppError::Forbidden("Your account has been banned".into()));
    }

    // Check if user is admin
    if !db_user.is_admin {
        return Err(AppError::Forbidden("Admin access required".into()));
    }

    Ok(next.run(request).await)
}
