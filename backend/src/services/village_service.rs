use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::building::{Building, BuildingType, CreateBuilding};
use crate::models::village::{CreateVillage, Village};
use crate::repositories::building_repo::BuildingRepository;
use crate::repositories::village_repo::VillageRepository;

pub struct VillageService;

impl VillageService {
    /// Create a new village with initial buildings
    pub async fn create_village_with_buildings(
        pool: &PgPool,
        input: CreateVillage,
    ) -> AppResult<(Village, Vec<Building>)> {
        // Create village
        let village = VillageRepository::create(pool, input).await?;

        // Create initial buildings
        let buildings = Self::create_initial_buildings(pool, village.id).await?;

        Ok((village, buildings))
    }

    /// Create initial buildings for a new village
    /// Based on Travian's starting layout
    async fn create_initial_buildings(
        pool: &PgPool,
        village_id: Uuid,
    ) -> AppResult<Vec<Building>> {
        let mut buildings = Vec::new();

        // Village center buildings (slots 1-22)
        // Start with Main Building and Rally Point
        let village_buildings = vec![
            (1, BuildingType::MainBuilding, 1),
            (2, BuildingType::RallyPoint, 1),
        ];

        for (slot, building_type, level) in village_buildings {
            let building = create_building_with_level(pool, village_id, slot, building_type, level).await?;
            buildings.push(building);
        }

        // Resource fields (slots 101-118)
        // 4 Woodcutter, 4 Clay Pit, 4 Iron Mine, 6 Crop Field
        let resource_fields = vec![
            // Woodcutters (slots 101-104)
            (101, BuildingType::Woodcutter),
            (102, BuildingType::Woodcutter),
            (103, BuildingType::Woodcutter),
            (104, BuildingType::Woodcutter),
            // Clay Pits (slots 105-108)
            (105, BuildingType::ClayPit),
            (106, BuildingType::ClayPit),
            (107, BuildingType::ClayPit),
            (108, BuildingType::ClayPit),
            // Iron Mines (slots 109-112)
            (109, BuildingType::IronMine),
            (110, BuildingType::IronMine),
            (111, BuildingType::IronMine),
            (112, BuildingType::IronMine),
            // Crop Fields (slots 113-118)
            (113, BuildingType::CropField),
            (114, BuildingType::CropField),
            (115, BuildingType::CropField),
            (116, BuildingType::CropField),
            (117, BuildingType::CropField),
            (118, BuildingType::CropField),
        ];

        for (slot, building_type) in resource_fields {
            let building = create_building_with_level(pool, village_id, slot, building_type, 0).await?;
            buildings.push(building);
        }

        Ok(buildings)
    }

    /// Find a random available coordinate for new village
    pub async fn find_available_coordinates(
        pool: &PgPool,
        near_x: i32,
        near_y: i32,
        max_distance: i32,
    ) -> AppResult<Option<(i32, i32)>> {
        // Search in expanding circles from the center
        for distance in 1..=max_distance {
            for dx in -distance..=distance {
                for dy in -distance..=distance {
                    // Only check the perimeter of each distance
                    if dx.abs() == distance || dy.abs() == distance {
                        let x = near_x + dx;
                        let y = near_y + dy;

                        if VillageRepository::is_coordinate_available(pool, x, y).await? {
                            return Ok(Some((x, y)));
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}

async fn create_building_with_level(
    pool: &PgPool,
    village_id: Uuid,
    slot: i32,
    building_type: BuildingType,
    level: i32,
) -> AppResult<Building> {
    let create = CreateBuilding {
        village_id,
        building_type,
        slot,
    };

    // Create building (starts at level 1 by default)
    let building = BuildingRepository::create(pool, create).await?;

    // If level is different, update it
    if level != 1 {
        // For level 0, we need to set it directly
        let updated = sqlx::query_as::<_, Building>(
            r#"
            UPDATE buildings
            SET level = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, village_id, building_type, slot, level,
                      is_upgrading, upgrade_ends_at, created_at, updated_at
            "#,
        )
        .bind(building.id)
        .bind(level)
        .fetch_one(pool)
        .await?;

        return Ok(updated);
    }

    Ok(building)
}
