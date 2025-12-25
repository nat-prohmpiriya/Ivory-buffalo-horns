-- Reverse shop migration

DROP VIEW IF EXISTS active_production_bonuses;

DROP TABLE IF EXISTS subscription_prices;
DROP TABLE IF EXISTS gold_feature_costs;
DROP TABLE IF EXISTS gold_usage;
DROP TABLE IF EXISTS user_subscriptions;
DROP TABLE IF EXISTS transactions;
DROP TABLE IF EXISTS gold_packages;

DROP TYPE IF EXISTS gold_feature;
DROP TYPE IF EXISTS subscription_type;
DROP TYPE IF EXISTS transaction_type;
DROP TYPE IF EXISTS transaction_status;

ALTER TABLE users DROP COLUMN IF EXISTS gold_balance;
