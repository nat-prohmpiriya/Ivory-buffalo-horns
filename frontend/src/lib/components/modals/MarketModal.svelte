<script lang="ts">
    import * as Dialog from '$lib/components/ui/dialog';
    import { Button } from '$lib/components/ui/button';
    import { Separator } from '$lib/components/ui/separator';
    import { Label } from '$lib/components/ui/label';
    import { Input } from '$lib/components/ui/input';
    import OrderCard from '$lib/components/game/OrderCard.svelte';
    import {
        tradeStore,
        type TradeOrder,
        type TradeResourceType,
        type TradeOrderType,
        type MarketSummary,
        type CreateOrderRequest,
        getResourceIcon,
        getResourceColor,
        formatGold,
        formatQuantity,
    } from '$lib/stores/trade';
    import { authStore } from '$lib/stores/auth';

    interface VillageResources {
        wood: number;
        clay: number;
        iron: number;
        crop: number;
    }

    interface Props {
        open: boolean;
        villageId: string;
        villageResources: VillageResources;
        userGold?: number;
    }

    let { open = $bindable(false), villageId, villageResources, userGold = 0 }: Props = $props();

    // Tabs
    type TabType = 'market' | 'my_orders' | 'create' | 'history';
    let activeTab = $state<TabType>('market');

    // Filters
    let filterResource = $state<TradeResourceType | ''>('');
    let filterOrderType = $state<TradeOrderType | ''>('');

    // Create order form
    let createOrderType = $state<TradeOrderType>('sell');
    let createResource = $state<TradeResourceType>('wood');
    let createQuantity = $state(100);
    let createPrice = $state(1);
    let createExpiry = $state(24);

    // Store state
    let tradeState = $state(tradeStore);
    let marketSummary = $derived($tradeState.marketSummary);
    let openOrders = $derived($tradeState.openOrders);
    let myOrders = $derived($tradeState.myOrders);
    let tradeHistory = $derived($tradeState.tradeHistory);
    let loading = $derived($tradeState.loading);

    let authState = $state(authStore);
    let backendUser = $derived($authState.backendUser);

    // Filtered orders
    const filteredOrders = $derived(
        openOrders.filter(order => {
            if (filterResource && order.resource_type !== filterResource) return false;
            if (filterOrderType && order.order_type !== filterOrderType) return false;
            return true;
        })
    );

    // Active my orders (not cancelled/filled)
    const activeMyOrders = $derived(
        myOrders.filter(o => o.status === 'open' || o.status === 'partially_filled')
    );

    // Load data when modal opens
    $effect(() => {
        if (open) {
            loadData();
        }
    });

    async function loadData() {
        try {
            await Promise.all([
                tradeStore.loadMarketSummary(),
                tradeStore.loadOpenOrders(),
                tradeStore.loadMyOrders(),
                tradeStore.loadTradeHistory(),
            ]);
        } catch (error) {
            console.error('Failed to load market data:', error);
        }
    }

    // Filter change handler
    function handleFilterChange() {
        tradeStore.loadOpenOrders(
            filterResource || undefined,
            filterOrderType || undefined
        );
    }

    // Create order handler
    async function handleCreateOrder() {
        if (!villageId) return;

        const request: CreateOrderRequest = {
            village_id: villageId,
            order_type: createOrderType,
            resource_type: createResource,
            quantity: createQuantity,
            price_per_unit: createPrice,
            expires_in_hours: createExpiry,
        };

        try {
            await tradeStore.createOrder(request);
            // Reset form
            createQuantity = 100;
            createPrice = 1;
            // Switch to my orders tab
            activeTab = 'my_orders';
        } catch {
            // Error handled in store
        }
    }

    // Accept order handler
    async function handleAcceptOrder(orderId: string, quantity?: number) {
        if (!villageId) return;

        try {
            await tradeStore.acceptOrder(orderId, {
                village_id: villageId,
                quantity,
            });
            // Reload orders
            await tradeStore.loadOpenOrders();
        } catch {
            // Error handled in store
        }
    }

    // Cancel order handler
    async function handleCancelOrder(orderId: string) {
        try {
            await tradeStore.cancelOrder(orderId);
        } catch {
            // Error handled in store
        }
    }

    // Validation
    const canCreateOrder = $derived(() => {
        if (createQuantity < 1) return false;
        if (createPrice < 1) return false;

        if (createOrderType === 'sell') {
            // Check if has enough resources
            const available = villageResources[createResource] || 0;
            return createQuantity <= available;
        } else {
            // Check if has enough gold
            const totalCost = createQuantity * createPrice;
            return totalCost <= userGold;
        }
    });

    const resourceTypes: TradeResourceType[] = ['wood', 'clay', 'iron', 'crop'];
</script>

<Dialog.Root bind:open>
    <Dialog.Content class="sm:max-w-3xl max-h-[90vh] flex flex-col">
        <Dialog.Header>
            <div class="flex items-center gap-3">
                <div class="w-12 h-12 rounded-xl bg-muted flex items-center justify-center text-2xl">
                    🏪
                </div>
                <div>
                    <Dialog.Title class="text-xl">Marketplace</Dialog.Title>
                    <Dialog.Description>
                        Trade resources with other players
                    </Dialog.Description>
                </div>
            </div>
        </Dialog.Header>

        <!-- Tabs -->
        <div class="flex gap-2 mt-4 flex-wrap">
            <Button
                variant={activeTab === 'market' ? 'default' : 'outline'}
                size="sm"
                onclick={() => activeTab = 'market'}
            >
                <span class="mr-1">📊</span> Market
            </Button>
            <Button
                variant={activeTab === 'my_orders' ? 'default' : 'outline'}
                size="sm"
                onclick={() => activeTab = 'my_orders'}
            >
                <span class="mr-1">📋</span> My Orders
                {#if activeMyOrders.length > 0}
                    <span class="ml-1 px-1.5 py-0.5 text-xs bg-primary/20 rounded-full">
                        {activeMyOrders.length}
                    </span>
                {/if}
            </Button>
            <Button
                variant={activeTab === 'create' ? 'default' : 'outline'}
                size="sm"
                onclick={() => activeTab = 'create'}
            >
                <span class="mr-1">➕</span> Create Order
            </Button>
            <Button
                variant={activeTab === 'history' ? 'default' : 'outline'}
                size="sm"
                onclick={() => activeTab = 'history'}
            >
                <span class="mr-1">📜</span> History
            </Button>
        </div>

        <!-- Content -->
        <div class="flex-1 overflow-y-auto mt-4 min-h-[400px]">
            {#if loading && !marketSummary}
                <div class="flex items-center justify-center py-12">
                    <span class="animate-spin text-2xl mr-2">...</span>
                    <span>Loading market data...</span>
                </div>
            {:else if activeTab === 'market'}
                <!-- Market Summary -->
                {#if marketSummary}
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-4">
                        {#each marketSummary.resources as resource}
                            <div class="p-3 rounded-lg border bg-card">
                                <div class="flex items-center gap-2 mb-2">
                                    <span class="text-xl">{getResourceIcon(resource.resource_type)}</span>
                                    <span class="font-medium {getResourceColor(resource.resource_type)}">
                                        {resource.resource_type.charAt(0).toUpperCase() + resource.resource_type.slice(1)}
                                    </span>
                                </div>
                                <div class="grid grid-cols-2 gap-1 text-xs">
                                    <div>
                                        <span class="text-muted-foreground">Buy:</span>
                                        <span class="text-green-600 ml-1">
                                            {resource.best_buy_price ? formatGold(resource.best_buy_price) : '-'}
                                        </span>
                                    </div>
                                    <div>
                                        <span class="text-muted-foreground">Sell:</span>
                                        <span class="text-red-600 ml-1">
                                            {resource.best_sell_price ? formatGold(resource.best_sell_price) : '-'}
                                        </span>
                                    </div>
                                    <div class="col-span-2">
                                        <span class="text-muted-foreground">24h Vol:</span>
                                        <span class="ml-1">{formatQuantity(resource.volume_24h)}</span>
                                    </div>
                                </div>
                            </div>
                        {/each}
                    </div>
                {/if}

                <Separator class="my-4" />

                <!-- Filters -->
                <div class="flex gap-2 mb-4 flex-wrap">
                    <select
                        class="px-3 py-1.5 text-sm border rounded-md bg-background"
                        bind:value={filterResource}
                        onchange={handleFilterChange}
                    >
                        <option value="">All Resources</option>
                        {#each resourceTypes as rt}
                            <option value={rt}>{rt.charAt(0).toUpperCase() + rt.slice(1)}</option>
                        {/each}
                    </select>
                    <select
                        class="px-3 py-1.5 text-sm border rounded-md bg-background"
                        bind:value={filterOrderType}
                        onchange={handleFilterChange}
                    >
                        <option value="">All Types</option>
                        <option value="buy">Buy Orders</option>
                        <option value="sell">Sell Orders</option>
                    </select>
                    <Button variant="outline" size="sm" onclick={() => tradeStore.loadOpenOrders()}>
                        Refresh
                    </Button>
                </div>

                <!-- Open Orders -->
                <div class="space-y-3">
                    {#if filteredOrders.length === 0}
                        <div class="text-center py-8 text-muted-foreground">
                            <p class="text-4xl mb-2">📭</p>
                            <p>No open orders</p>
                            <p class="text-sm">Create an order or wait for others</p>
                        </div>
                    {:else}
                        {#each filteredOrders as order (order.id)}
                            <OrderCard
                                {order}
                                currentUserId={backendUser?.id}
                                onAccept={(qty) => handleAcceptOrder(order.id, qty)}
                                {loading}
                            />
                        {/each}
                    {/if}
                </div>

            {:else if activeTab === 'my_orders'}
                <!-- My Orders -->
                <div class="space-y-3">
                    {#if myOrders.length === 0}
                        <div class="text-center py-8 text-muted-foreground">
                            <p class="text-4xl mb-2">📋</p>
                            <p>You have no orders</p>
                            <Button variant="outline" size="sm" class="mt-2" onclick={() => activeTab = 'create'}>
                                Create Order
                            </Button>
                        </div>
                    {:else}
                        <div class="mb-3 text-sm text-muted-foreground">
                            Active: {activeMyOrders.length} / Total: {myOrders.length}
                        </div>
                        {#each myOrders as order (order.id)}
                            <OrderCard
                                {order}
                                isOwner={true}
                                currentUserId={backendUser?.id}
                                onCancel={() => handleCancelOrder(order.id)}
                                {loading}
                            />
                        {/each}
                    {/if}
                </div>

            {:else if activeTab === 'create'}
                <!-- Create Order Form -->
                <div class="max-w-md mx-auto space-y-4">
                    <!-- Order Type -->
                    <div class="grid grid-cols-2 gap-2">
                        <Button
                            variant={createOrderType === 'sell' ? 'default' : 'outline'}
                            onclick={() => createOrderType = 'sell'}
                            class="h-16"
                        >
                            <div class="text-center">
                                <div class="text-2xl mb-1">📤</div>
                                <div>Sell Resources</div>
                            </div>
                        </Button>
                        <Button
                            variant={createOrderType === 'buy' ? 'default' : 'outline'}
                            onclick={() => createOrderType = 'buy'}
                            class="h-16"
                        >
                            <div class="text-center">
                                <div class="text-2xl mb-1">📥</div>
                                <div>Buy Resources</div>
                            </div>
                        </Button>
                    </div>

                    <!-- Resource Type -->
                    <div>
                        <Label>Resource</Label>
                        <div class="grid grid-cols-4 gap-2 mt-1">
                            {#each resourceTypes as rt}
                                <Button
                                    variant={createResource === rt ? 'default' : 'outline'}
                                    size="sm"
                                    onclick={() => createResource = rt}
                                    class="flex-col h-14"
                                >
                                    <span class="text-lg">{getResourceIcon(rt)}</span>
                                    <span class="text-xs">{rt.charAt(0).toUpperCase() + rt.slice(1)}</span>
                                </Button>
                            {/each}
                        </div>
                        {#if createOrderType === 'sell'}
                            <p class="text-xs text-muted-foreground mt-1">
                                Available: {formatQuantity(villageResources[createResource] || 0)}
                            </p>
                        {/if}
                    </div>

                    <!-- Quantity -->
                    <div>
                        <Label for="quantity">Quantity</Label>
                        <Input
                            id="quantity"
                            type="number"
                            min="1"
                            bind:value={createQuantity}
                            class="mt-1"
                        />
                    </div>

                    <!-- Price -->
                    <div>
                        <Label for="price">Price per unit (gold)</Label>
                        <Input
                            id="price"
                            type="number"
                            min="1"
                            bind:value={createPrice}
                            class="mt-1"
                        />
                    </div>

                    <!-- Expiry -->
                    <div>
                        <Label for="expiry">Expires in (hours)</Label>
                        <select
                            id="expiry"
                            class="w-full px-3 py-2 mt-1 border rounded-md bg-background"
                            bind:value={createExpiry}
                        >
                            <option value={1}>1 hour</option>
                            <option value={6}>6 hours</option>
                            <option value={12}>12 hours</option>
                            <option value={24}>24 hours</option>
                            <option value={48}>48 hours</option>
                            <option value={72}>72 hours</option>
                            <option value={168}>7 days</option>
                        </select>
                    </div>

                    <!-- Summary -->
                    <div class="p-4 rounded-lg bg-muted">
                        <div class="flex justify-between mb-2">
                            <span>Total:</span>
                            <span class="font-semibold text-yellow-600">
                                {formatGold(createQuantity * createPrice)} gold
                            </span>
                        </div>
                        {#if createOrderType === 'sell'}
                            <p class="text-xs text-muted-foreground">
                                You will receive gold when someone buys your resources.
                                Resources will be locked until order is filled or cancelled.
                            </p>
                        {:else}
                            <p class="text-xs text-muted-foreground">
                                Gold will be deducted immediately.
                                You will receive resources when someone fills your order.
                            </p>
                        {/if}
                    </div>

                    <!-- Submit -->
                    <Button
                        class="w-full"
                        onclick={handleCreateOrder}
                        disabled={loading || !canCreateOrder()}
                    >
                        {#if loading}
                            <span class="animate-spin mr-2">...</span>
                        {/if}
                        Create {createOrderType === 'sell' ? 'Sell' : 'Buy'} Order
                    </Button>
                </div>

            {:else if activeTab === 'history'}
                <!-- Trade History -->
                <div class="space-y-2">
                    {#if tradeHistory.length === 0}
                        <div class="text-center py-8 text-muted-foreground">
                            <p class="text-4xl mb-2">📜</p>
                            <p>No trade history</p>
                            <p class="text-sm">Your completed trades will appear here</p>
                        </div>
                    {:else}
                        <div class="overflow-x-auto">
                            <table class="w-full text-sm">
                                <thead>
                                    <tr class="border-b">
                                        <th class="text-left py-2">Resource</th>
                                        <th class="text-right py-2">Qty</th>
                                        <th class="text-right py-2">Price</th>
                                        <th class="text-right py-2">Total</th>
                                        <th class="text-right py-2">Date</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {#each tradeHistory as tx (tx.id)}
                                        <tr class="border-b">
                                            <td class="py-2">
                                                <span class="mr-1">{getResourceIcon(tx.resource_type)}</span>
                                                {tx.resource_type}
                                            </td>
                                            <td class="text-right py-2">{formatQuantity(tx.quantity)}</td>
                                            <td class="text-right py-2">{formatGold(tx.price_per_unit)}</td>
                                            <td class="text-right py-2 text-yellow-600">{formatGold(tx.total_price)}</td>
                                            <td class="text-right py-2 text-muted-foreground">
                                                {new Date(tx.created_at).toLocaleDateString()}
                                            </td>
                                        </tr>
                                    {/each}
                                </tbody>
                            </table>
                        </div>
                    {/if}
                </div>
            {/if}
        </div>

        <!-- Footer -->
        <Dialog.Footer class="mt-4">
            <div class="flex items-center gap-4 w-full">
                <div class="flex-1 text-sm text-muted-foreground">
                    <span class="text-yellow-600 font-medium">{formatGold(userGold)}</span> gold available
                </div>
                <Button variant="outline" onclick={() => open = false}>
                    Close
                </Button>
            </div>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
