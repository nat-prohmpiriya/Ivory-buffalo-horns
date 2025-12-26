-- Add hero_id to armies table for hero passive bonuses in combat

-- Add hero_id column to armies
ALTER TABLE armies ADD COLUMN hero_id UUID REFERENCES heroes(id);

-- Add index for faster lookup
CREATE INDEX idx_armies_hero_id ON armies(hero_id) WHERE hero_id IS NOT NULL;
