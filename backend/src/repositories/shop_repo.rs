use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::shop::{
    GoldFeature, GoldFeatureCost, GoldPackage, GoldUsage, SubscriptionPrice, SubscriptionType,
    Transaction, TransactionStatus, TransactionType, UserSubscription,
};

pub struct ShopRepository;

impl ShopRepository {
    // ==================== Gold Packages ====================

    /// Get all active gold packages
    pub async fn get_gold_packages(pool: &PgPool) -> AppResult<Vec<GoldPackage>> {
        let packages = sqlx::query_as::<_, GoldPackage>(
            r#"
            SELECT * FROM gold_packages
            WHERE is_active = TRUE
            ORDER BY price_cents ASC
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(packages)
    }

    /// Get gold package by ID
    pub async fn get_gold_package(pool: &PgPool, id: Uuid) -> AppResult<Option<GoldPackage>> {
        let package = sqlx::query_as::<_, GoldPackage>(
            r#"SELECT * FROM gold_packages WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(package)
    }

    // ==================== User Gold Balance ====================

    /// Get user's gold balance
    pub async fn get_gold_balance(pool: &PgPool, user_id: Uuid) -> AppResult<i32> {
        let result: (i32,) = sqlx::query_as(
            r#"SELECT gold_balance FROM users WHERE id = $1"#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(result.0)
    }

    /// Add gold to user's balance
    pub async fn add_gold(pool: &PgPool, user_id: Uuid, amount: i32) -> AppResult<i32> {
        let result: (i32,) = sqlx::query_as(
            r#"
            UPDATE users
            SET gold_balance = gold_balance + $2
            WHERE id = $1
            RETURNING gold_balance
            "#,
        )
        .bind(user_id)
        .bind(amount)
        .fetch_one(pool)
        .await?;

        Ok(result.0)
    }

    /// Deduct gold from user's balance (returns new balance or error if insufficient)
    pub async fn deduct_gold(pool: &PgPool, user_id: Uuid, amount: i32) -> AppResult<i32> {
        let result: (i32,) = sqlx::query_as(
            r#"
            UPDATE users
            SET gold_balance = gold_balance - $2
            WHERE id = $1 AND gold_balance >= $2
            RETURNING gold_balance
            "#,
        )
        .bind(user_id)
        .bind(amount)
        .fetch_one(pool)
        .await?;

        Ok(result.0)
    }

    // ==================== Transactions ====================

    /// Create a new transaction
    pub async fn create_transaction(
        pool: &PgPool,
        user_id: Uuid,
        transaction_type: TransactionType,
        gold_amount: i32,
        amount_cents: Option<i32>,
        currency: Option<&str>,
        stripe_session_id: Option<&str>,
        gold_package_id: Option<Uuid>,
        description: Option<&str>,
    ) -> AppResult<Transaction> {
        let tx = sqlx::query_as::<_, Transaction>(
            r#"
            INSERT INTO transactions (
                user_id, transaction_type, gold_amount, amount_cents, currency,
                stripe_session_id, gold_package_id, description
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(transaction_type)
        .bind(gold_amount)
        .bind(amount_cents)
        .bind(currency)
        .bind(stripe_session_id)
        .bind(gold_package_id)
        .bind(description)
        .fetch_one(pool)
        .await?;

        Ok(tx)
    }

    /// Update transaction status
    pub async fn update_transaction_status(
        pool: &PgPool,
        id: Uuid,
        status: TransactionStatus,
        stripe_payment_intent_id: Option<&str>,
    ) -> AppResult<Transaction> {
        let completed_at = if status == TransactionStatus::Completed {
            Some(Utc::now())
        } else {
            None
        };

        let tx = sqlx::query_as::<_, Transaction>(
            r#"
            UPDATE transactions
            SET status = $2, stripe_payment_intent_id = COALESCE($3, stripe_payment_intent_id),
                completed_at = COALESCE($4, completed_at)
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(stripe_payment_intent_id)
        .bind(completed_at)
        .fetch_one(pool)
        .await?;

        Ok(tx)
    }

    /// Get transaction by Stripe session ID
    pub async fn get_transaction_by_session(
        pool: &PgPool,
        session_id: &str,
    ) -> AppResult<Option<Transaction>> {
        let tx = sqlx::query_as::<_, Transaction>(
            r#"SELECT * FROM transactions WHERE stripe_session_id = $1"#,
        )
        .bind(session_id)
        .fetch_optional(pool)
        .await?;

        Ok(tx)
    }

    /// Get user's transaction history
    pub async fn get_user_transactions(
        pool: &PgPool,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<Transaction>> {
        let txs = sqlx::query_as::<_, Transaction>(
            r#"
            SELECT * FROM transactions
            WHERE user_id = $1
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

    // ==================== Subscriptions ====================

    /// Get user's active subscription
    pub async fn get_active_subscription(
        pool: &PgPool,
        user_id: Uuid,
        subscription_type: SubscriptionType,
    ) -> AppResult<Option<UserSubscription>> {
        let sub = sqlx::query_as::<_, UserSubscription>(
            r#"
            SELECT * FROM user_subscriptions
            WHERE user_id = $1
                AND subscription_type = $2
                AND is_active = TRUE
                AND expires_at > NOW()
            ORDER BY expires_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .bind(subscription_type)
        .fetch_optional(pool)
        .await?;

        Ok(sub)
    }

    /// Create or extend subscription
    pub async fn create_or_extend_subscription(
        pool: &PgPool,
        user_id: Uuid,
        subscription_type: SubscriptionType,
        duration_days: i32,
    ) -> AppResult<UserSubscription> {
        // Check if there's an existing active subscription
        let existing = Self::get_active_subscription(pool, user_id, subscription_type).await?;

        let starts_at = Utc::now();
        let expires_at = if let Some(existing) = existing {
            // Extend from current expiry
            existing.expires_at + Duration::days(duration_days as i64)
        } else {
            // New subscription
            starts_at + Duration::days(duration_days as i64)
        };

        let sub = sqlx::query_as::<_, UserSubscription>(
            r#"
            INSERT INTO user_subscriptions (user_id, subscription_type, starts_at, expires_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_id, subscription_type) WHERE is_active = TRUE
            DO UPDATE SET expires_at = $4, updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(subscription_type)
        .bind(starts_at)
        .bind(expires_at)
        .fetch_one(pool)
        .await?;

        Ok(sub)
    }

    /// Get subscription prices
    pub async fn get_subscription_prices(
        pool: &PgPool,
        subscription_type: SubscriptionType,
    ) -> AppResult<Vec<SubscriptionPrice>> {
        let prices = sqlx::query_as::<_, SubscriptionPrice>(
            r#"
            SELECT * FROM subscription_prices
            WHERE subscription_type = $1 AND is_active = TRUE
            ORDER BY duration_days ASC
            "#,
        )
        .bind(subscription_type)
        .fetch_all(pool)
        .await?;

        Ok(prices)
    }

    // ==================== Gold Usage ====================

    /// Get feature cost
    pub async fn get_feature_cost(
        pool: &PgPool,
        feature: GoldFeature,
    ) -> AppResult<Option<GoldFeatureCost>> {
        let cost = sqlx::query_as::<_, GoldFeatureCost>(
            r#"SELECT * FROM gold_feature_costs WHERE feature = $1"#,
        )
        .bind(feature)
        .fetch_optional(pool)
        .await?;

        Ok(cost)
    }

    /// Record gold usage
    pub async fn record_gold_usage(
        pool: &PgPool,
        user_id: Uuid,
        feature: GoldFeature,
        gold_spent: i32,
        target_type: Option<&str>,
        target_id: Option<Uuid>,
        effect_data: Option<serde_json::Value>,
        expires_at: Option<DateTime<Utc>>,
    ) -> AppResult<GoldUsage> {
        let usage = sqlx::query_as::<_, GoldUsage>(
            r#"
            INSERT INTO gold_usage (user_id, feature, gold_spent, target_type, target_id, effect_data, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(feature)
        .bind(gold_spent)
        .bind(target_type)
        .bind(target_id)
        .bind(effect_data)
        .bind(expires_at)
        .fetch_one(pool)
        .await?;

        Ok(usage)
    }

    /// Check if user has active production bonus for a village/resource
    pub async fn has_active_production_bonus(
        pool: &PgPool,
        user_id: Uuid,
        village_id: Uuid,
        resource_type: &str,
    ) -> AppResult<bool> {
        let result: Option<(Uuid,)> = sqlx::query_as(
            r#"
            SELECT id FROM gold_usage
            WHERE user_id = $1
                AND feature = 'production_bonus'
                AND target_id = $2
                AND effect_data->>'resource_type' = $3
                AND expires_at > NOW()
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .bind(village_id)
        .bind(resource_type)
        .fetch_optional(pool)
        .await?;

        Ok(result.is_some())
    }

    /// Check if user has active Book of Wisdom
    pub async fn has_active_book_of_wisdom(
        pool: &PgPool,
        user_id: Uuid,
        village_id: Uuid,
    ) -> AppResult<bool> {
        let result: Option<(Uuid,)> = sqlx::query_as(
            r#"
            SELECT id FROM gold_usage
            WHERE user_id = $1
                AND feature = 'book_of_wisdom'
                AND target_id = $2
                AND expires_at > NOW()
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .bind(village_id)
        .fetch_optional(pool)
        .await?;

        Ok(result.is_some())
    }

    /// Get user's gold usage history
    pub async fn get_user_gold_usage(
        pool: &PgPool,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<GoldUsage>> {
        let usage = sqlx::query_as::<_, GoldUsage>(
            r#"
            SELECT * FROM gold_usage
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(usage)
    }

    /// Get production bonus multiplier for a village
    pub async fn get_production_multiplier(
        pool: &PgPool,
        user_id: Uuid,
        village_id: Uuid,
        resource_type: &str,
    ) -> AppResult<f64> {
        let mut multiplier = 1.0;

        // Check Travian Plus (+25%)
        if Self::get_active_subscription(pool, user_id, SubscriptionType::TravianPlus)
            .await?
            .is_some()
        {
            multiplier += 0.25;
        }

        // Check specific production bonus (+25%)
        if Self::has_active_production_bonus(pool, user_id, village_id, resource_type).await? {
            multiplier += 0.25;
        }

        // Check Book of Wisdom (2x = +100%)
        if Self::has_active_book_of_wisdom(pool, user_id, village_id).await? {
            multiplier += 1.0;
        }

        Ok(multiplier)
    }
}
