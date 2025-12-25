use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::PgPool;
use stripe_rust::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData, Currency,
};
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::shop::{
    CheckoutResponse, GoldBalanceResponse, GoldFeature, GoldPackage, SubscriptionPrice,
    SubscriptionType, TransactionResponse, TransactionStatus, TransactionType, UseFeatureResponse,
};
use crate::repositories::building_repo::BuildingRepository;
use crate::repositories::shop_repo::ShopRepository;
use crate::repositories::troop_repo::TroopRepository;
use crate::repositories::village_repo::VillageRepository;

pub struct ShopService;

impl ShopService {
    // ==================== Gold Packages ====================

    /// Get all available gold packages
    pub async fn get_gold_packages(pool: &PgPool) -> AppResult<Vec<GoldPackage>> {
        ShopRepository::get_gold_packages(pool).await
    }

    /// Get user's gold balance and subscription status
    pub async fn get_balance(pool: &PgPool, user_id: Uuid) -> AppResult<GoldBalanceResponse> {
        let gold_balance = ShopRepository::get_gold_balance(pool, user_id).await?;

        let plus_sub =
            ShopRepository::get_active_subscription(pool, user_id, SubscriptionType::TravianPlus)
                .await?;

        Ok(GoldBalanceResponse {
            gold_balance,
            has_plus: plus_sub.is_some(),
            plus_expires_at: plus_sub.map(|s| s.expires_at),
        })
    }

    // ==================== Stripe Checkout ====================

    /// Create Stripe checkout session for gold purchase
    pub async fn create_checkout(
        pool: &PgPool,
        stripe_client: &Client,
        user_id: Uuid,
        package_id: Uuid,
        success_url: &str,
        cancel_url: &str,
    ) -> AppResult<CheckoutResponse> {
        // Get the package
        let package = ShopRepository::get_gold_package(pool, package_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Gold package not found".into()))?;

        if !package.is_active {
            return Err(AppError::BadRequest("This package is not available".into()));
        }

        // Calculate total gold including bonus
        let bonus_gold = (package.gold_amount * package.bonus_percent) / 100;
        let total_gold = package.gold_amount + bonus_gold;

        // Create pending transaction
        let transaction = ShopRepository::create_transaction(
            pool,
            user_id,
            TransactionType::GoldPurchase,
            total_gold,
            Some(package.price_cents),
            Some(&package.currency),
            None, // Will be updated after checkout created
            Some(package_id),
            Some(&format!("Purchase {} Gold", total_gold)),
        )
        .await?;

        // Create Stripe checkout session
        let client_reference_id = transaction.id.to_string();
        let mut params = CreateCheckoutSession::new();
        params.mode = Some(CheckoutSessionMode::Payment);
        params.success_url = Some(success_url);
        params.cancel_url = Some(cancel_url);
        params.client_reference_id = Some(&client_reference_id);

        let line_item = CreateCheckoutSessionLineItems {
            price_data: Some(CreateCheckoutSessionLineItemsPriceData {
                currency: Currency::USD,
                unit_amount: Some(package.price_cents as i64),
                product_data: Some(CreateCheckoutSessionLineItemsPriceDataProductData {
                    name: format!("{} Gold", total_gold),
                    description: if bonus_gold > 0 {
                        Some(format!(
                            "{} Gold + {} Bonus Gold ({}% extra)",
                            package.gold_amount, bonus_gold, package.bonus_percent
                        ))
                    } else {
                        Some(format!("{} Gold for your account", total_gold))
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
            quantity: Some(1),
            ..Default::default()
        };
        params.line_items = Some(vec![line_item]);

        let session = CheckoutSession::create(stripe_client, params)
            .await
            .map_err(|e| AppError::InternalError(anyhow::anyhow!("Stripe error: {}", e)))?;

        // Update transaction with session ID
        sqlx::query(
            r#"UPDATE transactions SET stripe_session_id = $1 WHERE id = $2"#,
        )
        .bind(&session.id.as_str())
        .bind(transaction.id)
        .execute(pool)
        .await?;

        Ok(CheckoutResponse {
            checkout_url: session.url.unwrap_or_default(),
            session_id: session.id.to_string(),
        })
    }

    /// Handle Stripe webhook
    pub async fn handle_webhook(
        pool: &PgPool,
        payload: &str,
        signature: &str,
        webhook_secret: &str,
    ) -> AppResult<()> {
        // Verify signature manually
        Self::verify_webhook_signature(payload, signature, webhook_secret)?;

        // Parse the event
        let event: serde_json::Value = serde_json::from_str(payload)
            .map_err(|e| AppError::BadRequest(format!("Invalid JSON: {}", e)))?;

        let event_type = event["type"].as_str().unwrap_or("");

        match event_type {
            "checkout.session.completed" => {
                let session_id = event["data"]["object"]["id"].as_str().unwrap_or("");
                let payment_intent = event["data"]["object"]["payment_intent"].as_str();
                Self::complete_checkout_by_id(pool, session_id, payment_intent).await?;
            }
            "checkout.session.expired" => {
                let session_id = event["data"]["object"]["id"].as_str().unwrap_or("");
                Self::expire_checkout_by_id(pool, session_id).await?;
            }
            _ => {
                tracing::debug!("Unhandled webhook event: {}", event_type);
            }
        }

        Ok(())
    }

    /// Verify Stripe webhook signature
    fn verify_webhook_signature(
        payload: &str,
        signature: &str,
        secret: &str,
    ) -> AppResult<()> {
        // Parse the signature header
        let mut timestamp: Option<&str> = None;
        let mut sig: Option<&str> = None;

        for part in signature.split(',') {
            let kv: Vec<&str> = part.splitn(2, '=').collect();
            if kv.len() == 2 {
                match kv[0] {
                    "t" => timestamp = Some(kv[1]),
                    "v1" => sig = Some(kv[1]),
                    _ => {}
                }
            }
        }

        let timestamp = timestamp
            .ok_or_else(|| AppError::BadRequest("Missing timestamp in signature".into()))?;
        let sig = sig
            .ok_or_else(|| AppError::BadRequest("Missing signature".into()))?;

        // Compute expected signature
        let signed_payload = format!("{}.{}", timestamp, payload);

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|_| AppError::InternalError(anyhow::anyhow!("Invalid secret key")))?;
        mac.update(signed_payload.as_bytes());

        let expected = hex::encode(mac.finalize().into_bytes());

        if expected != sig {
            return Err(AppError::BadRequest("Invalid signature".into()));
        }

        // Check timestamp (within 5 minutes)
        let ts: i64 = timestamp.parse()
            .map_err(|_| AppError::BadRequest("Invalid timestamp".into()))?;
        let now = Utc::now().timestamp();
        if (now - ts).abs() > 300 {
            return Err(AppError::BadRequest("Webhook timestamp too old".into()));
        }

        Ok(())
    }

    /// Complete checkout and credit gold (by session ID)
    async fn complete_checkout_by_id(
        pool: &PgPool,
        session_id: &str,
        payment_intent_id: Option<&str>,
    ) -> AppResult<()> {
        // Find the transaction
        let transaction = ShopRepository::get_transaction_by_session(pool, session_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;

        if transaction.status != TransactionStatus::Pending {
            tracing::warn!("Transaction {} already processed", transaction.id);
            return Ok(());
        }

        // Credit gold to user
        ShopRepository::add_gold(pool, transaction.user_id, transaction.gold_amount).await?;

        // Update transaction status
        ShopRepository::update_transaction_status(
            pool,
            transaction.id,
            TransactionStatus::Completed,
            payment_intent_id,
        )
        .await?;

        tracing::info!(
            "Gold purchase completed: {} gold for user {}",
            transaction.gold_amount,
            transaction.user_id
        );

        Ok(())
    }

    /// Mark checkout as expired/failed (by session ID)
    async fn expire_checkout_by_id(pool: &PgPool, session_id: &str) -> AppResult<()> {
        if let Some(transaction) =
            ShopRepository::get_transaction_by_session(pool, session_id).await?
        {
            if transaction.status == TransactionStatus::Pending {
                ShopRepository::update_transaction_status(
                    pool,
                    transaction.id,
                    TransactionStatus::Failed,
                    None,
                )
                .await?;
            }
        }

        Ok(())
    }

    // ==================== Subscriptions ====================

    /// Get subscription prices
    pub async fn get_subscription_prices(pool: &PgPool) -> AppResult<Vec<SubscriptionPrice>> {
        ShopRepository::get_subscription_prices(pool, SubscriptionType::TravianPlus).await
    }

    /// Buy Travian Plus subscription with gold
    pub async fn buy_subscription(
        pool: &PgPool,
        user_id: Uuid,
        duration_days: i32,
    ) -> AppResult<UseFeatureResponse> {
        // Get subscription price
        let prices =
            ShopRepository::get_subscription_prices(pool, SubscriptionType::TravianPlus).await?;

        let price = prices
            .into_iter()
            .find(|p| p.duration_days == duration_days)
            .ok_or_else(|| AppError::BadRequest("Invalid subscription duration".into()))?;

        // Check gold balance
        let balance = ShopRepository::get_gold_balance(pool, user_id).await?;
        if balance < price.gold_cost {
            return Err(AppError::BadRequest("Insufficient gold".into()));
        }

        // Deduct gold
        let new_balance = ShopRepository::deduct_gold(pool, user_id, price.gold_cost).await?;

        // Create or extend subscription
        let subscription = ShopRepository::create_or_extend_subscription(
            pool,
            user_id,
            SubscriptionType::TravianPlus,
            duration_days,
        )
        .await?;

        // Record transaction
        ShopRepository::create_transaction(
            pool,
            user_id,
            TransactionType::Subscription,
            -price.gold_cost,
            None,
            None,
            None,
            None,
            Some(&format!(
                "Travian Plus {} days subscription",
                duration_days
            )),
        )
        .await?;

        // Record gold usage
        ShopRepository::record_gold_usage(
            pool,
            user_id,
            GoldFeature::PlusSubscription,
            price.gold_cost,
            None,
            None,
            Some(serde_json::json!({
                "duration_days": duration_days,
                "expires_at": subscription.expires_at
            })),
            Some(subscription.expires_at),
        )
        .await?;

        Ok(UseFeatureResponse {
            success: true,
            gold_spent: price.gold_cost,
            new_balance,
            message: format!(
                "Travian Plus activated until {}",
                subscription.expires_at.format("%Y-%m-%d %H:%M")
            ),
        })
    }

    // ==================== Gold Features ====================

    /// Use "Finish Now" to instantly complete a building or training
    pub async fn use_finish_now(
        pool: &PgPool,
        user_id: Uuid,
        target_type: &str,
        target_id: Uuid,
    ) -> AppResult<UseFeatureResponse> {
        // Calculate cost based on remaining time
        let (remaining_seconds, village_id) = match target_type {
            "building" => {
                let building = BuildingRepository::find_by_id(pool, target_id)
                    .await?
                    .ok_or_else(|| AppError::NotFound("Building not found".into()))?;

                if !building.is_upgrading {
                    return Err(AppError::BadRequest("Building is not upgrading".into()));
                }

                let remaining = building
                    .upgrade_ends_at
                    .map(|ends| (ends - Utc::now()).num_seconds().max(0) as i32)
                    .unwrap_or(0);

                (remaining, building.village_id)
            }
            "troop_queue" => {
                let queue = TroopRepository::find_queue_by_id(pool, target_id)
                    .await?
                    .ok_or_else(|| AppError::NotFound("Training queue not found".into()))?;

                let remaining = (queue.ends_at - Utc::now()).num_seconds().max(0) as i32;

                (remaining, queue.village_id)
            }
            _ => return Err(AppError::BadRequest("Invalid target type".into())),
        };

        // Verify ownership
        let village = VillageRepository::find_by_id(pool, village_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Village not found".into()))?;

        if village.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        // Calculate gold cost: 1 gold per 5 minutes (300 seconds), minimum 1 gold
        let gold_cost = ((remaining_seconds as f64 / 300.0).ceil() as i32).max(1);

        // Check gold balance
        let balance = ShopRepository::get_gold_balance(pool, user_id).await?;
        if balance < gold_cost {
            return Err(AppError::BadRequest("Insufficient gold".into()));
        }

        // Deduct gold
        let new_balance = ShopRepository::deduct_gold(pool, user_id, gold_cost).await?;

        // Complete the target instantly
        match target_type {
            "building" => {
                BuildingRepository::complete_upgrade(pool, target_id).await?;
            }
            "troop_queue" => {
                TroopRepository::complete_training(pool, target_id).await?;
            }
            _ => {}
        }

        // Record transaction
        ShopRepository::create_transaction(
            pool,
            user_id,
            TransactionType::GoldSpend,
            -gold_cost,
            None,
            None,
            None,
            None,
            Some(&format!("Finish Now - {}", target_type)),
        )
        .await?;

        // Record usage
        ShopRepository::record_gold_usage(
            pool,
            user_id,
            GoldFeature::FinishNow,
            gold_cost,
            Some(target_type),
            Some(target_id),
            Some(serde_json::json!({ "saved_seconds": remaining_seconds })),
            None,
        )
        .await?;

        Ok(UseFeatureResponse {
            success: true,
            gold_spent: gold_cost,
            new_balance,
            message: format!("{} completed instantly!", target_type),
        })
    }

    /// Use NPC Merchant to exchange resources
    pub async fn use_npc_merchant(
        pool: &PgPool,
        user_id: Uuid,
        village_id: Uuid,
        wood: i32,
        clay: i32,
        iron: i32,
        crop: i32,
    ) -> AppResult<UseFeatureResponse> {
        let gold_cost = 3;

        // Verify village ownership
        let village = VillageRepository::find_by_id(pool, village_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Village not found".into()))?;

        if village.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        // Validate that total resources remain the same
        let current_total =
            village.wood as i32 + village.clay as i32 + village.iron as i32 + village.crop as i32;
        let new_total = wood + clay + iron + crop;

        if new_total != current_total {
            return Err(AppError::BadRequest(
                "Total resources must remain the same".into(),
            ));
        }

        // Validate no negative resources
        if wood < 0 || clay < 0 || iron < 0 || crop < 0 {
            return Err(AppError::BadRequest(
                "Resources cannot be negative".into(),
            ));
        }

        // Check warehouse capacity
        let max_storage = village.warehouse_capacity as i32;
        if wood > max_storage || clay > max_storage || iron > max_storage {
            return Err(AppError::BadRequest(
                "Resources exceed warehouse capacity".into(),
            ));
        }

        let max_granary = village.granary_capacity as i32;
        if crop > max_granary {
            return Err(AppError::BadRequest(
                "Crop exceeds granary capacity".into(),
            ));
        }

        // Check gold balance
        let balance = ShopRepository::get_gold_balance(pool, user_id).await?;
        if balance < gold_cost {
            return Err(AppError::BadRequest("Insufficient gold".into()));
        }

        // Deduct gold
        let new_balance = ShopRepository::deduct_gold(pool, user_id, gold_cost).await?;

        // Update village resources
        sqlx::query(
            r#"
            UPDATE villages
            SET wood = $2, clay = $3, iron = $4, crop = $5, updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(village_id)
        .bind(wood as f64)
        .bind(clay as f64)
        .bind(iron as f64)
        .bind(crop as f64)
        .execute(pool)
        .await?;

        // Record transaction
        ShopRepository::create_transaction(
            pool,
            user_id,
            TransactionType::GoldSpend,
            -gold_cost,
            None,
            None,
            None,
            None,
            Some("NPC Merchant - Resource exchange"),
        )
        .await?;

        // Record usage
        ShopRepository::record_gold_usage(
            pool,
            user_id,
            GoldFeature::NpcMerchant,
            gold_cost,
            Some("village"),
            Some(village_id),
            Some(serde_json::json!({
                "from": {
                    "wood": village.wood,
                    "clay": village.clay,
                    "iron": village.iron,
                    "crop": village.crop
                },
                "to": {
                    "wood": wood,
                    "clay": clay,
                    "iron": iron,
                    "crop": crop
                }
            })),
            None,
        )
        .await?;

        Ok(UseFeatureResponse {
            success: true,
            gold_spent: gold_cost,
            new_balance,
            message: "Resources exchanged successfully!".into(),
        })
    }

    /// Use +25% Production Bonus for one resource type
    pub async fn use_production_bonus(
        pool: &PgPool,
        user_id: Uuid,
        village_id: Uuid,
        resource_type: &str,
    ) -> AppResult<UseFeatureResponse> {
        let gold_cost = 5;
        let duration_hours = 24;

        // Validate resource type
        if !["wood", "clay", "iron", "crop"].contains(&resource_type) {
            return Err(AppError::BadRequest("Invalid resource type".into()));
        }

        // Verify village ownership
        let village = VillageRepository::find_by_id(pool, village_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Village not found".into()))?;

        if village.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        // Check if already has active bonus for this resource
        if ShopRepository::has_active_production_bonus(pool, user_id, village_id, resource_type)
            .await?
        {
            return Err(AppError::BadRequest(
                "Already have active bonus for this resource".into(),
            ));
        }

        // Check gold balance
        let balance = ShopRepository::get_gold_balance(pool, user_id).await?;
        if balance < gold_cost {
            return Err(AppError::BadRequest("Insufficient gold".into()));
        }

        // Deduct gold
        let new_balance = ShopRepository::deduct_gold(pool, user_id, gold_cost).await?;

        let expires_at = Utc::now() + Duration::hours(duration_hours);

        // Record transaction
        ShopRepository::create_transaction(
            pool,
            user_id,
            TransactionType::GoldSpend,
            -gold_cost,
            None,
            None,
            None,
            None,
            Some(&format!("+25% {} production bonus", resource_type)),
        )
        .await?;

        // Record usage
        ShopRepository::record_gold_usage(
            pool,
            user_id,
            GoldFeature::ProductionBonus,
            gold_cost,
            Some("village"),
            Some(village_id),
            Some(serde_json::json!({ "resource_type": resource_type })),
            Some(expires_at),
        )
        .await?;

        Ok(UseFeatureResponse {
            success: true,
            gold_spent: gold_cost,
            new_balance,
            message: format!(
                "+25% {} production bonus activated for 24 hours!",
                resource_type
            ),
        })
    }

    /// Use Book of Wisdom for 2x all production
    pub async fn use_book_of_wisdom(
        pool: &PgPool,
        user_id: Uuid,
        village_id: Uuid,
    ) -> AppResult<UseFeatureResponse> {
        let gold_cost = 15;
        let duration_hours = 24;

        // Verify village ownership
        let village = VillageRepository::find_by_id(pool, village_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Village not found".into()))?;

        if village.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        // Check if already active
        if ShopRepository::has_active_book_of_wisdom(pool, user_id, village_id).await? {
            return Err(AppError::BadRequest(
                "Book of Wisdom already active for this village".into(),
            ));
        }

        // Check gold balance
        let balance = ShopRepository::get_gold_balance(pool, user_id).await?;
        if balance < gold_cost {
            return Err(AppError::BadRequest("Insufficient gold".into()));
        }

        // Deduct gold
        let new_balance = ShopRepository::deduct_gold(pool, user_id, gold_cost).await?;

        let expires_at = Utc::now() + Duration::hours(duration_hours);

        // Record transaction
        ShopRepository::create_transaction(
            pool,
            user_id,
            TransactionType::GoldSpend,
            -gold_cost,
            None,
            None,
            None,
            None,
            Some("Book of Wisdom - 2x production"),
        )
        .await?;

        // Record usage
        ShopRepository::record_gold_usage(
            pool,
            user_id,
            GoldFeature::BookOfWisdom,
            gold_cost,
            Some("village"),
            Some(village_id),
            None,
            Some(expires_at),
        )
        .await?;

        Ok(UseFeatureResponse {
            success: true,
            gold_spent: gold_cost,
            new_balance,
            message: "Book of Wisdom activated! 2x production for 24 hours!".into(),
        })
    }

    // ==================== Transaction History ====================

    /// Get user's transaction history
    pub async fn get_transactions(
        pool: &PgPool,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> AppResult<Vec<TransactionResponse>> {
        let limit = limit.min(50).max(1);
        let transactions = ShopRepository::get_user_transactions(pool, user_id, limit, offset).await?;
        Ok(transactions.into_iter().map(|t| t.into()).collect())
    }
}
