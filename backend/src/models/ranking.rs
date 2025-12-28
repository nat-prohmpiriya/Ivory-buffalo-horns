use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ==================== Player Rankings ====================

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PlayerPopulationRanking {
    pub rank: i64,
    pub user_id: Uuid,
    pub display_name: Option<String>,
    pub alliance_tag: Option<String>,
    pub population: i64,
    pub village_count: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PlayerAttackRanking {
    pub rank: i64,
    pub user_id: Uuid,
    pub display_name: Option<String>,
    pub alliance_tag: Option<String>,
    pub attack_points: i64,
    pub battles_won: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PlayerDefenseRanking {
    pub rank: i64,
    pub user_id: Uuid,
    pub display_name: Option<String>,
    pub alliance_tag: Option<String>,
    pub defense_points: i64,
    pub battles_defended: i64,
}

// ==================== Hero Rankings ====================

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct HeroRanking {
    pub rank: i64,
    pub hero_id: Uuid,
    pub hero_name: String,
    pub owner_id: Uuid,
    pub owner_name: Option<String>,
    pub level: i32,
    pub experience: i32,
}

// ==================== Alliance Rankings ====================

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AllianceRanking {
    pub rank: i64,
    pub alliance_id: Uuid,
    pub name: String,
    pub tag: String,
    pub member_count: i64,
    pub total_population: i64,
}

// ==================== Response Wrappers ====================

#[derive(Debug, Clone, Serialize)]
pub struct RankingListResponse<T> {
    pub rankings: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

// ==================== Query Params ====================

#[derive(Debug, Clone, Deserialize)]
pub struct RankingQuery {
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
