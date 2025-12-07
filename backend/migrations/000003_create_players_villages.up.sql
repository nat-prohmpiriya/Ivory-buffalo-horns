CREATE TABLE players (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    server_id UUID NOT NULL REFERENCES servers(id),
    tribe_id UUID NOT NULL REFERENCES tribes(id),
    name VARCHAR(50) NOT NULL,
    gold INT DEFAULT 0,
    silver INT DEFAULT 0,
    vip_level INT DEFAULT 0,
    vip_expires_at TIMESTAMPTZ,
    protection_until TIMESTAMPTZ, -- Beginner protection
    is_banned BOOLEAN DEFAULT FALSE,
    last_active_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, server_id),
    UNIQUE(server_id, name)
);

CREATE TABLE villages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    server_id UUID NOT NULL REFERENCES servers(id), -- Denormalized for query optimization
    player_id UUID NOT NULL REFERENCES players(id),
    name VARCHAR(50) NOT NULL,
    x INT NOT NULL,
    y INT NOT NULL,
    is_capital BOOLEAN DEFAULT FALSE,
    population INT DEFAULT 0,
    
    -- Resources (stored as decimals for precise production)
    wood DECIMAL(15,2) DEFAULT 750.00,
    clay DECIMAL(15,2) DEFAULT 750.00,
    iron DECIMAL(15,2) DEFAULT 750.00,
    crop DECIMAL(15,2) DEFAULT 750.00,
    
    -- Production rates per hour
    wood_production DECIMAL(10,2) DEFAULT 10.00,
    clay_production DECIMAL(10,2) DEFAULT 10.00,
    iron_production DECIMAL(10,2) DEFAULT 10.00,
    crop_production DECIMAL(10,2) DEFAULT 10.00,
    crop_consumption DECIMAL(10,2) DEFAULT 0.00,
    
    -- Capacity
    warehouse_capacity INT DEFAULT 800,
    granary_capacity INT DEFAULT 800,
    
    -- Defense
    wall_level INT DEFAULT 0,
    loyalty INT DEFAULT 100,
    
    -- Timestamps
    resources_updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(server_id, x, y) -- One village per tile in a server
);

CREATE INDEX idx_villages_coords ON villages(server_id, x, y);
CREATE INDEX idx_villages_player ON villages(player_id);
