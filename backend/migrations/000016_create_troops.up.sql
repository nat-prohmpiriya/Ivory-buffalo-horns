-- Troop types for different tribes
CREATE TYPE troop_type AS ENUM (
    -- Phasuttha (Mainland/Thai-inspired)
    'infantry',
    'spearman',
    'war_elephant',
    'buffalo_wagon',
    -- Nava (Maritime/Malay-inspired)
    'kris_warrior',
    'sea_diver',
    'war_prahu',
    'merchant_ship',
    -- Kiri (Highland/Hill tribe-inspired)
    'crossbowman',
    'mountain_warrior',
    'highland_pony',
    'trap_maker',
    -- Special units (all tribes)
    'swamp_dragon',
    'locust_swarm',
    'battle_duck',
    'portuguese_musketeer'
);

CREATE TYPE tribe_type AS ENUM ('phasuttha', 'nava', 'kiri', 'special');

-- Troop definitions (base stats and costs)
CREATE TABLE troop_definitions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    troop_type troop_type UNIQUE NOT NULL,
    tribe tribe_type NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    -- Combat stats
    attack INT NOT NULL DEFAULT 0,
    defense_infantry INT NOT NULL DEFAULT 0,
    defense_cavalry INT NOT NULL DEFAULT 0,
    speed INT NOT NULL DEFAULT 1, -- Tiles per hour
    carry_capacity INT NOT NULL DEFAULT 0,
    crop_consumption INT NOT NULL DEFAULT 1,
    -- Training
    training_time_seconds INT NOT NULL DEFAULT 60,
    wood_cost INT NOT NULL DEFAULT 0,
    clay_cost INT NOT NULL DEFAULT 0,
    iron_cost INT NOT NULL DEFAULT 0,
    crop_cost INT NOT NULL DEFAULT 0,
    -- Requirements
    required_building building_type NOT NULL DEFAULT 'barracks',
    required_building_level INT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Troops owned by each village
CREATE TABLE troops (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    village_id UUID NOT NULL REFERENCES villages(id) ON DELETE CASCADE,
    troop_type troop_type NOT NULL,
    count INT NOT NULL DEFAULT 0,
    in_village INT NOT NULL DEFAULT 0, -- Currently in village (not on mission)
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(village_id, troop_type)
);

-- Training queue
CREATE TABLE troop_queue (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    village_id UUID NOT NULL REFERENCES villages(id) ON DELETE CASCADE,
    troop_type troop_type NOT NULL,
    count INT NOT NULL,
    each_duration_seconds INT NOT NULL,
    started_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_troops_village ON troops(village_id);
CREATE INDEX idx_troop_queue_village ON troop_queue(village_id);
CREATE INDEX idx_troop_queue_ends ON troop_queue(ends_at);

-- Seed troop definitions
INSERT INTO troop_definitions (troop_type, tribe, name, description, attack, defense_infantry, defense_cavalry, speed, carry_capacity, crop_consumption, training_time_seconds, wood_cost, clay_cost, iron_cost, crop_cost, required_building, required_building_level) VALUES
-- Phasuttha troops
('infantry', 'phasuttha', 'Infantry', 'Basic foot soldier with balanced stats', 40, 35, 50, 6, 50, 1, 1200, 120, 100, 150, 30, 'barracks', 1),
('spearman', 'phasuttha', 'Spearman', 'Anti-cavalry specialist', 10, 35, 60, 7, 20, 1, 1000, 140, 100, 30, 40, 'barracks', 3),
('war_elephant', 'phasuttha', 'War Elephant', 'Heavy cavalry with massive attack power', 120, 65, 50, 4, 80, 3, 3600, 450, 380, 420, 100, 'stable', 5),
('buffalo_wagon', 'phasuttha', 'Buffalo Wagon', 'Transport unit with high carry capacity', 0, 80, 80, 5, 750, 2, 2400, 250, 350, 200, 60, 'stable', 10),

-- Nava troops
('kris_warrior', 'nava', 'Kris Warrior', 'Fast raider with curved blade', 30, 40, 20, 9, 35, 1, 900, 80, 60, 120, 30, 'barracks', 1),
('sea_diver', 'nava', 'Sea Diver', 'Scout unit specialized in reconnaissance', 0, 20, 10, 18, 10, 1, 600, 30, 50, 40, 30, 'barracks', 5),
('war_prahu', 'nava', 'War Prahu', 'Naval warship with strong attack', 75, 40, 35, 10, 150, 2, 2700, 300, 150, 350, 80, 'workshop', 1),
('merchant_ship', 'nava', 'Merchant Ship', 'Trade transport with huge capacity', 0, 35, 35, 8, 500, 2, 2100, 180, 200, 100, 70, 'market', 10),

-- Kiri troops
('crossbowman', 'kiri', 'Crossbowman', 'Defensive ranged unit', 45, 60, 40, 6, 45, 1, 1500, 100, 150, 180, 35, 'barracks', 1),
('mountain_warrior', 'kiri', 'Mountain Warrior', 'Fast offensive infantry', 70, 25, 20, 10, 50, 1, 1300, 170, 90, 130, 40, 'barracks', 5),
('highland_pony', 'kiri', 'Highland Pony', 'Fastest cavalry unit', 55, 30, 40, 20, 70, 2, 1800, 220, 170, 280, 60, 'stable', 1),
('trap_maker', 'kiri', 'Trap Maker', 'Defensive specialist', 30, 80, 80, 4, 30, 1, 2000, 200, 200, 150, 50, 'academy', 10),

-- Special troops
('swamp_dragon', 'special', 'Swamp Dragon', 'Scout with stealth ability', 0, 10, 10, 25, 15, 1, 1200, 60, 40, 70, 40, 'barracks', 10),
('locust_swarm', 'special', 'Locust Swarm', 'Destroys enemy crops', 0, 0, 0, 5, 0, 1, 600, 50, 30, 30, 50, 'academy', 15),
('battle_duck', 'special', 'Battle Duck', 'Counters locust swarms', 10, 30, 30, 8, 20, 1, 800, 40, 60, 40, 40, 'barracks', 5),
('portuguese_musketeer', 'special', 'Portuguese Musketeer', 'High attack, fragile defense', 120, 20, 10, 5, 40, 2, 3000, 500, 200, 600, 100, 'academy', 20);
