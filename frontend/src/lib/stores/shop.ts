import { writable } from "svelte/store";
import { toast } from "svelte-sonner";
import { api } from "../api/client";

// Enums
export type TransactionStatus = 'pending' | 'completed' | 'failed' | 'refunded';
export type TransactionType = 'gold_purchase' | 'subscription' | 'gold_spend' | 'gold_refund' | 'gold_gift';
export type SubscriptionType = 'travian_plus';
export type GoldFeature = 'finish_now' | 'npc_merchant' | 'production_bonus' | 'book_of_wisdom' | 'artwork' | 'ointment' | 'plus_subscription' | 'hero_slot';

// Types
export interface GoldPackage {
    id: string;
    name: string;
    gold_amount: number;
    price_cents: number;
    currency: string;
    stripe_price_id: string | null;
    is_active: boolean;
    bonus_percent: number;
    created_at: string;
}

export interface SubscriptionPrice {
    id: string;
    subscription_type: SubscriptionType;
    duration_days: number;
    gold_cost: number;
    stripe_price_id: string | null;
    is_active: boolean;
}

export interface GoldBalanceResponse {
    gold_balance: number;
    has_plus: boolean;
    plus_expires_at: string | null;
}

export interface CheckoutResponse {
    checkout_url: string;
    session_id: string;
}

export interface TransactionResponse {
    id: string;
    transaction_type: TransactionType;
    status: TransactionStatus;
    gold_amount: number;
    description: string | null;
    created_at: string;
}

export interface UseFeatureResponse {
    success: boolean;
    gold_spent: number;
    new_balance: number;
    message: string;
}

// Request types
export interface PurchaseGoldRequest {
    package_id: string;
    success_url: string;
    cancel_url: string;
}

export interface UseFinishNowRequest {
    target_type: 'building' | 'troop_queue';
    target_id: string;
}

export interface UseNpcMerchantRequest {
    village_id: string;
    wood: number;
    clay: number;
    iron: number;
    crop: number;
}

export interface UseProductionBonusRequest {
    village_id: string;
    resource_type: 'wood' | 'clay' | 'iron' | 'crop';
}

export interface UseBookOfWisdomRequest {
    village_id: string;
}

interface ShopState {
    packages: GoldPackage[];
    subscriptionPrices: SubscriptionPrice[];
    balance: GoldBalanceResponse | null;
    transactions: TransactionResponse[];
    loading: boolean;
    error: string | null;
}

function createShopStore() {
    const { subscribe, set, update } = writable<ShopState>({
        packages: [],
        subscriptionPrices: [],
        balance: null,
        transactions: [],
        loading: false,
        error: null,
    });

    return {
        subscribe,

        // Load gold packages
        loadPackages: async () => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const packages = await api.get<GoldPackage[]>('/api/shop/packages');
                update(state => ({
                    ...state,
                    packages,
                    loading: false,
                }));
                return packages;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                return [];
            }
        },

        // Load balance
        loadBalance: async () => {
            try {
                const balance = await api.get<GoldBalanceResponse>('/api/shop/balance');
                update(state => ({
                    ...state,
                    balance,
                }));
                return balance;
            } catch (error: any) {
                console.error('Failed to load balance:', error);
                return null;
            }
        },

        // Load subscription prices
        loadSubscriptionPrices: async () => {
            try {
                const prices = await api.get<SubscriptionPrice[]>('/api/shop/subscriptions');
                update(state => ({
                    ...state,
                    subscriptionPrices: prices,
                }));
                return prices;
            } catch (error: any) {
                console.error('Failed to load subscription prices:', error);
                return [];
            }
        },

        // Create checkout session for gold purchase
        createCheckout: async (request: PurchaseGoldRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const checkout = await api.post<CheckoutResponse>('/api/shop/checkout', request);
                update(state => ({ ...state, loading: false }));
                // Redirect to Stripe checkout
                window.location.href = checkout.checkout_url;
                return checkout;
            } catch (error: any) {
                const message = error.message || 'Failed to create checkout';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Checkout Failed', { description: message });
                throw error;
            }
        },

        // Buy subscription with gold
        buySubscription: async (durationDays: number) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const result = await api.post<UseFeatureResponse>(
                    '/api/shop/subscriptions/buy',
                    { duration_days: durationDays }
                );
                // Reload balance
                const balance = await api.get<GoldBalanceResponse>('/api/shop/balance');
                update(state => ({
                    ...state,
                    balance,
                    loading: false,
                }));
                toast.success('Subscription Activated', { description: result.message });
                return result;
            } catch (error: any) {
                const message = error.message || 'Failed to buy subscription';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Use Finish Now feature
        useFinishNow: async (request: UseFinishNowRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const result = await api.post<UseFeatureResponse>(
                    '/api/shop/features/finish-now',
                    request
                );
                update(state => ({
                    ...state,
                    balance: state.balance ? { ...state.balance, gold_balance: result.new_balance } : null,
                    loading: false,
                }));
                toast.success('Completed Instantly', { description: result.message });
                return result;
            } catch (error: any) {
                const message = error.message || 'Failed to finish now';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Use NPC Merchant
        useNpcMerchant: async (request: UseNpcMerchantRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const result = await api.post<UseFeatureResponse>(
                    '/api/shop/features/npc-merchant',
                    request
                );
                update(state => ({
                    ...state,
                    balance: state.balance ? { ...state.balance, gold_balance: result.new_balance } : null,
                    loading: false,
                }));
                toast.success('Resources Exchanged', { description: result.message });
                return result;
            } catch (error: any) {
                const message = error.message || 'Failed to use NPC merchant';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Use Production Bonus
        useProductionBonus: async (request: UseProductionBonusRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const result = await api.post<UseFeatureResponse>(
                    '/api/shop/features/production-bonus',
                    request
                );
                update(state => ({
                    ...state,
                    balance: state.balance ? { ...state.balance, gold_balance: result.new_balance } : null,
                    loading: false,
                }));
                toast.success('Bonus Activated', { description: result.message });
                return result;
            } catch (error: any) {
                const message = error.message || 'Failed to activate bonus';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Use Book of Wisdom
        useBookOfWisdom: async (request: UseBookOfWisdomRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const result = await api.post<UseFeatureResponse>(
                    '/api/shop/features/book-of-wisdom',
                    request
                );
                update(state => ({
                    ...state,
                    balance: state.balance ? { ...state.balance, gold_balance: result.new_balance } : null,
                    loading: false,
                }));
                toast.success('Book of Wisdom Activated', { description: result.message });
                return result;
            } catch (error: any) {
                const message = error.message || 'Failed to use Book of Wisdom';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Failed', { description: message });
                throw error;
            }
        },

        // Load transactions
        loadTransactions: async (limit = 20, offset = 0) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const transactions = await api.get<TransactionResponse[]>(
                    `/api/shop/transactions?limit=${limit}&offset=${offset}`
                );
                update(state => ({
                    ...state,
                    transactions,
                    loading: false,
                }));
                return transactions;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                return [];
            }
        },

        // Clear error
        clearError: () => {
            update(state => ({ ...state, error: null }));
        },

        // Reset store
        reset: () => {
            set({
                packages: [],
                subscriptionPrices: [],
                balance: null,
                transactions: [],
                loading: false,
                error: null,
            });
        },
    };
}

export const shopStore = createShopStore();

// Helper functions
export function formatPrice(cents: number, currency = 'USD'): string {
    return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: currency.toUpperCase(),
    }).format(cents / 100);
}

export function formatGold(amount: number): string {
    if (amount >= 1000) {
        return (amount / 1000).toFixed(1) + 'K';
    }
    return amount.toLocaleString();
}

export function getTransactionTypeLabel(type: TransactionType): string {
    const labels: Record<TransactionType, string> = {
        gold_purchase: 'Gold Purchase',
        subscription: 'Subscription',
        gold_spend: 'Gold Spent',
        gold_refund: 'Refund',
        gold_gift: 'Gift',
    };
    return labels[type] || type;
}

export function getTransactionTypeIcon(type: TransactionType): string {
    const icons: Record<TransactionType, string> = {
        gold_purchase: '💰',
        subscription: '⭐',
        gold_spend: '🔻',
        gold_refund: '↩️',
        gold_gift: '🎁',
    };
    return icons[type] || '💰';
}

export function getTransactionStatusColor(status: TransactionStatus): string {
    const colors: Record<TransactionStatus, string> = {
        pending: 'text-yellow-600',
        completed: 'text-green-600',
        failed: 'text-red-600',
        refunded: 'text-blue-600',
    };
    return colors[status] || 'text-gray-600';
}

export function getFeatureIcon(feature: GoldFeature): string {
    const icons: Record<GoldFeature, string> = {
        finish_now: '⚡',
        npc_merchant: '🏪',
        production_bonus: '📈',
        book_of_wisdom: '📚',
        artwork: '🎨',
        ointment: '💊',
        plus_subscription: '⭐',
        hero_slot: '🦸',
    };
    return icons[feature] || '✨';
}

export function getFeatureLabel(feature: GoldFeature): string {
    const labels: Record<GoldFeature, string> = {
        finish_now: 'Finish Now',
        npc_merchant: 'NPC Merchant',
        production_bonus: 'Production Bonus',
        book_of_wisdom: 'Book of Wisdom',
        artwork: 'Artwork',
        ointment: 'Ointment',
        plus_subscription: 'Travian Plus',
        hero_slot: 'Hero Slot',
    };
    return labels[feature] || feature;
}

export function formatDate(dateString: string): string {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
    });
}

export function getTimeRemaining(expiresAt: string): string {
    const now = new Date();
    const expires = new Date(expiresAt);
    const diff = expires.getTime() - now.getTime();

    if (diff <= 0) return 'Expired';

    const days = Math.floor(diff / (24 * 60 * 60 * 1000));
    const hours = Math.floor((diff % (24 * 60 * 60 * 1000)) / (60 * 60 * 1000));

    if (days > 0) {
        return `${days}d ${hours}h`;
    }
    return `${hours}h`;
}
