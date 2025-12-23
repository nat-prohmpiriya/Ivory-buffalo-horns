-- Create building type enum
CREATE TYPE building_type AS ENUM (
    -- Village buildings
    'main_building',
    'warehouse',
    'granary',
    'barracks',
    'stable',
    'workshop',
    'academy',
    'smithy',
    'rally_point',
    'market',
    'embassy',
    'town_hall',
    'residence',
    'palace',
    'treasury',
    'trade_office',
    'wall',
    -- Resource fields
    'woodcutter',
    'clay_pit',
    'iron_mine',
    'crop_field'
);

-- Create buildings table
CREATE TABLE IF NOT EXISTS buildings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    village_id UUID NOT NULL REFERENCES villages(id) ON DELETE CASCADE,
    building_type building_type NOT NULL,
    slot INT NOT NULL,
    level INT NOT NULL DEFAULT 0,
    is_upgrading BOOLEAN NOT NULL DEFAULT FALSE,
    upgrade_ends_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Each slot can only have one building per village
    UNIQUE(village_id, slot)
);

-- Indexes
CREATE INDEX idx_buildings_village_id ON buildings(village_id);
CREATE INDEX idx_buildings_upgrading ON buildings(is_upgrading) WHERE is_upgrading = TRUE;
