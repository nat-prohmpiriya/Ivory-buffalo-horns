//! Map Generation Script for Tusk & Horn
//!
//! This script generates NPC (Natarian) villages across the game map.
//! Run with: cargo run --bin generate_map
//!
//! Options:
//!   --clear    Clear existing Natarian villages before generating
//!   --count N  Number of villages to generate (default: 80)

use rand::Rng;
use sqlx::PgPool;
use std::collections::HashSet;
use uuid::Uuid;

// Map configuration
const MAP_SIZE: i32 = 200; // Map goes from -MAP_SIZE to +MAP_SIZE
const DEFAULT_VILLAGE_COUNT: usize = 80;
const NATARIAN_FIREBASE_UID: &str = "natarian-npc-system";
const NATARIAN_DISPLAY_NAME: &str = "Natarian";

// Village name prefixes and suffixes for variety
const VILLAGE_PREFIXES: &[&str] = &[
    "Ancient", "Dark", "Shadow", "Lost", "Fallen", "Cursed", "Hidden", "Forgotten",
    "Mystic", "Sacred", "Wild", "Stone", "Iron", "Golden", "Silver", "Crystal",
];

const VILLAGE_SUFFIXES: &[&str] = &[
    "Outpost", "Stronghold", "Fortress", "Citadel", "Keep", "Watch", "Guard",
    "Haven", "Refuge", "Sanctuary", "Temple", "Shrine", "Ruins", "Camp", "Settlement",
];

#[derive(Debug, Clone, Copy)]
enum TroopType {
    Infantry,
    Spearman,
    WarElephant,
    Crossbowman,
    MountainWarrior,
}

impl TroopType {
    fn as_str(&self) -> &'static str {
        match self {
            TroopType::Infantry => "infantry",
            TroopType::Spearman => "spearman",
            TroopType::WarElephant => "war_elephant",
            TroopType::Crossbowman => "crossbowman",
            TroopType::MountainWarrior => "mountain_warrior",
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum BuildingType {
    MainBuilding,
    RallyPoint,
    Wall,
    Warehouse,
    Granary,
    Woodcutter,
    ClayPit,
    IronMine,
    CropField,
}

impl BuildingType {
    fn as_str(&self) -> &'static str {
        match self {
            BuildingType::MainBuilding => "main_building",
            BuildingType::RallyPoint => "rally_point",
            BuildingType::Wall => "wall",
            BuildingType::Warehouse => "warehouse",
            BuildingType::Granary => "granary",
            BuildingType::Woodcutter => "woodcutter",
            BuildingType::ClayPit => "clay_pit",
            BuildingType::IronMine => "iron_mine",
            BuildingType::CropField => "crop_field",
        }
    }
}

/// Village difficulty tier based on distance from center
#[derive(Debug, Clone, Copy)]
enum VillageTier {
    /// Close to center (0-50): Very strong, for experienced players
    Elite,
    /// Medium distance (50-100): Moderate difficulty
    Veteran,
    /// Far from center (100-150): Easier targets
    Regular,
    /// Edge of map (150-200): Beginner friendly
    Beginner,
}

impl VillageTier {
    fn from_distance(distance: f64) -> Self {
        if distance < 50.0 {
            VillageTier::Elite
        } else if distance < 100.0 {
            VillageTier::Veteran
        } else if distance < 150.0 {
            VillageTier::Regular
        } else {
            VillageTier::Beginner
        }
    }

    /// Get troop counts for this tier
    fn troop_config(&self) -> Vec<(TroopType, i32)> {
        match self {
            VillageTier::Elite => vec![
                (TroopType::Infantry, 200),
                (TroopType::Spearman, 150),
                (TroopType::WarElephant, 30),
                (TroopType::Crossbowman, 100),
                (TroopType::MountainWarrior, 50),
            ],
            VillageTier::Veteran => vec![
                (TroopType::Infantry, 100),
                (TroopType::Spearman, 80),
                (TroopType::WarElephant, 10),
                (TroopType::Crossbowman, 50),
            ],
            VillageTier::Regular => vec![
                (TroopType::Infantry, 50),
                (TroopType::Spearman, 30),
                (TroopType::Crossbowman, 20),
            ],
            VillageTier::Beginner => vec![
                (TroopType::Infantry, 15),
                (TroopType::Spearman, 10),
            ],
        }
    }

    /// Get building levels for this tier
    fn building_config(&self) -> Vec<(BuildingType, i32, i32)> {
        // (building_type, slot, level)
        match self {
            VillageTier::Elite => vec![
                (BuildingType::MainBuilding, 1, 15),
                (BuildingType::RallyPoint, 2, 10),
                (BuildingType::Wall, 3, 15),
                (BuildingType::Warehouse, 4, 12),
                (BuildingType::Granary, 5, 12),
                // Resource fields
                (BuildingType::Woodcutter, 101, 10),
                (BuildingType::Woodcutter, 102, 10),
                (BuildingType::Woodcutter, 103, 10),
                (BuildingType::Woodcutter, 104, 10),
                (BuildingType::ClayPit, 105, 10),
                (BuildingType::ClayPit, 106, 10),
                (BuildingType::ClayPit, 107, 10),
                (BuildingType::ClayPit, 108, 10),
                (BuildingType::IronMine, 109, 10),
                (BuildingType::IronMine, 110, 10),
                (BuildingType::IronMine, 111, 10),
                (BuildingType::IronMine, 112, 10),
                (BuildingType::CropField, 113, 10),
                (BuildingType::CropField, 114, 10),
                (BuildingType::CropField, 115, 10),
                (BuildingType::CropField, 116, 10),
                (BuildingType::CropField, 117, 10),
                (BuildingType::CropField, 118, 10),
            ],
            VillageTier::Veteran => vec![
                (BuildingType::MainBuilding, 1, 10),
                (BuildingType::RallyPoint, 2, 5),
                (BuildingType::Wall, 3, 10),
                (BuildingType::Warehouse, 4, 8),
                (BuildingType::Granary, 5, 8),
                // Resource fields
                (BuildingType::Woodcutter, 101, 7),
                (BuildingType::Woodcutter, 102, 7),
                (BuildingType::Woodcutter, 103, 7),
                (BuildingType::Woodcutter, 104, 7),
                (BuildingType::ClayPit, 105, 7),
                (BuildingType::ClayPit, 106, 7),
                (BuildingType::ClayPit, 107, 7),
                (BuildingType::ClayPit, 108, 7),
                (BuildingType::IronMine, 109, 7),
                (BuildingType::IronMine, 110, 7),
                (BuildingType::IronMine, 111, 7),
                (BuildingType::IronMine, 112, 7),
                (BuildingType::CropField, 113, 7),
                (BuildingType::CropField, 114, 7),
                (BuildingType::CropField, 115, 7),
                (BuildingType::CropField, 116, 7),
                (BuildingType::CropField, 117, 7),
                (BuildingType::CropField, 118, 7),
            ],
            VillageTier::Regular => vec![
                (BuildingType::MainBuilding, 1, 5),
                (BuildingType::RallyPoint, 2, 3),
                (BuildingType::Wall, 3, 5),
                (BuildingType::Warehouse, 4, 5),
                (BuildingType::Granary, 5, 5),
                // Resource fields
                (BuildingType::Woodcutter, 101, 4),
                (BuildingType::Woodcutter, 102, 4),
                (BuildingType::Woodcutter, 103, 4),
                (BuildingType::Woodcutter, 104, 4),
                (BuildingType::ClayPit, 105, 4),
                (BuildingType::ClayPit, 106, 4),
                (BuildingType::ClayPit, 107, 4),
                (BuildingType::ClayPit, 108, 4),
                (BuildingType::IronMine, 109, 4),
                (BuildingType::IronMine, 110, 4),
                (BuildingType::IronMine, 111, 4),
                (BuildingType::IronMine, 112, 4),
                (BuildingType::CropField, 113, 4),
                (BuildingType::CropField, 114, 4),
                (BuildingType::CropField, 115, 4),
                (BuildingType::CropField, 116, 4),
                (BuildingType::CropField, 117, 4),
                (BuildingType::CropField, 118, 4),
            ],
            VillageTier::Beginner => vec![
                (BuildingType::MainBuilding, 1, 3),
                (BuildingType::RallyPoint, 2, 1),
                (BuildingType::Wall, 3, 2),
                (BuildingType::Warehouse, 4, 3),
                (BuildingType::Granary, 5, 3),
                // Resource fields (minimal)
                (BuildingType::Woodcutter, 101, 2),
                (BuildingType::Woodcutter, 102, 2),
                (BuildingType::Woodcutter, 103, 2),
                (BuildingType::Woodcutter, 104, 2),
                (BuildingType::ClayPit, 105, 2),
                (BuildingType::ClayPit, 106, 2),
                (BuildingType::ClayPit, 107, 2),
                (BuildingType::ClayPit, 108, 2),
                (BuildingType::IronMine, 109, 2),
                (BuildingType::IronMine, 110, 2),
                (BuildingType::IronMine, 111, 2),
                (BuildingType::IronMine, 112, 2),
                (BuildingType::CropField, 113, 2),
                (BuildingType::CropField, 114, 2),
                (BuildingType::CropField, 115, 2),
                (BuildingType::CropField, 116, 2),
                (BuildingType::CropField, 117, 2),
                (BuildingType::CropField, 118, 2),
            ],
        }
    }

    /// Get base population for this tier
    fn population(&self) -> i32 {
        match self {
            VillageTier::Elite => 500,
            VillageTier::Veteran => 300,
            VillageTier::Regular => 150,
            VillageTier::Beginner => 50,
        }
    }

    /// Get resource amounts for this tier
    fn resources(&self) -> (i32, i32, i32, i32) {
        // (wood, clay, iron, crop)
        match self {
            VillageTier::Elite => (5000, 5000, 5000, 5000),
            VillageTier::Veteran => (3000, 3000, 3000, 3000),
            VillageTier::Regular => (1500, 1500, 1500, 1500),
            VillageTier::Beginner => (800, 800, 800, 800),
        }
    }

    /// Get storage capacity for this tier
    fn storage(&self) -> (i32, i32) {
        // (warehouse, granary)
        match self {
            VillageTier::Elite => (10000, 10000),
            VillageTier::Veteran => (6000, 6000),
            VillageTier::Regular => (3000, 3000),
            VillageTier::Beginner => (1200, 1200),
        }
    }
}

fn generate_village_name(rng: &mut impl Rng) -> String {
    let prefix = VILLAGE_PREFIXES[rng.gen_range(0..VILLAGE_PREFIXES.len())];
    let suffix = VILLAGE_SUFFIXES[rng.gen_range(0..VILLAGE_SUFFIXES.len())];
    format!("{} {}", prefix, suffix)
}

fn calculate_distance(x: i32, y: i32) -> f64 {
    ((x as f64).powi(2) + (y as f64).powi(2)).sqrt()
}

/// Generate random coordinates that are not too close to other villages
fn generate_coordinates(
    rng: &mut impl Rng,
    existing: &HashSet<(i32, i32)>,
    min_distance: i32,
) -> Option<(i32, i32)> {
    for _ in 0..1000 {
        let x = rng.gen_range(-MAP_SIZE..=MAP_SIZE);
        let y = rng.gen_range(-MAP_SIZE..=MAP_SIZE);

        // Skip center area (reserved for players)
        if x.abs() < 10 && y.abs() < 10 {
            continue;
        }

        // Check minimum distance from existing villages
        let too_close = existing.iter().any(|(ex, ey)| {
            let dx = (x - ex).abs();
            let dy = (y - ey).abs();
            dx < min_distance && dy < min_distance
        });

        if !too_close {
            return Some((x, y));
        }
    }
    None
}

async fn get_or_create_natarian_user(pool: &PgPool) -> anyhow::Result<Uuid> {
    // Check if Natarian user exists
    let existing: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM users WHERE firebase_uid = $1"
    )
    .bind(NATARIAN_FIREBASE_UID)
    .fetch_optional(pool)
    .await?;

    if let Some((id,)) = existing {
        println!("Found existing Natarian user: {}", id);
        return Ok(id);
    }

    // Create Natarian user
    let user: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO users (firebase_uid, email, display_name, provider)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#
    )
    .bind(NATARIAN_FIREBASE_UID)
    .bind("natarian@tusk-horn.local")
    .bind(NATARIAN_DISPLAY_NAME)
    .bind("system")
    .fetch_one(pool)
    .await?;

    println!("Created Natarian user: {}", user.0);
    Ok(user.0)
}

async fn get_existing_coordinates(pool: &PgPool) -> anyhow::Result<HashSet<(i32, i32)>> {
    let rows: Vec<(i32, i32)> = sqlx::query_as(
        "SELECT x, y FROM villages"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().collect())
}

async fn clear_natarian_villages(pool: &PgPool, natarian_id: Uuid) -> anyhow::Result<u64> {
    // Get all Natarian village IDs
    let village_ids: Vec<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM villages WHERE user_id = $1"
    )
    .bind(natarian_id)
    .fetch_all(pool)
    .await?;

    if village_ids.is_empty() {
        return Ok(0);
    }

    let ids: Vec<Uuid> = village_ids.into_iter().map(|(id,)| id).collect();
    let count = ids.len() as u64;

    // Delete related data first
    for id in &ids {
        sqlx::query("DELETE FROM troops WHERE village_id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        sqlx::query("DELETE FROM buildings WHERE village_id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        sqlx::query("DELETE FROM troop_queue WHERE village_id = $1")
            .bind(id)
            .execute(pool)
            .await?;
    }

    // Delete villages
    sqlx::query("DELETE FROM villages WHERE user_id = $1")
        .bind(natarian_id)
        .execute(pool)
        .await?;

    Ok(count)
}

async fn create_village(
    pool: &PgPool,
    user_id: Uuid,
    name: &str,
    x: i32,
    y: i32,
    tier: VillageTier,
) -> anyhow::Result<Uuid> {
    let (wood, clay, iron, crop) = tier.resources();
    let (warehouse, granary) = tier.storage();
    let population = tier.population();

    let village: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO villages (
            user_id, name, x, y, is_capital,
            wood, clay, iron, crop,
            warehouse_capacity, granary_capacity,
            population
        )
        VALUES ($1, $2, $3, $4, false, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id
        "#
    )
    .bind(user_id)
    .bind(name)
    .bind(x)
    .bind(y)
    .bind(wood)
    .bind(clay)
    .bind(iron)
    .bind(crop)
    .bind(warehouse)
    .bind(granary)
    .bind(population)
    .fetch_one(pool)
    .await?;

    Ok(village.0)
}

async fn create_buildings(
    pool: &PgPool,
    village_id: Uuid,
    tier: VillageTier,
) -> anyhow::Result<()> {
    for (building_type, slot, level) in tier.building_config() {
        sqlx::query(
            r#"
            INSERT INTO buildings (village_id, building_type, slot, level)
            VALUES ($1, $2::building_type, $3, $4)
            "#
        )
        .bind(village_id)
        .bind(building_type.as_str())
        .bind(slot)
        .bind(level)
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn create_troops(
    pool: &PgPool,
    village_id: Uuid,
    tier: VillageTier,
) -> anyhow::Result<()> {
    for (troop_type, count) in tier.troop_config() {
        sqlx::query(
            r#"
            INSERT INTO troops (village_id, troop_type, count, in_village)
            VALUES ($1, $2::troop_type, $3, $3)
            "#
        )
        .bind(village_id)
        .bind(troop_type.as_str())
        .bind(count)
        .execute(pool)
        .await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("generate_map=info")
        .init();

    // Parse arguments
    let args: Vec<String> = std::env::args().collect();
    let clear_existing = args.contains(&"--clear".to_string());
    let village_count = args
        .iter()
        .position(|a| a == "--count")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_VILLAGE_COUNT);

    println!("=== Tusk & Horn Map Generator ===");
    println!("Map size: {}x{} (±{})", MAP_SIZE * 2, MAP_SIZE * 2, MAP_SIZE);
    println!("Villages to generate: {}", village_count);
    println!("Clear existing: {}", clear_existing);
    println!();

    // Load environment
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Connect to database
    println!("Connecting to database...");
    let pool = PgPool::connect(&database_url).await?;
    println!("Connected!");
    println!();

    // Get or create Natarian user
    let natarian_id = get_or_create_natarian_user(&pool).await?;

    // Clear existing if requested
    if clear_existing {
        println!("Clearing existing Natarian villages...");
        let cleared = clear_natarian_villages(&pool, natarian_id).await?;
        println!("Cleared {} villages", cleared);
        println!();
    }

    // Get existing coordinates
    let mut existing_coords = get_existing_coordinates(&pool).await?;
    println!("Existing villages on map: {}", existing_coords.len());

    // Generate villages
    let mut rng = rand::thread_rng();
    let mut created = 0;
    let mut tier_counts = [0usize; 4]; // [Elite, Veteran, Regular, Beginner]

    println!();
    println!("Generating villages...");

    for i in 0..village_count {
        // Generate coordinates with minimum distance of 5 tiles
        let coords = match generate_coordinates(&mut rng, &existing_coords, 5) {
            Some(c) => c,
            None => {
                println!("Warning: Could not find valid coordinates for village {}", i + 1);
                continue;
            }
        };

        let (x, y) = coords;
        let distance = calculate_distance(x, y);
        let tier = VillageTier::from_distance(distance);
        let name = generate_village_name(&mut rng);

        // Create village
        let village_id = create_village(&pool, natarian_id, &name, x, y, tier).await?;

        // Create buildings
        create_buildings(&pool, village_id, tier).await?;

        // Create troops
        create_troops(&pool, village_id, tier).await?;

        // Track stats
        existing_coords.insert(coords);
        created += 1;
        match tier {
            VillageTier::Elite => tier_counts[0] += 1,
            VillageTier::Veteran => tier_counts[1] += 1,
            VillageTier::Regular => tier_counts[2] += 1,
            VillageTier::Beginner => tier_counts[3] += 1,
        }

        // Progress indicator
        if (i + 1) % 10 == 0 {
            println!("  Created {}/{} villages...", i + 1, village_count);
        }
    }

    println!();
    println!("=== Generation Complete ===");
    println!("Total villages created: {}", created);
    println!("  - Elite (center): {}", tier_counts[0]);
    println!("  - Veteran: {}", tier_counts[1]);
    println!("  - Regular: {}", tier_counts[2]);
    println!("  - Beginner (edge): {}", tier_counts[3]);
    println!();
    println!("Total villages on map: {}", existing_coords.len());

    Ok(())
}
