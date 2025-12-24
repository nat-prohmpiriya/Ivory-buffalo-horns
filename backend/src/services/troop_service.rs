use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::troop::{Troop, TroopCost, TroopDefinition, TroopQueue, TroopType, TrainTroopsResponse};
use crate::repositories::building_repo::BuildingRepository;
use crate::repositories::troop_repo::TroopRepository;
use crate::repositories::village_repo::VillageRepository;

pub struct TroopService;

impl TroopService {
    /// Get all available troop definitions
    pub async fn get_definitions(pool: &PgPool) -> AppResult<Vec<TroopDefinition>> {
        TroopRepository::get_all_definitions(pool).await
    }

    /// Get troops in a village
    pub async fn get_village_troops(pool: &PgPool, village_id: Uuid) -> AppResult<Vec<Troop>> {
        TroopRepository::find_by_village(pool, village_id).await
    }

    /// Get training queue for a village
    pub async fn get_training_queue(pool: &PgPool, village_id: Uuid) -> AppResult<Vec<TroopQueue>> {
        TroopRepository::get_queue_by_village(pool, village_id).await
    }

    /// Check if training requirements are met
    pub async fn check_training_requirements(
        pool: &PgPool,
        village_id: Uuid,
        troop_type: TroopType,
    ) -> AppResult<TroopDefinition> {
        // Get troop definition
        let definition = TroopRepository::get_definition(pool, troop_type)
            .await?
            .ok_or_else(|| AppError::NotFound("Troop type not found".into()))?;

        // Check if required building exists at required level
        let buildings = BuildingRepository::find_by_type(pool, village_id, definition.required_building.clone()).await?;

        let max_level = buildings.iter().map(|b| b.level).max().unwrap_or(0);

        if max_level < definition.required_building_level {
            return Err(AppError::BadRequest(format!(
                "{:?} level {} required (current: {})",
                definition.required_building, definition.required_building_level, max_level
            )));
        }

        Ok(definition)
    }

    /// Train troops
    pub async fn train_troops(
        pool: &PgPool,
        village_id: Uuid,
        troop_type: TroopType,
        count: i32,
    ) -> AppResult<TrainTroopsResponse> {
        if count <= 0 {
            return Err(AppError::BadRequest("Count must be positive".into()));
        }

        // Check requirements
        let definition = Self::check_training_requirements(pool, village_id, troop_type).await?;

        // Calculate total cost
        let total_cost = TroopCost {
            wood: definition.wood_cost * count,
            clay: definition.clay_cost * count,
            iron: definition.iron_cost * count,
            crop: definition.crop_cost * count,
            time_seconds: definition.training_time_seconds * count,
        };

        // Check and deduct resources
        let village = VillageRepository::find_by_id(pool, village_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Village not found".into()))?;

        if village.wood < total_cost.wood
            || village.clay < total_cost.clay
            || village.iron < total_cost.iron
            || village.crop < total_cost.crop
        {
            return Err(AppError::BadRequest("Not enough resources".into()));
        }

        // Deduct resources
        VillageRepository::deduct_resources(
            pool,
            village_id,
            total_cost.wood,
            total_cost.clay,
            total_cost.iron,
            total_cost.crop,
        )
        .await?;

        // Calculate start and end time
        // If there's already a queue, start after the last item
        let now = Utc::now();
        let started_at = TroopRepository::get_last_queue_end_time(pool, village_id)
            .await?
            .unwrap_or(now);
        let ends_at = started_at + Duration::seconds(total_cost.time_seconds as i64);

        // Add to queue
        let queue_entry = TroopRepository::add_to_queue(
            pool,
            village_id,
            troop_type,
            count,
            definition.training_time_seconds,
            started_at,
            ends_at,
        )
        .await?;

        Ok(TrainTroopsResponse {
            queue_entry: queue_entry.into(),
            cost: total_cost,
        })
    }

    /// Complete training from queue (called by background job)
    pub async fn complete_training(pool: &PgPool, queue_id: Uuid) -> AppResult<()> {
        // Get queue entry
        let queue = TroopRepository::get_queue_by_village(pool, Uuid::nil()).await?;
        let entry = queue.iter().find(|q| q.id == queue_id);

        if let Some(entry) = entry {
            // Add troops to village
            TroopRepository::add_troops(pool, entry.village_id, entry.troop_type, entry.count).await?;

            // Remove from queue
            TroopRepository::remove_from_queue(pool, queue_id).await?;
        }

        Ok(())
    }

    /// Process all completed training (called by background job)
    pub async fn process_completed_training(pool: &PgPool) -> AppResult<i32> {
        let completed = TroopRepository::find_completed_training(pool).await?;
        let count = completed.len() as i32;

        for entry in completed {
            // Add troops to village
            TroopRepository::add_troops(pool, entry.village_id, entry.troop_type, entry.count).await?;

            // Remove from queue
            TroopRepository::remove_from_queue(pool, entry.id).await?;
        }

        Ok(count)
    }

    /// Cancel training (if not yet started)
    pub async fn cancel_training(
        pool: &PgPool,
        village_id: Uuid,
        queue_id: Uuid,
    ) -> AppResult<()> {
        let queue = TroopRepository::get_queue_by_village(pool, village_id).await?;
        let entry = queue
            .iter()
            .find(|q| q.id == queue_id)
            .ok_or_else(|| AppError::NotFound("Queue entry not found".into()))?;

        // Only allow canceling if not yet started
        let now = Utc::now();
        if entry.started_at <= now {
            return Err(AppError::BadRequest("Cannot cancel training in progress".into()));
        }

        // Get troop definition for refund calculation
        let definition = TroopRepository::get_definition(pool, entry.troop_type)
            .await?
            .ok_or_else(|| AppError::NotFound("Troop definition not found".into()))?;

        // Refund resources (75% refund for cancellation)
        let wood_refund = (definition.wood_cost * entry.count * 3) / 4;
        let clay_refund = (definition.clay_cost * entry.count * 3) / 4;
        let iron_refund = (definition.iron_cost * entry.count * 3) / 4;
        let crop_refund = (definition.crop_cost * entry.count * 3) / 4;

        VillageRepository::add_resources(pool, village_id, wood_refund, clay_refund, iron_refund, crop_refund)
            .await?;

        // Remove from queue
        TroopRepository::remove_from_queue(pool, queue_id).await?;

        Ok(())
    }

    /// Get total crop consumption for a village from troops
    pub async fn get_crop_consumption(pool: &PgPool, village_id: Uuid) -> AppResult<i32> {
        TroopRepository::get_total_crop_consumption(pool, village_id).await
    }
}
