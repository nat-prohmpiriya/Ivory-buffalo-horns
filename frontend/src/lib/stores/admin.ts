import { writable } from "svelte/store";
import { toast } from "svelte-sonner";
import { api } from "../api/client";

// Types
export interface AdminUserResponse {
    id: string;
    firebase_uid: string;
    email: string | null;
    display_name: string | null;
    photo_url: string | null;
    provider: string;
    is_admin: boolean;
    banned_at: string | null;
    banned_reason: string | null;
    created_at: string;
    last_login_at: string;
    village_count: number;
}

export interface ServerStatsResponse {
    total_users: number;
    active_users_24h: number;
    banned_users: number;
    total_villages: number;
    total_alliances: number;
    total_battles_today: number;
}

export interface AdminVillageResponse {
    id: string;
    name: string;
    x: number;
    y: number;
    is_capital: boolean;
    wood: number;
    clay: number;
    iron: number;
    crop: number;
    population: number;
}

export interface AdminHeroResponse {
    id: string;
    name: string;
    level: number;
    health: number;
    status: string;
}

export interface AdminAllianceInfoResponse {
    id: string;
    name: string;
    tag: string;
    role: string;
}

export interface PlayerDetailResponse {
    user: AdminUserResponse;
    villages: AdminVillageResponse[];
    heroes: AdminHeroResponse[];
    alliance: AdminAllianceInfoResponse | null;
}

export interface AdjustResourcesRequest {
    wood?: number;
    clay?: number;
    iron?: number;
    crop?: number;
    reason: string;
}

interface AdminState {
    users: AdminUserResponse[];
    selectedUser: PlayerDetailResponse | null;
    stats: ServerStatsResponse | null;
    currentPage: number;
    perPage: number;
    searchQuery: string;
    loading: boolean;
    error: string | null;
}

function createAdminStore() {
    const { subscribe, set, update } = writable<AdminState>({
        users: [],
        selectedUser: null,
        stats: null,
        currentPage: 1,
        perPage: 20,
        searchQuery: '',
        loading: false,
        error: null,
    });

    return {
        subscribe,

        // Load users
        loadUsers: async (page = 1, perPage = 20) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const users = await api.get<AdminUserResponse[]>(
                    `/api/admin/users?page=${page}&per_page=${perPage}`
                );
                update(state => ({
                    ...state,
                    users,
                    currentPage: page,
                    perPage,
                    loading: false,
                }));
                return users;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                return [];
            }
        },

        // Search users
        searchUsers: async (query: string) => {
            update(state => ({ ...state, loading: true, error: null, searchQuery: query }));
            try {
                const users = await api.get<AdminUserResponse[]>(
                    `/api/admin/users/search?q=${encodeURIComponent(query)}`
                );
                update(state => ({
                    ...state,
                    users,
                    loading: false,
                }));
                return users;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                return [];
            }
        },

        // Get player detail
        getPlayerDetail: async (userId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const detail = await api.get<PlayerDetailResponse>(
                    `/api/admin/users/${userId}`
                );
                update(state => ({
                    ...state,
                    selectedUser: detail,
                    loading: false,
                }));
                return detail;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                throw error;
            }
        },

        // Ban user
        banUser: async (userId: string, reason?: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const user = await api.post<AdminUserResponse>(
                    `/api/admin/users/${userId}/ban`,
                    { reason }
                );
                update(state => ({
                    ...state,
                    users: state.users.map(u => u.id === userId ? user : u),
                    selectedUser: state.selectedUser?.user.id === userId
                        ? { ...state.selectedUser, user }
                        : state.selectedUser,
                    loading: false,
                }));
                toast.success('User Banned', { description: user.display_name || user.email || 'User' });
                return user;
            } catch (error: any) {
                const message = error.message || 'Failed to ban user';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Unban user
        unbanUser: async (userId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const user = await api.post<AdminUserResponse>(
                    `/api/admin/users/${userId}/unban`,
                    {}
                );
                update(state => ({
                    ...state,
                    users: state.users.map(u => u.id === userId ? user : u),
                    selectedUser: state.selectedUser?.user.id === userId
                        ? { ...state.selectedUser, user }
                        : state.selectedUser,
                    loading: false,
                }));
                toast.success('User Unbanned', { description: user.display_name || user.email || 'User' });
                return user;
            } catch (error: any) {
                const message = error.message || 'Failed to unban user';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Set admin status
        setAdmin: async (userId: string, isAdmin: boolean) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const user = await api.put<AdminUserResponse>(
                    `/api/admin/users/${userId}/admin`,
                    { is_admin: isAdmin }
                );
                update(state => ({
                    ...state,
                    users: state.users.map(u => u.id === userId ? user : u),
                    selectedUser: state.selectedUser?.user.id === userId
                        ? { ...state.selectedUser, user }
                        : state.selectedUser,
                    loading: false,
                }));
                toast.success(isAdmin ? 'Admin Granted' : 'Admin Revoked', {
                    description: user.display_name || user.email || 'User'
                });
                return user;
            } catch (error: any) {
                const message = error.message || 'Failed to update admin status';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Get server stats
        getServerStats: async () => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const stats = await api.get<ServerStatsResponse>('/api/admin/stats');
                update(state => ({
                    ...state,
                    stats,
                    loading: false,
                }));
                return stats;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                return null;
            }
        },

        // Adjust resources
        adjustResources: async (villageId: string, request: AdjustResourcesRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                await api.post(`/api/admin/villages/${villageId}/resources`, request);
                toast.success('Resources Adjusted');
                update(state => ({ ...state, loading: false }));
                return true;
            } catch (error: any) {
                const message = error.message || 'Failed to adjust resources';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Clear selected user
        clearSelectedUser: () => {
            update(state => ({ ...state, selectedUser: null }));
        },

        // Clear error
        clearError: () => {
            update(state => ({ ...state, error: null }));
        },

        // Reset store
        reset: () => {
            set({
                users: [],
                selectedUser: null,
                stats: null,
                currentPage: 1,
                perPage: 20,
                searchQuery: '',
                loading: false,
                error: null,
            });
        },
    };
}

export const adminStore = createAdminStore();

// Helper functions
export function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
    });
}

export function formatDateShort(dateString: string): string {
    return new Date(dateString).toLocaleDateString('en-US', {
        month: 'short',
        day: 'numeric',
    });
}

export function getStatusBadge(user: AdminUserResponse): { label: string; class: string } {
    if (user.banned_at) {
        return { label: 'Banned', class: 'bg-red-100 text-red-800' };
    }
    if (user.is_admin) {
        return { label: 'Admin', class: 'bg-purple-100 text-purple-800' };
    }
    return { label: 'Active', class: 'bg-green-100 text-green-800' };
}

export function getProviderIcon(provider: string): string {
    const icons: Record<string, string> = {
        google: '🔵',
        facebook: '🔷',
        github: '⚫',
        email: '📧',
        password: '🔑',
    };
    return icons[provider.toLowerCase()] || '👤';
}
