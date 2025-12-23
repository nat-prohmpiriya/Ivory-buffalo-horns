use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::building::{Building, BuildingType, CreateBuilding};

pub struct BuildingRepository;

impl BuildingRepository {
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<Building>> {
        let building = sqlx::query_as::<_, Building>(
            r#"
            SELECT id, village_id, building_type, slot, level,
                   is_upgrading, upgrade_ends_at, created_at, updated_at
            FROM buildings
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(building)
    }

    pub async fn find_by_village_id(pool: &PgPool, village_id: Uuid) -> AppResult<Vec<Building>> {
        let buildings = sqlx::query_as::<_, Building>(
            r#"
            SELECT id, village_id, building_type, slot, level,
                   is_upgrading, upgrade_ends_at, created_at, updated_at
            FROM buildings
            WHERE village_id = $1
            ORDER BY slot ASC
            "#,
        )
        .bind(village_id)
        .fetch_all(pool)
        .await?;

        Ok(buildings)
    }

    pub async fn find_by_village_and_slot(
        pool: &PgPool,
        village_id: Uuid,
        slot: i32,
    ) -> AppResult<Option<Building>> {
        let building = sqlx::query_as::<_, Building>(
            r#"
            SELECT id, village_id, building_type, slot, level,
                   is_upgrading, upgrade_ends_at, created_at, updated_at
            FROM buildings
            WHERE village_id = $1 AND slot = $2
            "#,
        )
        .bind(village_id)
        .bind(slot)
        .fetch_optional(pool)
        .await?;

        Ok(building)
    }

    pub async fn find_upgrading_by_village(
        pool: &PgPool,
        village_id: Uuid,
    ) -> AppResult<Vec<Building>> {
        let buildings = sqlx::query_as::<_, Building>(
            r#"
            SELECT id, village_id, building_type, slot, level,
                   is_upgrading, upgrade_ends_at, created_at, updated_at
            FROM buildings
            WHERE village_id = $1 AND is_upgrading = TRUE
            ORDER BY upgrade_ends_at ASC
            "#,
        )
        .bind(village_id)
        .fetch_all(pool)
        .await?;

        Ok(buildings)
    }

    pub async fn create(pool: &PgPool, input: CreateBuilding) -> AppResult<Building> {
        let building = sqlx::query_as::<_, Building>(
            r#"
            INSERT INTO buildings (village_id, building_type, slot, level)
            VALUES ($1, $2, $3, 1)
            RETURNING id, village_id, building_type, slot, level,
                      is_upgrading, upgrade_ends_at, created_at, updated_at
            "#,
        )
        .bind(&input.village_id)
        .bind(&input.building_type)
        .bind(input.slot)
        .fetch_one(pool)
        .await?;

        Ok(building)
    }

    pub async fn start_upgrade(
        pool: &PgPool,
        id: Uuid,
        upgrade_ends_at: DateTime<Utc>,
    ) -> AppResult<Building> {
        let building = sqlx::query_as::<_, Building>(
            r#"
            UPDATE buildings
            SET is_upgrading = TRUE,
                upgrade_ends_at = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, village_id, building_type, slot, level,
                      is_upgrading, upgrade_ends_at, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(upgrade_ends_at)
        .fetch_one(pool)
        .await?;

        Ok(building)
    }

    pub async fn complete_upgrade(pool: &PgPool, id: Uuid) -> AppResult<Building> {
        let building = sqlx::query_as::<_, Building>(
            r#"
            UPDATE buildings
            SET level = level + 1,
                is_upgrading = FALSE,
                upgrade_ends_at = NULL,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, village_id, building_type, slot, level,
                      is_upgrading, upgrade_ends_at, created_at, updated_at
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(building)
    }

    pub async fn cancel_upgrade(pool: &PgPool, id: Uuid) -> AppResult<Building> {
        let building = sqlx::query_as::<_, Building>(
            r#"
            UPDATE buildings
            SET is_upgrading = FALSE,
                upgrade_ends_at = NULL,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, village_id, building_type, slot, level,
                      is_upgrading, upgrade_ends_at, created_at, updated_at
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(building)
    }

    pub async fn demolish(pool: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            DELETE FROM buildings WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_completed_upgrades(pool: &PgPool) -> AppResult<Vec<Building>> {
        let buildings = sqlx::query_as::<_, Building>(
            r#"
            SELECT id, village_id, building_type, slot, level,
                   is_upgrading, upgrade_ends_at, created_at, updated_at
            FROM buildings
            WHERE is_upgrading = TRUE AND upgrade_ends_at <= NOW()
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(buildings)
    }

    pub async fn count_upgrading_by_village(pool: &PgPool, village_id: Uuid) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM buildings
            WHERE village_id = $1 AND is_upgrading = TRUE
            "#,
        )
        .bind(village_id)
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    pub async fn find_by_type(
        pool: &PgPool,
        village_id: Uuid,
        building_type: BuildingType,
    ) -> AppResult<Vec<Building>> {
        let buildings = sqlx::query_as::<_, Building>(
            r#"
            SELECT id, village_id, building_type, slot, level,
                   is_upgrading, upgrade_ends_at, created_at, updated_at
            FROM buildings
            WHERE village_id = $1 AND building_type = $2
            ORDER BY level DESC
            "#,
        )
        .bind(village_id)
        .bind(&building_type)
        .fetch_all(pool)
        .await?;

        Ok(buildings)
    }
}
