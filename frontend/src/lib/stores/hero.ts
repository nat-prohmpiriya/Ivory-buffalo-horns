import { writable } from "svelte/store";
import { toast } from "svelte-sonner";
import { api } from "../api/client";

// Enums
export type HeroStatus = 'idle' | 'moving' | 'in_adventure' | 'in_battle' | 'dead' | 'reviving';
export type ItemRarity = 'common' | 'uncommon' | 'rare' | 'epic' | 'legendary';
export type ItemSlot = 'helmet' | 'weapon' | 'armor_left' | 'armor_right' | 'boots' | 'horse' | 'bag' | 'bandage' | 'consumable';
export type AdventureDifficulty = 'short' | 'long';
export type TribeType = 'romans' | 'gauls' | 'teutons' | 'egyptians' | 'huns';

// Types
export interface PassiveBonus {
    bonus_type: string;
    value: number;
    description: string;
}

export interface HeroDefinitionResponse {
    id: string;
    name: string;
    name_en: string;
    tribe: TribeType;
    rarity: number;
    rarity_stars: string;
    description: string | null;
    description_en: string | null;
    passive_bonuses: PassiveBonus[];
    portrait_url: string | null;
}

export interface HeroResponse {
    id: string;
    slot_number: number;
    name: string;
    tribe: TribeType;
    hero_definition: HeroDefinitionResponse | null;
    home_village_id: string;
    current_village_id: string | null;
    status: HeroStatus;
    level: number;
    experience: number;
    experience_to_next: number;
    health: number;
    health_regen_rate: number;
    unassigned_points: number;
    fighting_strength: number;
    off_bonus: number;
    def_bonus: number;
    resources_bonus: number;
    total_attack: number;
    total_defense: number;
    off_bonus_percent: number;
    def_bonus_percent: number;
    base_speed: number;
    active_bonuses: PassiveBonus[];
    died_at: string | null;
    revive_at: string | null;
}

export interface HeroListResponse {
    heroes: HeroResponse[];
    total_slots: number;
    used_slots: number;
    next_slot_cost: number | null;
}

export interface ItemDefinitionResponse {
    id: string;
    name: string;
    description: string | null;
    slot: ItemSlot;
    rarity: ItemRarity;
    required_level: number;
    attack_bonus: number;
    defense_bonus: number;
    speed_bonus: number;
    health_regen_bonus: number;
    experience_bonus: number;
    resource_bonus: number;
    carry_bonus: number;
    health_restore: number;
    is_consumable: boolean;
    extra_inventory_slots: number;
    sell_value: number;
}

export interface HeroItemResponse {
    id: string;
    item: ItemDefinitionResponse;
    is_equipped: boolean;
    equipped_slot: ItemSlot | null;
    quantity: number;
    obtained_at: string;
}

export interface EquippedItemsResponse {
    helmet: HeroItemResponse | null;
    weapon: HeroItemResponse | null;
    armor_left: HeroItemResponse | null;
    armor_right: HeroItemResponse | null;
    boots: HeroItemResponse | null;
    horse: HeroItemResponse | null;
    bag: HeroItemResponse | null;
    bandage: HeroItemResponse | null;
}

export interface InventoryResponse {
    equipped: EquippedItemsResponse;
    items: HeroItemResponse[];
    total_slots: number;
    used_slots: number;
}

export interface AvailableAdventureResponse {
    id: string;
    difficulty: AdventureDifficulty;
    duration_range: string;
    potential_reward: string | null;
    potential_rarity: ItemRarity | null;
    expires_at: string;
}

export interface HeroAdventureResponse {
    id: string;
    hero_id: string;
    difficulty: AdventureDifficulty;
    started_at: string;
    ends_at: string;
    is_completed: boolean;
    reward_experience: number | null;
    reward_silver: number | null;
    reward_resources: Record<string, number> | null;
    reward_item: ItemDefinitionResponse | null;
    health_lost: number | null;
}

export interface ReviveInfoResponse {
    hero_id: string;
    revive_at: string;
    remaining_seconds: number;
    gold_cost_instant: number;
    resource_cost: {
        wood: number;
        clay: number;
        iron: number;
        crop: number;
    };
}

export interface HeroSlotPurchaseResponse {
    success: boolean;
    new_slot_number: number;
    gold_spent: number;
    new_balance: number;
    total_slots: number;
}

// Request types
export interface CreateHeroRequest {
    name?: string;
    hero_definition_id?: string;
    home_village_id: string;
}

export interface AssignAttributesRequest {
    fighting_strength: number;
    off_bonus: number;
    def_bonus: number;
    resources_bonus: number;
}

interface HeroState {
    heroes: HeroResponse[];
    selectedHero: HeroResponse | null;
    inventory: InventoryResponse | null;
    availableAdventures: AvailableAdventureResponse[];
    activeAdventure: HeroAdventureResponse | null;
    tavernHeroes: HeroDefinitionResponse[];
    totalSlots: number;
    usedSlots: number;
    nextSlotCost: number | null;
    loading: boolean;
    error: string | null;
}

function createHeroStore() {
    const { subscribe, set, update } = writable<HeroState>({
        heroes: [],
        selectedHero: null,
        inventory: null,
        availableAdventures: [],
        activeAdventure: null,
        tavernHeroes: [],
        totalSlots: 1,
        usedSlots: 0,
        nextSlotCost: null,
        loading: false,
        error: null,
    });

    return {
        subscribe,

        // Load all heroes
        loadHeroes: async () => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const response = await api.get<HeroListResponse>('/api/heroes');
                update(state => ({
                    ...state,
                    heroes: response.heroes,
                    totalSlots: response.total_slots,
                    usedSlots: response.used_slots,
                    nextSlotCost: response.next_slot_cost,
                    loading: false,
                }));
                return response;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                return null;
            }
        },

        // Load single hero
        loadHero: async (heroId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const hero = await api.get<HeroResponse>(`/api/heroes/${heroId}`);
                update(state => ({
                    ...state,
                    selectedHero: hero,
                    loading: false,
                }));
                return hero;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                throw error;
            }
        },

        // Create new hero
        createHero: async (request: CreateHeroRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const hero = await api.post<HeroResponse>('/api/heroes', request);
                update(state => ({
                    ...state,
                    heroes: [...state.heroes, hero],
                    selectedHero: hero,
                    usedSlots: state.usedSlots + 1,
                    loading: false,
                }));
                toast.success('Hero Created', { description: `${hero.name} is ready for battle!` });
                return hero;
            } catch (error: any) {
                const message = error.message || 'Failed to create hero';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Assign attributes
        assignAttributes: async (heroId: string, request: AssignAttributesRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const hero = await api.put<HeroResponse>(
                    `/api/heroes/${heroId}/attributes`,
                    request
                );
                update(state => ({
                    ...state,
                    selectedHero: hero,
                    heroes: state.heroes.map(h => h.id === heroId ? hero : h),
                    loading: false,
                }));
                toast.success('Attributes Assigned');
                return hero;
            } catch (error: any) {
                const message = error.message || 'Failed to assign attributes';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Change home village
        changeHomeVillage: async (heroId: string, villageId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const hero = await api.put<HeroResponse>(
                    `/api/heroes/${heroId}/home`,
                    { village_id: villageId }
                );
                update(state => ({
                    ...state,
                    selectedHero: hero,
                    heroes: state.heroes.map(h => h.id === heroId ? hero : h),
                    loading: false,
                }));
                toast.success('Home Village Changed');
                return hero;
            } catch (error: any) {
                const message = error.message || 'Failed to change home village';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Buy hero slot
        buyHeroSlot: async () => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const result = await api.post<HeroSlotPurchaseResponse>('/api/heroes/slots/buy', {});
                update(state => ({
                    ...state,
                    totalSlots: result.total_slots,
                    loading: false,
                }));
                toast.success('Hero Slot Purchased', {
                    description: `You now have ${result.total_slots} hero slots`
                });
                return result;
            } catch (error: any) {
                const message = error.message || 'Failed to buy hero slot';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Load inventory
        loadInventory: async (heroId: string) => {
            try {
                const inventory = await api.get<InventoryResponse>(`/api/heroes/${heroId}/inventory`);
                update(state => ({
                    ...state,
                    inventory,
                }));
                return inventory;
            } catch (error: any) {
                console.error('Failed to load inventory:', error);
                return null;
            }
        },

        // Equip item
        equipItem: async (heroId: string, itemId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                await api.post<HeroItemResponse>(
                    `/api/heroes/${heroId}/equip`,
                    { item_id: itemId }
                );
                // Reload inventory
                const inventory = await api.get<InventoryResponse>(`/api/heroes/${heroId}/inventory`);
                update(state => ({
                    ...state,
                    inventory,
                    loading: false,
                }));
                toast.success('Item Equipped');
            } catch (error: any) {
                const message = error.message || 'Failed to equip item';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Unequip item
        unequipItem: async (heroId: string, slot: ItemSlot) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                await api.post(`/api/heroes/${heroId}/unequip`, { slot });
                // Reload inventory
                const inventory = await api.get<InventoryResponse>(`/api/heroes/${heroId}/inventory`);
                update(state => ({
                    ...state,
                    inventory,
                    loading: false,
                }));
                toast.success('Item Unequipped');
            } catch (error: any) {
                const message = error.message || 'Failed to unequip item';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Use item
        useItem: async (heroId: string, itemId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const hero = await api.post<HeroResponse>(
                    `/api/heroes/${heroId}/use-item`,
                    { item_id: itemId }
                );
                // Reload inventory
                const inventory = await api.get<InventoryResponse>(`/api/heroes/${heroId}/inventory`);
                update(state => ({
                    ...state,
                    selectedHero: hero,
                    heroes: state.heroes.map(h => h.id === heroId ? hero : h),
                    inventory,
                    loading: false,
                }));
                toast.success('Item Used');
                return hero;
            } catch (error: any) {
                const message = error.message || 'Failed to use item';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Sell item
        sellItem: async (heroId: string, itemId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const result = await api.delete<{ success: boolean; silver_gained: number }>(
                    `/api/heroes/${heroId}/items/${itemId}`
                );
                // Reload inventory
                const inventory = await api.get<InventoryResponse>(`/api/heroes/${heroId}/inventory`);
                update(state => ({
                    ...state,
                    inventory,
                    loading: false,
                }));
                toast.success('Item Sold', { description: `Gained ${result.silver_gained} silver` });
                return result;
            } catch (error: any) {
                const message = error.message || 'Failed to sell item';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Load available adventures
        loadAvailableAdventures: async () => {
            try {
                const adventures = await api.get<AvailableAdventureResponse[]>(
                    '/api/heroes/adventures/available'
                );
                update(state => ({
                    ...state,
                    availableAdventures: adventures,
                }));
                return adventures;
            } catch (error: any) {
                console.error('Failed to load adventures:', error);
                return [];
            }
        },

        // Start adventure
        startAdventure: async (heroId: string, adventureId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const adventure = await api.post<HeroAdventureResponse>(
                    `/api/heroes/${heroId}/adventures`,
                    { adventure_id: adventureId }
                );
                // Reload hero
                const hero = await api.get<HeroResponse>(`/api/heroes/${heroId}`);
                update(state => ({
                    ...state,
                    activeAdventure: adventure,
                    selectedHero: hero,
                    heroes: state.heroes.map(h => h.id === heroId ? hero : h),
                    loading: false,
                }));
                toast.success('Adventure Started', {
                    description: `${adventure.difficulty} adventure in progress`
                });
                return adventure;
            } catch (error: any) {
                const message = error.message || 'Failed to start adventure';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Load active adventure
        loadActiveAdventure: async (heroId: string) => {
            try {
                const adventure = await api.get<HeroAdventureResponse | null>(
                    `/api/heroes/${heroId}/adventures/active`
                );
                update(state => ({
                    ...state,
                    activeAdventure: adventure,
                }));
                return adventure;
            } catch (error: any) {
                console.error('Failed to load active adventure:', error);
                return null;
            }
        },

        // Load tavern heroes
        loadTavernHeroes: async () => {
            try {
                const heroes = await api.get<HeroDefinitionResponse[]>('/api/heroes/tavern');
                update(state => ({
                    ...state,
                    tavernHeroes: heroes,
                }));
                return heroes;
            } catch (error: any) {
                console.error('Failed to load tavern heroes:', error);
                return [];
            }
        },

        // Get revive info
        getReviveInfo: async (heroId: string) => {
            try {
                const info = await api.get<ReviveInfoResponse>(`/api/heroes/${heroId}/revive-info`);
                return info;
            } catch (error: any) {
                console.error('Failed to get revive info:', error);
                return null;
            }
        },

        // Revive hero
        reviveHero: async (heroId: string, useGold: boolean) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const hero = await api.post<HeroResponse>(
                    `/api/heroes/${heroId}/revive`,
                    { use_gold: useGold }
                );
                update(state => ({
                    ...state,
                    selectedHero: hero,
                    heroes: state.heroes.map(h => h.id === heroId ? hero : h),
                    loading: false,
                }));
                toast.success('Hero Revived', { description: `${hero.name} is back!` });
                return hero;
            } catch (error: any) {
                const message = error.message || 'Failed to revive hero';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Select hero
        selectHero: (hero: HeroResponse | null) => {
            update(state => ({ ...state, selectedHero: hero }));
        },

        // Clear error
        clearError: () => {
            update(state => ({ ...state, error: null }));
        },

        // Reset store
        reset: () => {
            set({
                heroes: [],
                selectedHero: null,
                inventory: null,
                availableAdventures: [],
                activeAdventure: null,
                tavernHeroes: [],
                totalSlots: 1,
                usedSlots: 0,
                nextSlotCost: null,
                loading: false,
                error: null,
            });
        },
    };
}

export const heroStore = createHeroStore();

// Helper functions
export function getStatusLabel(status: HeroStatus): string {
    const labels: Record<HeroStatus, string> = {
        idle: 'Idle',
        moving: 'Moving',
        in_adventure: 'On Adventure',
        in_battle: 'In Battle',
        dead: 'Dead',
        reviving: 'Reviving',
    };
    return labels[status] || status;
}

export function getStatusIcon(status: HeroStatus): string {
    const icons: Record<HeroStatus, string> = {
        idle: '🟢',
        moving: '🚶',
        in_adventure: '⚔️',
        in_battle: '⚔️',
        dead: '💀',
        reviving: '⏳',
    };
    return icons[status] || '❓';
}

export function getStatusColor(status: HeroStatus): string {
    const colors: Record<HeroStatus, string> = {
        idle: 'text-green-600',
        moving: 'text-blue-600',
        in_adventure: 'text-yellow-600',
        in_battle: 'text-red-600',
        dead: 'text-gray-600',
        reviving: 'text-purple-600',
    };
    return colors[status] || 'text-gray-600';
}

export function getRarityColor(rarity: ItemRarity): string {
    const colors: Record<ItemRarity, string> = {
        common: 'text-gray-600 border-gray-300',
        uncommon: 'text-green-600 border-green-300',
        rare: 'text-blue-600 border-blue-300',
        epic: 'text-purple-600 border-purple-300',
        legendary: 'text-yellow-600 border-yellow-300',
    };
    return colors[rarity] || 'text-gray-600';
}

export function getRarityBg(rarity: ItemRarity): string {
    const colors: Record<ItemRarity, string> = {
        common: 'bg-gray-50',
        uncommon: 'bg-green-50',
        rare: 'bg-blue-50',
        epic: 'bg-purple-50',
        legendary: 'bg-yellow-50',
    };
    return colors[rarity] || 'bg-gray-50';
}

export function getSlotIcon(slot: ItemSlot): string {
    const icons: Record<ItemSlot, string> = {
        helmet: '🪖',
        weapon: '⚔️',
        armor_left: '🛡️',
        armor_right: '🛡️',
        boots: '👢',
        horse: '🐴',
        bag: '🎒',
        bandage: '🩹',
        consumable: '🧪',
    };
    return icons[slot] || '📦';
}

export function getSlotLabel(slot: ItemSlot): string {
    const labels: Record<ItemSlot, string> = {
        helmet: 'Helmet',
        weapon: 'Weapon',
        armor_left: 'Left Armor',
        armor_right: 'Right Armor',
        boots: 'Boots',
        horse: 'Horse',
        bag: 'Bag',
        bandage: 'Bandage',
        consumable: 'Consumable',
    };
    return labels[slot] || slot;
}

export function getTribeIcon(tribe: TribeType): string {
    const icons: Record<TribeType, string> = {
        romans: '🏛️',
        gauls: '🌲',
        teutons: '⚔️',
        egyptians: '🏺',
        huns: '🐎',
    };
    return icons[tribe] || '👤';
}

export function formatHealthBar(health: number): { percent: number; color: string } {
    const percent = Math.min(100, Math.max(0, health));
    let color = 'bg-green-500';
    if (percent <= 25) color = 'bg-red-500';
    else if (percent <= 50) color = 'bg-yellow-500';
    return { percent, color };
}

export function formatExpBar(exp: number, expToNext: number): { percent: number } {
    const percent = expToNext > 0 ? Math.min(100, (exp / expToNext) * 100) : 0;
    return { percent };
}
