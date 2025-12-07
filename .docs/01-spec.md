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

### 3.3 Map System

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

### 3.4 Alliance System

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

### 3.5 Real-Time Features

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

### 3.6 Monetization Features

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

### 3.7 Merchandise Integration

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

## 7. Future Features (Roadmap)

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

## 8. Appendix

### 8.1 Competitive Analysis

| Feature | Travian | Tribal Wars | Tusk & Horn |
|---------|---------|-------------|-------------|
| Theme | Roman/Germanic | Medieval European | SEA Ancient |
| Art Style | Realistic | Cartoon | Thai Mural Toon |
| Mobile Responsive | Partial | Good | Excellent (Mobile-First) |
| Free-to-Play Friendly | Medium | Low | High |
| Unique Selling Point | Classic | Simple | Cultural + Art |

### 8.2 Reference Links
- Travian: https://www.travian.com
- Tribal Wars: https://www.tribalwars.net
- Google Gemini API: https://ai.google.dev

---

*Document Version: 1.0*
*Last Updated: December 2025*
*Author: Product Team*
