import { writable } from "svelte/store";
import { toast } from "svelte-sonner";
import { api } from "../api/client";

export interface MapVillageInfo {
    id: string;
    name: string;
    player_name: string | null;
    population: number;
    is_own: boolean;
}

export interface MapTile {
    x: number;
    y: number;
    village: MapVillageInfo | null;
}

export interface MapSearchResult {
    result_type: 'player' | 'village' | 'alliance';
    id: string;
    name: string;
    x: number | null;
    y: number | null;
    population: number | null;
    player_name: string | null;
    alliance_tag: string | null;
    member_count: number | null;
}

interface MapState {
    tiles: MapTile[];
    centerX: number;
    centerY: number;
    range: number;
    loading: boolean;
    error: string | null;
    searchResults: MapSearchResult[];
    searchLoading: boolean;
}

function createMapStore() {
    const { subscribe, set, update } = writable<MapState>({
        tiles: [],
        centerX: 0,
        centerY: 0,
        range: 7,
        loading: false,
        error: null,
        searchResults: [],
        searchLoading: false,
    });

    return {
        subscribe,

        // Load map tiles from API
        loadTiles: async (x: number, y: number, range: number = 7) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const tiles = await api.get<MapTile[]>(`/api/map?x=${x}&y=${y}&range=${range}`);

                update(state => ({
                    ...state,
                    tiles,
                    centerX: x,
                    centerY: y,
                    range,
                    loading: false,
                }));

                return tiles;
            } catch (error: any) {
                const message = error.message || 'Failed to load map';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Map Load Failed', { description: message });
                throw error;
            }
        },

        // Search map for players, villages, alliances
        search: async (query: string) => {
            if (!query.trim()) {
                update(state => ({ ...state, searchResults: [], searchLoading: false }));
                return [];
            }

            update(state => ({ ...state, searchLoading: true }));
            try {
                const results = await api.get<MapSearchResult[]>(`/api/map/search?q=${encodeURIComponent(query)}&limit=20`);

                update(state => ({
                    ...state,
                    searchResults: results,
                    searchLoading: false,
                }));

                return results;
            } catch (error: any) {
                const message = error.message || 'Search failed';
                update(state => ({
                    ...state,
                    searchLoading: false,
                }));
                toast.error('Search Failed', { description: message });
                return [];
            }
        },

        // Clear search results
        clearSearch: () => {
            update(state => ({ ...state, searchResults: [], searchLoading: false }));
        },

        // Set center coordinates
        setCenter: (x: number, y: number) => {
            update(state => ({ ...state, centerX: x, centerY: y }));
        },

        // Clear error
        clearError: () => {
            update(state => ({ ...state, error: null }));
        },

        // Reset store
        reset: () => {
            set({
                tiles: [],
                centerX: 0,
                centerY: 0,
                range: 7,
                loading: false,
                error: null,
                searchResults: [],
                searchLoading: false,
            });
        },
    };
}

export const mapStore = createMapStore();

// Helper function to get village at coordinates from tiles array
export function getVillageAt(tiles: MapTile[], x: number, y: number): MapVillageInfo | null {
    const tile = tiles.find(t => t.x === x && t.y === y);
    return tile?.village || null;
}

// Helper to determine owner type from API response
export function getOwnerType(village: MapVillageInfo | null): 'self' | 'ally' | 'enemy' | 'neutral' | 'npc' | null {
    if (!village) return null;

    if (village.is_own) return 'self';

    // For now, treat all non-own villages as neutral
    // In the future, this would check alliance status
    if (village.player_name === 'Natarian') return 'npc';

    return 'neutral';
}
