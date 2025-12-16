# Product Specification: Tusk & Horn

## Project Overview
- **Product Name:** Tusk & Horn
- **Genre:** Real-Time Strategy (RTS) / Browser-based MMO Strategy Game
- **Theme:** Southeast Asian Ancient Kingdoms
- **Art Style:** Thai Mural Toon (Stylized Thai temple mural painting)

---

## 1. User Personas

### Persona 1: The Nostalgic Veteran
**Demographics:**
- Male, Age 28-45
- Office worker / Programmer / Business owner
- Medium-High income

**Behaviors:**
- Previously played Travian / Tribal Wars / Ikariam during school years
- No time to monitor screen all day, but has purchasing power
- Plays during work breaks or checks phone during free time
- Wants the same excitement but more convenient

**Pain Points:**
- Original Travian requires too much time commitment
- Most mobile games are too Pay-to-Win
- Wants a game playable on both PC and mobile

**Goals:**
- Find a strategy game that can be played casually but has depth
- Wants to spend money for convenience, not for winning

---

### Persona 2: The Cultural Patriot
**Demographics:**
- Male/Female, Age 18-35
- Student or early career professional
- Interested in history, period dramas

**Behaviors:**
- Enjoys Civilization / Age of Empires
- Watches Thai historical dramas
- Comments and debates about history topics
- Shares Thai culture content on Social Media

**Pain Points:**
- Most Strategy games have European/Chinese/Japanese themes
- Wants to see SEA culture in world-class games

**Goals:**
- Play a game with characters and culture they're proud of
- Wants historically deep content

---

### Persona 3: The Collector
**Demographics:**
- Male/Female, Age 16-30 (Gen Z)
- Collects Art Toys, Stickers, cute items
- Active on TikTok / Instagram

**Behaviors:**
- Buys blind boxes (Pop Mart / Molly)
- Uses LINE stickers frequently
- May not play games hardcore but loves Characters
- Shares collection photos on Social

**Pain Points:**
- Most Art Toys are foreign styles
- Wants Thai Characters that are cool and not outdated

**Goals:**
- Collect Art Toys with unique identity
- Wants Characters usable in daily life (Sticker, apparel)

---

## 2. User Journeys

### Journey 1: First-Time Player

**Stage 1: Discovery**
1. User sees game advertisement/post on Facebook/TikTok
2. Notices unique Thai art style
3. Sees interesting characters like "Buffalo" or "Water Monitor"
4. Clicks to game website

**Stage 2: Registration**
1. Select language (Thai/English/Vietnamese/Indonesian)
2. Register with Email or Social Login (Google/Facebook)
3. Select server (e.g., TH-01, SEA-01)
4. Set player name

**Stage 3: Tribe Selection**
1. System presents 3 main tribes:
   - **Mainland Tribe (Phasuttha):** Balanced, War Elephants, Thick walls
   - **Maritime Tribe (Nava):** Raiding, Fast ships, Good trading
   - **Highland Tribe (Kiri):** Defense, Traps, Mountains
2. View Animation introducing each tribe's strengths
3. Select tribe matching playstyle
4. System randomly assigns map position

**Stage 4: Tutorial**
1. Teach building first structure (Rice field)
2. Teach 4 resource types (Wood/Clay/Iron/Crop)
3. Teach training first soldier
4. Introduce Beginner's Protection (7 days)
5. Give easy quests with rewards

**Stage 5: Early Game**
1. Build basic buildings completely
2. Join an Alliance
3. Start exploring surrounding map
4. Complete daily quests

---

### Journey 2: Combat & Raiding

**Stage 1: Scout**
1. Select target from map
2. Send scout unit (Water Monitor/Swamp Dragon)
3. Wait for report (view enemy resources/troops)

**Stage 2: Prepare**
1. Select army type (Raid/Destroy/Conquer)
2. Select number of each troop type
3. View travel time and arrival time
4. Confirm sending army

**Stage 3: Battle**
1. System auto-calculates battle results
2. Notify player when army arrives
3. Display Battle Report:
   - Troops lost/survived
   - Resources raided
   - Buildings destroyed

**Stage 4: Return**
1. Army travels back with resources
2. Resources auto-deposit to granary
3. Wounded troops go to hospital

---

### Journey 3: Alliance Warfare

**Stage 1: Communication**
1. Alliance leader announces target
2. Members discuss in Alliance Chat
3. Plan synchronized attack time

**Stage 2: Coordination**
1. Send armies to arrive simultaneously
2. Assign roles (front attack, rear attack)
3. Send Support to allies

**Stage 3: Siege**
1. Destroy walls with War Elephants
2. Eliminate defending troops
3. Conquer enemy village

---

### Journey 4: Purchasing

**Stage 1: Browse**
1. Open in-game store
2. View items: Gold, VIP, Skins
3. View prices and promotions

**Stage 2: Purchase**
1. Select desired item
2. Select payment method (Credit Card/PromptPay/True Wallet)
3. Confirm purchase
4. Receive Gold/Item immediately

**Stage 3: Redeem**
1. Use Gold to speed up construction
2. Buy VIP for special privileges
3. Buy Skin to decorate village/troops

---

## 3. Core Features

### 3.1 Village Management

**Resource Production:**
- 4 main resources: Wood, Clay, Iron, Crop
- 6-9 resource fields per village
- Upgrade levels to increase production

**Buildings:**
- Resource buildings: Rice field, Mine, etc.
- Military buildings: Barracks, Stable, Elephant Training Ground
- Defense buildings: Wall, Tower
- Special buildings: Market, Embassy, Palace

**Construction Queue:**
- Regular player: Build 1 building at a time
- VIP: Build 2-3 buildings at a time

---

### 3.2 Military System

**Unit Types per Tribe:**

**Mainland Tribe (Phasuttha):**
| Unit | Role | Strength |
|------|------|----------|
| Infantry | Infantry | Balanced attack/defense |
| Spearman | Spearman | Anti-cavalry |
| War Elephant | Heavy Cavalry | Highest HP/attack, slow |
| Buffalo Wagon | Transport | Highest carry capacity |

**Maritime Tribe (Nava):**
| Unit | Role | Strength |
|------|------|----------|
| Kris Warrior | Raider | Cheap, fast, good raider |
| Sea Diver | Scout | Water scouting |
| War Prahu | Naval | Can cross water |
| Merchant Ship | Trade | Fast, long distance |

**Highland Tribe (Kiri):**
| Unit | Role | Strength |
|------|------|----------|
| Crossbowman | Ranged Defense | Strong shot, good defense |
| Mountain Warrior | Infantry | Fast on mountains |
| Highland Pony | Light Cavalry | Fast on mountains |
| Trap Maker | Special | Creates traps to catch enemies |

**Special Units (All tribes can access):**
| Unit | How to Get | Strength |
|------|------------|----------|
| Swamp Dragon (Water Monitor) | Granary Lv.10 | Scout, stealth, fast swimming |
| Locust Swarm | Shaman | Destroys enemy rice fields |
| Battle Ducks | Duck Farm | Counters locusts |
| Portuguese Musketeer | Tavern | Highest attack, but fragile |

---

### 3.3 Hero System

**Overview:**
- 1 หมู่บ้าน สามารถมี 1 Hero ประจำการ
- Hero ผูกกับ Tribe (เลือกได้เฉพาะ Hero ของ Tribe ตัวเอง)
- Hero ตายได้ แต่รักษาฟื้นคืนได้

---

#### Heroes by Tribe

**Mainland Tribe (Phasuttha) - ไทย/สยาม:**
| Hero | Rarity | Passive Skill | Active Skill |
|------|--------|---------------|--------------|
| พ่อขุนรามคำแหง | ⭐⭐⭐⭐⭐ | +20% Resource production | Diplomacy: ลดเวลา NAP 50% |
| สมเด็จพระนเรศวร | ⭐⭐⭐⭐⭐ | +30% Elephant attack | Royal Duel: ท้าดวล Hero ศัตรู |
| พระยาตากสิน | ⭐⭐⭐⭐ | +25% Army speed | Night Raid: โจมตีกลางคืน +50% damage |
| สมเด็จพระสุริโยทัย | ⭐⭐⭐⭐ | +20% Defense | Inspire: เพิ่ม morale ทหารใกล้เคียง |
| ท้าวเทพกระษัตรี | ⭐⭐⭐ | +15% Wall defense | Rally: ระดมชาวบ้านช่วยรบ |
| พระยาพิชัยดาบหัก | ⭐⭐⭐ | +20% Infantry attack | Last Stand: ไม่ตายจนกว่าทหารหมด |
| ขุนแผน | ⭐⭐ | +10% Scout range | Stealth: ซ่อนกองทัพ |

**Maritime Tribe (Nava) - อินโดนีเซีย/มาเลเซีย/ฟิลิปปินส์:**
| Hero | Rarity | Passive Skill | Active Skill |
|------|--------|---------------|--------------|
| Gajah Mada | ⭐⭐⭐⭐⭐ | +30% Conquest speed | Palapa Oath: ยึดหมู่บ้าน loyalty -20 |
| Hang Tuah | ⭐⭐⭐⭐⭐ | +25% Naval attack | Legendary Strike: Critical hit 2x |
| Lapu-Lapu | ⭐⭐⭐⭐ | +35% vs Colonial NPC | First Blood: โบนัสโจมตีแรก |
| Sultan Agung | ⭐⭐⭐⭐ | +25% Siege damage | Resistance: +50% vs NPC invaders |
| Parameswara | ⭐⭐⭐ | +20% Trade income | Trade Route: เปิดเส้นทางค้าขาย |
| Rajah Sulayman | ⭐⭐⭐ | +15% Port defense | Maritime Network: ข้ามน้ำเร็วขึ้น |
| Kris Warrior | ⭐⭐ | +10% Raid capacity | Swift Raid: ปล้นเร็วขึ้น |

**Highland Tribe (Kiri) - เวียดนาม/เขมร/พม่า:**
| Hero | Rarity | Passive Skill | Active Skill |
|------|--------|---------------|--------------|
| Trần Hưng Đạo | ⭐⭐⭐⭐⭐ | +35% Anti-invasion | Scorched Earth: ทำลายทรัพยากรก่อนถูกปล้น |
| พระเจ้าชัยวรมันที่ 7 | ⭐⭐⭐⭐⭐ | +25% Building speed | Wonder: สร้างอาคารพิเศษ |
| บุเรงนอง | ⭐⭐⭐⭐ | +30% Elephant damage | Conqueror: ลด loyalty เป้าหมาย |
| Lê Lợi | ⭐⭐⭐⭐ | +30% Guerrilla | Ambush: ซุ่มโจมตี +40% damage |
| Hai Bà Trưng | ⭐⭐⭐ | +25% Rebellion bonus | Sister Bond: Hero slot +1 |
| มหาบันดุละ | ⭐⭐⭐ | +20% Infantry defense | Tactical Retreat: ถอยทัพไม่เสียทหาร |
| Crossbow Master | ⭐⭐ | +15% Ranged attack | Volley: ยิงถล่ม |

---

#### Hero Mechanics

**Recruitment:**
| วิธี | รายละเอียด | ค่าใช้จ่าย |
|------|------------|-----------|
| Starting Hero | เลือก 1 ตัว (⭐⭐) ตอนเริ่มเกม | ฟรี |
| Tavern Recruit | สุ่มจาก Pool ของ Tribe | 100-500 Gold |
| Quest Reward | ทำ Quest พิเศษ | เวลา + Resources |
| Season Reward | จบ Season ติด Top | ฟรี |

**Hero Stats:**
```
┌─────────────────────────────────────┐
│  ❤️ HP        - ความทนทาน            │
│  ⚔️ Attack    - พลังโจมตี            │
│  🛡️ Defense   - พลังป้องกัน          │
│  🏃 Speed     - ความเร็วเดินทาง       │
│  ⭐ Level     - เพิ่มจาก EXP การรบ    │
└─────────────────────────────────────┘
```

**Level Up:**
- Hero ได้ EXP จากการรบ (โจมตี/ป้องกัน)
- Level สูงสุด: 100
- ทุก 10 Level ปลดล็อค Skill ใหม่

**Death & Revival:**
| สถานะ | เงื่อนไข | ผลกระทบ |
|-------|---------|---------|
| 🟢 Active | HP > 0 | ใช้งานได้ปกติ |
| 🟡 Wounded | HP = 0 แต่ยังไม่ตาย | ต้องรักษา, ใช้งานไม่ได้ |
| 🔴 Dead | ถูกโจมตีขณะ Wounded | ต้อง Revive |

**Healing & Revival:**
| การรักษา | เวลา | ค่าใช้จ่าย |
|----------|------|-----------|
| Natural Heal | 1 HP / 10 min | ฟรี |
| Hospital | 10 HP / 10 min | Resources |
| Instant Heal | ทันที | 50 Gold |
| Revive (Dead) | 24 ชั่วโมง | 200 Gold หรือ Resources มาก |

**Hero Assignment:**
- 1 หมู่บ้าน = 1 Hero ประจำการ
- Hero ไม่ประจำการ = พักใน Hero Hall
- ย้าย Hero ระหว่างหมู่บ้านได้ (ใช้เวลาเดินทาง)
- Hero ต้องประจำการจึงจะใช้ Skill ได้

---

### 3.4 Map System

**Map Structure:**
- Grid-based Map size 200x200 tiles
- Shape similar to SEA map
- Divided zones by geography

**Terrain Types:**
| Terrain | Effect |
|---------|--------|
| Grassland | Normal movement |
| Forest | 20% slower movement |
| Mountain | Kiri tribe normal, others 40% slower |
| Water | Need harbor to cross (except Nava tribe) |
| Swamp | Water Monitor moves at normal speed |

**Zones:**
- Mainland Zone: Large plains (Thailand, Myanmar, Laos, Cambodia)
- Peninsula Zone: Peninsula (Vietnam, Malaysia)
- Archipelago Zone: Islands (Indonesia, Philippines)
- Highland Zone: Mountain ranges (Northern regions)

---

### 3.5 Alliance System

**Features:**
- Create/Join alliance
- Alliance Chat (Group chat)
- Shared Defense (Send troops to help allies)
- Alliance Bank (Shared treasury)
- War Declaration
- NAP (Non-Aggression Pact)

**Roles:**
- Leader: Supreme leader
- Co-Leader: Vice leader
- Officer: Manage members
- Diplomat: Negotiate with other alliances
- Member: Regular member

---

### 3.6 Real-Time Features

**Notifications:**
- Alert when attacked
- Alert when construction complete
- Alert when troops trained
- Alert for new messages

**Live Updates:**
- Resources increase Real-time on screen
- Countdown timer for construction/training
- Map updates when changes occur

---

### 3.7 Monetization Features

**Premium Currency (Gold):**
- Purchase with real money
- Use to speed up construction/training
- Exchange resources via NPC merchant

**VIP Subscription (Tusk Plus):**
- Price: ~199 THB/month
- Special privileges:
  - Construction queue increased to 3 slots
  - View larger map area
  - Auto-Evade (automatic escape)
  - Advanced statistics

**Cosmetic Skins:**
- Village skins (Myanmar theme, Khmer theme, Lanna theme)
- Troop skins (Decorated elephant, Albino buffalo)
- Profile frames

**Battle Pass (Season Pass):**
- Free Track: Basic rewards
- Premium Track: Special rewards (Gold, Rare Skin)
- Duration: 3-6 months per Season

---

### 3.8 Merchandise Integration

**Art Toys:**
- The Buffalo Collection (Buffalo blind box)
- Swamp Dragon Series (Water Monitor set)
- War Elephant Limited Edition

**Apparel:**
- Thai art style t-shirts
- Streetwear x Thai Art shirts

**Digital Goods:**
- LINE Sticker Pack
- Physical Sticker (Die-cut)

**Phygital (Physical + Digital):**
- Buy Art Toy, get QR Code
- Scan to receive Exclusive Skin in game

---

## 4. Success Metrics

### 4.1 User Acquisition Metrics
| Metric | Target (Month 1) | Target (Month 6) |
|--------|------------------|------------------|
| Total Registrations | 10,000 | 100,000 |
| Daily Active Users (DAU) | 1,000 | 15,000 |
| Monthly Active Users (MAU) | 5,000 | 50,000 |
| DAU/MAU Ratio | 20% | 30% |

### 4.2 Engagement Metrics
| Metric | Target |
|--------|--------|
| Average Session Duration | 15+ minutes |
| Sessions per Day | 3+ times |
| Day 1 Retention | 40% |
| Day 7 Retention | 25% |
| Day 30 Retention | 15% |

### 4.3 Revenue Metrics
| Metric | Target |
|--------|--------|
| Conversion Rate (Free to Paid) | 3-5% |
| Average Revenue Per Paying User (ARPPU) | 500 THB/month |
| VIP Subscription Rate | 10% of Active Users |
| Monthly Recurring Revenue (MRR) | Growth 20% MoM |

### 4.4 Community Metrics
| Metric | Target |
|--------|--------|
| Average Alliance Size | 20+ members |
| Alliance Participation Rate | 70% of active players |
| Chat Messages per Day | 1,000+ |
| User-Generated Content (Social Shares) | 100+ posts/week |

### 4.5 Technical Metrics
| Metric | Target |
|--------|--------|
| Server Uptime | 99.5% |
| API Response Time | < 200ms |
| Page Load Time | < 3 seconds |
| Concurrent Users Capacity | 10,000+ |

---

## 5. Edge Cases

### 5.1 Gameplay Edge Cases

**Case: Player attacked while Offline**
- **Problem:** Player sleeps 8 hours, gets raided completely
- **Solution:**
  - Resource hiding pit (Cranny) stores some resources
  - VIP has Auto-Evade system to auto-move troops
  - Beginner's Protection 7 days for new players

**Case: Army stuck mid-travel (Server Crash)**
- **Problem:** Server crashes, army disappears
- **Solution:**
  - Save army state in Database every Tick
  - When Server returns, army continues from stop point
  - Auto-notify Admin when Anomaly detected

**Case: Multi-Accounting**
- **Problem:** One person creates multiple Accounts to Feed themselves
- **Solution:**
  - Check IP Address and Device Fingerprint
  - Limit resource sending between new Accounts
  - Report system and Admin Review

**Case: Alliance too large, dominates server**
- **Problem:** One Alliance wins everything, others quit
- **Solution:**
  - Limit Alliance member count (e.g., 50 people)
  - World Boss system (NPC Invasion) forces cooperation
  - Season Reset every 6 months, restart

---

### 5.2 Technical Edge Cases

**Case: 2 players send troops to attack simultaneously**
- **Problem:** Race Condition - who arrives first?
- **Solution:**
  - Use accurate Timestamp (Milliseconds)
  - Queue System for Battle Calculation
  - Database Transaction Lock

**Case: Resources go negative**
- **Problem:** Troops eat crops until negative -> troops die
- **Solution:**
  - Starvation system: Troops die gradually
  - Alert before crops run out
  - Don't allow troop creation if crop production insufficient

**Case: Connection Lost while sending army**
- **Problem:** Player clicks send, but Internet disconnects
- **Solution:**
  - Backend receives command and responds Confirm immediately
  - If no Confirm -> not actually sent
  - Retry Mechanism on Client side

---

### 5.3 Payment Edge Cases

**Case: Paid but didn't receive Gold**
- **Problem:** Payment Gateway sends data slowly
- **Solution:**
  - Webhook from Payment Gateway
  - Retry Queue for Item delivery
  - Support Ticket system + Transaction Log
  - Clear Refund Policy

**Case: Chargeback after receiving items**
- **Problem:** Player requests refund from Bank after using all Gold
- **Solution:**
  - Record all Transactions
  - Deduct Gold back (if negative = Ban Account)
  - Fraud Detection System

---

### 5.4 Community Edge Cases

**Case: Toxic Chat / Hate Speech**
- **Problem:** Players cursing each other, profanity
- **Solution:**
  - Automatic Word Filter
  - Report + Mute/Ban system
  - AI Content Moderation (if possible)

**Case: Real Money Trading (RMT)**
- **Problem:** Players sell resources/Account for real money
- **Solution:**
  - Clearly state in Terms of Service it's forbidden
  - Check abnormal Trade Patterns
  - Ban violating Accounts

---

## 6. Localization (Multi-language Support)

### Supported Languages:
| Language | Priority | Status |
|----------|----------|--------|
| Thai (TH) | P0 | Primary |
| English (EN) | P0 | Primary |
| Vietnamese (VN) | P1 | Phase 2 |
| Indonesian (ID) | P1 | Phase 2 |
| Burmese (MM) | P2 | Future |

### Content to Localize:
- UI Text (Buttons, menus, descriptions)
- Unit Names & Descriptions
- Building Names & Descriptions
- Tutorial & Help Text
- Push Notifications
- Email Templates
- Terms of Service & Privacy Policy

---

## 7. Season & End Game System

### 7.1 Season Overview

เกมดำเนินเป็น Season ละ **90 วัน** แบ่งเป็น 3 ยุค:

| ยุค | ช่วงเวลา | ชื่อ | เหตุการณ์หลัก |
|-----|---------|------|---------------|
| Era 1 | Day 1-30 | ยุคสร้างอาณาจักร (Kingdom Building) | สร้างหมู่บ้าน, รวม Alliance |
| Era 2 | Day 30-60 | ยุคมหันตภัย (Calamity) | ภัยธรรมชาติสุ่มเกิด |
| Era 3 | Day 60-90 | ยุคล่าอาณานิคม (Colonial Invasion) | ชาติตะวันตก NPC บุก |

---

### 7.2 Era 1: Kingdom Building (Day 1-30)

**Timeline:**
| วัน | Event |
|-----|-------|
| Day 1 | Server เปิด, ผู้เล่นเริ่มต้นใหม่ทั้งหมด |
| Day 1-7 | Beginner's Protection (ถูกโจมตีไม่ได้) |
| Day 7 | Protection หมด, เริ่มสงครามได้ |
| Day 25 | ⚠️ แจ้งเตือน: ภัยธรรมชาติใกล้มาใน 5 วัน |

**เป้าหมายผู้เล่น:**
- สร้างหมู่บ้าน + Resource fields
- ฝึกกองทัพ
- หา Alliance เข้าร่วม
- เตรียมป้องกันภัยธรรมชาติ

---

### 7.3 Era 2: Calamity (Day 30-60)

**ภัยธรรมชาติ (สุ่มเกิด 2-3 ครั้งในช่วงนี้):**

#### 🌪️ พายุ (Storm)
| รายละเอียด | ค่า |
|------------|-----|
| รัศมีผลกระทบ | 10-20 tiles จากจุดศูนย์กลาง |
| แจ้งเตือนล่วงหน้า | 24 ชั่วโมง |
| ผลกระทบ | อาคาร -1 level, กองทัพเดินทางช้าลง 50% |
| ป้องกัน | สร้าง Shelter (อาคารพิเศษ) |

#### 🌊 น้ำท่วม (Flood)
| รายละเอียด | ค่า |
|------------|-----|
| พื้นที่ผลกระทบ | ตามแนวแม่น้ำ + พื้นที่ราบต่ำ |
| แจ้งเตือนล่วงหน้า | 48 ชั่วโมง |
| ผลกระทบ | ไร่นาผลิต -50%, บาง tiles จมชั่วคราว |
| ป้องกัน | สร้าง Levee (เขื่อน), หมู่บ้านบนที่สูงปลอดภัย |

#### 🏔️ แผ่นดินไหว (Earthquake)
| รายละเอียด | ค่า |
|------------|-----|
| รัศมีผลกระทบ | 15-30 tiles จากจุดศูนย์กลาง |
| แจ้งเตือนล่วงหน้า | ไม่มี (เกิดทันที) |
| ผลกระทบ | กำแพง -2 level, Terrain เปลี่ยน (เปิด/ปิดเส้นทาง) |
| ป้องกัน | หมู่บ้านเผ่า Kiri (บนเขา) ได้รับผลกระทบน้อยกว่า |

**Alliance Cooperation:**
- ส่งทรัพยากรช่วยสมาชิกที่โดนภัย
- ร่วมกันซ่อมแซมฟื้นฟู
- Alliance ที่แข็งแกร่งจะรอดได้ดีกว่า

---

### 7.4 Era 3: Colonial Invasion (Day 60-90)

**ชาติตะวันตกที่บุกรุก (NPC):**

| ชาติ | ความแข็งแกร่ง | ช่วงเวลา | ลักษณะการรุก |
|------|--------------|---------|--------------|
| 🇵🇹 Portugal | ⭐⭐ | Day 60+ | โจมตีชายฝั่ง, ปล้นทรัพยากร |
| 🇪🇸 Spain | ⭐⭐ | Day 60+ | โจมตีหมู่เกาะ (Archipelago Zone) |
| 🇳🇱 Netherlands | ⭐⭐⭐ | Day 70+ | ยึดท่าเรือ, ตัด Trade Routes |
| 🇬🇧 Britain | ⭐⭐⭐⭐⭐ | Day 80+ | กองทัพใหญ่บุกยึดเมืองหลวง |
| 🇫🇷 France | ⭐⭐⭐⭐⭐ | Day 80+ | บุกจากทางบก, ปืนใหญ่ทำลายกำแพง |

**Invasion Timeline:**
| วัน | Event |
|-----|-------|
| Day 60 | 🚢 กองเรือ Portugal + Spain ปรากฏที่ชายฝั่ง |
| Day 60-70 | โจมตีหมู่บ้านชายฝั่งเป็นระยะ |
| Day 70 | 🚢 กองเรือ Netherlands มาถึง |
| Day 70-80 | ยึดครอง Trade Posts, ปิดล้อมท่าเรือ |
| Day 80 | ⚔️ Britain + France เปิดฉาก Main Invasion |
| Day 80-85 | บุกเข้าแผ่นดินใหญ่ |
| Day 85-90 | 🏰 Final Battle - โจมตี Capital Cities |
| Day 90 | 🏆 Season End - คำนวณผลและประกาศผู้ชนะ |

**NPC Army Strength:**
- กองทัพ NPC แข็งแกร่งขึ้นตามวัน
- ต้องรวมพลัง Alliance หลายกลุ่มจึงจะต้านได้
- สามารถ "ยอมจำนน" เพื่อรอดแต่เสียคะแนน

---

### 7.5 Victory Conditions & Rewards

**การคำนวณคะแนน:**
| หมวด | คะแนน |
|------|-------|
| หมู่บ้านที่รอด | 100 คะแนน/หมู่บ้าน |
| Capital ที่รอด | 500 คะแนน |
| NPC ที่ฆ่าได้ | 10 คะแนน/หน่วย |
| ทรัพยากรสะสม | 1 คะแนน/1000 หน่วย |
| สมาชิก Alliance ที่รอด | 50 คะแนน/คน |

**อันดับและรางวัล:**
| อันดับ | เงื่อนไข | รางวัล |
|--------|---------|--------|
| 🥇 **Champion Alliance** | Capital รอด + คะแนนสูงสุด | Exclusive Season Skin + 1000 Gold + Title "ผู้พิทักษ์แผ่นดิน" |
| 🥈 **Runner-up Alliance** | Capital รอด + คะแนนอันดับ 2-3 | Rare Skin + 500 Gold |
| 🥉 **Survivor Alliance** | มีสมาชิกรอด 50%+ | Uncommon Skin + 200 Gold |
| ⚔️ **War Hero** | ผู้เล่นที่ฆ่า NPC มากสุด (Top 10) | Special Badge + 300 Gold |
| 🛡️ **Defender** | ป้องกันหมู่บ้านสำเร็จ 10+ ครั้ง | Special Badge + 100 Gold |
| 💀 **Fallen** | ถูกยึดครองทั้งหมด | Participation Reward: 50 Gold |

**Season Rewards Carry-over:**
- Gold และ Skin เก็บไว้ใน Account ถาวร
- Title แสดงใน Profile ตลอดไป
- Badge สะสมได้ทุก Season

---

### 7.6 Between Seasons

**หลัง Season จบ:**
1. Server ปิด 3-7 วันเพื่อ Reset
2. ผู้เล่นรับรางวัลเข้า Account
3. เปิด Season ใหม่ - ทุกคนเริ่มใหม่จาก 0
4. อาจเปิด Server ใหม่หลายตัวพร้อมกัน

**สิ่งที่ Reset:**
- หมู่บ้าน, ทรัพยากร, กองทัพ
- Alliance
- แผนที่

**สิ่งที่คงอยู่:**
- Account (User)
- Gold ที่ซื้อ/ได้รับ
- Skins, Titles, Badges
- สถิติรวม (Total games played, Total NPC killed, etc.)

---

## 8. Future Features (Roadmap)

### Phase 1: MVP (Minimum Viable Product)
- 3 main tribes
- Basic village system
- Combat system (Raid/Attack)
- Alliance system
- Gold/VIP system

### Phase 2: Enhancement
- Add tribes 4-5 (Fire Dragon, Forest Mystics)
- Hero/General system
- World Boss (NPC Invasion)
- Tournament System
- AI Diplomacy (Gemini Integration)

### Phase 3: Expansion
- Art Toys & Merchandise
- Mobile App (Native iOS/Android)
- Spin-off Games
- Anime/Manga Adaptation (if popular)

---

## 9. Appendix

### 9.1 Competitive Analysis

| Feature | Travian | Tribal Wars | Tusk & Horn |
|---------|---------|-------------|-------------|
| Theme | Roman/Germanic | Medieval European | SEA Ancient |
| Art Style | Realistic | Cartoon | Thai Mural Toon |
| Mobile Responsive | Partial | Good | Excellent (Mobile-First) |
| Free-to-Play Friendly | Medium | Low | High |
| Unique Selling Point | Classic | Simple | Cultural + Art |

### 9.2 Reference Links
- Travian: https://www.travian.com
- Tribal Wars: https://www.tribalwars.net
- Google Gemini API: https://ai.google.dev

---

*Document Version: 1.0*
*Last Updated: December 2025*
*Author: Product Team*
