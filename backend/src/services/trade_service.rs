use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::trade::{
    AcceptOrderRequest, AcceptOrderResponse, CancelOrderResponse, CreateOrderRequest,
    CreateOrderResponse, MarketSummary, TradeOrder, TradeOrderStatus, TradeOrderType,
    TradeResourceType, Resources,
};
use crate::models::village::Village;
use crate::repositories::trade_repo::TradeRepository;
use crate::repositories::village_repo::VillageRepository;

// ==================== Constants ====================

/// Minimum quantity per order
pub const MIN_QUANTITY: i32 = 100;

/// Maximum quantity per order
pub const MAX_QUANTITY: i32 = 1_000_000;

/// Minimum price per unit (gold)
pub const MIN_PRICE: i32 = 1;

/// Maximum price per unit (gold)
pub const MAX_PRICE: i32 = 10_000;

/// Maximum open orders per user
pub const MAX_OPEN_ORDERS_PER_USER: i64 = 50;

/// Maximum expiry time in hours
pub const MAX_EXPIRY_HOURS: i32 = 168; // 7 days

/// Lock type for trade orders
pub const LOCK_TYPE_TRADE_ORDER: &str = "trade_order";

pub struct TradeService;

impl TradeService {
    // ==================== Validation Functions ====================

    /// Validate create order request
    pub fn validate_create_order_request(request: &CreateOrderRequest) -> AppResult<()> {
        // Validate quantity
        if request.quantity < MIN_QUANTITY {
            return Err(AppError::BadRequest(format!(
                "Minimum quantity is {}",
                MIN_QUANTITY
            )));
        }
        if request.quantity > MAX_QUANTITY {
            return Err(AppError::BadRequest(format!(
                "Maximum quantity is {}",
                MAX_QUANTITY
            )));
        }

        // Validate price
        if request.price_per_unit < MIN_PRICE {
            return Err(AppError::BadRequest(format!(
                "Minimum price is {} gold per unit",
                MIN_PRICE
            )));
        }
        if request.price_per_unit > MAX_PRICE {
            return Err(AppError::BadRequest(format!(
                "Maximum price is {} gold per unit",
                MAX_PRICE
            )));
        }

        // Validate expiry
        if let Some(hours) = request.expires_in_hours {
            if hours < 1 {
                return Err(AppError::BadRequest(
                    "Expiry time must be at least 1 hour".into(),
                ));
            }
            if hours > MAX_EXPIRY_HOURS {
                return Err(AppError::BadRequest(format!(
                    "Maximum expiry time is {} hours",
                    MAX_EXPIRY_HOURS
                )));
            }
        }

        Ok(())
    }

    /// Validate village ownership
    pub fn validate_village_ownership(village: &Village, user_id: Uuid) -> AppResult<()> {
        if village.user_id != user_id {
            return Err(AppError::Forbidden(
                "You do not own this village".into(),
            ));
        }
        Ok(())
    }

    /// Check if user has reached max open orders
    pub async fn check_order_limit(pool: &PgPool, user_id: Uuid) -> AppResult<()> {
        let open_count = TradeRepository::count_user_open_orders(pool, user_id).await?;
        if open_count >= MAX_OPEN_ORDERS_PER_USER {
            return Err(AppError::BadRequest(format!(
                "You have reached the maximum of {} open orders",
                MAX_OPEN_ORDERS_PER_USER
            )));
        }
        Ok(())
    }

    /// Validate sell order - check if village has enough resources
    pub async fn validate_sell_order_resources(
        pool: &PgPool,
        village: &Village,
        resource_type: TradeResourceType,
        quantity: i32,
    ) -> AppResult<()> {
        // Get current village resources
        let available = Self::get_village_resource(village, resource_type);

        // Get locked resources for this village
        let (locked_wood, locked_clay, locked_iron, locked_crop) =
            TradeRepository::get_village_locked_resources(pool, village.id).await?;

        let locked = match resource_type {
            TradeResourceType::Wood => locked_wood,
            TradeResourceType::Clay => locked_clay,
            TradeResourceType::Iron => locked_iron,
            TradeResourceType::Crop => locked_crop,
        };

        let available_after_locks = available - locked as i32;

        if available_after_locks < quantity {
            return Err(AppError::BadRequest(format!(
                "Insufficient {}. Available: {}, Required: {}",
                resource_type_name(resource_type),
                available_after_locks,
                quantity
            )));
        }

        Ok(())
    }

    /// Validate buy order - check if user has enough gold
    pub async fn validate_buy_order_gold(
        pool: &PgPool,
        user_id: Uuid,
        quantity: i32,
        price_per_unit: i32,
    ) -> AppResult<()> {
        let total_cost = (quantity as i64) * (price_per_unit as i64);

        // Get user's gold balance
        let balance: (i32,) = sqlx::query_as(
            r#"SELECT gold_balance FROM users WHERE id = $1"#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        if (balance.0 as i64) < total_cost {
            return Err(AppError::BadRequest(format!(
                "Insufficient gold. Available: {}, Required: {}",
                balance.0, total_cost
            )));
        }

        Ok(())
    }

    /// Validate accept order request
    pub fn validate_accept_order(
        order: &TradeOrder,
        user_id: Uuid,
        quantity: Option<i32>,
    ) -> AppResult<i32> {
        // Check order status
        if !order.can_fill() {
            return Err(AppError::BadRequest(
                "This order cannot be filled".into(),
            ));
        }

        // Check expiration
        if order.is_expired() {
            return Err(AppError::BadRequest(
                "This order has expired".into(),
            ));
        }

        // Cannot accept own order
        if order.user_id == user_id {
            return Err(AppError::BadRequest(
                "You cannot accept your own order".into(),
            ));
        }

        // Calculate fill quantity
        let remaining = order.quantity_remaining();
        let fill_quantity = quantity.unwrap_or(remaining);

        if fill_quantity <= 0 {
            return Err(AppError::BadRequest(
                "Fill quantity must be positive".into(),
            ));
        }

        if fill_quantity > remaining {
            return Err(AppError::BadRequest(format!(
                "Cannot fill {} units. Only {} available",
                fill_quantity, remaining
            )));
        }

        if fill_quantity < MIN_QUANTITY && fill_quantity != remaining {
            return Err(AppError::BadRequest(format!(
                "Minimum fill quantity is {} (or remaining {})",
                MIN_QUANTITY, remaining
            )));
        }

        Ok(fill_quantity)
    }

    /// Validate cancel order request
    pub fn validate_cancel_order(order: &TradeOrder, user_id: Uuid) -> AppResult<()> {
        // Check ownership
        if order.user_id != user_id {
            return Err(AppError::Forbidden(
                "You do not own this order".into(),
            ));
        }

        // Check if cancellable
        if !order.can_cancel() {
            return Err(AppError::BadRequest(format!(
                "Cannot cancel order with status: {:?}",
                order.status
            )));
        }

        Ok(())
    }

    // ==================== Create Order Functions ====================

    /// Create a new trade order (buy or sell)
    pub async fn create_order(
        pool: &PgPool,
        user_id: Uuid,
        request: CreateOrderRequest,
    ) -> AppResult<CreateOrderResponse> {
        // Validate request parameters
        Self::validate_create_order_request(&request)?;

        // Check order limit
        Self::check_order_limit(pool, user_id).await?;

        // Get village and validate ownership
        let village = VillageRepository::find_by_id(pool, request.village_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Village not found".into()))?;

        Self::validate_village_ownership(&village, user_id)?;

        // Route to appropriate handler based on order type
        match request.order_type {
            TradeOrderType::Sell => Self::create_sell_order(pool, user_id, &village, request).await,
            TradeOrderType::Buy => Self::create_buy_order(pool, user_id, &village, request).await,
        }
    }

    /// Create a sell order (selling resources for gold)
    async fn create_sell_order(
        pool: &PgPool,
        user_id: Uuid,
        village: &Village,
        request: CreateOrderRequest,
    ) -> AppResult<CreateOrderResponse> {
        // Validate resources available
        Self::validate_sell_order_resources(
            pool,
            village,
            request.resource_type,
            request.quantity,
        )
        .await?;

        // Start transaction
        let mut tx = pool.begin().await?;

        // Create the order
        let order = TradeRepository::create_order(
            pool,
            user_id,
            request.village_id,
            TradeOrderType::Sell,
            request.resource_type,
            request.quantity,
            request.price_per_unit,
            request.expires_in_hours,
        )
        .await?;

        // Lock the resources
        let (wood, clay, iron, crop) = match request.resource_type {
            TradeResourceType::Wood => (request.quantity, 0, 0, 0),
            TradeResourceType::Clay => (0, request.quantity, 0, 0),
            TradeResourceType::Iron => (0, 0, request.quantity, 0),
            TradeResourceType::Crop => (0, 0, 0, request.quantity),
        };

        TradeRepository::create_resource_lock_tx(
            &mut tx,
            village.id,
            LOCK_TYPE_TRADE_ORDER,
            order.id,
            wood,
            clay,
            iron,
            crop,
        )
        .await?;

        // Commit transaction
        tx.commit().await?;

        let locked_resources = Self::single_resource(request.resource_type, request.quantity);

        Ok(CreateOrderResponse {
            order,
            locked_resources: Some(locked_resources),
            locked_gold: None,
        })
    }

    /// Create a buy order (buying resources with gold)
    async fn create_buy_order(
        pool: &PgPool,
        user_id: Uuid,
        village: &Village,
        request: CreateOrderRequest,
    ) -> AppResult<CreateOrderResponse> {
        let total_cost = (request.quantity as i64) * (request.price_per_unit as i64);

        // Validate gold balance
        Self::validate_buy_order_gold(
            pool,
            user_id,
            request.quantity,
            request.price_per_unit,
        )
        .await?;

        // Start transaction
        let mut tx = pool.begin().await?;

        // Deduct gold from user (lock it)
        let deduct_result = sqlx::query(
            r#"
            UPDATE users
            SET gold_balance = gold_balance - $2
            WHERE id = $1 AND gold_balance >= $2
            "#,
        )
        .bind(user_id)
        .bind(total_cost as i32)
        .execute(&mut *tx)
        .await?;

        if deduct_result.rows_affected() == 0 {
            return Err(AppError::BadRequest(
                "Insufficient gold balance".into(),
            ));
        }

        // Create the order
        let order = sqlx::query_as::<_, TradeOrder>(
            r#"
            INSERT INTO trade_orders (
                user_id, village_id, order_type, resource_type,
                quantity, price_per_unit, expires_at
            )
            VALUES ($1, $2, $3, $4, $5, $6,
                CASE WHEN $7::INT IS NOT NULL
                    THEN NOW() + ($7 || ' hours')::INTERVAL
                    ELSE NULL
                END
            )
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(village.id)
        .bind(TradeOrderType::Buy)
        .bind(request.resource_type)
        .bind(request.quantity)
        .bind(request.price_per_unit)
        .bind(request.expires_in_hours)
        .fetch_one(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        Ok(CreateOrderResponse {
            order,
            locked_resources: None,
            locked_gold: Some(total_cost as i32),
        })
    }

    // ==================== Cancel Order Function ====================

    /// Cancel a trade order and refund resources/gold
    pub async fn cancel_order(
        pool: &PgPool,
        user_id: Uuid,
        order_id: Uuid,
    ) -> AppResult<CancelOrderResponse> {
        // Get the order
        let order = TradeRepository::get_order_by_id(pool, order_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Order not found".into()))?;

        // Validate cancel request
        Self::validate_cancel_order(&order, user_id)?;

        // Calculate refund amount (only unfilled portion)
        let remaining_quantity = order.quantity_remaining();

        // Start transaction
        let mut tx = pool.begin().await?;

        // Update order status to cancelled
        let updated_order = TradeRepository::update_order_status_tx(
            &mut tx,
            order_id,
            TradeOrderStatus::Cancelled,
        )
        .await?;

        // Process refund based on order type
        let (refunded_resources, refunded_gold) = match order.order_type {
            TradeOrderType::Sell => {
                // Release resource lock
                let lock = TradeRepository::release_resource_lock_tx(
                    &mut tx,
                    LOCK_TYPE_TRADE_ORDER,
                    order_id,
                )
                .await?;

                let resources = lock.map(|l| l.to_resources());
                (resources, None)
            }
            TradeOrderType::Buy => {
                // Refund gold for unfilled portion
                let refund_amount = (remaining_quantity as i64) * (order.price_per_unit as i64);

                if refund_amount > 0 {
                    sqlx::query(
                        r#"
                        UPDATE users
                        SET gold_balance = gold_balance + $2
                        WHERE id = $1
                        "#,
                    )
                    .bind(user_id)
                    .bind(refund_amount as i32)
                    .execute(&mut *tx)
                    .await?;
                }

                (None, Some(refund_amount as i32))
            }
        };

        // Commit transaction
        tx.commit().await?;

        Ok(CancelOrderResponse {
            order: updated_order,
            refunded_resources,
            refunded_gold,
        })
    }

    // ==================== Accept Order Function ====================

    /// Accept (fill) a trade order
    pub async fn accept_order(
        pool: &PgPool,
        user_id: Uuid,
        order_id: Uuid,
        request: AcceptOrderRequest,
    ) -> AppResult<AcceptOrderResponse> {
        // Start transaction
        let mut tx = pool.begin().await?;

        // Get order with lock
        let order = TradeRepository::get_order_for_update(&mut tx, order_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Order not found".into()))?;

        // Validate accept request and get fill quantity
        let fill_quantity = Self::validate_accept_order(&order, user_id, request.quantity)?;

        // Get acceptor's village
        let acceptor_village = VillageRepository::find_by_id(pool, request.village_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Village not found".into()))?;

        Self::validate_village_ownership(&acceptor_village, user_id)?;

        // Calculate gold amount
        let gold_amount = (fill_quantity as i64) * (order.price_per_unit as i64);

        // Process based on order type
        let (resources_received, gold_received, transaction) = match order.order_type {
            TradeOrderType::Sell => {
                // Accepting a SELL order: acceptor is BUYER
                // Buyer pays gold, receives resources
                Self::process_accept_sell_order(
                    &mut tx,
                    &order,
                    user_id,
                    &acceptor_village,
                    fill_quantity,
                    gold_amount,
                )
                .await?
            }
            TradeOrderType::Buy => {
                // Accepting a BUY order: acceptor is SELLER
                // Seller provides resources, receives gold
                Self::process_accept_buy_order(
                    &mut tx,
                    &order,
                    user_id,
                    &acceptor_village,
                    fill_quantity,
                    gold_amount,
                )
                .await?
            }
        };

        // Update order filled quantity and status
        let new_quantity_filled = order.quantity_filled + fill_quantity;
        let new_status = Self::calculate_order_status(order.quantity, new_quantity_filled);

        let updated_order = TradeRepository::update_order_filled_tx(
            &mut tx,
            order_id,
            new_quantity_filled,
            new_status,
        )
        .await?;

        // If order is fully filled and it's a sell order, release the resource lock
        if new_status == TradeOrderStatus::Filled && order.order_type == TradeOrderType::Sell {
            TradeRepository::release_resource_lock_tx(&mut tx, LOCK_TYPE_TRADE_ORDER, order_id)
                .await?;
        }

        // Commit transaction
        tx.commit().await?;

        Ok(AcceptOrderResponse {
            transaction,
            order_status: updated_order.status,
            resources_received,
            gold_received,
        })
    }

    /// Process accepting a sell order (buyer side)
    async fn process_accept_sell_order(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        order: &TradeOrder,
        buyer_id: Uuid,
        buyer_village: &Village,
        quantity: i32,
        gold_amount: i64,
    ) -> AppResult<(Option<Resources>, Option<i32>, crate::models::trade::TradeTransaction)> {
        // Deduct gold from buyer
        let deduct_result = sqlx::query(
            r#"
            UPDATE users
            SET gold_balance = gold_balance - $2
            WHERE id = $1 AND gold_balance >= $2
            "#,
        )
        .bind(buyer_id)
        .bind(gold_amount as i32)
        .execute(&mut **tx)
        .await?;

        if deduct_result.rows_affected() == 0 {
            return Err(AppError::BadRequest("Insufficient gold balance".into()));
        }

        // Add gold to seller
        sqlx::query(
            r#"
            UPDATE users
            SET gold_balance = gold_balance + $2
            WHERE id = $1
            "#,
        )
        .bind(order.user_id)
        .bind(gold_amount as i32)
        .execute(&mut **tx)
        .await?;

        // Add resources to buyer's village
        Self::add_resource_to_village(tx, buyer_village.id, order.resource_type, quantity).await?;

        // Create transaction record
        let trade_tx = TradeRepository::create_transaction_tx(
            tx,
            order.id, // This sell order becomes the sell_order_id
            order.id, // For now, using same ID - in real matching we'd have separate buy order
            buyer_id,
            order.user_id,
            buyer_village.id,
            order.village_id,
            order.resource_type,
            quantity,
            order.price_per_unit,
        )
        .await?;

        let resources = Self::single_resource(order.resource_type, quantity);

        Ok((Some(resources), None, trade_tx))
    }

    /// Process accepting a buy order (seller side)
    async fn process_accept_buy_order(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        order: &TradeOrder,
        seller_id: Uuid,
        seller_village: &Village,
        quantity: i32,
        gold_amount: i64,
    ) -> AppResult<(Option<Resources>, Option<i32>, crate::models::trade::TradeTransaction)> {
        // Check seller has enough resources
        let available = Self::get_village_resource(seller_village, order.resource_type);

        // Get locked resources for this village
        let (locked_wood, locked_clay, locked_iron, locked_crop) =
            TradeRepository::get_village_locked_resources_tx(tx, seller_village.id).await?;

        let locked = match order.resource_type {
            TradeResourceType::Wood => locked_wood,
            TradeResourceType::Clay => locked_clay,
            TradeResourceType::Iron => locked_iron,
            TradeResourceType::Crop => locked_crop,
        };

        let available_after_locks = available - locked as i32;

        if available_after_locks < quantity {
            return Err(AppError::BadRequest(format!(
                "Insufficient resources. Available: {}, Required: {}",
                available_after_locks, quantity
            )));
        }

        // Deduct resources from seller's village
        Self::deduct_resource_from_village(tx, seller_village.id, order.resource_type, quantity)
            .await?;

        // Add resources to buyer's village (order owner)
        Self::add_resource_to_village(tx, order.village_id, order.resource_type, quantity).await?;

        // Gold was already deducted from buyer when they created the buy order
        // Add gold to seller
        sqlx::query(
            r#"
            UPDATE users
            SET gold_balance = gold_balance + $2
            WHERE id = $1
            "#,
        )
        .bind(seller_id)
        .bind(gold_amount as i32)
        .execute(&mut **tx)
        .await?;

        // Create transaction record
        let trade_tx = TradeRepository::create_transaction_tx(
            tx,
            order.id, // This buy order becomes the buy_order_id
            order.id, // For now, using same ID
            order.user_id,
            seller_id,
            order.village_id,
            seller_village.id,
            order.resource_type,
            quantity,
            order.price_per_unit,
        )
        .await?;

        Ok((None, Some(gold_amount as i32), trade_tx))
    }

    /// Add resources to a village
    async fn add_resource_to_village(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        village_id: Uuid,
        resource_type: TradeResourceType,
        amount: i32,
    ) -> AppResult<()> {
        let column = match resource_type {
            TradeResourceType::Wood => "wood",
            TradeResourceType::Clay => "clay",
            TradeResourceType::Iron => "iron",
            TradeResourceType::Crop => "crop",
        };

        sqlx::query(&format!(
            r#"
            UPDATE villages
            SET {} = LEAST({} + $2,
                CASE WHEN $3 = 'crop' THEN granary_capacity ELSE warehouse_capacity END)
            WHERE id = $1
            "#,
            column, column
        ))
        .bind(village_id)
        .bind(amount as f64)
        .bind(column)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Deduct resources from a village
    async fn deduct_resource_from_village(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        village_id: Uuid,
        resource_type: TradeResourceType,
        amount: i32,
    ) -> AppResult<()> {
        let column = match resource_type {
            TradeResourceType::Wood => "wood",
            TradeResourceType::Clay => "clay",
            TradeResourceType::Iron => "iron",
            TradeResourceType::Crop => "crop",
        };

        let result = sqlx::query(&format!(
            r#"
            UPDATE villages
            SET {} = {} - $2
            WHERE id = $1 AND {} >= $2
            "#,
            column, column, column
        ))
        .bind(village_id)
        .bind(amount as f64)
        .execute(&mut **tx)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::BadRequest("Insufficient resources".into()));
        }

        Ok(())
    }

    // ==================== Helper Functions ====================

    /// Get resource amount from village
    pub fn get_village_resource(village: &Village, resource_type: TradeResourceType) -> i32 {
        match resource_type {
            TradeResourceType::Wood => village.wood as i32,
            TradeResourceType::Clay => village.clay as i32,
            TradeResourceType::Iron => village.iron as i32,
            TradeResourceType::Crop => village.crop as i32,
        }
    }

    /// Create Resources struct with single resource
    pub fn single_resource(resource_type: TradeResourceType, amount: i32) -> Resources {
        let mut resources = Resources::default();
        resources.set(resource_type, amount);
        resources
    }

    /// Get market summary for all resources
    pub async fn get_market_summary(pool: &PgPool) -> AppResult<Vec<MarketSummary>> {
        let mut summaries = Vec::new();

        for resource_type in TradeResourceType::all() {
            let best_buy = TradeRepository::get_best_buy_price(pool, resource_type).await?;
            let best_sell = TradeRepository::get_best_sell_price(pool, resource_type).await?;
            let last_price = TradeRepository::get_last_trade_price(pool, resource_type).await?;
            let (volume, trade_count) = TradeRepository::get_24h_volume(pool, resource_type).await?;

            let spread = match (best_sell, best_buy) {
                (Some(sell), Some(buy)) => Some(sell - buy),
                _ => None,
            };

            summaries.push(MarketSummary {
                resource_type,
                best_buy_price: best_buy,
                best_sell_price: best_sell,
                spread,
                last_trade_price: last_price,
                volume_24h: volume as i32,
                trade_count_24h: trade_count as i32,
            });
        }

        Ok(summaries)
    }

    /// Calculate new order status based on filled quantity
    pub fn calculate_order_status(quantity: i32, quantity_filled: i32) -> TradeOrderStatus {
        if quantity_filled >= quantity {
            TradeOrderStatus::Filled
        } else if quantity_filled > 0 {
            TradeOrderStatus::PartiallyFilled
        } else {
            TradeOrderStatus::Open
        }
    }
}

// ==================== Helper Functions ====================

fn resource_type_name(resource_type: TradeResourceType) -> &'static str {
    match resource_type {
        TradeResourceType::Wood => "wood",
        TradeResourceType::Clay => "clay",
        TradeResourceType::Iron => "iron",
        TradeResourceType::Crop => "crop",
    }
}

/// Expired order result for background job
#[derive(Debug)]
pub struct ExpiredOrderResult {
    pub order: TradeOrder,
    pub user_id: Uuid,
    pub refunded_gold: Option<i32>,
}

impl TradeService {
    /// Process expired orders - called by background job
    /// Returns list of expired orders with their refund info for notification
    pub async fn process_expired_orders(pool: &PgPool, limit: i32) -> anyhow::Result<Vec<ExpiredOrderResult>> {
        let expired_orders = TradeRepository::get_expired_orders(pool, limit).await?;

        if expired_orders.is_empty() {
            return Ok(vec![]);
        }

        let mut results = Vec::new();

        for order in expired_orders {
            match Self::expire_single_order(pool, &order).await {
                Ok(refunded_gold) => {
                    results.push(ExpiredOrderResult {
                        user_id: order.user_id,
                        refunded_gold,
                        order,
                    });
                }
                Err(e) => {
                    tracing::error!("Failed to expire order {}: {:?}", order.id, e);
                }
            }
        }

        Ok(results)
    }

    /// Expire a single order and process refunds
    async fn expire_single_order(pool: &PgPool, order: &TradeOrder) -> anyhow::Result<Option<i32>> {
        let remaining_quantity = order.quantity_remaining();

        // Start transaction
        let mut tx = pool.begin().await?;

        // Update order status to expired
        sqlx::query(
            r#"
            UPDATE trade_orders
            SET status = 'expired', updated_at = NOW()
            WHERE id = $1 AND status IN ('open', 'partially_filled')
            "#,
        )
        .bind(order.id)
        .execute(&mut *tx)
        .await?;

        let refunded_gold = match order.order_type {
            TradeOrderType::Sell => {
                // Release resource lock - resources are freed back to village
                TradeRepository::release_resource_lock_tx(
                    &mut tx,
                    LOCK_TYPE_TRADE_ORDER,
                    order.id,
                )
                .await?;
                None
            }
            TradeOrderType::Buy => {
                // Refund gold for unfilled portion
                let refund_amount = (remaining_quantity as i64) * (order.price_per_unit as i64);

                if refund_amount > 0 {
                    sqlx::query(
                        r#"
                        UPDATE users
                        SET gold_balance = gold_balance + $2
                        WHERE id = $1
                        "#,
                    )
                    .bind(order.user_id)
                    .bind(refund_amount as i32)
                    .execute(&mut *tx)
                    .await?;
                }

                Some(refund_amount as i32)
            }
        };

        // Commit transaction
        tx.commit().await?;

        Ok(refunded_gold)
    }
}
