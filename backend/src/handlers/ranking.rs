use axum::{extract::{Query, State}, Json};

use crate::error::AppResult;
use crate::models::ranking::{
    AllianceRanking, HeroRanking, PlayerAttackRanking, PlayerDefenseRanking,
    PlayerPopulationRanking, RankingListResponse, RankingQuery,
};
use crate::services::ranking_service::RankingService;
use crate::AppState;

// GET /api/rankings/players/population - Top players by population
pub async fn get_population_ranking(
    State(state): State<AppState>,
    Query(query): Query<RankingQuery>,
) -> AppResult<Json<RankingListResponse<PlayerPopulationRanking>>> {
    let rankings = RankingService::get_population_ranking(&state.db, query.page, query.per_page).await?;
    Ok(Json(rankings))
}

// GET /api/rankings/players/attackers - Top attackers
pub async fn get_attack_ranking(
    State(state): State<AppState>,
    Query(query): Query<RankingQuery>,
) -> AppResult<Json<RankingListResponse<PlayerAttackRanking>>> {
    let rankings = RankingService::get_attack_ranking(&state.db, query.page, query.per_page).await?;
    Ok(Json(rankings))
}

// GET /api/rankings/players/defenders - Top defenders
pub async fn get_defense_ranking(
    State(state): State<AppState>,
    Query(query): Query<RankingQuery>,
) -> AppResult<Json<RankingListResponse<PlayerDefenseRanking>>> {
    let rankings = RankingService::get_defense_ranking(&state.db, query.page, query.per_page).await?;
    Ok(Json(rankings))
}

// GET /api/rankings/heroes - Top heroes by level
pub async fn get_hero_ranking(
    State(state): State<AppState>,
    Query(query): Query<RankingQuery>,
) -> AppResult<Json<RankingListResponse<HeroRanking>>> {
    let rankings = RankingService::get_hero_ranking(&state.db, query.page, query.per_page).await?;
    Ok(Json(rankings))
}

// GET /api/rankings/alliances - Top alliances by population
pub async fn get_alliance_ranking(
    State(state): State<AppState>,
    Query(query): Query<RankingQuery>,
) -> AppResult<Json<RankingListResponse<AllianceRanking>>> {
    let rankings = RankingService::get_alliance_ranking(&state.db, query.page, query.per_page).await?;
    Ok(Json(rankings))
}
