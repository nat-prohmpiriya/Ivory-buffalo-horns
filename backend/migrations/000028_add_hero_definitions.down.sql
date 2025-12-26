-- Remove hero_definition_id from heroes
DROP INDEX IF EXISTS idx_heroes_definition;
ALTER TABLE heroes DROP COLUMN IF EXISTS hero_definition_id;

-- Drop hero_definitions table
DROP TABLE IF EXISTS hero_definitions;
