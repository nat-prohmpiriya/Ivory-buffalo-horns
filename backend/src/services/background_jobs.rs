use sqlx::PgPool;
use std::time::Duration;
use tokio::time::interval;
use tracing::{error, info};

use crate::repositories::building_repo::BuildingRepository;
use crate::repositories::village_repo::VillageRepository;
use crate::services::army_service::ArmyService;
use crate::services::building_service::BuildingService;
use crate::services::resource_service::ResourceService;
use crate::services::ws_service::{BuildingCompleteData, WsEvent, WsManager};

/// Start all background jobs
pub async fn start_background_jobs(pool: PgPool, ws_manager: WsManager) {
    // Spawn building completion job
    let pool_clone = pool.clone();
    let ws_clone = ws_manager.clone();
    tokio::spawn(async move {
        run_building_completion_job(pool_clone, ws_clone).await;
    });

    // Spawn resource production job
    let pool_clone = pool.clone();
    let ws_clone = ws_manager.clone();
    tokio::spawn(async move {
        run_resource_production_job(pool_clone, ws_clone).await;
    });

    // Spawn army processing job
    let pool_clone = pool.clone();
    let ws_clone = ws_manager.clone();
    tokio::spawn(async move {
        run_army_processing_job(pool_clone, ws_clone).await;
    });

    info!("Background jobs started");
}

/// Check and complete building upgrades every 10 seconds
async fn run_building_completion_job(pool: PgPool, ws_manager: WsManager) {
    let mut ticker = interval(Duration::from_secs(10));

    loop {
        ticker.tick().await;

        match complete_building_upgrades(&pool, &ws_manager).await {
            Ok(count) => {
                if count > 0 {
                    info!("Completed {} building upgrades", count);
                }
            }
            Err(e) => {
                error!("Error completing building upgrades: {:?}", e);
            }
        }
    }
}

/// Complete all buildings that have finished upgrading
async fn complete_building_upgrades(pool: &PgPool, ws_manager: &WsManager) -> anyhow::Result<i32> {
    let buildings = BuildingRepository::find_completed_upgrades(pool).await?;
    let mut completed = 0;

    for building in buildings {
        // Use BuildingService to handle upgrade completion with side effects
        match BuildingService::complete_upgrade(pool, building.id).await {
            Ok(updated) => {
                info!(
                    "Building {:?} upgraded to level {} in village {}",
                    updated.building_type, updated.level, updated.village_id
                );

                // Broadcast to village owner
                if let Ok(Some(village)) = VillageRepository::find_by_id(pool, updated.village_id).await {
                    let event = WsEvent::BuildingComplete(BuildingCompleteData {
                        village_id: updated.village_id,
                        building_type: format!("{:?}", updated.building_type),
                        slot: updated.slot,
                        level: updated.level,
                    });
                    ws_manager.send_to_user(village.user_id, &event).await;
                }

                completed += 1;
            }
            Err(e) => {
                error!("Error completing upgrade for building {}: {:?}", building.id, e);
            }
        }
    }

    Ok(completed)
}

/// Update resource production every 5 minutes
async fn run_resource_production_job(pool: PgPool, _ws_manager: WsManager) {
    let mut ticker = interval(Duration::from_secs(300)); // 5 minutes

    loop {
        ticker.tick().await;

        match ResourceService::update_all_village_resources(&pool).await {
            Ok(count) => {
                if count > 0 {
                    info!("Updated resources for {} villages", count);
                    // Note: Resource updates are frequent and for all villages
                    // We don't broadcast here to avoid spam - clients should poll or
                    // we broadcast only when user is actively viewing
                }
            }
            Err(e) => {
                error!("Error updating village resources: {:?}", e);
            }
        }
    }
}

/// Process army arrivals every 5 seconds
async fn run_army_processing_job(pool: PgPool, ws_manager: WsManager) {
    let mut ticker = interval(Duration::from_secs(5));

    loop {
        ticker.tick().await;

        match ArmyService::process_arrived_armies_with_ws(&pool, &ws_manager).await {
            Ok(count) => {
                if count > 0 {
                    info!("Processed {} army arrivals", count);
                }
            }
            Err(e) => {
                error!("Error processing army arrivals: {:?}", e);
            }
        }
    }
}
