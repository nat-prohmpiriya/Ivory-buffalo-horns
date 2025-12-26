-- Named Heroes with Passive Bonuses

-- Hero definitions table (named historical heroes)
CREATE TABLE hero_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Basic info
    name VARCHAR(100) NOT NULL,           -- Thai/local name
    name_en VARCHAR(100) NOT NULL,        -- English name
    tribe tribe_type NOT NULL,            -- phasuttha, nava, kiri
    rarity INTEGER NOT NULL CHECK (rarity >= 1 AND rarity <= 5),  -- 1-5 stars

    -- Description
    description TEXT,
    description_en TEXT,

    -- Base stats (multiplier %, 100 = normal)
    base_attack INTEGER NOT NULL DEFAULT 100,
    base_defense INTEGER NOT NULL DEFAULT 100,
    base_speed INTEGER NOT NULL DEFAULT 100,

    -- Passive bonuses as JSON array
    -- e.g. [{"bonus_type": "elephant_attack", "value": 30, "description": "+30% War Elephant attack"}]
    passive_bonuses JSONB NOT NULL DEFAULT '[]',

    -- Recruitment settings
    tavern_cost_gold INTEGER,             -- Gold cost to recruit from tavern (NULL = not available)
    quest_obtainable BOOLEAN NOT NULL DEFAULT FALSE,
    season_reward BOOLEAN NOT NULL DEFAULT FALSE,

    -- Image/assets
    portrait_url VARCHAR(500),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for tribe lookup
CREATE INDEX idx_hero_definitions_tribe ON hero_definitions(tribe);
CREATE INDEX idx_hero_definitions_rarity ON hero_definitions(rarity);

-- Add hero_definition_id to heroes table
ALTER TABLE heroes ADD COLUMN hero_definition_id UUID REFERENCES hero_definitions(id);

-- Seed Phasuttha Tribe Heroes (Thai/Siam)
INSERT INTO hero_definitions (name, name_en, tribe, rarity, description, description_en, base_attack, base_defense, base_speed, passive_bonuses, tavern_cost_gold) VALUES
-- 5 Star
('พ่อขุนรามคำแหง', 'King Ramkhamhaeng', 'phasuttha', 5,
 'กษัตริย์ผู้ยิ่งใหญ่แห่งสุโขทัย ผู้ประดิษฐ์อักษรไทย',
 'The great king of Sukhothai who invented Thai script',
 100, 110, 100,
 '[{"bonus_type": "resource_production", "value": 20, "description": "+20% Resource production"}, {"bonus_type": "diplomacy_speed", "value": 50, "description": "+50% NAP negotiation speed"}]',
 500),

('สมเด็จพระนเรศวร', 'King Naresuan', 'phasuttha', 5,
 'มหาราชผู้กอบกู้เอกราช ทรงชนช้างกับพระมหาอุปราชา',
 'The great king who restored independence, famous for elephant duel',
 130, 100, 110,
 '[{"bonus_type": "elephant_attack", "value": 30, "description": "+30% War Elephant attack"}, {"bonus_type": "hero_combat", "value": 20, "description": "+20% Hero combat strength"}]',
 500),

-- 4 Star
('พระยาตากสิน', 'King Taksin', 'phasuttha', 4,
 'ผู้กอบกู้กรุงศรีอยุธยา และสถาปนากรุงธนบุรี',
 'The liberator who restored Ayutthaya and founded Thonburi',
 120, 90, 120,
 '[{"bonus_type": "army_speed", "value": 25, "description": "+25% Army travel speed"}, {"bonus_type": "night_attack", "value": 30, "description": "+30% Night raid damage"}]',
 300),

('สมเด็จพระสุริโยทัย', 'Queen Suriyothai', 'phasuttha', 4,
 'พระมเหสีผู้เสียสละพระชนม์ปกป้องพระราชสวามี',
 'The queen who sacrificed her life protecting the king',
 90, 130, 100,
 '[{"bonus_type": "defense_bonus", "value": 20, "description": "+20% Village defense"}, {"bonus_type": "troop_morale", "value": 15, "description": "+15% Troop morale"}]',
 300),

-- 3 Star
('ท้าวเทพกระษัตรี', 'Thao Thep Krasattri', 'phasuttha', 3,
 'วีรสตรีผู้ป้องกันเมืองถลาง',
 'Heroine who defended Thalang from Burmese invasion',
 100, 120, 90,
 '[{"bonus_type": "wall_defense", "value": 15, "description": "+15% Wall defense"}, {"bonus_type": "rally_villagers", "value": 10, "description": "+10% Villager defense contribution"}]',
 150),

('พระยาพิชัยดาบหัก', 'Phraya Pichai', 'phasuttha', 3,
 'แม่ทัพคู่บารมีพระเจ้าตากสิน ผู้รบจนดาบหัก',
 'The loyal general who fought until his sword broke',
 120, 100, 100,
 '[{"bonus_type": "infantry_attack", "value": 20, "description": "+20% Infantry attack"}, {"bonus_type": "last_stand", "value": 25, "description": "+25% Attack when outnumbered"}]',
 150),

-- 2 Star (Starting heroes)
('ขุนแผน', 'Khun Phaen', 'phasuttha', 2,
 'นักรบในตำนานผู้เชี่ยวชาญไสยศาสตร์',
 'Legendary warrior skilled in mystical arts',
 100, 90, 110,
 '[{"bonus_type": "scout_range", "value": 10, "description": "+10% Scout range"}, {"bonus_type": "stealth", "value": 15, "description": "+15% Chance to avoid detection"}]',
 50),

('นางนาก', 'Nang Nak', 'phasuttha', 2,
 'วิญญาณหญิงผู้ซื่อสัตย์ต่อสามี',
 'The faithful spirit protecting her beloved',
 80, 100, 100,
 '[{"bonus_type": "crop_production", "value": 15, "description": "+15% Crop production"}, {"bonus_type": "village_loyalty", "value": 10, "description": "+10% Village loyalty"}]',
 50);

-- Seed Nava Tribe Heroes (Indonesia/Malaysia/Philippines)
INSERT INTO hero_definitions (name, name_en, tribe, rarity, description, description_en, base_attack, base_defense, base_speed, passive_bonuses, tavern_cost_gold) VALUES
-- 5 Star
('Gajah Mada', 'Gajah Mada', 'nava', 5,
 'มหาปาติห์ผู้ยิ่งใหญ่แห่งมัชปาหิต ผู้สาบานปาลาปา',
 'The great prime minister of Majapahit who swore the Palapa oath',
 120, 100, 110,
 '[{"bonus_type": "conquest_speed", "value": 30, "description": "+30% Conquest speed"}, {"bonus_type": "loyalty_reduction", "value": 20, "description": "+20% Enemy loyalty reduction"}]',
 500),

('Hang Tuah', 'Hang Tuah', 'nava', 5,
 'นักรบในตำนานแห่งมะละกา ผู้ไม่เคยพ่ายแพ้',
 'Legendary warrior of Malacca, never defeated',
 130, 90, 120,
 '[{"bonus_type": "naval_attack", "value": 25, "description": "+25% Naval attack"}, {"bonus_type": "critical_hit", "value": 15, "description": "+15% Critical hit chance"}]',
 500),

-- 4 Star
('Lapu-Lapu', 'Lapu-Lapu', 'nava', 4,
 'หัวหน้าเผ่าผู้สังหาร Magellan',
 'The chieftain who killed Magellan',
 120, 100, 100,
 '[{"bonus_type": "anti_colonial", "value": 35, "description": "+35% Damage vs NPC invaders"}, {"bonus_type": "first_strike", "value": 20, "description": "+20% First attack bonus"}]',
 300),

('Sultan Agung', 'Sultan Agung', 'nava', 4,
 'สุลต่านแห่งมะตะรัม ผู้ต่อต้าน VOC',
 'Sultan of Mataram who resisted the Dutch VOC',
 110, 110, 100,
 '[{"bonus_type": "siege_damage", "value": 25, "description": "+25% Siege damage"}, {"bonus_type": "resistance", "value": 30, "description": "+30% Defense vs NPC"}]',
 300),

-- 3 Star
('Parameswara', 'Parameswara', 'nava', 3,
 'ผู้ก่อตั้งรัฐสุลต่านมะละกา',
 'Founder of the Malacca Sultanate',
 90, 100, 110,
 '[{"bonus_type": "trade_income", "value": 20, "description": "+20% Trade income"}, {"bonus_type": "merchant_speed", "value": 15, "description": "+15% Merchant speed"}]',
 150),

('Rajah Sulayman', 'Rajah Sulayman', 'nava', 3,
 'ราชาแห่งมะนิลา ผู้ต่อต้านสเปน',
 'Raja of Manila who resisted Spanish colonization',
 100, 110, 100,
 '[{"bonus_type": "port_defense", "value": 15, "description": "+15% Port defense"}, {"bonus_type": "water_crossing", "value": 20, "description": "+20% Water crossing speed"}]',
 150),

-- 2 Star
('Kris Warrior', 'Kris Warrior', 'nava', 2,
 'นักรบกริชผู้คล่องแคล่ว',
 'Swift warrior wielding the sacred kris',
 100, 80, 120,
 '[{"bonus_type": "raid_capacity", "value": 10, "description": "+10% Raid carry capacity"}, {"bonus_type": "raid_speed", "value": 15, "description": "+15% Raid speed"}]',
 50),

('Srikandi', 'Srikandi', 'nava', 2,
 'นักธนูหญิงในวรรณกรรม',
 'The legendary female archer from wayang',
 90, 90, 110,
 '[{"bonus_type": "ranged_attack", "value": 15, "description": "+15% Ranged attack"}, {"bonus_type": "accuracy", "value": 10, "description": "+10% Attack accuracy"}]',
 50);

-- Seed Kiri Tribe Heroes (Vietnam/Cambodia/Myanmar)
INSERT INTO hero_definitions (name, name_en, tribe, rarity, description, description_en, base_attack, base_defense, base_speed, passive_bonuses, tavern_cost_gold) VALUES
-- 5 Star
('Tran Hung Dao', 'Tran Hung Dao', 'kiri', 5,
 'แม่ทัพผู้เอาชนะมองโกลสามครั้ง',
 'The general who defeated Mongols three times',
 110, 130, 100,
 '[{"bonus_type": "anti_invasion", "value": 35, "description": "+35% Defense vs invaders"}, {"bonus_type": "scorched_earth", "value": 25, "description": "Destroy 25% resources before being raided"}]',
 500),

('พระเจ้าชัยวรมันที่ 7', 'King Jayavarman VII', 'kiri', 5,
 'กษัตริย์ผู้ยิ่งใหญ่แห่งเขมร ผู้สร้างนครธม',
 'The great Khmer king who built Angkor Thom',
 100, 120, 90,
 '[{"bonus_type": "building_speed", "value": 25, "description": "+25% Building construction speed"}, {"bonus_type": "wonder_bonus", "value": 20, "description": "+20% Special building effects"}]',
 500),

-- 4 Star
('บุเรงนอง', 'Bayinnaung', 'kiri', 4,
 'กษัตริย์นักรบแห่งพม่า ผู้รวบรวมดินแดน',
 'The warrior king of Burma who united the realm',
 130, 100, 100,
 '[{"bonus_type": "elephant_damage", "value": 30, "description": "+30% Elephant damage"}, {"bonus_type": "conqueror", "value": 20, "description": "+20% Loyalty reduction on conquest"}]',
 300),

('Le Loi', 'Le Loi', 'kiri', 4,
 'ผู้ก่อตั้งราชวงศ์เล ผู้ขับไล่จีน',
 'Founder of Le dynasty who expelled the Chinese',
 120, 100, 110,
 '[{"bonus_type": "guerrilla", "value": 30, "description": "+30% Guerrilla warfare bonus"}, {"bonus_type": "ambush", "value": 25, "description": "+25% Ambush damage"}]',
 300),

-- 3 Star
('Hai Ba Trung', 'Trung Sisters', 'kiri', 3,
 'สองพี่น้องผู้นำกบฏต่อต้านจีน',
 'The two sisters who led rebellion against Chinese rule',
 100, 100, 110,
 '[{"bonus_type": "rebellion_bonus", "value": 25, "description": "+25% Rebellion effectiveness"}, {"bonus_type": "hero_slot", "value": 1, "description": "+1 Hero slot"}]',
 150),

('Maha Bandula', 'Maha Bandula', 'kiri', 3,
 'แม่ทัพพม่าผู้กล้าหาญ',
 'The brave Burmese general',
 100, 120, 90,
 '[{"bonus_type": "infantry_defense", "value": 20, "description": "+20% Infantry defense"}, {"bonus_type": "tactical_retreat", "value": 15, "description": "Save 15% troops on retreat"}]',
 150),

-- 2 Star
('Crossbow Master', 'Crossbow Master', 'kiri', 2,
 'ปรมาจารย์หน้าไม้แห่งขุนเขา',
 'Master crossbowman of the highlands',
 110, 90, 90,
 '[{"bonus_type": "ranged_attack", "value": 15, "description": "+15% Ranged unit attack"}, {"bonus_type": "volley", "value": 10, "description": "+10% First volley damage"}]',
 50),

('Mountain Scout', 'Mountain Scout', 'kiri', 2,
 'นักสอดแนมผู้รู้จักเทือกเขาทุกซอกมุม',
 'Scout who knows every corner of the mountains',
 80, 90, 120,
 '[{"bonus_type": "mountain_speed", "value": 20, "description": "+20% Speed in mountains"}, {"bonus_type": "terrain_bonus", "value": 15, "description": "+15% Combat bonus in highlands"}]',
 50);

-- Add index for faster hero lookup by definition
CREATE INDEX idx_heroes_definition ON heroes(hero_definition_id) WHERE hero_definition_id IS NOT NULL;
