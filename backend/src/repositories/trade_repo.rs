use chrono::{Duration, Utc};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::trade::{
    ResourceLock, TradeOrder, TradeOrderStatus, TradeOrderType, TradeResourceType,
    TradeTransaction,
};

pub struct TradeRepository;

impl TradeRepository {
    // ==================== Trade Orders CRUD ====================

    /// Create a new trade order
    pub async fn create_order(
        pool: &PgPool,
        user_id: Uuid,
        village_id: Uuid,
        order_type: TradeOrderType,
        resource_type: TradeResourceType,
        quantity: i32,
        price_per_unit: i32,
        expires_in_hours: Option<i32>,
    ) -> AppResult<TradeOrder> {
        let expires_at = expires_in_hours.map(|hours| Utc::now() + Duration::hours(hours as i64));

        let order = sqlx::query_as::<_, TradeOrder>(
            r#"
            INSERT INTO trade_orders (
                user_id, village_id, order_type, resource_type,
                quantity, price_per_unit, expires_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(village_id)
        .bind(order_type)
        .bind(resource_type)
        .bind(quantity)
        .bind(price_per_unit)
        .bind(expires_at)
        .fetch_one(pool)
        .await?;

        Ok(order)
    }

    /// Get order by ID
    pub async fn get_order_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<TradeOrder>> {
        let order = sqlx::query_as::<_, TradeOrder>(
            r#"SELECT * FROM trade_orders WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(order)
    }

    /// Get order by ID with row lock (FOR UPDATE) - for use within transaction
    pub async fn get_order_for_update(
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
    ) -> AppResult<Option<TradeOrder>> {
        let order = sqlx::query_as::<_, TradeOrder>(
            r#"SELECT * FROM trade_orders WHERE id = $1 FOR UPDATE"#,
        )
        .bind(id)
        .fetch_optional(&mut **tx)
        .await?;

        Ok(order)
    }

    /// Update order status
    pub async fn update_order_status(
        pool: &PgPool,
        id: Uuid,
        status: TradeOrderStatus,
    ) -> AppResult<TradeOrder> {
        let now = Utc::now();
        let cancelled_at = if status == TradeOrderStatus::Cancelled {
            Some(now)
        } else {
            None
        };
        let filled_at = if status == TradeOrderStatus::Filled {
            Some(now)
        } else {
            None
        };

        let order = sqlx::query_as::<_, TradeOrder>(
            r#"
            UPDATE trade_orders
            SET status = $2,
                updated_at = $3,
                cancelled_at = COALESCE($4, cancelled_at),
                filled_at = COALESCE($5, filled_at)
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(now)
        .bind(cancelled_at)
        .bind(filled_at)
        .fetch_one(pool)
        .await?;

        Ok(order)
    }

    /// Update order status within a transaction
    pub async fn update_order_status_tx(
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        status: TradeOrderStatus,
    ) -> AppResult<TradeOrder> {
        let now = Utc::now();
        let cancelled_at = if status == TradeOrderStatus::Cancelled {
            Some(now)
        } else {
            None
        };
        let filled_at = if status == TradeOrderStatus::Filled {
            Some(now)
        } else {
            None
        };

        let order = sqlx::query_as::<_, TradeOrder>(
            r#"
            UPDATE trade_orders
            SET status = $2,
                updated_at = $3,
                cancelled_at = COALESCE($4, cancelled_at),
                filled_at = COALESCE($5, filled_at)
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(now)
        .bind(cancelled_at)
        .bind(filled_at)
        .fetch_one(&mut **tx)
        .await?;

        Ok(order)
    }

    /// Update order filled quantity and status
    pub async fn update_order_filled(
        pool: &PgPool,
        id: Uuid,
        quantity_filled: i32,
        status: TradeOrderStatus,
    ) -> AppResult<TradeOrder> {
        let now = Utc::now();
        let filled_at = if status == TradeOrderStatus::Filled {
            Some(now)
        } else {
            None
        };

        let order = sqlx::query_as::<_, TradeOrder>(
            r#"
            UPDATE trade_orders
            SET quantity_filled = $2,
                status = $3,
                updated_at = $4,
                filled_at = COALESCE($5, filled_at)
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(quantity_filled)
        .bind(status)
        .bind(now)
        .bind(filled_at)
        .fetch_one(pool)
        .await?;

        Ok(order)
    }

    /// Update order filled quantity and status within a transaction
    pub async fn update_order_filled_tx(
        tx: &mut Transaction<'_, Postgres>,
        id: Uuid,
        quantity_filled: i32,
        status: TradeOrderStatus,
    ) -> AppResult<TradeOrder> {
        let now = Utc::now();
        let filled_at = if status == TradeOrderStatus::Filled {
            Some(now)
        } else {
            None
        };

        let order = sqlx::query_as::<_, TradeOrder>(
            r#"
            UPDATE trade_orders
            SET quantity_filled = $2,
                status = $3,
                updated_at = $4,
                filled_at = COALESCE($5, filled_at)
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(quantity_filled)
        .bind(status)
        .bind(now)
        .bind(filled_at)
        .fetch_one(&mut **tx)
        .await?;

        Ok(order)
    }

    /// Delete order (hard delete - use with caution)
    pub async fn delete_order(pool: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query(r#"DELETE FROM trade_orders WHERE id = $1"#)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    // ==================== Resource Locks ====================

    /// Create a resource lock (escrow)
    pub async fn create_resource_lock(
        pool: &PgPool,
        village_id: Uuid,
        lock_type: &str,
        reference_id: Uuid,
        wood: i32,
        clay: i32,
        iron: i32,
        crop: i32,
    ) -> AppResult<ResourceLock> {
        let lock = sqlx::query_as::<_, ResourceLock>(
            r#"
            INSERT INTO resource_locks (village_id, lock_type, reference_id, wood, clay, iron, crop)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(village_id)
        .bind(lock_type)
        .bind(reference_id)
        .bind(wood)
        .bind(clay)
        .bind(iron)
        .bind(crop)
        .fetch_one(pool)
        .await?;

        Ok(lock)
    }

    /// Create a resource lock within a transaction
    pub async fn create_resource_lock_tx(
        tx: &mut Transaction<'_, Postgres>,
        village_id: Uuid,
        lock_type: &str,
        reference_id: Uuid,
        wood: i32,
        clay: i32,
        iron: i32,
        crop: i32,
    ) -> AppResult<ResourceLock> {
        let lock = sqlx::query_as::<_, ResourceLock>(
            r#"
            INSERT INTO resource_locks (village_id, lock_type, reference_id, wood, clay, iron, crop)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(village_id)
        .bind(lock_type)
        .bind(reference_id)
        .bind(wood)
        .bind(clay)
        .bind(iron)
        .bind(crop)
        .fetch_one(&mut **tx)
        .await?;

        Ok(lock)
    }

    /// Get resource lock by reference
    pub async fn get_resource_lock(
        pool: &PgPool,
        lock_type: &str,
        reference_id: Uuid,
    ) -> AppResult<Option<ResourceLock>> {
        let lock = sqlx::query_as::<_, ResourceLock>(
            r#"
            SELECT * FROM resource_locks
            WHERE lock_type = $1 AND reference_id = $2 AND released_at IS NULL
            "#,
        )
        .bind(lock_type)
        .bind(reference_id)
        .fetch_optional(pool)
        .await?;

        Ok(lock)
    }

    /// Release resource lock (mark as released)
    pub async fn release_resource_lock(
        pool: &PgPool,
        lock_type: &str,
        reference_id: Uuid,
    ) -> AppResult<Option<ResourceLock>> {
        let lock = sqlx::query_as::<_, ResourceLock>(
            r#"
            UPDATE resource_locks
            SET released_at = NOW()
            WHERE lock_type = $1 AND reference_id = $2 AND released_at IS NULL
            RETURNING *
            "#,
        )
        .bind(lock_type)
        .bind(reference_id)
        .fetch_optional(pool)
        .await?;

        Ok(lock)
    }

    /// Release resource lock within a transaction
    pub async fn release_resource_lock_tx(
        tx: &mut Transaction<'_, Postgres>,
        lock_type: &str,
        reference_id: Uuid,
    ) -> AppResult<Option<ResourceLock>> {
        let lock = sqlx::query_as::<_, ResourceLock>(
            r#"
            UPDATE resource_locks
            SET released_at = NOW()
            WHERE lock_type = $1 AND reference_id = $2 AND released_at IS NULL
            RETURNING *
            "#,
        )
        .bind(lock_type)
        .bind(reference_id)
        .fetch_optional(&mut **tx)
        .await?;

        Ok(lock)
    }

    /// Get total locked resources for a village
    pub async fn get_village_locked_resources(
        pool: &PgPool,
        village_id: Uuid,
    ) -> AppResult<(i64, i64, i64, i64)> {
        let result: (Option<i64>, Option<i64>, Option<i64>, Option<i64>) = sqlx::query_as(
            r#"
            SELECT
                COALESCE(SUM(wood), 0),
                COALESCE(SUM(clay), 0),
                COALESCE(SUM(iron), 0),
                COALESCE(SUM(crop), 0)
            FROM resource_locks
            WHERE village_id = $1 AND released_at IS NULL
            "#,
        )
        .bind(village_id)
        .fetch_one(pool)
        .await?;

        Ok((
            result.0.unwrap_or(0),
            result.1.unwrap_or(0),
            result.2.unwrap_or(0),
            result.3.unwrap_or(0),
        ))
    }

    /// Get total locked resources for a village within a transaction
    pub async fn get_village_locked_resources_tx(
        tx: &mut Transaction<'_, Postgres>,
        village_id: Uuid,
    ) -> AppResult<(i64, i64, i64, i64)> {
        let result: (Option<i64>, Option<i64>, Option<i64>, Option<i64>) = sqlx::query_as(
            r#"
            SELECT
                COALESCE(SUM(wood), 0),
                COALESCE(SUM(clay), 0),
                COALESCE(SUM(iron), 0),
                COALESCE(SUM(crop), 0)
            FROM resource_locks
            WHERE village_id = $1 AND released_at IS NULL
            "#,
        )
        .bind(village_id)
        .fetch_one(&mut **tx)
        .await?;

        Ok((
            result.0.unwrap_or(0),
            result.1.unwrap_or(0),
            result.2.unwrap_or(0),
            result.3.unwrap_or(0),
        ))
    }

    // ==================== Query Functions ====================

    /// Get open orders with optional filters
    pub async fn get_open_orders(
        pool: &PgPool,
        resource_type: Option<TradeResourceType>,
        order_type: Option<TradeOrderType>,
        min_price: Option<i32>,
        max_price: Option<i32>,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<TradeOrder>> {
        let orders = sqlx::query_as::<_, TradeOrder>(
            r#"
            SELECT * FROM trade_orders
            WHERE status = 'open'
                AND (expires_at IS NULL OR expires_at > NOW())
                AND ($1::trade_resource_type IS NULL OR resource_type = $1)
                AND ($2::trade_order_type IS NULL OR order_type = $2)
                AND ($3::INT IS NULL OR price_per_unit >= $3)
                AND ($4::INT IS NULL OR price_per_unit <= $4)
            ORDER BY
                CASE WHEN order_type = 'sell' THEN price_per_unit END ASC,
                CASE WHEN order_type = 'buy' THEN price_per_unit END DESC,
                created_at ASC
            LIMIT $5 OFFSET $6
            "#,
        )
        .bind(resource_type)
        .bind(order_type)
        .bind(min_price)
        .bind(max_price)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(orders)
    }

    /// Count open orders with optional filters
    pub async fn count_open_orders(
        pool: &PgPool,
        resource_type: Option<TradeResourceType>,
        order_type: Option<TradeOrderType>,
        min_price: Option<i32>,
        max_price: Option<i32>,
    ) -> AppResult<i64> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM trade_orders
            WHERE status = 'open'
                AND (expires_at IS NULL OR expires_at > NOW())
                AND ($1::trade_resource_type IS NULL OR resource_type = $1)
                AND ($2::trade_order_type IS NULL OR order_type = $2)
                AND ($3::INT IS NULL OR price_per_unit >= $3)
                AND ($4::INT IS NULL OR price_per_unit <= $4)
            "#,
        )
        .bind(resource_type)
        .bind(order_type)
        .bind(min_price)
        .bind(max_price)
        .fetch_one(pool)
        .await?;

        Ok(result.0)
    }

    /// Get user's orders with optional status filter
    pub async fn get_user_orders(
        pool: &PgPool,
        user_id: Uuid,
        status: Option<TradeOrderStatus>,
    ) -> AppResult<Vec<TradeOrder>> {
        let orders = sqlx::query_as::<_, TradeOrder>(
            r#"
            SELECT * FROM trade_orders
            WHERE user_id = $1
                AND ($2::trade_order_status IS NULL OR status = $2)
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .bind(status)
        .fetch_all(pool)
        .await?;

        Ok(orders)
    }

    /// Get orders for a specific village
    pub async fn get_village_orders(
        pool: &PgPool,
        village_id: Uuid,
    ) -> AppResult<Vec<TradeOrder>> {
        let orders = sqlx::query_as::<_, TradeOrder>(
            r#"
            SELECT * FROM trade_orders
            WHERE village_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(village_id)
        .fetch_all(pool)
        .await?;

        Ok(orders)
    }

    /// Count user's open orders (for rate limiting)
    pub async fn count_user_open_orders(pool: &PgPool, user_id: Uuid) -> AppResult<i64> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM trade_orders
            WHERE user_id = $1
                AND status IN ('open', 'partially_filled')
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(result.0)
    }

    /// Get best buy price for a resource (highest buy offer)
    pub async fn get_best_buy_price(
        pool: &PgPool,
        resource_type: TradeResourceType,
    ) -> AppResult<Option<i32>> {
        let result: Option<(i32,)> = sqlx::query_as(
            r#"
            SELECT price_per_unit FROM trade_orders
            WHERE resource_type = $1
                AND order_type = 'buy'
                AND status = 'open'
                AND (expires_at IS NULL OR expires_at > NOW())
            ORDER BY price_per_unit DESC
            LIMIT 1
            "#,
        )
        .bind(resource_type)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|r| r.0))
    }

    /// Get best sell price for a resource (lowest sell offer)
    pub async fn get_best_sell_price(
        pool: &PgPool,
        resource_type: TradeResourceType,
    ) -> AppResult<Option<i32>> {
        let result: Option<(i32,)> = sqlx::query_as(
            r#"
            SELECT price_per_unit FROM trade_orders
            WHERE resource_type = $1
                AND order_type = 'sell'
                AND status = 'open'
                AND (expires_at IS NULL OR expires_at > NOW())
            ORDER BY price_per_unit ASC
            LIMIT 1
            "#,
        )
        .bind(resource_type)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|r| r.0))
    }

    /// Get expired orders that need to be processed
    pub async fn get_expired_orders(pool: &PgPool, limit: i32) -> AppResult<Vec<TradeOrder>> {
        let orders = sqlx::query_as::<_, TradeOrder>(
            r#"
            SELECT * FROM trade_orders
            WHERE status IN ('open', 'partially_filled')
                AND expires_at IS NOT NULL
                AND expires_at <= NOW()
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(orders)
    }

    /// Expire orders (batch update)
    pub async fn expire_orders(pool: &PgPool, order_ids: &[Uuid]) -> AppResult<u64> {
        if order_ids.is_empty() {
            return Ok(0);
        }

        let result = sqlx::query(
            r#"
            UPDATE trade_orders
            SET status = 'expired', updated_at = NOW()
            WHERE id = ANY($1)
                AND status IN ('open', 'partially_filled')
            "#,
        )
        .bind(order_ids)
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    // ==================== Trade Transactions ====================

    /// Create a trade transaction record
    pub async fn create_transaction(
        pool: &PgPool,
        buy_order_id: Uuid,
        sell_order_id: Uuid,
        buyer_id: Uuid,
        seller_id: Uuid,
        buyer_village_id: Uuid,
        seller_village_id: Uuid,
        resource_type: TradeResourceType,
        quantity: i32,
        price_per_unit: i32,
    ) -> AppResult<TradeTransaction> {
        let total_gold = quantity * price_per_unit;

        let tx = sqlx::query_as::<_, TradeTransaction>(
            r#"
            INSERT INTO trade_transactions (
                buy_order_id, sell_order_id, buyer_id, seller_id,
                buyer_village_id, seller_village_id, resource_type,
                quantity, price_per_unit, total_gold
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(buy_order_id)
        .bind(sell_order_id)
        .bind(buyer_id)
        .bind(seller_id)
        .bind(buyer_village_id)
        .bind(seller_village_id)
        .bind(resource_type)
        .bind(quantity)
        .bind(price_per_unit)
        .bind(total_gold)
        .fetch_one(pool)
        .await?;

        Ok(tx)
    }

    /// Create a trade transaction within a database transaction
    pub async fn create_transaction_tx(
        tx: &mut Transaction<'_, Postgres>,
        buy_order_id: Uuid,
        sell_order_id: Uuid,
        buyer_id: Uuid,
        seller_id: Uuid,
        buyer_village_id: Uuid,
        seller_village_id: Uuid,
        resource_type: TradeResourceType,
        quantity: i32,
        price_per_unit: i32,
    ) -> AppResult<TradeTransaction> {
        let total_gold = quantity * price_per_unit;

        let trade_tx = sqlx::query_as::<_, TradeTransaction>(
            r#"
            INSERT INTO trade_transactions (
                buy_order_id, sell_order_id, buyer_id, seller_id,
                buyer_village_id, seller_village_id, resource_type,
                quantity, price_per_unit, total_gold
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(buy_order_id)
        .bind(sell_order_id)
        .bind(buyer_id)
        .bind(seller_id)
        .bind(buyer_village_id)
        .bind(seller_village_id)
        .bind(resource_type)
        .bind(quantity)
        .bind(price_per_unit)
        .bind(total_gold)
        .fetch_one(&mut **tx)
        .await?;

        Ok(trade_tx)
    }

    /// Get user's trade transactions (as buyer or seller)
    pub async fn get_user_transactions(
        pool: &PgPool,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<TradeTransaction>> {
        let txs = sqlx::query_as::<_, TradeTransaction>(
            r#"
            SELECT * FROM trade_transactions
            WHERE buyer_id = $1 OR seller_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(txs)
    }

    /// Count user's trade transactions
    pub async fn count_user_transactions(pool: &PgPool, user_id: Uuid) -> AppResult<i64> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM trade_transactions
            WHERE buyer_id = $1 OR seller_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(result.0)
    }

    /// Get transaction by ID
    pub async fn get_transaction_by_id(
        pool: &PgPool,
        id: Uuid,
    ) -> AppResult<Option<TradeTransaction>> {
        let tx = sqlx::query_as::<_, TradeTransaction>(
            r#"SELECT * FROM trade_transactions WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(tx)
    }

    /// Get last trade price for a resource type
    pub async fn get_last_trade_price(
        pool: &PgPool,
        resource_type: TradeResourceType,
    ) -> AppResult<Option<i32>> {
        let result: Option<(i32,)> = sqlx::query_as(
            r#"
            SELECT price_per_unit FROM trade_transactions
            WHERE resource_type = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(resource_type)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|r| r.0))
    }

    /// Get 24h trading volume for a resource type
    pub async fn get_24h_volume(
        pool: &PgPool,
        resource_type: TradeResourceType,
    ) -> AppResult<(i64, i64)> {
        // Returns (total_quantity, trade_count)
        let result: (Option<i64>, i64) = sqlx::query_as(
            r#"
            SELECT
                COALESCE(SUM(quantity), 0),
                COUNT(*)
            FROM trade_transactions
            WHERE resource_type = $1
                AND created_at > NOW() - INTERVAL '24 hours'
            "#,
        )
        .bind(resource_type)
        .fetch_one(pool)
        .await?;

        Ok((result.0.unwrap_or(0), result.1))
    }

    /// Get recent transactions (for market activity display)
    pub async fn get_recent_transactions(
        pool: &PgPool,
        resource_type: Option<TradeResourceType>,
        limit: i32,
    ) -> AppResult<Vec<TradeTransaction>> {
        let txs = sqlx::query_as::<_, TradeTransaction>(
            r#"
            SELECT * FROM trade_transactions
            WHERE ($1::trade_resource_type IS NULL OR resource_type = $1)
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(resource_type)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(txs)
    }

    /// Get transactions for a specific order
    pub async fn get_order_transactions(
        pool: &PgPool,
        order_id: Uuid,
    ) -> AppResult<Vec<TradeTransaction>> {
        let txs = sqlx::query_as::<_, TradeTransaction>(
            r#"
            SELECT * FROM trade_transactions
            WHERE buy_order_id = $1 OR sell_order_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(order_id)
        .fetch_all(pool)
        .await?;

        Ok(txs)
    }
}
