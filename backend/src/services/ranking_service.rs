use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::ranking::{
    AllianceRanking, HeroRanking, PlayerAttackRanking, PlayerDefenseRanking,
    PlayerPopulationRanking, RankingListResponse,
};
use crate::repositories::ranking_repo::RankingRepository;

pub struct RankingService;

impl RankingService {
    // ==================== Player Population Ranking ====================

    /// Get player population rankings with pagination
    pub async fn get_population_ranking(
        pool: &PgPool,
        page: i64,
        per_page: i64,
    ) -> AppResult<RankingListResponse<PlayerPopulationRanking>> {
        let offset = (page - 1) * per_page;
        let rankings = RankingRepository::get_population_ranking(pool, per_page, offset).await?;
        let total = RankingRepository::count_population_ranking(pool).await?;

        Ok(RankingListResponse {
            rankings,
            total,
            page,
            per_page,
        })
    }

    // ==================== Player Attack Ranking ====================

    /// Get player attack rankings with pagination
    pub async fn get_attack_ranking(
        pool: &PgPool,
        page: i64,
        per_page: i64,
    ) -> AppResult<RankingListResponse<PlayerAttackRanking>> {
        let offset = (page - 1) * per_page;
        let rankings = RankingRepository::get_attack_ranking(pool, per_page, offset).await?;
        let total = RankingRepository::count_attack_ranking(pool).await?;

        Ok(RankingListResponse {
            rankings,
            total,
            page,
            per_page,
        })
    }

    // ==================== Player Defense Ranking ====================

    /// Get player defense rankings with pagination
    pub async fn get_defense_ranking(
        pool: &PgPool,
        page: i64,
        per_page: i64,
    ) -> AppResult<RankingListResponse<PlayerDefenseRanking>> {
        let offset = (page - 1) * per_page;
        let rankings = RankingRepository::get_defense_ranking(pool, per_page, offset).await?;
        let total = RankingRepository::count_defense_ranking(pool).await?;

        Ok(RankingListResponse {
            rankings,
            total,
            page,
            per_page,
        })
    }

    // ==================== Hero Ranking ====================

    /// Get hero rankings with pagination
    pub async fn get_hero_ranking(
        pool: &PgPool,
        page: i64,
        per_page: i64,
    ) -> AppResult<RankingListResponse<HeroRanking>> {
        let offset = (page - 1) * per_page;
        let rankings = RankingRepository::get_hero_ranking(pool, per_page, offset).await?;
        let total = RankingRepository::count_hero_ranking(pool).await?;

        Ok(RankingListResponse {
            rankings,
            total,
            page,
            per_page,
        })
    }

    // ==================== Alliance Ranking ====================

    /// Get alliance rankings with pagination
    pub async fn get_alliance_ranking(
        pool: &PgPool,
        page: i64,
        per_page: i64,
    ) -> AppResult<RankingListResponse<AllianceRanking>> {
        let offset = (page - 1) * per_page;
        let rankings = RankingRepository::get_alliance_ranking(pool, per_page, offset).await?;
        let total = RankingRepository::count_alliance_ranking(pool).await?;

        Ok(RankingListResponse {
            rankings,
            total,
            page,
            per_page,
        })
    }

    // ==================== Player Position ====================

    /// Get a specific player's rank
    pub async fn get_player_rank(pool: &PgPool, user_id: Uuid) -> AppResult<Option<i64>> {
        RankingRepository::get_player_population_rank(pool, user_id).await
    }
}
