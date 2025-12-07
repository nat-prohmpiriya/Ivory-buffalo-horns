CREATE TYPE troop_type AS ENUM (
    -- Phasuttha
    'infantry', 'spearman', 'war_elephant', 'buffalo_wagon',
    -- Nava
    'kris_warrior', 'sea_diver', 'war_prahu', 'merchant_ship',
    -- Kiri
    'crossbowman', 'mountain_warrior', 'highland_pony', 'trap_maker',
    -- Special
    'swamp_dragon', 'locust_swarm', 'battle_duck', 'portuguese_musketeer'
);

CREATE TABLE troop_definitions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    troop_type troop_type UNIQUE NOT NULL,
    tribe_id UUID REFERENCES tribes(id), -- NULL for special troops
    name_i18n JSONB NOT NULL,
    description_i18n JSONB NOT NULL,
    -- Base stats
    attack INT NOT NULL,
    defense_infantry INT NOT NULL,
    defense_cavalry INT NOT NULL,
    speed INT NOT NULL, -- Tiles per hour
    carry_capacity INT NOT NULL,
    crop_consumption INT NOT NULL,
    -- Training
    training_time_seconds INT NOT NULL,
    wood_cost INT NOT NULL,
    clay_cost INT NOT NULL,
    iron_cost INT NOT NULL,
    crop_cost INT NOT NULL,
    -- Requirements
    required_building building_type NOT NULL,
    required_building_level INT DEFAULT 1
);

CREATE TABLE troops (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    village_id UUID NOT NULL REFERENCES villages(id) ON DELETE CASCADE,
    troop_type troop_type NOT NULL,
    count INT DEFAULT 0,
    in_training INT DEFAULT 0,
    UNIQUE(village_id, troop_type)
);

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
