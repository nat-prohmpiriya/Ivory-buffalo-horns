use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::middleware::auth::AuthenticatedUser;
use crate::models::trade::{
    AcceptOrderRequest, AcceptOrderResponse, CancelOrderResponse, CreateOrderRequest,
    CreateOrderResponse, GetOrdersQuery, GetOrdersResponse, MarketSummaryResponse,
    MyOrdersResponse, TradeHistoryResponse, TradeOrder, TradeOrderStatus, TradeResourceType,
    TradeTransaction,
};
use crate::repositories::trade_repo::TradeRepository;
use crate::repositories::user_repo::UserRepository;
use crate::services::trade_service::TradeService;
use crate::AppState;

// ==================== Query Parameters ====================

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

fn default_page() -> i32 {
    1
}

fn default_limit() -> i32 {
    20
}

#[derive(Debug, Deserialize)]
pub struct TransactionQuery {
    pub resource_type: Option<TradeResourceType>,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

#[derive(Debug, Deserialize)]
pub struct UserOrdersQuery {
    pub status: Option<TradeOrderStatus>,
}

// ==================== Public Market Endpoints ====================

/// GET /api/market/summary - Get market summary for all resources
pub async fn get_market_summary(
    State(state): State<AppState>,
) -> AppResult<Json<MarketSummaryResponse>> {
    let summaries = TradeService::get_market_summary(&state.db).await?;

    Ok(Json(MarketSummaryResponse {
        summaries,
        updated_at: Utc::now(),
    }))
}

/// GET /api/market/orders - Get open orders with optional filters
pub async fn get_open_orders(
    State(state): State<AppState>,
    Query(query): Query<GetOrdersQuery>,
) -> AppResult<Json<GetOrdersResponse>> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).min(100).max(1);
    let offset = (page - 1) * limit;

    let orders = TradeRepository::get_open_orders(
        &state.db,
        query.resource_type,
        query.order_type,
        query.min_price,
        query.max_price,
        limit,
        offset,
    )
    .await?;

    let total = TradeRepository::count_open_orders(
        &state.db,
        query.resource_type,
        query.order_type,
        query.min_price,
        query.max_price,
    )
    .await?;

    // TODO: Add village/user details for display
    // For now, return orders without additional details
    let orders_with_details = orders
        .into_iter()
        .map(|order| crate::models::trade::TradeOrderWithDetails {
            order,
            village_name: String::new(),
            village_x: 0,
            village_y: 0,
            user_display_name: None,
        })
        .collect();

    Ok(Json(GetOrdersResponse {
        orders: orders_with_details,
        total,
        page,
        limit,
    }))
}

/// GET /api/market/orders/:id - Get order details
pub async fn get_order(
    State(state): State<AppState>,
    Path(order_id): Path<Uuid>,
) -> AppResult<Json<TradeOrder>> {
    let order = TradeRepository::get_order_by_id(&state.db, order_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Order not found".into()))?;

    Ok(Json(order))
}

/// GET /api/market/transactions - Get recent transactions
pub async fn get_recent_transactions(
    State(state): State<AppState>,
    Query(query): Query<TransactionQuery>,
) -> AppResult<Json<Vec<TradeTransaction>>> {
    let transactions = TradeRepository::get_recent_transactions(
        &state.db,
        query.resource_type,
        query.limit.min(100).max(1),
    )
    .await?;

    Ok(Json(transactions))
}

// ==================== Authenticated Trade Endpoints ====================

/// POST /api/trade/orders - Create a new trade order
pub async fn create_order(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<CreateOrderRequest>,
) -> AppResult<Json<CreateOrderResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let response = TradeService::create_order(&state.db, db_user.id, request).await?;

    Ok(Json(response))
}

/// POST /api/trade/orders/:id/accept - Accept (fill) a trade order
pub async fn accept_order(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(order_id): Path<Uuid>,
    Json(request): Json<AcceptOrderRequest>,
) -> AppResult<Json<AcceptOrderResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let response = TradeService::accept_order(&state.db, db_user.id, order_id, request).await?;

    Ok(Json(response))
}

/// POST /api/trade/orders/:id/cancel - Cancel a trade order
pub async fn cancel_order(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(order_id): Path<Uuid>,
) -> AppResult<Json<CancelOrderResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let response = TradeService::cancel_order(&state.db, db_user.id, order_id).await?;

    Ok(Json(response))
}

/// GET /api/trade/orders - Get user's own orders
pub async fn get_my_orders(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Query(query): Query<UserOrdersQuery>,
) -> AppResult<Json<MyOrdersResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let orders = TradeRepository::get_user_orders(&state.db, db_user.id, query.status).await?;

    Ok(Json(MyOrdersResponse { orders }))
}

/// GET /api/trade/history - Get user's trade history
pub async fn get_trade_history(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Query(query): Query<PaginationQuery>,
) -> AppResult<Json<TradeHistoryResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let limit = query.limit.min(100).max(1);
    let offset = (query.page.max(1) - 1) * limit;

    let transactions =
        TradeRepository::get_user_transactions(&state.db, db_user.id, limit, offset).await?;
    let total = TradeRepository::count_user_transactions(&state.db, db_user.id).await?;

    Ok(Json(TradeHistoryResponse {
        transactions,
        total,
    }))
}
