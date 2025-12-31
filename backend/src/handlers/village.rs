use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::middleware::AuthenticatedUser;
use crate::models::village::{CreateVillage, ProductionRates, UpdateVillage, VillageResponse};
use crate::repositories::user_repo::UserRepository;
use crate::repositories::village_repo::VillageRepository;
use crate::services::resource_service::ResourceService;
use crate::services::village_service::VillageService;
use crate::AppState;

// GET /api/villages - List current user's villages
pub async fn list_villages(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
) -> AppResult<Json<Vec<VillageResponse>>> {
    // Get user from database
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let villages = VillageRepository::find_by_user_id(&state.db, user.id).await?;

    Ok(Json(villages.into_iter().map(|v| v.into()).collect()))
}

// GET /api/villages/:id - Get village detail
pub async fn get_village(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(village_id): Path<Uuid>,
) -> AppResult<Json<VillageResponse>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let village = VillageRepository::find_by_id(&state.db, village_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Village not found".to_string()))?;

    // Check ownership
    if village.user_id != user.id {
        return Err(AppError::Forbidden("Access denied".into()));
    }

    // Update resources based on time elapsed before returning
    let village = ResourceService::update_village_resources(&state.db, village_id).await?;

    // Calculate production rates
    let production = ResourceService::calculate_production(&state.db, village_id).await?;
    let production_rates = ProductionRates {
        wood_per_hour: production.wood_per_hour,
        clay_per_hour: production.clay_per_hour,
        iron_per_hour: production.iron_per_hour,
        crop_per_hour: production.crop_per_hour,
        crop_consumption: production.crop_consumption,
        net_crop_per_hour: production.net_crop_per_hour,
    };

    let response: VillageResponse = village.into();
    Ok(Json(response.with_production(production_rates)))
}

#[derive(Debug, Deserialize)]
pub struct CreateVillageRequest {
    pub name: String,
    pub x: i32,
    pub y: i32,
}

// POST /api/villages - Create new village (for settling)
pub async fn create_village(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Json(body): Json<CreateVillageRequest>,
) -> AppResult<Json<VillageResponse>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // Check if coordinates are available
    if !VillageRepository::is_coordinate_available(&state.db, body.x, body.y).await? {
        return Err(AppError::Conflict("Coordinates already occupied".to_string()));
    }

    // Check if this is the first village (capital)
    let village_count = VillageRepository::count_by_user_id(&state.db, user.id).await?;
    let is_capital = village_count == 0;

    let create_village = CreateVillage {
        user_id: user.id,
        name: body.name,
        x: body.x,
        y: body.y,
        is_capital,
    };

    // Create village with initial buildings
    let (village, buildings) = VillageService::create_village_with_buildings(&state.db, create_village).await?;

    info!(
        "Village created: {} at ({}, {}) for user {} with {} initial buildings",
        village.name, village.x, village.y, user.id, buildings.len()
    );

    Ok(Json(village.into()))
}

#[derive(Debug, Deserialize)]
pub struct UpdateVillageRequest {
    pub name: Option<String>,
}

// PUT /api/villages/:id - Update village
pub async fn update_village(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(village_id): Path<Uuid>,
    Json(body): Json<UpdateVillageRequest>,
) -> AppResult<Json<VillageResponse>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let village = VillageRepository::find_by_id(&state.db, village_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Village not found".to_string()))?;

    if village.user_id != user.id {
        return Err(AppError::Forbidden("Access denied".into()));
    }

    let update = UpdateVillage { name: body.name };
    let updated = VillageRepository::update(&state.db, village_id, update).await?;

    Ok(Json(updated.into()))
}

// Map endpoints

#[derive(Debug, Deserialize)]
pub struct MapQuery {
    pub x: i32,
    pub y: i32,
    #[serde(default = "default_range")]
    pub range: i32,
}

fn default_range() -> i32 {
    7
}

#[derive(Debug, Serialize)]
pub struct MapTileResponse {
    pub x: i32,
    pub y: i32,
    pub village: Option<MapVillageInfo>,
}

#[derive(Debug, Serialize)]
pub struct MapVillageInfo {
    pub id: Uuid,
    pub name: String,
    pub player_name: Option<String>,
    pub population: i32,
    pub is_own: bool,
}

// GET /api/map - Get map tiles around coordinates
pub async fn get_map(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Query(query): Query<MapQuery>,
) -> AppResult<Json<Vec<MapTileResponse>>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // Limit range to prevent abuse
    let range = query.range.min(15).max(1);

    let villages = VillageRepository::find_in_range(&state.db, query.x, query.y, range).await?;

    // Generate tiles for the range
    let mut tiles = Vec::new();
    for dy in -range..=range {
        for dx in -range..=range {
            let x = query.x + dx;
            let y = query.y + dy;

            let village = villages.iter().find(|v| v.x == x && v.y == y);

            tiles.push(MapTileResponse {
                x,
                y,
                village: village.map(|v| MapVillageInfo {
                    id: v.id,
                    name: v.name.clone(),
                    player_name: v.player_name.clone(),
                    population: v.population,
                    is_own: v.user_id == user.id,
                }),
            });
        }
    }

    Ok(Json(tiles))
}

// ==================== Map Search ====================

#[derive(Debug, Deserialize)]
pub struct MapSearchQuery {
    pub q: String,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

fn default_limit() -> i32 {
    20
}

#[derive(Debug, Serialize)]
pub struct MapSearchResult {
    pub result_type: String, // "player", "village", "alliance"
    pub id: Uuid,
    pub name: String,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub population: Option<i32>,
    pub player_name: Option<String>,
    pub alliance_tag: Option<String>,
    pub member_count: Option<i32>,
}

// GET /api/map/search?q=... - Search players, villages, alliances
pub async fn search_map(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Query(query): Query<MapSearchQuery>,
) -> AppResult<Json<Vec<MapSearchResult>>> {
    // Verify user is authenticated
    let _user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let search_term = query.q.trim();
    if search_term.is_empty() {
        return Ok(Json(vec![]));
    }

    let limit = query.limit.min(50).max(1);
    let mut results = Vec::new();

    // Search villages by name
    let villages = VillageRepository::search_by_name(&state.db, search_term, limit).await?;
    for v in villages {
        results.push(MapSearchResult {
            result_type: "village".to_string(),
            id: v.id,
            name: v.name,
            x: Some(v.x),
            y: Some(v.y),
            population: Some(v.population),
            player_name: v.player_name,
            alliance_tag: None,
            member_count: None,
        });
    }

    // Search players by name
    let players = VillageRepository::search_players(&state.db, search_term, limit).await?;
    for p in players {
        results.push(MapSearchResult {
            result_type: "player".to_string(),
            id: p.user_id,
            name: p.player_name.unwrap_or_default(),
            x: p.x,
            y: p.y,
            population: Some(p.total_population),
            player_name: None,
            alliance_tag: None,
            member_count: None,
        });
    }

    // Search alliances by name or tag
    let alliances = VillageRepository::search_alliances(&state.db, search_term, limit).await?;
    for a in alliances {
        results.push(MapSearchResult {
            result_type: "alliance".to_string(),
            id: a.id,
            name: a.name,
            x: None,
            y: None,
            population: None,
            player_name: None,
            alliance_tag: Some(a.tag),
            member_count: Some(a.member_count),
        });
    }

    // Sort by relevance (exact matches first)
    let search_lower = search_term.to_lowercase();
    results.sort_by(|a, b| {
        let a_exact = a.name.to_lowercase() == search_lower;
        let b_exact = b.name.to_lowercase() == search_lower;
        b_exact.cmp(&a_exact)
    });

    // Limit total results
    results.truncate(limit as usize);

    Ok(Json(results))
}
