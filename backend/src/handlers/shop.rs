use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::HeaderMap,
    Extension, Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::middleware::auth::AuthenticatedUser;
use crate::models::shop::{
    BuySubscriptionRequest, CheckoutResponse, GoldBalanceResponse, GoldPackage,
    PurchaseGoldRequest, SubscriptionPrice, TransactionResponse, UseBookOfWisdomRequest,
    UseFeatureResponse, UseFinishNowRequest, UseNpcMerchantRequest, UseProductionBonusRequest,
};
use crate::repositories::user_repo::UserRepository;
use crate::services::shop_service::ShopService;
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

// ==================== Gold Packages ====================

/// GET /api/shop/packages - Get available gold packages
pub async fn get_packages(State(state): State<AppState>) -> AppResult<Json<Vec<GoldPackage>>> {
    let packages = ShopService::get_gold_packages(&state.db).await?;
    Ok(Json(packages))
}

/// GET /api/shop/balance - Get user's gold balance and subscription status
pub async fn get_balance(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<GoldBalanceResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let balance = ShopService::get_balance(&state.db, db_user.id).await?;
    Ok(Json(balance))
}

/// POST /api/shop/checkout - Create Stripe checkout session
pub async fn create_checkout(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<PurchaseGoldRequest>,
) -> AppResult<Json<CheckoutResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // Get Stripe client from config
    let stripe_secret = std::env::var("STRIPE_SECRET_KEY")
        .map_err(|_| AppError::InternalError(anyhow::anyhow!("Stripe not configured")))?;
    let stripe_client = stripe_rust::Client::new(stripe_secret);

    let checkout = ShopService::create_checkout(
        &state.db,
        &stripe_client,
        db_user.id,
        request.package_id,
        &request.success_url,
        &request.cancel_url,
    )
    .await?;

    Ok(Json(checkout))
}

/// POST /api/shop/webhook - Stripe webhook handler
pub async fn stripe_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> AppResult<Json<serde_json::Value>> {
    let signature = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::BadRequest("Missing Stripe signature".into()))?;

    let webhook_secret = std::env::var("STRIPE_WEBHOOK_SECRET")
        .map_err(|_| AppError::InternalError(anyhow::anyhow!("Webhook secret not configured")))?;

    let payload = std::str::from_utf8(&body)
        .map_err(|_| AppError::BadRequest("Invalid payload".into()))?;

    ShopService::handle_webhook(&state.db, payload, signature, &webhook_secret).await?;

    Ok(Json(serde_json::json!({ "received": true })))
}

// ==================== Subscriptions ====================

/// GET /api/shop/subscriptions - Get subscription prices
pub async fn get_subscription_prices(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<SubscriptionPrice>>> {
    let prices = ShopService::get_subscription_prices(&state.db).await?;
    Ok(Json(prices))
}

/// POST /api/shop/subscriptions/buy - Buy Travian Plus with gold
pub async fn buy_subscription(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<BuySubscriptionRequest>,
) -> AppResult<Json<UseFeatureResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let result =
        ShopService::buy_subscription(&state.db, db_user.id, request.duration_days).await?;
    Ok(Json(result))
}

// ==================== Gold Features ====================

/// POST /api/shop/features/finish-now - Finish building/training instantly
pub async fn use_finish_now(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<UseFinishNowRequest>,
) -> AppResult<Json<UseFeatureResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let result = ShopService::use_finish_now(
        &state.db,
        db_user.id,
        &request.target_type,
        request.target_id,
    )
    .await?;
    Ok(Json(result))
}

/// POST /api/shop/features/npc-merchant - Exchange resources
pub async fn use_npc_merchant(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<UseNpcMerchantRequest>,
) -> AppResult<Json<UseFeatureResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let result = ShopService::use_npc_merchant(
        &state.db,
        db_user.id,
        request.village_id,
        request.wood,
        request.clay,
        request.iron,
        request.crop,
    )
    .await?;
    Ok(Json(result))
}

/// POST /api/shop/features/production-bonus - Activate +25% production
pub async fn use_production_bonus(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<UseProductionBonusRequest>,
) -> AppResult<Json<UseFeatureResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let result = ShopService::use_production_bonus(
        &state.db,
        db_user.id,
        request.village_id,
        &request.resource_type,
    )
    .await?;
    Ok(Json(result))
}

/// POST /api/shop/features/book-of-wisdom - Activate 2x production
pub async fn use_book_of_wisdom(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<UseBookOfWisdomRequest>,
) -> AppResult<Json<UseFeatureResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let result = ShopService::use_book_of_wisdom(&state.db, db_user.id, request.village_id).await?;
    Ok(Json(result))
}

// ==================== Transactions ====================

/// GET /api/shop/transactions - Get transaction history
pub async fn get_transactions(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Query(query): Query<PaginationQuery>,
) -> AppResult<Json<Vec<TransactionResponse>>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let transactions =
        ShopService::get_transactions(&state.db, db_user.id, query.limit, query.offset).await?;
    Ok(Json(transactions))
}
