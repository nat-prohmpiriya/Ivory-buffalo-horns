-- Trade order status
CREATE TYPE trade_order_status AS ENUM (
    'open',              -- Order is active and can be filled
    'partially_filled',  -- Order has been partially filled
    'filled',            -- Order has been completely filled
    'cancelled',         -- Order was cancelled by owner
    'expired'            -- Order expired (if expiry was set)
);

-- Trade order type (buy or sell)
CREATE TYPE trade_order_type AS ENUM (
    'buy',   -- Want to buy resources with gold
    'sell'   -- Want to sell resources for gold
);

-- Resource types for trading
CREATE TYPE trade_resource_type AS ENUM (
    'wood',
    'clay',
    'iron',
    'crop'
);
