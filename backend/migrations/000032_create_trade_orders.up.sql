-- Trade orders table (Market Board listings)
CREATE TABLE trade_orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    village_id UUID NOT NULL REFERENCES villages(id) ON DELETE CASCADE,

    -- Order details
    order_type trade_order_type NOT NULL,
    resource_type trade_resource_type NOT NULL,
    quantity INT NOT NULL,
    quantity_filled INT NOT NULL DEFAULT 0,
    price_per_unit INT NOT NULL, -- gold per resource unit

    -- Status and timing
    status trade_order_status NOT NULL DEFAULT 'open',
    expires_at TIMESTAMPTZ, -- NULL = no expiration

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    filled_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,

    -- Constraints
    CONSTRAINT positive_quantity CHECK (quantity > 0),
    CONSTRAINT positive_price CHECK (price_per_unit > 0),
    CONSTRAINT valid_filled CHECK (quantity_filled >= 0 AND quantity_filled <= quantity)
);

-- Indexes
CREATE INDEX idx_trade_orders_user ON trade_orders(user_id);
CREATE INDEX idx_trade_orders_village ON trade_orders(village_id);
CREATE INDEX idx_trade_orders_open ON trade_orders(status) WHERE status = 'open';
CREATE INDEX idx_trade_orders_resource_type ON trade_orders(resource_type, order_type);
CREATE INDEX idx_trade_orders_price ON trade_orders(resource_type, order_type, price_per_unit);
CREATE INDEX idx_trade_orders_expires ON trade_orders(expires_at) WHERE expires_at IS NOT NULL;
