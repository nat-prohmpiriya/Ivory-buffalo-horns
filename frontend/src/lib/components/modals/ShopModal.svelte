<script lang="ts">
    import { onMount } from 'svelte';
    import * as Dialog from '$lib/components/ui/dialog';
    import * as Tabs from '$lib/components/ui/tabs';
    import { Button } from '$lib/components/ui/button';
    import { Input } from '$lib/components/ui/input';
    import { Label } from '$lib/components/ui/label';
    import {
        shopStore,
        formatPrice,
        formatGold,
        formatDate,
        getTimeRemaining,
        getTransactionTypeLabel,
        getTransactionTypeIcon,
        getTransactionStatusColor,
        type GoldPackage,
        type SubscriptionPrice,
        type TransactionResponse,
    } from '$lib/stores/shop';
    import { villageStore } from '$lib/stores/village';

    interface Props {
        open?: boolean;
    }

    let { open = $bindable(false) }: Props = $props();

    let shopState = $state(shopStore);
    let packages = $derived($shopState.packages);
    let subscriptionPrices = $derived($shopState.subscriptionPrices);
    let balance = $derived($shopState.balance);
    let transactions = $derived($shopState.transactions);
    let loading = $derived($shopState.loading);

    let villageState = $state(villageStore);
    let currentVillage = $derived($villageState.currentVillage);

    let activeTab = $state('gold');

    // NPC Merchant form
    let npcWood = $state(0);
    let npcClay = $state(0);
    let npcIron = $state(0);
    let npcCrop = $state(0);

    // Production bonus selected resource
    let selectedResource = $state<'wood' | 'clay' | 'iron' | 'crop'>('wood');

    // Feature costs (hardcoded for now, should come from API)
    const featureCosts = {
        npc_merchant: 3,
        production_bonus: 5,
        book_of_wisdom: 10,
    };

    $effect(() => {
        if (open) {
            loadInitialData();
        }
    });

    async function loadInitialData() {
        await Promise.all([
            shopStore.loadPackages(),
            shopStore.loadBalance(),
            shopStore.loadSubscriptionPrices(),
        ]);
    }

    async function handleBuyGold(pkg: GoldPackage) {
        const currentUrl = window.location.href;
        await shopStore.createCheckout({
            package_id: pkg.id,
            success_url: `${currentUrl}?payment=success`,
            cancel_url: `${currentUrl}?payment=cancelled`,
        });
    }

    async function handleBuySubscription(price: SubscriptionPrice) {
        await shopStore.buySubscription(price.duration_days);
    }

    async function handleUseNpcMerchant() {
        if (!currentVillage) return;
        await shopStore.useNpcMerchant({
            village_id: currentVillage.id,
            wood: npcWood,
            clay: npcClay,
            iron: npcIron,
            crop: npcCrop,
        });
        // Reset form
        npcWood = 0;
        npcClay = 0;
        npcIron = 0;
        npcCrop = 0;
        // Reload village
        villageStore.loadVillage(currentVillage.id);
    }

    async function handleUseProductionBonus() {
        if (!currentVillage) return;
        await shopStore.useProductionBonus({
            village_id: currentVillage.id,
            resource_type: selectedResource,
        });
    }

    async function handleUseBookOfWisdom() {
        if (!currentVillage) return;
        await shopStore.useBookOfWisdom({
            village_id: currentVillage.id,
        });
    }

    function handleTabChange(tab: string) {
        activeTab = tab;
        if (tab === 'history') {
            shopStore.loadTransactions();
        }
    }

    function getTotalResources(): number {
        if (!currentVillage) return 0;
        return Math.floor(currentVillage.wood + currentVillage.clay + currentVillage.iron + currentVillage.crop);
    }

    function getNpcTotal(): number {
        return npcWood + npcClay + npcIron + npcCrop;
    }
</script>

<Dialog.Root bind:open>
    <Dialog.Content class="max-w-2xl max-h-[85vh] overflow-y-auto">
        <Dialog.Header>
            <Dialog.Title class="flex items-center gap-2">
                <span class="text-xl">🏪</span>
                Gold Shop
                {#if balance}
                    <span class="ml-auto text-sm font-normal text-muted-foreground">
                        Balance: <span class="text-yellow-600 font-semibold">{formatGold(balance.gold_balance)} Gold</span>
                    </span>
                {/if}
            </Dialog.Title>
            <Dialog.Description>
                Purchase gold, activate Travian Plus, or use premium features
            </Dialog.Description>
        </Dialog.Header>

        <Tabs.Root value={activeTab} onValueChange={handleTabChange} class="mt-4">
            <Tabs.List class="grid w-full grid-cols-4">
                <Tabs.Trigger value="gold">Buy Gold</Tabs.Trigger>
                <Tabs.Trigger value="plus">Travian Plus</Tabs.Trigger>
                <Tabs.Trigger value="features">Features</Tabs.Trigger>
                <Tabs.Trigger value="history">History</Tabs.Trigger>
            </Tabs.List>

            <!-- Buy Gold Tab -->
            <Tabs.Content value="gold" class="space-y-4 mt-4">
                {#if packages.length === 0}
                    <div class="text-center py-8 text-muted-foreground">
                        {loading ? 'Loading packages...' : 'No packages available'}
                    </div>
                {:else}
                    <div class="grid grid-cols-2 gap-4">
                        {#each packages as pkg}
                            <div class="border rounded-lg p-4 hover:border-yellow-500 transition-colors">
                                <div class="flex items-center justify-between mb-2">
                                    <span class="text-2xl">💰</span>
                                    {#if pkg.bonus_percent > 0}
                                        <span class="bg-green-100 text-green-800 text-xs px-2 py-0.5 rounded">
                                            +{pkg.bonus_percent}% Bonus
                                        </span>
                                    {/if}
                                </div>
                                <h4 class="font-semibold">{pkg.name}</h4>
                                <p class="text-2xl font-bold text-yellow-600 my-2">
                                    {formatGold(pkg.gold_amount)} Gold
                                </p>
                                <p class="text-sm text-muted-foreground mb-3">
                                    {formatPrice(pkg.price_cents, pkg.currency)}
                                </p>
                                <Button
                                    class="w-full"
                                    onclick={() => handleBuyGold(pkg)}
                                    disabled={loading}
                                >
                                    Buy Now
                                </Button>
                            </div>
                        {/each}
                    </div>
                {/if}
            </Tabs.Content>

            <!-- Travian Plus Tab -->
            <Tabs.Content value="plus" class="space-y-4 mt-4">
                <!-- Current Status -->
                {#if balance?.has_plus}
                    <div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
                        <div class="flex items-center gap-2">
                            <span class="text-2xl">⭐</span>
                            <div>
                                <p class="font-semibold text-yellow-800">Travian Plus Active</p>
                                {#if balance.plus_expires_at}
                                    <p class="text-sm text-yellow-700">
                                        Expires in: {getTimeRemaining(balance.plus_expires_at)}
                                    </p>
                                {/if}
                            </div>
                        </div>
                    </div>
                {:else}
                    <div class="bg-gray-50 border rounded-lg p-4">
                        <p class="text-muted-foreground">You don't have Travian Plus active</p>
                    </div>
                {/if}

                <!-- Benefits -->
                <div class="border rounded-lg p-4">
                    <h4 class="font-semibold mb-3">Travian Plus Benefits</h4>
                    <ul class="space-y-2 text-sm">
                        <li class="flex items-center gap-2">
                            <span class="text-green-600">✓</span>
                            +25% Resource Production
                        </li>
                        <li class="flex items-center gap-2">
                            <span class="text-green-600">✓</span>
                            Extended Building Queue
                        </li>
                        <li class="flex items-center gap-2">
                            <span class="text-green-600">✓</span>
                            Detailed Battle Reports
                        </li>
                        <li class="flex items-center gap-2">
                            <span class="text-green-600">✓</span>
                            Auto-Extend Protection
                        </li>
                    </ul>
                </div>

                <!-- Subscription Options -->
                {#if subscriptionPrices.length === 0}
                    <div class="text-center py-4 text-muted-foreground">
                        {loading ? 'Loading prices...' : 'No subscription options available'}
                    </div>
                {:else}
                    <div class="grid grid-cols-3 gap-4">
                        {#each subscriptionPrices as price}
                            <div class="border rounded-lg p-4 text-center hover:border-yellow-500 transition-colors">
                                <p class="text-sm text-muted-foreground">
                                    {price.duration_days} Days
                                </p>
                                <p class="text-xl font-bold text-yellow-600 my-2">
                                    {formatGold(price.gold_cost)} Gold
                                </p>
                                <p class="text-xs text-muted-foreground mb-3">
                                    {(price.gold_cost / price.duration_days).toFixed(1)} gold/day
                                </p>
                                <Button
                                    size="sm"
                                    class="w-full"
                                    onclick={() => handleBuySubscription(price)}
                                    disabled={loading || !balance || balance.gold_balance < price.gold_cost}
                                >
                                    Activate
                                </Button>
                            </div>
                        {/each}
                    </div>
                {/if}
            </Tabs.Content>

            <!-- Features Tab -->
            <Tabs.Content value="features" class="space-y-4 mt-4">
                <!-- NPC Merchant -->
                <div class="border rounded-lg p-4">
                    <div class="flex items-center gap-2 mb-3">
                        <span class="text-xl">🏪</span>
                        <h4 class="font-semibold">NPC Merchant</h4>
                        <span class="ml-auto text-sm text-yellow-600">
                            {featureCosts.npc_merchant} Gold
                        </span>
                    </div>
                    <p class="text-sm text-muted-foreground mb-3">
                        Redistribute your resources. Total must equal current resources ({getTotalResources()}).
                    </p>
                    <div class="grid grid-cols-4 gap-2 mb-3">
                        <div>
                            <Label class="text-xs">Wood</Label>
                            <Input type="number" bind:value={npcWood} min={0} />
                        </div>
                        <div>
                            <Label class="text-xs">Clay</Label>
                            <Input type="number" bind:value={npcClay} min={0} />
                        </div>
                        <div>
                            <Label class="text-xs">Iron</Label>
                            <Input type="number" bind:value={npcIron} min={0} />
                        </div>
                        <div>
                            <Label class="text-xs">Crop</Label>
                            <Input type="number" bind:value={npcCrop} min={0} />
                        </div>
                    </div>
                    <div class="flex items-center justify-between">
                        <span class="text-sm">
                            Total: {getNpcTotal()} / {getTotalResources()}
                        </span>
                        <Button
                            size="sm"
                            onclick={handleUseNpcMerchant}
                            disabled={loading || getNpcTotal() !== getTotalResources() || !balance || balance.gold_balance < featureCosts.npc_merchant}
                        >
                            Use NPC Merchant
                        </Button>
                    </div>
                </div>

                <!-- Production Bonus -->
                <div class="border rounded-lg p-4">
                    <div class="flex items-center gap-2 mb-3">
                        <span class="text-xl">📈</span>
                        <h4 class="font-semibold">Production Bonus</h4>
                        <span class="ml-auto text-sm text-yellow-600">
                            {featureCosts.production_bonus} Gold
                        </span>
                    </div>
                    <p class="text-sm text-muted-foreground mb-3">
                        +25% production for selected resource for 24 hours
                    </p>
                    <div class="flex items-center gap-2">
                        <select
                            bind:value={selectedResource}
                            class="flex-1 h-9 rounded-md border border-input bg-background px-3 text-sm"
                        >
                            <option value="wood">🪵 Wood</option>
                            <option value="clay">🧱 Clay</option>
                            <option value="iron">⛏️ Iron</option>
                            <option value="crop">🌾 Crop</option>
                        </select>
                        <Button
                            size="sm"
                            onclick={handleUseProductionBonus}
                            disabled={loading || !balance || balance.gold_balance < featureCosts.production_bonus}
                        >
                            Activate
                        </Button>
                    </div>
                </div>

                <!-- Book of Wisdom -->
                <div class="border rounded-lg p-4">
                    <div class="flex items-center gap-2 mb-3">
                        <span class="text-xl">📚</span>
                        <h4 class="font-semibold">Book of Wisdom</h4>
                        <span class="ml-auto text-sm text-yellow-600">
                            {featureCosts.book_of_wisdom} Gold
                        </span>
                    </div>
                    <p class="text-sm text-muted-foreground mb-3">
                        2x production for ALL resources for 4 hours
                    </p>
                    <Button
                        size="sm"
                        onclick={handleUseBookOfWisdom}
                        disabled={loading || !balance || balance.gold_balance < featureCosts.book_of_wisdom}
                    >
                        Activate Book of Wisdom
                    </Button>
                </div>
            </Tabs.Content>

            <!-- History Tab -->
            <Tabs.Content value="history" class="mt-4">
                {#if transactions.length === 0}
                    <div class="text-center py-8 text-muted-foreground">
                        {loading ? 'Loading transactions...' : 'No transactions yet'}
                    </div>
                {:else}
                    <div class="space-y-2 max-h-[400px] overflow-y-auto">
                        {#each transactions as tx}
                            <div class="flex items-center gap-3 p-3 border rounded-lg">
                                <span class="text-xl">
                                    {getTransactionTypeIcon(tx.transaction_type)}
                                </span>
                                <div class="flex-1">
                                    <p class="font-medium text-sm">
                                        {getTransactionTypeLabel(tx.transaction_type)}
                                    </p>
                                    {#if tx.description}
                                        <p class="text-xs text-muted-foreground">
                                            {tx.description}
                                        </p>
                                    {/if}
                                    <p class="text-xs text-muted-foreground">
                                        {formatDate(tx.created_at)}
                                    </p>
                                </div>
                                <div class="text-right">
                                    <p class="font-semibold {tx.gold_amount >= 0 ? 'text-green-600' : 'text-red-600'}">
                                        {tx.gold_amount >= 0 ? '+' : ''}{formatGold(tx.gold_amount)}
                                    </p>
                                    <p class="text-xs {getTransactionStatusColor(tx.status)}">
                                        {tx.status}
                                    </p>
                                </div>
                            </div>
                        {/each}
                    </div>
                {/if}
            </Tabs.Content>
        </Tabs.Root>
    </Dialog.Content>
</Dialog.Root>
