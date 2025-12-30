<script lang="ts">
    import { Button } from '$lib/components/ui/button';
    import {
        type TradeOrder,
        type TradeResourceType,
        getResourceIcon,
        getResourceColor,
        getOrderTypeLabel,
        getOrderTypeColor,
        getOrderStatusLabel,
        getOrderStatusColor,
        formatGold,
        formatQuantity,
    } from '$lib/stores/trade';

    interface Props {
        order: TradeOrder;
        isOwner?: boolean;
        currentUserId?: string;
        onAccept?: (quantity?: number) => void;
        onCancel?: () => void;
        loading?: boolean;
    }

    let { order, isOwner = false, currentUserId, onAccept, onCancel, loading = false }: Props = $props();

    let acceptQuantity = $state(order.quantity_remaining);

    const isOwnOrder = $derived(currentUserId === order.user_id);
    const canAccept = $derived(!isOwnOrder && order.status === 'open' || order.status === 'partially_filled');
    const canCancel = $derived(isOwnOrder && (order.status === 'open' || order.status === 'partially_filled'));

    function handleAccept() {
        if (onAccept) {
            onAccept(acceptQuantity);
        }
    }

    function formatTimeLeft(expiresAt: string | null): string {
        if (!expiresAt) return 'No expiry';
        const now = new Date();
        const expires = new Date(expiresAt);
        const diff = expires.getTime() - now.getTime();

        if (diff <= 0) return 'Expired';

        const hours = Math.floor(diff / (1000 * 60 * 60));
        const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));

        if (hours > 24) {
            const days = Math.floor(hours / 24);
            return `${days}d ${hours % 24}h`;
        }
        if (hours > 0) {
            return `${hours}h ${minutes}m`;
        }
        return `${minutes}m`;
    }
</script>

<div class="p-4 rounded-lg border bg-card hover:bg-accent/50 transition-colors">
    <div class="flex items-start justify-between gap-4">
        <!-- Left: Resource info -->
        <div class="flex items-center gap-3">
            <div class="w-12 h-12 rounded-lg bg-muted flex items-center justify-center text-2xl">
                {getResourceIcon(order.resource_type)}
            </div>
            <div>
                <div class="flex items-center gap-2">
                    <span class="font-semibold {getOrderTypeColor(order.order_type)}">
                        {getOrderTypeLabel(order.order_type)}
                    </span>
                    <span class="font-medium {getResourceColor(order.resource_type)}">
                        {order.resource_type.charAt(0).toUpperCase() + order.resource_type.slice(1)}
                    </span>
                </div>
                <div class="text-sm text-muted-foreground">
                    {#if order.village_name}
                        <span>{order.village_name}</span>
                        <span class="mx-1">-</span>
                    {/if}
                    <span class="{getOrderStatusColor(order.status)}">
                        {getOrderStatusLabel(order.status)}
                    </span>
                </div>
            </div>
        </div>

        <!-- Right: Price and quantity -->
        <div class="text-right">
            <div class="font-semibold">
                {formatQuantity(order.quantity_remaining)}
                <span class="text-muted-foreground">/ {formatQuantity(order.quantity)}</span>
            </div>
            <div class="text-sm">
                <span class="text-yellow-600">{formatGold(order.price_per_unit)}</span>
                <span class="text-muted-foreground"> gold/unit</span>
            </div>
            <div class="text-xs text-muted-foreground">
                Total: <span class="text-yellow-600">{formatGold(order.quantity_remaining * order.price_per_unit)}</span> gold
            </div>
        </div>
    </div>

    <!-- Expiry -->
    {#if order.expires_at}
        <div class="mt-2 text-xs text-muted-foreground">
            Expires in: {formatTimeLeft(order.expires_at)}
        </div>
    {/if}

    <!-- Actions -->
    {#if canAccept || canCancel}
        <div class="mt-3 pt-3 border-t flex items-center gap-2">
            {#if canAccept && onAccept}
                <div class="flex items-center gap-2 flex-1">
                    <input
                        type="number"
                        min="1"
                        max={order.quantity_remaining}
                        bind:value={acceptQuantity}
                        class="w-24 px-2 py-1 text-sm border rounded"
                        placeholder="Qty"
                    />
                    <Button
                        size="sm"
                        onclick={handleAccept}
                        disabled={loading || acceptQuantity < 1 || acceptQuantity > order.quantity_remaining}
                    >
                        {#if loading}
                            <span class="animate-spin mr-1">...</span>
                        {/if}
                        {order.order_type === 'sell' ? 'Buy' : 'Sell'}
                    </Button>
                </div>
            {/if}

            {#if canCancel && onCancel}
                <Button
                    variant="outline"
                    size="sm"
                    onclick={onCancel}
                    disabled={loading}
                >
                    {#if loading}
                        <span class="animate-spin mr-1">...</span>
                    {/if}
                    Cancel
                </Button>
            {/if}
        </div>
    {/if}
</div>
