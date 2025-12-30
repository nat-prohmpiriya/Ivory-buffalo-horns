import { writable } from "svelte/store";
import { toast } from "svelte-sonner";
import { api } from "../api/client";

// Enums
export type AllianceRole = 'leader' | 'officer' | 'member';
export type InvitationStatus = 'pending' | 'accepted' | 'rejected' | 'expired';
export type DiplomacyStatus = 'neutral' | 'ally' | 'nap' | 'enemy';

// Types
export interface Alliance {
    id: string;
    name: string;
    tag: string;
    description: string | null;
    founder_id: string;
    leader_id: string;
    max_members: number;
    member_count: number;
    created_at: string;
}

export interface AllianceListItem {
    id: string;
    name: string;
    tag: string;
    member_count: number;
    total_population: number;
}

export interface AllianceMember {
    id: string;
    user_id: string;
    player_name: string;
    role: AllianceRole;
    villages_count: number;
    population: number;
    joined_at: string;
}

export interface AllianceInvitation {
    id: string;
    alliance_id: string;
    inviter_id: string;
    invitee_id: string;
    status: InvitationStatus;
    message: string | null;
    created_at: string;
    expires_at: string;
    responded_at: string | null;
    // Extended fields
    alliance_name?: string;
    alliance_tag?: string;
    inviter_name?: string;
}

export interface AllianceDiplomacy {
    id: string;
    alliance_id: string;
    target_alliance_id: string;
    status: DiplomacyStatus;
    proposed_by: string | null;
    created_at: string;
    updated_at: string;
    // Extended fields
    target_alliance_name?: string;
    target_alliance_tag?: string;
}

export interface CreateAllianceRequest {
    name: string;
    tag: string;
    description?: string;
}

export interface InvitePlayerRequest {
    player_id: string;
    message?: string;
}

export interface SetDiplomacyRequest {
    target_alliance_id: string;
    status: DiplomacyStatus;
}

interface AllianceState {
    myAlliance: Alliance | null;
    members: AllianceMember[];
    invitations: AllianceInvitation[];
    diplomacy: AllianceDiplomacy[];
    allAlliances: AllianceListItem[];
    loading: boolean;
    error: string | null;
}

function createAllianceStore() {
    const { subscribe, set, update } = writable<AllianceState>({
        myAlliance: null,
        members: [],
        invitations: [],
        diplomacy: [],
        allAlliances: [],
        loading: false,
        error: null,
    });

    return {
        subscribe,

        // Load my alliance
        loadMyAlliance: async () => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const alliance = await api.get<Alliance | null>('/api/alliances/my');
                update(state => ({
                    ...state,
                    myAlliance: alliance,
                    loading: false,
                }));
                return alliance;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                return null;
            }
        },

        // Load alliance by ID
        loadAlliance: async (allianceId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const alliance = await api.get<Alliance>(`/api/alliances/${allianceId}`);
                return alliance;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                throw error;
            }
        },

        // List all alliances
        loadAllAlliances: async (limit = 20, offset = 0) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const alliances = await api.get<AllianceListItem[]>(
                    `/api/alliances?limit=${limit}&offset=${offset}`
                );
                update(state => ({
                    ...state,
                    allAlliances: alliances,
                    loading: false,
                }));
                return alliances;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                return [];
            }
        },

        // Create alliance
        createAlliance: async (request: CreateAllianceRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const alliance = await api.post<Alliance>('/api/alliances', request);
                update(state => ({
                    ...state,
                    myAlliance: alliance,
                    loading: false,
                }));
                toast.success('Alliance Created', {
                    description: `[${alliance.tag}] ${alliance.name}`
                });
                return alliance;
            } catch (error: any) {
                const message = error.message || 'Failed to create alliance';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Update alliance
        updateAlliance: async (allianceId: string, name?: string, description?: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const alliance = await api.put<Alliance>(
                    `/api/alliances/${allianceId}`,
                    { name, description }
                );
                update(state => ({
                    ...state,
                    myAlliance: alliance,
                    loading: false,
                }));
                toast.success('Alliance Updated');
                return alliance;
            } catch (error: any) {
                const message = error.message || 'Failed to update alliance';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Disband alliance
        disbandAlliance: async (allianceId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                await api.delete(`/api/alliances/${allianceId}`);
                update(state => ({
                    ...state,
                    myAlliance: null,
                    members: [],
                    diplomacy: [],
                    loading: false,
                }));
                toast.success('Alliance Disbanded');
                return true;
            } catch (error: any) {
                const message = error.message || 'Failed to disband alliance';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Load members
        loadMembers: async (allianceId: string) => {
            try {
                const members = await api.get<AllianceMember[]>(
                    `/api/alliances/${allianceId}/members`
                );
                update(state => ({
                    ...state,
                    members,
                }));
                return members;
            } catch (error: any) {
                console.error('Failed to load members:', error);
                return [];
            }
        },

        // Invite player
        invitePlayer: async (allianceId: string, request: InvitePlayerRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const invitation = await api.post<AllianceInvitation>(
                    `/api/alliances/${allianceId}/invite`,
                    request
                );
                toast.success('Invitation Sent');
                update(state => ({ ...state, loading: false }));
                return invitation;
            } catch (error: any) {
                const message = error.message || 'Failed to send invitation';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Leave alliance
        leaveAlliance: async () => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                await api.post('/api/alliances/leave', {});
                update(state => ({
                    ...state,
                    myAlliance: null,
                    members: [],
                    diplomacy: [],
                    loading: false,
                }));
                toast.success('Left Alliance');
                return true;
            } catch (error: any) {
                const message = error.message || 'Failed to leave alliance';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Kick member
        kickMember: async (allianceId: string, userId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                await api.delete(`/api/alliances/${allianceId}/members/${userId}`);
                update(state => ({
                    ...state,
                    members: state.members.filter(m => m.user_id !== userId),
                    loading: false,
                }));
                toast.success('Member Kicked');
                return true;
            } catch (error: any) {
                const message = error.message || 'Failed to kick member';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Update member role
        updateMemberRole: async (allianceId: string, userId: string, role: AllianceRole) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                await api.put(
                    `/api/alliances/${allianceId}/members/${userId}/role`,
                    { role }
                );
                update(state => ({
                    ...state,
                    members: state.members.map(m =>
                        m.user_id === userId ? { ...m, role } : m
                    ),
                    loading: false,
                }));
                toast.success('Role Updated');
                return true;
            } catch (error: any) {
                const message = error.message || 'Failed to update role';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Load invitations (for current user)
        loadInvitations: async () => {
            try {
                const invitations = await api.get<AllianceInvitation[]>(
                    '/api/alliances/invitations'
                );
                update(state => ({
                    ...state,
                    invitations,
                }));
                return invitations;
            } catch (error: any) {
                console.error('Failed to load invitations:', error);
                return [];
            }
        },

        // Respond to invitation
        respondInvitation: async (invitationId: string, accept: boolean) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                await api.post(
                    `/api/alliances/invitations/${invitationId}/respond`,
                    { accept }
                );

                // Remove from invitations list
                update(state => ({
                    ...state,
                    invitations: state.invitations.filter(i => i.id !== invitationId),
                    loading: false,
                }));

                if (accept) {
                    toast.success('Joined Alliance');
                    // Reload my alliance
                    const alliance = await api.get<Alliance | null>('/api/alliances/my');
                    update(state => ({ ...state, myAlliance: alliance }));
                } else {
                    toast.success('Invitation Declined');
                }

                return true;
            } catch (error: any) {
                const message = error.message || 'Failed to respond';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Load diplomacy
        loadDiplomacy: async (allianceId: string) => {
            try {
                const diplomacy = await api.get<AllianceDiplomacy[]>(
                    `/api/alliances/${allianceId}/diplomacy`
                );
                update(state => ({
                    ...state,
                    diplomacy,
                }));
                return diplomacy;
            } catch (error: any) {
                console.error('Failed to load diplomacy:', error);
                return [];
            }
        },

        // Set diplomacy
        setDiplomacy: async (allianceId: string, request: SetDiplomacyRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const diplomacy = await api.post<AllianceDiplomacy>(
                    `/api/alliances/${allianceId}/diplomacy`,
                    request
                );

                // Update diplomacy list
                update(state => {
                    const existing = state.diplomacy.findIndex(
                        d => d.target_alliance_id === request.target_alliance_id
                    );
                    const newDiplomacy = existing >= 0
                        ? state.diplomacy.map((d, i) => i === existing ? diplomacy : d)
                        : [...state.diplomacy, diplomacy];

                    return {
                        ...state,
                        diplomacy: newDiplomacy,
                        loading: false,
                    };
                });

                toast.success('Diplomacy Updated');
                return diplomacy;
            } catch (error: any) {
                const message = error.message || 'Failed to set diplomacy';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Clear error
        clearError: () => {
            update(state => ({ ...state, error: null }));
        },

        // Reset store
        reset: () => {
            set({
                myAlliance: null,
                members: [],
                invitations: [],
                diplomacy: [],
                allAlliances: [],
                loading: false,
                error: null,
            });
        },
    };
}

export const allianceStore = createAllianceStore();

// Helper functions
export function getRoleLabel(role: AllianceRole): string {
    const labels: Record<AllianceRole, string> = {
        leader: 'Leader',
        officer: 'Officer',
        member: 'Member',
    };
    return labels[role] || role;
}

export function getRoleIcon(role: AllianceRole): string {
    const icons: Record<AllianceRole, string> = {
        leader: '👑',
        officer: '⭐',
        member: '👤',
    };
    return icons[role] || '👤';
}

export function getRoleColor(role: AllianceRole): string {
    const colors: Record<AllianceRole, string> = {
        leader: 'text-yellow-600',
        officer: 'text-blue-600',
        member: 'text-gray-600',
    };
    return colors[role] || 'text-gray-600';
}

export function getDiplomacyLabel(status: DiplomacyStatus): string {
    const labels: Record<DiplomacyStatus, string> = {
        neutral: 'Neutral',
        ally: 'Ally',
        nap: 'NAP',
        enemy: 'Enemy',
    };
    return labels[status] || status;
}

export function getDiplomacyIcon(status: DiplomacyStatus): string {
    const icons: Record<DiplomacyStatus, string> = {
        neutral: '⚪',
        ally: '🤝',
        nap: '🕊️',
        enemy: '⚔️',
    };
    return icons[status] || '⚪';
}

export function getDiplomacyColor(status: DiplomacyStatus): string {
    const colors: Record<DiplomacyStatus, string> = {
        neutral: 'text-gray-600',
        ally: 'text-green-600',
        nap: 'text-blue-600',
        enemy: 'text-red-600',
    };
    return colors[status] || 'text-gray-600';
}

export function formatPopulation(pop: number): string {
    if (pop >= 1000000) {
        return (pop / 1000000).toFixed(1) + 'M';
    }
    if (pop >= 1000) {
        return (pop / 1000).toFixed(1) + 'K';
    }
    return pop.toLocaleString();
}
