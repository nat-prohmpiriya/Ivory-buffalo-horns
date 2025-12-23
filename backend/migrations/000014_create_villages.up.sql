-- Create villages table
CREATE TABLE IF NOT EXISTS villages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    x INT NOT NULL,
    y INT NOT NULL,
    is_capital BOOLEAN NOT NULL DEFAULT FALSE,
    -- Resources
    wood INT NOT NULL DEFAULT 500,
    clay INT NOT NULL DEFAULT 500,
    iron INT NOT NULL DEFAULT 500,
    crop INT NOT NULL DEFAULT 500,
    -- Storage limits
    warehouse_capacity INT NOT NULL DEFAULT 800,
    granary_capacity INT NOT NULL DEFAULT 800,
    -- Stats
    population INT NOT NULL DEFAULT 2,
    culture_points INT NOT NULL DEFAULT 0,
    loyalty INT NOT NULL DEFAULT 100,
    -- Timestamps
    resources_updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Constraints
    UNIQUE(x, y)
);

-- Indexes
CREATE INDEX idx_villages_user_id ON villages(user_id);
CREATE INDEX idx_villages_coordinates ON villages(x, y);
