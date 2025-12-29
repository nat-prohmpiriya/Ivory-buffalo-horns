use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ==================== Enums ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "trade_order_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TradeOrderStatus {
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "trade_order_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TradeOrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "trade_resource_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TradeResourceType {
    Wood,
    Clay,
    Iron,
    Crop,
}

impl TradeResourceType {
    pub fn all() -> Vec<TradeResourceType> {
        vec![
            TradeResourceType::Wood,
            TradeResourceType::Clay,
            TradeResourceType::Iron,
            TradeResourceType::Crop,
        ]
    }
}

// ==================== Helper Structs ====================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Resources {
    #[serde(default)]
    pub wood: i32,
    #[serde(default)]
    pub clay: i32,
    #[serde(default)]
    pub iron: i32,
    #[serde(default)]
    pub crop: i32,
}

impl Resources {
    pub fn new(wood: i32, clay: i32, iron: i32, crop: i32) -> Self {
        Self { wood, clay, iron, crop }
    }

    pub fn is_empty(&self) -> bool {
        self.wood == 0 && self.clay == 0 && self.iron == 0 && self.crop == 0
    }

    pub fn total(&self) -> i32 {
        self.wood + self.clay + self.iron + self.crop
    }

    pub fn get(&self, resource_type: TradeResourceType) -> i32 {
        match resource_type {
            TradeResourceType::Wood => self.wood,
            TradeResourceType::Clay => self.clay,
            TradeResourceType::Iron => self.iron,
            TradeResourceType::Crop => self.crop,
        }
    }

    pub fn set(&mut self, resource_type: TradeResourceType, amount: i32) {
        match resource_type {
            TradeResourceType::Wood => self.wood = amount,
            TradeResourceType::Clay => self.clay = amount,
            TradeResourceType::Iron => self.iron = amount,
            TradeResourceType::Crop => self.crop = amount,
        }
    }

    pub fn add(&mut self, resource_type: TradeResourceType, amount: i32) {
        match resource_type {
            TradeResourceType::Wood => self.wood += amount,
            TradeResourceType::Clay => self.clay += amount,
            TradeResourceType::Iron => self.iron += amount,
            TradeResourceType::Crop => self.crop += amount,
        }
    }

    pub fn subtract(&mut self, resource_type: TradeResourceType, amount: i32) {
        match resource_type {
            TradeResourceType::Wood => self.wood -= amount,
            TradeResourceType::Clay => self.clay -= amount,
            TradeResourceType::Iron => self.iron -= amount,
            TradeResourceType::Crop => self.crop -= amount,
        }
    }
}

// ==================== Database Models ====================

/// Trade order record (Market Board listing)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TradeOrder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub village_id: Uuid,
    pub order_type: TradeOrderType,
    pub resource_type: TradeResourceType,
    pub quantity: i32,
    pub quantity_filled: i32,
    pub price_per_unit: i32,
    pub status: TradeOrderStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub filled_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

impl TradeOrder {
    /// Get remaining quantity to be filled
    pub fn quantity_remaining(&self) -> i32 {
        self.quantity - self.quantity_filled
    }

    /// Calculate total cost/revenue in gold
    pub fn total_cost(&self) -> i64 {
        (self.quantity as i64) * (self.price_per_unit as i64)
    }

    /// Calculate remaining cost/revenue in gold
    pub fn remaining_cost(&self) -> i64 {
        (self.quantity_remaining() as i64) * (self.price_per_unit as i64)
    }

    /// Check if order can be cancelled
    pub fn can_cancel(&self) -> bool {
        matches!(self.status, TradeOrderStatus::Open | TradeOrderStatus::PartiallyFilled)
    }

    /// Check if order can be filled
    pub fn can_fill(&self) -> bool {
        matches!(self.status, TradeOrderStatus::Open | TradeOrderStatus::PartiallyFilled)
            && self.quantity_remaining() > 0
    }

    /// Check if order is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }
}

/// Trade order with additional details for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOrderWithDetails {
    #[serde(flatten)]
    pub order: TradeOrder,
    pub village_name: String,
    pub village_x: i32,
    pub village_y: i32,
    pub user_display_name: Option<String>,
}

/// Trade transaction record (completed trade)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TradeTransaction {
    pub id: Uuid,
    pub buy_order_id: Uuid,
    pub sell_order_id: Uuid,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub buyer_village_id: Uuid,
    pub seller_village_id: Uuid,
    pub resource_type: TradeResourceType,
    pub quantity: i32,
    pub price_per_unit: i32,
    pub total_gold: i32,
    pub created_at: DateTime<Utc>,
}

/// Resource lock record (escrow)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ResourceLock {
    pub id: Uuid,
    pub village_id: Uuid,
    pub lock_type: String,
    pub reference_id: Uuid,
    pub wood: i32,
    pub clay: i32,
    pub iron: i32,
    pub crop: i32,
    pub created_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
}

impl ResourceLock {
    pub fn to_resources(&self) -> Resources {
        Resources {
            wood: self.wood,
            clay: self.clay,
            iron: self.iron,
            crop: self.crop,
        }
    }
}

/// Market summary for a resource type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSummary {
    pub resource_type: TradeResourceType,
    pub best_buy_price: Option<i32>,  // highest buy offer
    pub best_sell_price: Option<i32>, // lowest sell offer
    pub spread: Option<i32>,          // difference between best sell and buy
    pub last_trade_price: Option<i32>,
    pub volume_24h: i32,
    pub trade_count_24h: i32,
}

// ==================== Request DTOs ====================

#[derive(Debug, Clone, Deserialize)]
pub struct CreateOrderRequest {
    pub village_id: Uuid,
    pub order_type: TradeOrderType,
    pub resource_type: TradeResourceType,
    pub quantity: i32,
    pub price_per_unit: i32,
    pub expires_in_hours: Option<i32>, // None = no expiry
}

#[derive(Debug, Clone, Deserialize)]
pub struct AcceptOrderRequest {
    pub village_id: Uuid,
    pub quantity: Option<i32>, // None = fill all available
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetOrdersQuery {
    pub resource_type: Option<TradeResourceType>,
    pub order_type: Option<TradeOrderType>,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

// ==================== Response DTOs ====================

#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderResponse {
    pub order: TradeOrder,
    pub locked_resources: Option<Resources>, // for sell orders
    pub locked_gold: Option<i32>,            // for buy orders
}

#[derive(Debug, Clone, Serialize)]
pub struct AcceptOrderResponse {
    pub transaction: TradeTransaction,
    pub order_status: TradeOrderStatus,
    pub resources_received: Option<Resources>,
    pub gold_received: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderResponse {
    pub order: TradeOrder,
    pub refunded_resources: Option<Resources>,
    pub refunded_gold: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MarketSummaryResponse {
    pub summaries: Vec<MarketSummary>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetOrdersResponse {
    pub orders: Vec<TradeOrderWithDetails>,
    pub total: i64,
    pub page: i32,
    pub limit: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct TradeHistoryResponse {
    pub transactions: Vec<TradeTransaction>,
    pub total: i64,
}

/// Response for user's own orders
#[derive(Debug, Clone, Serialize)]
pub struct MyOrdersResponse {
    pub orders: Vec<TradeOrder>,
}
