import { writable } from "svelte/store";
import { api } from "../api/client";

// Types
export interface PlayerPopulationRanking {
    rank: number;
    user_id: string;
    display_name: string | null;
    alliance_tag: string | null;
    population: number;
    village_count: number;
}

export interface PlayerAttackRanking {
    rank: number;
    user_id: string;
    display_name: string | null;
    alliance_tag: string | null;
    attack_points: number;
    battles_won: number;
}

export interface PlayerDefenseRanking {
    rank: number;
    user_id: string;
    display_name: string | null;
    alliance_tag: string | null;
    defense_points: number;
    battles_defended: number;
}

export interface HeroRanking {
    rank: number;
    hero_id: string;
    hero_name: string;
    owner_id: string;
    owner_name: string | null;
    level: number;
    experience: number;
}

export interface AllianceRanking {
    rank: number;
    alliance_id: string;
    name: string;
    tag: string;
    member_count: number;
    total_population: number;
}

export interface RankingListResponse<T> {
    rankings: T[];
    total: number;
    page: number;
    per_page: number;
}

export type RankingType = 'population' | 'attackers' | 'defenders' | 'heroes' | 'alliances';

interface RankingState {
    populationRankings: PlayerPopulationRanking[];
    attackRankings: PlayerAttackRanking[];
    defenseRankings: PlayerDefenseRanking[];
    heroRankings: HeroRanking[];
    allianceRankings: AllianceRanking[];
    totals: Record<RankingType, number>;
    currentPage: Record<RankingType, number>;
    perPage: number;
    loading: boolean;
    error: string | null;
}

function createRankingStore() {
    const { subscribe, set, update } = writable<RankingState>({
        populationRankings: [],
        attackRankings: [],
        defenseRankings: [],
        heroRankings: [],
        allianceRankings: [],
        totals: {
            population: 0,
            attackers: 0,
            defenders: 0,
            heroes: 0,
            alliances: 0,
        },
        currentPage: {
            population: 1,
            attackers: 1,
            defenders: 1,
            heroes: 1,
            alliances: 1,
        },
        perPage: 20,
        loading: false,
        error: null,
    });

    return {
        subscribe,

        // Load population rankings
        loadPopulationRankings: async (page = 1, perPage = 20) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const response = await api.get<RankingListResponse<PlayerPopulationRanking>>(
                    `/api/rankings/players/population?page=${page}&per_page=${perPage}`
                );
                update(state => ({
                    ...state,
                    populationRankings: response.rankings,
                    totals: { ...state.totals, population: response.total },
                    currentPage: { ...state.currentPage, population: response.page },
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

        // Load attack rankings
        loadAttackRankings: async (page = 1, perPage = 20) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const response = await api.get<RankingListResponse<PlayerAttackRanking>>(
                    `/api/rankings/players/attackers?page=${page}&per_page=${perPage}`
                );
                update(state => ({
                    ...state,
                    attackRankings: response.rankings,
                    totals: { ...state.totals, attackers: response.total },
                    currentPage: { ...state.currentPage, attackers: response.page },
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

        // Load defense rankings
        loadDefenseRankings: async (page = 1, perPage = 20) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const response = await api.get<RankingListResponse<PlayerDefenseRanking>>(
                    `/api/rankings/players/defenders?page=${page}&per_page=${perPage}`
                );
                update(state => ({
                    ...state,
                    defenseRankings: response.rankings,
                    totals: { ...state.totals, defenders: response.total },
                    currentPage: { ...state.currentPage, defenders: response.page },
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

        // Load hero rankings
        loadHeroRankings: async (page = 1, perPage = 20) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const response = await api.get<RankingListResponse<HeroRanking>>(
                    `/api/rankings/heroes?page=${page}&per_page=${perPage}`
                );
                update(state => ({
                    ...state,
                    heroRankings: response.rankings,
                    totals: { ...state.totals, heroes: response.total },
                    currentPage: { ...state.currentPage, heroes: response.page },
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

        // Load alliance rankings
        loadAllianceRankings: async (page = 1, perPage = 20) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const response = await api.get<RankingListResponse<AllianceRanking>>(
                    `/api/rankings/alliances?page=${page}&per_page=${perPage}`
                );
                update(state => ({
                    ...state,
                    allianceRankings: response.rankings,
                    totals: { ...state.totals, alliances: response.total },
                    currentPage: { ...state.currentPage, alliances: response.page },
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

        // Clear error
        clearError: () => {
            update(state => ({ ...state, error: null }));
        },

        // Reset store
        reset: () => {
            set({
                populationRankings: [],
                attackRankings: [],
                defenseRankings: [],
                heroRankings: [],
                allianceRankings: [],
                totals: {
                    population: 0,
                    attackers: 0,
                    defenders: 0,
                    heroes: 0,
                    alliances: 0,
                },
                currentPage: {
                    population: 1,
                    attackers: 1,
                    defenders: 1,
                    heroes: 1,
                    alliances: 1,
                },
                perPage: 20,
                loading: false,
                error: null,
            });
        },
    };
}

export const rankingStore = createRankingStore();

// Helper functions
export function getRankIcon(rank: number): string {
    if (rank === 1) return '🥇';
    if (rank === 2) return '🥈';
    if (rank === 3) return '🥉';
    return `#${rank}`;
}

export function getRankClass(rank: number): string {
    if (rank === 1) return 'text-yellow-600 font-bold';
    if (rank === 2) return 'text-gray-500 font-semibold';
    if (rank === 3) return 'text-amber-700 font-semibold';
    return 'text-muted-foreground';
}

export function formatNumber(num: number): string {
    if (num >= 1000000) {
        return (num / 1000000).toFixed(1) + 'M';
    }
    if (num >= 1000) {
        return (num / 1000).toFixed(1) + 'K';
    }
    return num.toLocaleString();
}

export function getRankingTypeLabel(type: RankingType): string {
    const labels: Record<RankingType, string> = {
        population: 'Population',
        attackers: 'Top Attackers',
        defenders: 'Top Defenders',
        heroes: 'Top Heroes',
        alliances: 'Top Alliances',
    };
    return labels[type] || type;
}

export function getRankingTypeIcon(type: RankingType): string {
    const icons: Record<RankingType, string> = {
        population: '👥',
        attackers: '⚔️',
        defenders: '🛡️',
        heroes: '🦸',
        alliances: '🏰',
    };
    return icons[type] || '📊';
}
