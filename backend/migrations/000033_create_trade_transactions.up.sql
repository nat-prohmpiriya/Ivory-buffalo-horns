-- Trade transactions table (completed trade history)
CREATE TABLE trade_transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Order references
    buy_order_id UUID NOT NULL REFERENCES trade_orders(id),
    sell_order_id UUID NOT NULL REFERENCES trade_orders(id),

    -- Parties involved
    buyer_id UUID NOT NULL REFERENCES users(id),
    seller_id UUID NOT NULL REFERENCES users(id),
    buyer_village_id UUID NOT NULL REFERENCES villages(id),
    seller_village_id UUID NOT NULL REFERENCES villages(id),

    -- Transaction details
    resource_type trade_resource_type NOT NULL,
    quantity INT NOT NULL,
    price_per_unit INT NOT NULL,
    total_gold INT NOT NULL, -- quantity * price_per_unit

    -- Timestamp
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    CONSTRAINT positive_tx_quantity CHECK (quantity > 0),
    CONSTRAINT positive_tx_price CHECK (price_per_unit > 0),
    CONSTRAINT positive_tx_total CHECK (total_gold > 0),
    CONSTRAINT different_parties CHECK (buyer_id != seller_id)
);

-- Indexes
CREATE INDEX idx_trade_tx_buyer ON trade_transactions(buyer_id);
CREATE INDEX idx_trade_tx_seller ON trade_transactions(seller_id);
CREATE INDEX idx_trade_tx_created ON trade_transactions(created_at DESC);
CREATE INDEX idx_trade_tx_buy_order ON trade_transactions(buy_order_id);
CREATE INDEX idx_trade_tx_sell_order ON trade_transactions(sell_order_id);
