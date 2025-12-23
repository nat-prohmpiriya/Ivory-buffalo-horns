use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::building::{Building, BuildingType};
use crate::repositories::building_repo::BuildingRepository;
use crate::repositories::village_repo::VillageRepository;

pub struct BuildingService;

#[derive(Debug)]
pub struct MissingPrerequisite {
    pub building_type: BuildingType,
    pub required_level: i32,
    pub current_level: i32,
}

impl BuildingService {
    /// Check if prerequisites are met for building a new building
    pub async fn check_prerequisites(
        pool: &PgPool,
        village_id: Uuid,
        building_type: &BuildingType,
    ) -> AppResult<Vec<MissingPrerequisite>> {
        let prerequisites = building_type.prerequisites();

        if prerequisites.is_empty() {
            return Ok(vec![]);
        }

        let buildings = BuildingRepository::find_by_village_id(pool, village_id).await?;
        let mut missing = Vec::new();

        for prereq in prerequisites {
            let current_level = buildings
                .iter()
                .filter(|b| b.building_type == prereq.building_type)
                .map(|b| b.level)
                .max()
                .unwrap_or(0);

            if current_level < prereq.min_level {
                missing.push(MissingPrerequisite {
                    building_type: prereq.building_type,
                    required_level: prereq.min_level,
                    current_level,
                });
            }
        }

        Ok(missing)
    }

    /// Validate building can be built (returns error if prerequisites not met)
    pub async fn validate_can_build(
        pool: &PgPool,
        village_id: Uuid,
        building_type: &BuildingType,
    ) -> AppResult<()> {
        let missing = Self::check_prerequisites(pool, village_id, building_type).await?;

        if !missing.is_empty() {
            let msg = missing
                .iter()
                .map(|m| format!("{:?} Lv.{}", m.building_type, m.required_level))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(AppError::BadRequest(format!("Missing prerequisites: {}", msg)));
        }

        Ok(())
    }

    /// Complete a building upgrade and handle side effects
    pub async fn complete_upgrade(pool: &PgPool, building_id: Uuid) -> AppResult<Building> {
        // Complete the upgrade
        let building = BuildingRepository::complete_upgrade(pool, building_id).await?;

        // Handle side effects based on building type
        match building.building_type {
            BuildingType::Warehouse | BuildingType::Granary => {
                Self::update_village_storage(pool, building.village_id).await?;
            }
            _ => {}
        }

        // Always update population after any building upgrade
        Self::update_village_population(pool, building.village_id).await?;

        Ok(building)
    }

    /// Recalculate and update village storage capacity based on all Warehouse/Granary buildings
    pub async fn update_village_storage(pool: &PgPool, village_id: Uuid) -> AppResult<()> {
        let buildings = BuildingRepository::find_by_village_id(pool, village_id).await?;

        let mut warehouse_capacity = 800; // Base capacity
        let mut granary_capacity = 800; // Base capacity

        for building in buildings {
            match building.building_type {
                BuildingType::Warehouse => {
                    warehouse_capacity += building.building_type.storage_capacity(building.level);
                }
                BuildingType::Granary => {
                    granary_capacity += building.building_type.storage_capacity(building.level);
                }
                _ => {}
            }
        }

        VillageRepository::update_storage_capacity(pool, village_id, warehouse_capacity, granary_capacity)
            .await?;

        Ok(())
    }

    /// Recalculate and update village population based on all buildings
    pub async fn update_village_population(pool: &PgPool, village_id: Uuid) -> AppResult<()> {
        let buildings = BuildingRepository::find_by_village_id(pool, village_id).await?;

        let population: i32 = buildings
            .iter()
            .map(|b| b.building_type.population_at_level(b.level))
            .sum();

        VillageRepository::update_population(pool, village_id, population).await?;

        Ok(())
    }
}
