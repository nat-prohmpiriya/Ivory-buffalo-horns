-- Shop & Premium System with Stripe

-- Add gold balance to users
ALTER TABLE users ADD COLUMN IF NOT EXISTS gold_balance INTEGER NOT NULL DEFAULT 0;

-- Gold packages available for purchase
CREATE TABLE gold_packages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL,
    gold_amount INTEGER NOT NULL,
    price_cents INTEGER NOT NULL, -- Price in cents (e.g., 500 = $5.00)
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    stripe_price_id VARCHAR(100), -- Stripe Price ID
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    bonus_percent INTEGER NOT NULL DEFAULT 0, -- Bonus gold percentage
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed gold packages
INSERT INTO gold_packages (name, gold_amount, price_cents, currency, bonus_percent) VALUES
    ('Starter', 30, 200, 'USD', 0),
    ('Small', 100, 500, 'USD', 0),
    ('Medium', 325, 1500, 'USD', 8),
    ('Large', 750, 3000, 'USD', 15),
    ('Huge', 1500, 5000, 'USD', 20),
    ('Ultimate', 4000, 10000, 'USD', 33);

-- Transaction status enum
CREATE TYPE transaction_status AS ENUM ('pending', 'completed', 'failed', 'refunded');

-- Transaction type enum
CREATE TYPE transaction_type AS ENUM ('gold_purchase', 'subscription', 'gold_spend', 'gold_refund', 'gold_gift');

-- Transactions table
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    transaction_type transaction_type NOT NULL,
    status transaction_status NOT NULL DEFAULT 'pending',

    -- Gold changes
    gold_amount INTEGER NOT NULL DEFAULT 0,

    -- Payment info (for purchases)
    amount_cents INTEGER, -- Amount paid in cents
    currency VARCHAR(3),

    -- Stripe info
    stripe_session_id VARCHAR(200),
    stripe_payment_intent_id VARCHAR(200),
    stripe_subscription_id VARCHAR(200),

    -- Package reference (for gold purchases)
    gold_package_id UUID REFERENCES gold_packages(id),

    -- Metadata
    description TEXT,
    metadata JSONB,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- Index for user transactions
CREATE INDEX idx_transactions_user ON transactions(user_id, created_at DESC);
CREATE INDEX idx_transactions_stripe_session ON transactions(stripe_session_id) WHERE stripe_session_id IS NOT NULL;
CREATE INDEX idx_transactions_stripe_payment ON transactions(stripe_payment_intent_id) WHERE stripe_payment_intent_id IS NOT NULL;

-- Subscription type enum
CREATE TYPE subscription_type AS ENUM ('travian_plus');

-- User subscriptions (Travian Plus)
CREATE TABLE user_subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    subscription_type subscription_type NOT NULL,

    -- Stripe subscription info
    stripe_subscription_id VARCHAR(200),
    stripe_customer_id VARCHAR(200),

    -- Duration
    starts_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,

    -- Auto-renew
    auto_renew BOOLEAN NOT NULL DEFAULT FALSE,

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    cancelled_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for active subscriptions
CREATE INDEX idx_user_subscriptions_active ON user_subscriptions(user_id, subscription_type)
    WHERE is_active = TRUE;
CREATE INDEX idx_user_subscriptions_expiry ON user_subscriptions(expires_at)
    WHERE is_active = TRUE;

-- Gold feature type enum
CREATE TYPE gold_feature AS ENUM (
    'finish_now',           -- Instant complete building/training
    'npc_merchant',         -- Exchange resources
    'production_bonus',     -- +25% production for 1 resource
    'book_of_wisdom',       -- 2x all production for 1 day
    'artwork',              -- Protect resources from raid
    'ointment',             -- Heal troops
    'plus_subscription'     -- Travian Plus days
);

-- Gold usage log
CREATE TABLE gold_usage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    feature gold_feature NOT NULL,
    gold_spent INTEGER NOT NULL,

    -- Target (building, troop queue, village, etc.)
    target_type VARCHAR(50),
    target_id UUID,

    -- Effect details
    effect_data JSONB,

    -- Duration-based features
    expires_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for gold usage
CREATE INDEX idx_gold_usage_user ON gold_usage(user_id, created_at DESC);
CREATE INDEX idx_gold_usage_active ON gold_usage(user_id, feature, expires_at)
    WHERE expires_at IS NOT NULL;

-- Active production bonuses view
CREATE VIEW active_production_bonuses AS
SELECT
    user_id,
    feature,
    effect_data,
    expires_at
FROM gold_usage
WHERE feature IN ('production_bonus', 'book_of_wisdom')
    AND expires_at > NOW();

-- Gold feature costs (configurable)
CREATE TABLE gold_feature_costs (
    feature gold_feature PRIMARY KEY,
    base_cost INTEGER NOT NULL,
    description TEXT
);

-- Seed feature costs
INSERT INTO gold_feature_costs (feature, base_cost, description) VALUES
    ('finish_now', 0, 'Cost calculated based on remaining time'),
    ('npc_merchant', 3, 'Exchange resources instantly'),
    ('production_bonus', 5, '+25% production for one resource type per day'),
    ('book_of_wisdom', 15, '2x production for all resources for 1 day'),
    ('artwork', 2, 'Protect resources from next raid'),
    ('ointment', 2, 'Heal troops after battle'),
    ('plus_subscription', 10, 'Travian Plus per day');

-- Travian Plus subscription prices
CREATE TABLE subscription_prices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    subscription_type subscription_type NOT NULL,
    duration_days INTEGER NOT NULL,
    gold_cost INTEGER NOT NULL,
    stripe_price_id VARCHAR(100),
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Seed subscription prices (buy with gold or money)
INSERT INTO subscription_prices (subscription_type, duration_days, gold_cost) VALUES
    ('travian_plus', 7, 70),
    ('travian_plus', 30, 250),
    ('travian_plus', 90, 600);
