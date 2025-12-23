use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::building::BuildingType;
use crate::models::village::Village;
use crate::repositories::building_repo::BuildingRepository;
use crate::repositories::village_repo::VillageRepository;

pub struct ResourceService;

#[derive(Debug, Clone)]
pub struct ProductionRates {
    pub wood_per_hour: i32,
    pub clay_per_hour: i32,
    pub iron_per_hour: i32,
    pub crop_per_hour: i32,
    pub crop_consumption: i32,  // Population eats crop
    pub net_crop_per_hour: i32, // crop_per_hour - crop_consumption
}

impl ResourceService {
    /// Calculate production rates for a village based on its buildings
    pub async fn calculate_production(
        pool: &PgPool,
        village_id: Uuid,
    ) -> AppResult<ProductionRates> {
        let village = VillageRepository::find_by_id(pool, village_id)
            .await?
            .ok_or_else(|| crate::error::AppError::NotFound("Village not found".to_string()))?;

        let buildings = BuildingRepository::find_by_village_id(pool, village_id).await?;

        let mut wood_per_hour = 3; // Base production
        let mut clay_per_hour = 3;
        let mut iron_per_hour = 3;
        let mut crop_per_hour = 3;

        for building in buildings {
            if building.level == 0 {
                continue;
            }

            let production = building.building_type.production_per_hour(building.level);

            match building.building_type {
                BuildingType::Woodcutter => wood_per_hour += production,
                BuildingType::ClayPit => clay_per_hour += production,
                BuildingType::IronMine => iron_per_hour += production,
                BuildingType::CropField => crop_per_hour += production,
                _ => {}
            }
        }

        // Population consumes crop (1 crop per population per hour)
        let crop_consumption = village.population;
        let net_crop_per_hour = crop_per_hour - crop_consumption;

        Ok(ProductionRates {
            wood_per_hour,
            clay_per_hour,
            iron_per_hour,
            crop_per_hour,
            crop_consumption,
            net_crop_per_hour,
        })
    }

    /// Update resources for a village based on time elapsed
    pub async fn update_village_resources(pool: &PgPool, village_id: Uuid) -> AppResult<Village> {
        let village = VillageRepository::find_by_id(pool, village_id)
            .await?
            .ok_or_else(|| crate::error::AppError::NotFound("Village not found".to_string()))?;

        let now = Utc::now();
        let elapsed_seconds = (now - village.resources_updated_at).num_seconds();

        if elapsed_seconds <= 0 {
            return Ok(village);
        }

        let production = Self::calculate_production(pool, village_id).await?;

        // Calculate resources produced
        let hours_elapsed = elapsed_seconds as f64 / 3600.0;

        let wood_produced = (production.wood_per_hour as f64 * hours_elapsed) as i32;
        let clay_produced = (production.clay_per_hour as f64 * hours_elapsed) as i32;
        let iron_produced = (production.iron_per_hour as f64 * hours_elapsed) as i32;
        // Use net_crop which accounts for population consumption
        let crop_change = (production.net_crop_per_hour as f64 * hours_elapsed) as i32;

        // Calculate new resource amounts (capped at storage, min 0)
        let new_wood = (village.wood + wood_produced).min(village.warehouse_capacity).max(0);
        let new_clay = (village.clay + clay_produced).min(village.warehouse_capacity).max(0);
        let new_iron = (village.iron + iron_produced).min(village.warehouse_capacity).max(0);
        let new_crop = (village.crop + crop_change).min(village.granary_capacity).max(0);

        // Update village resources
        let updated =
            VillageRepository::update_resources(pool, village_id, new_wood, new_clay, new_iron, new_crop)
                .await?;

        Ok(updated)
    }

    /// Update resources for all villages (for background job)
    pub async fn update_all_village_resources(pool: &PgPool) -> AppResult<i32> {
        // Get all villages that need updating (not updated in last minute)
        let villages: Vec<(Uuid,)> = sqlx::query_as(
            r#"
            SELECT id FROM villages
            WHERE resources_updated_at < NOW() - INTERVAL '1 minute'
            "#,
        )
        .fetch_all(pool)
        .await?;

        let mut updated_count = 0;

        for (village_id,) in villages {
            if let Ok(_) = Self::update_village_resources(pool, village_id).await {
                updated_count += 1;
            }
        }

        Ok(updated_count)
    }
}
