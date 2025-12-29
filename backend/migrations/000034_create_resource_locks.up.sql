-- Resource locks table (escrow system for trades)
CREATE TABLE resource_locks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    village_id UUID NOT NULL REFERENCES villages(id) ON DELETE CASCADE,

    -- Lock reference
    lock_type VARCHAR(20) NOT NULL, -- 'trade_order', 'direct_trade'
    reference_id UUID NOT NULL,     -- trade_order_id or direct_trade_offer_id

    -- Locked resources
    wood INT NOT NULL DEFAULT 0,
    clay INT NOT NULL DEFAULT 0,
    iron INT NOT NULL DEFAULT 0,
    crop INT NOT NULL DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    released_at TIMESTAMPTZ,

    -- Constraints
    CONSTRAINT non_negative_wood CHECK (wood >= 0),
    CONSTRAINT non_negative_clay CHECK (clay >= 0),
    CONSTRAINT non_negative_iron CHECK (iron >= 0),
    CONSTRAINT non_negative_crop CHECK (crop >= 0),
    CONSTRAINT has_locked_resources CHECK (wood > 0 OR clay > 0 OR iron > 0 OR crop > 0)
);

-- Indexes
CREATE INDEX idx_resource_locks_village ON resource_locks(village_id);
CREATE INDEX idx_resource_locks_reference ON resource_locks(lock_type, reference_id);
CREATE INDEX idx_resource_locks_active ON resource_locks(village_id) WHERE released_at IS NULL;
