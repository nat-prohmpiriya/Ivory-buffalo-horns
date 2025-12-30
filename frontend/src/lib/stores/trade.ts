import { writable } from "svelte/store";
import { toast } from "svelte-sonner";
import { api } from "../api/client";

// Resource types for trading
export type TradeResourceType = 'wood' | 'clay' | 'iron' | 'crop';

// Order types
export type TradeOrderType = 'buy' | 'sell';

// Order status
export type TradeOrderStatus = 'open' | 'partially_filled' | 'filled' | 'cancelled' | 'expired';

export interface TradeOrder {
    id: string;
    user_id: string;
    village_id: string;
    order_type: TradeOrderType;
    resource_type: TradeResourceType;
    quantity: number;
    quantity_remaining: number;
    price_per_unit: number;
    total_price: number;
    status: TradeOrderStatus;
    expires_at: string | null;
    created_at: string;
    updated_at: string;
    // Optional joined fields
    village_name?: string;
    user_display_name?: string;
}

export interface TradeTransaction {
    id: string;
    order_id: string;
    buyer_id: string;
    seller_id: string;
    resource_type: TradeResourceType;
    quantity: number;
    price_per_unit: number;
    total_price: number;
    created_at: string;
}

export interface ResourceMarketSummary {
    resource_type: TradeResourceType;
    best_buy_price: number | null;
    best_sell_price: number | null;
    spread: number | null;
    volume_24h: number;
    total_buy_orders: number;
    total_sell_orders: number;
}

export interface MarketSummary {
    resources: ResourceMarketSummary[];
    updated_at: string;
}

export interface CreateOrderRequest {
    village_id: string;
    order_type: TradeOrderType;
    resource_type: TradeResourceType;
    quantity: number;
    price_per_unit: number;
    expires_in_hours?: number;
}

export interface AcceptOrderRequest {
    village_id: string;
    quantity?: number; // For partial fill
}

interface TradeState {
    marketSummary: MarketSummary | null;
    openOrders: TradeOrder[];
    myOrders: TradeOrder[];
    tradeHistory: TradeTransaction[];
    recentTransactions: TradeTransaction[];
    loading: boolean;
    error: string | null;
}

function createTradeStore() {
    const { subscribe, set, update } = writable<TradeState>({
        marketSummary: null,
        openOrders: [],
        myOrders: [],
        tradeHistory: [],
        recentTransactions: [],
        loading: false,
        error: null,
    });

    return {
        subscribe,

        // Load market summary (public)
        loadMarketSummary: async () => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const summary = await api.get<MarketSummary>('/api/market/summary', { auth: false });
                update(state => ({
                    ...state,
                    marketSummary: summary,
                    loading: false,
                }));
                return summary;
            } catch (error: any) {
                const message = error.message || 'Failed to load market summary';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                throw error;
            }
        },

        // Load open orders (public) with optional filters
        loadOpenOrders: async (resourceType?: TradeResourceType, orderType?: TradeOrderType) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                let endpoint = '/api/market/orders';
                const params = new URLSearchParams();
                if (resourceType) params.append('resource_type', resourceType);
                if (orderType) params.append('order_type', orderType);
                if (params.toString()) endpoint += `?${params.toString()}`;

                const orders = await api.get<TradeOrder[]>(endpoint, { auth: false });
                update(state => ({
                    ...state,
                    openOrders: orders,
                    loading: false,
                }));
                return orders;
            } catch (error: any) {
                const message = error.message || 'Failed to load open orders';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                throw error;
            }
        },

        // Load recent transactions (public)
        loadRecentTransactions: async () => {
            try {
                const transactions = await api.get<TradeTransaction[]>('/api/market/transactions', { auth: false });
                update(state => ({
                    ...state,
                    recentTransactions: transactions,
                }));
                return transactions;
            } catch (error: any) {
                console.error('Failed to load recent transactions:', error);
                return [];
            }
        },

        // Load my orders (authenticated)
        loadMyOrders: async () => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const orders = await api.get<TradeOrder[]>('/api/trade/orders');
                update(state => ({
                    ...state,
                    myOrders: orders,
                    loading: false,
                }));
                return orders;
            } catch (error: any) {
                const message = error.message || 'Failed to load my orders';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                throw error;
            }
        },

        // Load trade history (authenticated)
        loadTradeHistory: async () => {
            try {
                const history = await api.get<TradeTransaction[]>('/api/trade/history');
                update(state => ({
                    ...state,
                    tradeHistory: history,
                }));
                return history;
            } catch (error: any) {
                console.error('Failed to load trade history:', error);
                return [];
            }
        },

        // Create order (authenticated)
        createOrder: async (request: CreateOrderRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const order = await api.post<TradeOrder>('/api/trade/orders', request);

                update(state => ({
                    ...state,
                    myOrders: [order, ...state.myOrders],
                    loading: false,
                }));

                const action = request.order_type === 'sell' ? 'Sell' : 'Buy';
                toast.success(`${action} Order Created`, {
                    description: `${request.quantity} ${formatResourceType(request.resource_type)} at ${request.price_per_unit} gold each`
                });

                return order;
            } catch (error: any) {
                const message = error.message || 'Failed to create order';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Order Failed', { description: message });
                throw error;
            }
        },

        // Accept order (authenticated)
        acceptOrder: async (orderId: string, request: AcceptOrderRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const result = await api.post<{ order: TradeOrder; transaction: TradeTransaction }>(
                    `/api/trade/orders/${orderId}/accept`,
                    request
                );

                // Update open orders
                update(state => ({
                    ...state,
                    openOrders: state.openOrders.map(o =>
                        o.id === orderId ? result.order : o
                    ).filter(o => o.status === 'open' || o.status === 'partially_filled'),
                    tradeHistory: [result.transaction, ...state.tradeHistory],
                    loading: false,
                }));

                toast.success('Trade Completed', {
                    description: `Traded ${result.transaction.quantity} ${formatResourceType(result.transaction.resource_type)}`
                });

                return result;
            } catch (error: any) {
                const message = error.message || 'Failed to accept order';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Trade Failed', { description: message });
                throw error;
            }
        },

        // Cancel order (authenticated)
        cancelOrder: async (orderId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                await api.post(`/api/trade/orders/${orderId}/cancel`, {});

                update(state => ({
                    ...state,
                    myOrders: state.myOrders.map(o =>
                        o.id === orderId ? { ...o, status: 'cancelled' as TradeOrderStatus } : o
                    ),
                    loading: false,
                }));

                toast.success('Order Cancelled', {
                    description: 'Resources have been refunded'
                });

                return true;
            } catch (error: any) {
                const message = error.message || 'Failed to cancel order';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Cancel Failed', { description: message });
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
                marketSummary: null,
                openOrders: [],
                myOrders: [],
                tradeHistory: [],
                recentTransactions: [],
                loading: false,
                error: null,
            });
        },
    };
}

export const tradeStore = createTradeStore();

// Helper functions
export function formatResourceType(type: TradeResourceType): string {
    const labels: Record<TradeResourceType, string> = {
        wood: 'Wood',
        clay: 'Clay',
        iron: 'Iron',
        crop: 'Crop',
    };
    return labels[type] || type;
}

export function getResourceIcon(type: TradeResourceType): string {
    const icons: Record<TradeResourceType, string> = {
        wood: '🪵',
        clay: '🧱',
        iron: '⛏️',
        crop: '🌾',
    };
    return icons[type] || '📦';
}

export function getResourceColor(type: TradeResourceType): string {
    const colors: Record<TradeResourceType, string> = {
        wood: 'text-amber-600',
        clay: 'text-orange-600',
        iron: 'text-slate-600',
        crop: 'text-green-600',
    };
    return colors[type] || 'text-gray-600';
}

export function getOrderTypeLabel(type: TradeOrderType): string {
    return type === 'buy' ? 'Buy' : 'Sell';
}

export function getOrderTypeColor(type: TradeOrderType): string {
    return type === 'buy' ? 'text-green-600' : 'text-red-600';
}

export function getOrderStatusLabel(status: TradeOrderStatus): string {
    const labels: Record<TradeOrderStatus, string> = {
        open: 'Open',
        partially_filled: 'Partial',
        filled: 'Filled',
        cancelled: 'Cancelled',
        expired: 'Expired',
    };
    return labels[status] || status;
}

export function getOrderStatusColor(status: TradeOrderStatus): string {
    const colors: Record<TradeOrderStatus, string> = {
        open: 'text-blue-600',
        partially_filled: 'text-yellow-600',
        filled: 'text-green-600',
        cancelled: 'text-gray-600',
        expired: 'text-red-600',
    };
    return colors[status] || 'text-gray-600';
}

export function formatGold(amount: number): string {
    return amount.toLocaleString();
}

export function formatQuantity(amount: number): string {
    if (amount >= 1000000) {
        return (amount / 1000000).toFixed(1) + 'M';
    }
    if (amount >= 1000) {
        return (amount / 1000).toFixed(1) + 'K';
    }
    return amount.toLocaleString();
}
