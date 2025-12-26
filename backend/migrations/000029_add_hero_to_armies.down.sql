-- Revert hero_id from armies table

-- Remove index
DROP INDEX IF EXISTS idx_armies_hero_id;

-- Remove hero_id column
ALTER TABLE armies DROP COLUMN IF EXISTS hero_id;
