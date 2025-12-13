-- Seed 3 tribes based on Travian reference
-- Phasuttha = Romans (Balanced, Build bonus)
-- Nava = Teutons (Raid, Cheap troops, High capacity)
-- Kiri = Gauls (Defense, Speed, Traps)

INSERT INTO tribes (code, name_i18n, description_i18n, bonus_attack, bonus_defense, bonus_speed, bonus_capacity)
VALUES 
(
    'phasuttha',
    '{"th": "พสุธา", "en": "Phasuttha"}',
    '{"th": "เผ่าแห่งที่ราบภาคกลาง ผู้เชี่ยวชาญในการสร้างอาณาจักรที่มั่นคง มีช้างศึกอันแข็งแกร่งและกำแพงที่หนาแน่น เหมาะสำหรับผู้เล่นที่ต้องการความสมดุลระหว่างการโจมตีและป้องกัน", "en": "The Mainland Tribe, masters of building stable kingdoms. Known for their powerful War Elephants and thick walls. Ideal for players seeking balance between offense and defense."}',
    1.00,  -- Balanced attack
    1.10,  -- +10% defense (thick walls)
    1.00,  -- Normal speed
    1.00   -- Normal capacity
),
(
    'nava',
    '{"th": "นาวา", "en": "Nava"}',
    '{"th": "เผ่าชาวน้ำแห่งหมู่เกาะ ผู้เชี่ยวชาญในการปล้นสะดมและการค้าขาย มีเรือรบที่รวดเร็วและพ่อค้าที่ขนของได้มากที่สุด ทหารราคาถูกและฝึกได้เร็ว", "en": "The Maritime Tribe of the islands, masters of raiding and trading. Known for fast warships and merchants with the highest carry capacity. Cheap and fast-training troops."}',
    1.05,  -- +5% attack (raid bonus)
    0.95,  -- -5% defense (weak defense)
    1.00,  -- Normal speed
    1.15   -- +15% capacity (best traders)
),
(
    'kiri',
    '{"th": "คีรี", "en": "Kiri"}',
    '{"th": "เผ่าแห่งเขาสูง ผู้เชี่ยวชาญในการป้องกันและการซุ่มโจมตี มีกับดักที่ร้ายกาจและทหารที่เคลื่อนที่รวดเร็วบนภูเขา เหมาะสำหรับผู้เล่นที่ชอบเล่นแบบตั้งรับ", "en": "The Highland Tribe, masters of defense and ambush tactics. Known for deadly traps and troops that move swiftly through mountains. Perfect for defensive playstyles."}',
    0.95,  -- -5% attack (defensive tribe)
    1.15,  -- +15% defense (best defense)
    1.10,  -- +10% speed (fast troops)
    0.90   -- -10% capacity (smaller traders)
);
