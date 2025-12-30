use sqlx::PgPool;
use std::time::Duration;
use tokio::time::interval;
use tracing::{error, info};

use crate::repositories::building_repo::BuildingRepository;
use crate::repositories::troop_repo::TroopRepository;
use crate::repositories::village_repo::VillageRepository;
use crate::services::army_service::ArmyService;
use crate::services::building_service::BuildingService;
use crate::services::resource_service::ResourceService;
use crate::services::trade_service::TradeService;
use crate::services::ws_service::{BuildingCompleteData, TradeOrderExpiredData, TroopTrainingCompleteData, TroopsStarvedData, WsEvent, WsManager};

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

    // Spawn troop training completion job
    let pool_clone = pool.clone();
    let ws_clone = ws_manager.clone();
    tokio::spawn(async move {
        run_troop_training_job(pool_clone, ws_clone).await;
    });

    // Spawn starvation job
    let pool_clone = pool.clone();
    let ws_clone = ws_manager.clone();
    tokio::spawn(async move {
        run_starvation_job(pool_clone, ws_clone).await;
    });

    // Spawn trade order expiry job
    let pool_clone = pool.clone();
    let ws_clone = ws_manager.clone();
    tokio::spawn(async move {
        run_trade_expiry_job(pool_clone, ws_clone).await;
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

/// Process troop training completion every 10 seconds
async fn run_troop_training_job(pool: PgPool, ws_manager: WsManager) {
    let mut ticker = interval(Duration::from_secs(10));

    loop {
        ticker.tick().await;

        match complete_troop_training(&pool, &ws_manager).await {
            Ok(count) => {
                if count > 0 {
                    info!("Completed {} troop training batches", count);
                }
            }
            Err(e) => {
                error!("Error completing troop training: {:?}", e);
            }
        }
    }
}

/// Complete all troop training that has finished
async fn complete_troop_training(pool: &PgPool, ws_manager: &WsManager) -> anyhow::Result<i32> {
    let completed = TroopRepository::find_completed_training(pool).await?;
    let mut count = 0;

    for entry in completed {
        // Add troops to village
        match TroopRepository::add_troops(pool, entry.village_id, entry.troop_type, entry.count).await {
            Ok(_) => {
                // Remove from queue
                if let Err(e) = TroopRepository::remove_from_queue(pool, entry.id).await {
                    error!("Failed to remove training queue entry {}: {:?}", entry.id, e);
                    continue;
                }

                info!(
                    "Troop training complete: {} x {:?} in village {}",
                    entry.count, entry.troop_type, entry.village_id
                );

                // Broadcast to village owner
                if let Ok(Some(village)) = VillageRepository::find_by_id(pool, entry.village_id).await {
                    let event = WsEvent::TroopTrainingComplete(TroopTrainingCompleteData {
                        village_id: entry.village_id,
                        troop_type: format!("{:?}", entry.troop_type),
                        quantity: entry.count,
                    });
                    ws_manager.send_to_user(village.user_id, &event).await;
                }

                count += 1;
            }
            Err(e) => {
                error!("Failed to add troops for queue entry {}: {:?}", entry.id, e);
            }
        }
    }

    Ok(count)
}

/// Process starvation every 60 seconds
async fn run_starvation_job(pool: PgPool, ws_manager: WsManager) {
    let mut ticker = interval(Duration::from_secs(60));

    loop {
        ticker.tick().await;

        match process_starvation(&pool, &ws_manager).await {
            Ok(count) => {
                if count > 0 {
                    info!("Starvation: {} troops died from hunger", count);
                }
            }
            Err(e) => {
                error!("Error processing starvation: {:?}", e);
            }
        }
    }
}

/// Troop with consumption info for starvation calculation
#[derive(Debug, sqlx::FromRow)]
struct TroopWithConsumption {
    village_id: uuid::Uuid,
    troop_type: crate::models::troop::TroopType,
    in_village: i32,
    crop_consumption: i32,
}

/// Process starvation for villages with no crop
async fn process_starvation(pool: &PgPool, ws_manager: &WsManager) -> anyhow::Result<i32> {
    // Find villages with crop <= 0
    let starving_villages: Vec<(uuid::Uuid, uuid::Uuid)> = sqlx::query_as(
        r#"
        SELECT id, user_id FROM villages
        WHERE crop <= 0
        "#,
    )
    .fetch_all(pool)
    .await?;

    if starving_villages.is_empty() {
        return Ok(0);
    }

    let mut total_killed = 0;

    for (village_id, user_id) in starving_villages {
        // Get troops with highest crop consumption first
        let troops: Vec<TroopWithConsumption> = sqlx::query_as(
            r#"
            SELECT t.village_id, t.troop_type, t.in_village, td.crop_consumption
            FROM troops t
            JOIN troop_definitions td ON t.troop_type = td.troop_type
            WHERE t.village_id = $1 AND t.in_village > 0
            ORDER BY td.crop_consumption DESC
            "#,
        )
        .bind(village_id)
        .fetch_all(pool)
        .await?;

        if troops.is_empty() {
            continue;
        }

        // Kill 1 troop from the type with highest consumption
        let victim = &troops[0];
        let kill_count = 1.min(victim.in_village);

        if kill_count > 0 {
            if let Err(e) = TroopRepository::kill_troops(pool, village_id, victim.troop_type, kill_count).await {
                error!("Failed to kill starving troops in village {}: {:?}", village_id, e);
                continue;
            }

            info!(
                "Starvation: {} {:?} died in village {} due to lack of food",
                kill_count, victim.troop_type, village_id
            );

            // Broadcast to village owner
            let event = WsEvent::TroopsStarved(TroopsStarvedData {
                village_id,
                troop_type: format!("{:?}", victim.troop_type),
                quantity: kill_count,
            });
            ws_manager.send_to_user(user_id, &event).await;

            total_killed += kill_count;
        }
    }

    Ok(total_killed)
}

/// Process expired trade orders every 30 seconds
async fn run_trade_expiry_job(pool: PgPool, ws_manager: WsManager) {
    let mut ticker = interval(Duration::from_secs(30));

    loop {
        ticker.tick().await;

        match process_expired_trade_orders(&pool, &ws_manager).await {
            Ok(count) => {
                if count > 0 {
                    info!("Expired {} trade orders", count);
                }
            }
            Err(e) => {
                error!("Error processing expired trade orders: {:?}", e);
            }
        }
    }
}

/// Process expired trade orders and refund resources/gold
async fn process_expired_trade_orders(pool: &PgPool, ws_manager: &WsManager) -> anyhow::Result<i32> {
    let results = TradeService::process_expired_orders(pool, 100).await?;

    if results.is_empty() {
        return Ok(0);
    }

    let count = results.len() as i32;

    // Send notifications to users
    for result in results {
        let event = WsEvent::TradeOrderExpired(TradeOrderExpiredData {
            order_id: result.order.id,
            order_type: format!("{:?}", result.order.order_type),
            resource_type: format!("{:?}", result.order.resource_type),
            quantity_remaining: result.order.quantity_remaining(),
            refunded_gold: result.refunded_gold,
        });

        ws_manager.send_to_user(result.user_id, &event).await;

        info!(
            "Trade order {} expired: {:?} {:?}, remaining={}, refunded_gold={:?}",
            result.order.id,
            result.order.order_type,
            result.order.resource_type,
            result.order.quantity_remaining(),
            result.refunded_gold
        );
    }

    Ok(count)
}
