-- Seed 16 troop definitions based on Travian reference
-- Mapping: Phasuttha=Romans, Nava=Teutons, Kiri=Gauls

-- Get tribe IDs
WITH tribe_ids AS (
    SELECT id, code FROM tribes
)

INSERT INTO troop_definitions (
    troop_type, tribe_id, name_i18n, description_i18n,
    attack, defense_infantry, defense_cavalry, speed, carry_capacity, crop_consumption,
    training_time_seconds, wood_cost, clay_cost, iron_cost, crop_cost,
    required_building, required_building_level
)
SELECT * FROM (
    VALUES
    -- ============ PHASUTTHA (Mainland/Romans) ============
    -- Infantry: Balanced like Legionnaire
    ('infantry'::troop_type, 
     (SELECT id FROM tribes WHERE code = 'phasuttha'),
     '{"th": "ทหารราบ", "en": "Infantry"}'::jsonb,
     '{"th": "ทหารราบพื้นฐาน สมดุลระหว่างการโจมตีและป้องกัน", "en": "Basic infantry unit, balanced between offense and defense."}'::jsonb,
     40, 35, 50, 6, 50, 1,
     1200, 120, 100, 150, 30,
     'barracks'::building_type, 1),
    
    -- Spearman: Anti-cavalry like Praetorian
    ('spearman'::troop_type,
     (SELECT id FROM tribes WHERE code = 'phasuttha'),
     '{"th": "ทหารหอก", "en": "Spearman"}'::jsonb,
     '{"th": "ผู้เชี่ยวชาญในการต่อต้านทหารม้า มีโล่หนักป้องกัน", "en": "Anti-cavalry specialist with heavy shield defense."}'::jsonb,
     30, 65, 35, 5, 35, 1,
     1500, 100, 130, 160, 70,
     'barracks'::building_type, 3),
    
    -- War Elephant: Heavy cavalry like Equites Caesaris
    ('war_elephant'::troop_type,
     (SELECT id FROM tribes WHERE code = 'phasuttha'),
     '{"th": "ช้างศึก", "en": "War Elephant"}'::jsonb,
     '{"th": "หน่วยรบหนักที่สุด พลังโจมตีสูง แต่เคลื่อนที่ช้า", "en": "Heaviest combat unit with massive attack power but slow movement."}'::jsonb,
     180, 80, 105, 4, 80, 4,
     4800, 550, 640, 800, 180,
     'elephant_ground'::building_type, 1),
    
    -- Buffalo Wagon: Transport like Settler capacity
    ('buffalo_wagon'::troop_type,
     (SELECT id FROM tribes WHERE code = 'phasuttha'),
     '{"th": "เกวียนควาย", "en": "Buffalo Wagon"}'::jsonb,
     '{"th": "หน่วยขนส่งที่บรรทุกทรัพยากรได้มากที่สุด", "en": "Transport unit with the highest carry capacity."}'::jsonb,
     10, 30, 30, 5, 500, 2,
     2400, 400, 300, 200, 100,
     'stable'::building_type, 5),

    -- ============ NAVA (Maritime/Teutons) ============
    -- Kris Warrior: Raider like Clubswinger
    ('kris_warrior'::troop_type,
     (SELECT id FROM tribes WHERE code = 'nava'),
     '{"th": "นักรบกริช", "en": "Kris Warrior"}'::jsonb,
     '{"th": "นักรบราคาถูก ฝึกเร็ว เหมาะสำหรับการปล้นสะดม", "en": "Cheap and fast-training warrior, excellent for raiding."}'::jsonb,
     45, 20, 5, 7, 60, 1,
     800, 90, 30, 60, 40,
     'barracks'::building_type, 1),
    
    -- Sea Diver: Scout
    ('sea_diver'::troop_type,
     (SELECT id FROM tribes WHERE code = 'nava'),
     '{"th": "นักดำน้ำ", "en": "Sea Diver"}'::jsonb,
     '{"th": "หน่วยสอดแนมทางน้ำ เคลื่อนที่เงียบและรวดเร็ว", "en": "Water scout unit, moves silently and quickly."}'::jsonb,
     0, 10, 5, 9, 10, 1,
     600, 60, 30, 40, 20,
     'barracks'::building_type, 5),
    
    -- War Prahu: Naval attack like Teutonic Knight
    ('war_prahu'::troop_type,
     (SELECT id FROM tribes WHERE code = 'nava'),
     '{"th": "เรือรบปราหู", "en": "War Prahu"}'::jsonb,
     '{"th": "เรือรบที่แล่นข้ามน้ำได้ พลังโจมตีสูง", "en": "Warship that can cross water, high attack power."}'::jsonb,
     150, 50, 75, 8, 100, 3,
     3600, 450, 510, 480, 80,
     'workshop'::building_type, 1),
    
    -- Merchant Ship: Trade
    ('merchant_ship'::troop_type,
     (SELECT id FROM tribes WHERE code = 'nava'),
     '{"th": "เรือสินค้า", "en": "Merchant Ship"}'::jsonb,
     '{"th": "เรือค้าขายที่เร็วและบรรทุกได้มาก", "en": "Fast trading ship with high carrying capacity."}'::jsonb,
     5, 20, 20, 12, 750, 2,
     2000, 300, 200, 150, 80,
     'market'::building_type, 10),

    -- ============ KIRI (Highland/Gauls) ============
    -- Crossbowman: Defense like Phalanx
    ('crossbowman'::troop_type,
     (SELECT id FROM tribes WHERE code = 'kiri'),
     '{"th": "ทหารหน้าไม้", "en": "Crossbowman"}'::jsonb,
     '{"th": "นักธนูป้องกัน ยิงทะลุเกราะได้ดี", "en": "Defensive archer with armor-piercing bolts."}'::jsonb,
     15, 40, 50, 7, 35, 1,
     1000, 100, 130, 50, 30,
     'barracks'::building_type, 1),
    
    -- Mountain Warrior: Offense like Swordsman
    ('mountain_warrior'::troop_type,
     (SELECT id FROM tribes WHERE code = 'kiri'),
     '{"th": "นักรบภูเขา", "en": "Mountain Warrior"}'::jsonb,
     '{"th": "ทหารราบโจมตีที่เคลื่อนที่เร็วบนภูเขา", "en": "Offensive infantry that moves fast on mountains."}'::jsonb,
     65, 35, 20, 8, 45, 1,
     1400, 140, 150, 185, 60,
     'barracks'::building_type, 3),
    
    -- Highland Pony: Fast cavalry like Theutates Thunder
    ('highland_pony'::troop_type,
     (SELECT id FROM tribes WHERE code = 'kiri'),
     '{"th": "ม้าเขาสูง", "en": "Highland Pony"}'::jsonb,
     '{"th": "ทหารม้าเร็วที่สุด เหมาะสำหรับการโจมตีกองหลัง", "en": "Fastest cavalry, ideal for flanking attacks."}'::jsonb,
     90, 25, 40, 19, 75, 2,
     2000, 350, 450, 230, 60,
     'stable'::building_type, 1),
    
    -- Trap Maker: Defensive special like Druidrider
    ('trap_maker'::troop_type,
     (SELECT id FROM tribes WHERE code = 'kiri'),
     '{"th": "ช่างกับดัก", "en": "Trap Maker"}'::jsonb,
     '{"th": "หน่วยพิเศษที่วางกับดักจับศัตรู", "en": "Special unit that sets traps to capture enemies."}'::jsonb,
     45, 115, 40, 6, 30, 2,
     2400, 360, 330, 280, 120,
     'academy'::building_type, 10),

    -- ============ SPECIAL (All tribes) ============
    -- Swamp Dragon (Water Monitor): Scout/Stealth
    ('swamp_dragon'::troop_type,
     NULL,
     '{"th": "เหี้ย", "en": "Swamp Dragon"}'::jsonb,
     '{"th": "หน่วยสอดแนมพิเศษที่ว่ายน้ำได้เร็ว", "en": "Special scout that swims quickly through swamps."}'::jsonb,
     25, 30, 20, 14, 20, 1,
     1800, 200, 100, 150, 100,
     'granary'::building_type, 10),
    
    -- Locust Swarm: Destroys crops
    ('locust_swarm'::troop_type,
     NULL,
     '{"th": "ฝูงตั๊กแตน", "en": "Locust Swarm"}'::jsonb,
     '{"th": "หน่วยพิเศษที่ทำลายนาข้าวศัตรู", "en": "Special unit that destroys enemy crop fields."}'::jsonb,
     10, 5, 5, 10, 0, 3,
     3600, 500, 200, 100, 500,
     'academy'::building_type, 15),
    
    -- Battle Duck: Counter locusts
    ('battle_duck'::troop_type,
     NULL,
     '{"th": "เป็ดรบ", "en": "Battle Duck"}'::jsonb,
     '{"th": "หน่วยพิเศษที่ต่อต้านฝูงตั๊กแตน", "en": "Special unit that counters locust swarms."}'::jsonb,
     35, 40, 30, 8, 15, 1,
     1500, 150, 100, 80, 200,
     'duck_farm'::building_type, 1),
    
    -- Portuguese Musketeer: High attack fragile
    ('portuguese_musketeer'::troop_type,
     NULL,
     '{"th": "ทหารปืนโปรตุเกส", "en": "Portuguese Musketeer"}'::jsonb,
     '{"th": "ทหารโจมตีสูงสุดแต่ป้องกันต่ำมาก", "en": "Highest attack but very fragile defense."}'::jsonb,
     200, 15, 10, 5, 40, 2,
     4200, 400, 300, 600, 150,
     'tavern'::building_type, 10)
) AS v(
    troop_type, tribe_id, name_i18n, description_i18n,
    attack, defense_infantry, defense_cavalry, speed, carry_capacity, crop_consumption,
    training_time_seconds, wood_cost, clay_cost, iron_cost, crop_cost,
    required_building, required_building_level
);
