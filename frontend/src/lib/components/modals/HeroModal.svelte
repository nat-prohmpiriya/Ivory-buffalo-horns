<script lang="ts">
    import { onMount } from 'svelte';
    import * as Dialog from '$lib/components/ui/dialog';
    import * as Tabs from '$lib/components/ui/tabs';
    import { Button } from '$lib/components/ui/button';
    import { Input } from '$lib/components/ui/input';
    import { Label } from '$lib/components/ui/label';
    import {
        heroStore,
        getStatusLabel,
        getStatusIcon,
        getStatusColor,
        getRarityColor,
        getRarityBg,
        getSlotIcon,
        getSlotLabel,
        getTribeIcon,
        formatHealthBar,
        formatExpBar,
        type HeroResponse,
        type ItemSlot,
    } from '$lib/stores/hero';
    import { villageStore } from '$lib/stores/village';

    interface Props {
        open?: boolean;
    }

    let { open = $bindable(false) }: Props = $props();

    let heroState = $state(heroStore);
    let heroes = $derived($heroState.heroes);
    let selectedHero = $derived($heroState.selectedHero);
    let inventory = $derived($heroState.inventory);
    let availableAdventures = $derived($heroState.availableAdventures);
    let activeAdventure = $derived($heroState.activeAdventure);
    let tavernHeroes = $derived($heroState.tavernHeroes);
    let totalSlots = $derived($heroState.totalSlots);
    let usedSlots = $derived($heroState.usedSlots);
    let nextSlotCost = $derived($heroState.nextSlotCost);
    let loading = $derived($heroState.loading);

    let villageState = $state(villageStore);
    let villages = $derived($villageState.villages);
    let currentVillage = $derived($villageState.currentVillage);

    let activeTab = $state('heroes');

    // Attribute assignment form
    let attrFighting = $state(0);
    let attrOff = $state(0);
    let attrDef = $state(0);
    let attrResources = $state(0);

    // Create hero form
    let newHeroName = $state('');
    let selectedDefinitionId = $state<string | null>(null);

    $effect(() => {
        if (open) {
            loadInitialData();
        }
    });

    $effect(() => {
        if (selectedHero) {
            attrFighting = selectedHero.fighting_strength;
            attrOff = selectedHero.off_bonus;
            attrDef = selectedHero.def_bonus;
            attrResources = selectedHero.resources_bonus;
        }
    });

    async function loadInitialData() {
        await heroStore.loadHeroes();
    }

    function handleTabChange(tab: string) {
        activeTab = tab;
        if (tab === 'inventory' && selectedHero) {
            heroStore.loadInventory(selectedHero.id);
        } else if (tab === 'adventures') {
            heroStore.loadAvailableAdventures();
            if (selectedHero) {
                heroStore.loadActiveAdventure(selectedHero.id);
            }
        } else if (tab === 'tavern') {
            heroStore.loadTavernHeroes();
        }
    }

    function selectHero(hero: HeroResponse) {
        heroStore.selectHero(hero);
        heroStore.loadHero(hero.id);
    }

    async function handleAssignAttributes() {
        if (!selectedHero) return;
        const totalAssigned = attrFighting + attrOff + attrDef + attrResources;
        const totalOriginal = selectedHero.fighting_strength + selectedHero.off_bonus +
                            selectedHero.def_bonus + selectedHero.resources_bonus;

        if (totalAssigned - totalOriginal > selectedHero.unassigned_points) {
            return;
        }

        await heroStore.assignAttributes(selectedHero.id, {
            fighting_strength: attrFighting,
            off_bonus: attrOff,
            def_bonus: attrDef,
            resources_bonus: attrResources,
        });
    }

    async function handleEquipItem(itemId: string) {
        if (!selectedHero) return;
        await heroStore.equipItem(selectedHero.id, itemId);
    }

    async function handleUnequipItem(slot: ItemSlot) {
        if (!selectedHero) return;
        await heroStore.unequipItem(selectedHero.id, slot);
    }

    async function handleUseItem(itemId: string) {
        if (!selectedHero) return;
        await heroStore.useItem(selectedHero.id, itemId);
    }

    async function handleStartAdventure(adventureId: string) {
        if (!selectedHero) return;
        await heroStore.startAdventure(selectedHero.id, adventureId);
        heroStore.loadAvailableAdventures();
    }

    async function handleCreateHero() {
        if (!currentVillage) return;
        await heroStore.createHero({
            name: newHeroName || undefined,
            hero_definition_id: selectedDefinitionId || undefined,
            home_village_id: currentVillage.id,
        });
        newHeroName = '';
        selectedDefinitionId = null;
        activeTab = 'heroes';
    }

    async function handleBuySlot() {
        await heroStore.buyHeroSlot();
        heroStore.loadHeroes();
    }

    function getPointsUsed(): number {
        if (!selectedHero) return 0;
        const totalOriginal = selectedHero.fighting_strength + selectedHero.off_bonus +
                            selectedHero.def_bonus + selectedHero.resources_bonus;
        const totalNew = attrFighting + attrOff + attrDef + attrResources;
        return totalNew - totalOriginal;
    }

    function getPointsRemaining(): number {
        if (!selectedHero) return 0;
        return selectedHero.unassigned_points - getPointsUsed();
    }

    const equipmentSlots: ItemSlot[] = ['helmet', 'weapon', 'armor_left', 'armor_right', 'boots', 'horse', 'bag', 'bandage'];
</script>

<Dialog.Root bind:open>
    <Dialog.Content class="max-w-3xl max-h-[85vh] overflow-y-auto">
        <Dialog.Header>
            <Dialog.Title class="flex items-center gap-2">
                <span class="text-xl">🦸</span>
                Heroes
                <span class="ml-auto text-sm font-normal text-muted-foreground">
                    Slots: {usedSlots}/{totalSlots}
                </span>
            </Dialog.Title>
            <Dialog.Description>
                Manage your heroes, equipment, and adventures
            </Dialog.Description>
        </Dialog.Header>

        <Tabs.Root value={activeTab} onValueChange={handleTabChange} class="mt-4">
            <Tabs.List class="grid w-full grid-cols-5">
                <Tabs.Trigger value="heroes">Heroes</Tabs.Trigger>
                <Tabs.Trigger value="attributes" disabled={!selectedHero}>Stats</Tabs.Trigger>
                <Tabs.Trigger value="inventory" disabled={!selectedHero}>Inventory</Tabs.Trigger>
                <Tabs.Trigger value="adventures">Adventures</Tabs.Trigger>
                <Tabs.Trigger value="tavern">Tavern</Tabs.Trigger>
            </Tabs.List>

            <!-- Heroes Tab -->
            <Tabs.Content value="heroes" class="space-y-4 mt-4">
                {#if heroes.length === 0}
                    <div class="text-center py-8 text-muted-foreground">
                        {loading ? 'Loading heroes...' : 'No heroes yet. Visit the Tavern to recruit one!'}
                    </div>
                {:else}
                    <div class="space-y-3">
                        {#each heroes as hero}
                            <button
                                class="w-full p-4 border rounded-lg text-left transition-colors hover:border-primary
                                       {selectedHero?.id === hero.id ? 'border-primary bg-primary/5' : ''}"
                                onclick={() => selectHero(hero)}
                            >
                                <div class="flex items-center gap-4">
                                    <!-- Hero Icon -->
                                    <div class="w-12 h-12 rounded-full bg-muted flex items-center justify-center text-2xl">
                                        {getTribeIcon(hero.tribe)}
                                    </div>

                                    <!-- Hero Info -->
                                    <div class="flex-1">
                                        <div class="flex items-center gap-2">
                                            <span class="font-semibold">{hero.name}</span>
                                            {#if hero.hero_definition}
                                                <span class="text-yellow-500 text-xs">
                                                    {hero.hero_definition.rarity_stars}
                                                </span>
                                            {/if}
                                            <span class="text-sm {getStatusColor(hero.status)}">
                                                {getStatusIcon(hero.status)} {getStatusLabel(hero.status)}
                                            </span>
                                        </div>
                                        <div class="text-sm text-muted-foreground">
                                            Level {hero.level} | ATK: {hero.total_attack} | DEF: {hero.total_defense}
                                        </div>
                                        <!-- Health bar -->
                                        <div class="mt-2 h-2 bg-gray-200 rounded-full overflow-hidden">
                                            <div
                                                class="h-full transition-all {formatHealthBar(hero.health).color}"
                                                style="width: {formatHealthBar(hero.health).percent}%"
                                            ></div>
                                        </div>
                                    </div>

                                    <!-- Quick Stats -->
                                    <div class="text-right text-sm">
                                        <div>HP: {hero.health}%</div>
                                        <div class="text-muted-foreground">
                                            EXP: {hero.experience}/{hero.experience_to_next}
                                        </div>
                                    </div>
                                </div>
                            </button>
                        {/each}
                    </div>
                {/if}

                {#if usedSlots < totalSlots}
                    <p class="text-sm text-muted-foreground text-center">
                        You have {totalSlots - usedSlots} empty slot(s). Visit the Tavern to recruit more heroes!
                    </p>
                {:else if nextSlotCost}
                    <div class="flex items-center justify-center gap-2">
                        <span class="text-sm text-muted-foreground">Need more slots?</span>
                        <Button size="sm" variant="outline" onclick={handleBuySlot} disabled={loading}>
                            Buy Slot ({nextSlotCost} Gold)
                        </Button>
                    </div>
                {/if}
            </Tabs.Content>

            <!-- Attributes Tab -->
            <Tabs.Content value="attributes" class="space-y-4 mt-4">
                {#if selectedHero}
                    <div class="grid grid-cols-2 gap-4">
                        <!-- Hero Summary -->
                        <div class="border rounded-lg p-4">
                            <h4 class="font-semibold mb-3">{selectedHero.name}</h4>
                            <div class="space-y-2 text-sm">
                                <div class="flex justify-between">
                                    <span>Level</span>
                                    <span class="font-semibold">{selectedHero.level}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span>Total Attack</span>
                                    <span class="font-semibold text-red-600">{selectedHero.total_attack}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span>Total Defense</span>
                                    <span class="font-semibold text-blue-600">{selectedHero.total_defense}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span>Off Bonus</span>
                                    <span>+{selectedHero.off_bonus_percent.toFixed(1)}%</span>
                                </div>
                                <div class="flex justify-between">
                                    <span>Def Bonus</span>
                                    <span>+{selectedHero.def_bonus_percent.toFixed(1)}%</span>
                                </div>
                            </div>

                            {#if selectedHero.active_bonuses.length > 0}
                                <div class="mt-4 pt-4 border-t">
                                    <h5 class="text-sm font-medium mb-2">Passive Bonuses</h5>
                                    {#each selectedHero.active_bonuses as bonus}
                                        <div class="text-xs text-muted-foreground">
                                            {bonus.description}
                                        </div>
                                    {/each}
                                </div>
                            {/if}
                        </div>

                        <!-- Attribute Assignment -->
                        <div class="border rounded-lg p-4">
                            <div class="flex items-center justify-between mb-3">
                                <h4 class="font-semibold">Assign Points</h4>
                                <span class="text-sm {getPointsRemaining() < 0 ? 'text-red-600' : 'text-green-600'}">
                                    {getPointsRemaining()} points left
                                </span>
                            </div>

                            <div class="space-y-3">
                                <div>
                                    <Label class="text-xs">Fighting Strength (+80 ATK each)</Label>
                                    <Input type="number" bind:value={attrFighting} min={0} />
                                </div>
                                <div>
                                    <Label class="text-xs">Off Bonus (+0.2% each)</Label>
                                    <Input type="number" bind:value={attrOff} min={0} />
                                </div>
                                <div>
                                    <Label class="text-xs">Def Bonus (+0.2% each)</Label>
                                    <Input type="number" bind:value={attrDef} min={0} />
                                </div>
                                <div>
                                    <Label class="text-xs">Resources Bonus</Label>
                                    <Input type="number" bind:value={attrResources} min={0} />
                                </div>
                            </div>

                            <Button
                                class="w-full mt-4"
                                onclick={handleAssignAttributes}
                                disabled={loading || getPointsRemaining() < 0 || getPointsUsed() === 0}
                            >
                                Apply Changes
                            </Button>
                        </div>
                    </div>
                {/if}
            </Tabs.Content>

            <!-- Inventory Tab -->
            <Tabs.Content value="inventory" class="space-y-4 mt-4">
                {#if selectedHero && inventory}
                    <!-- Equipped Items -->
                    <div class="border rounded-lg p-4">
                        <h4 class="font-semibold mb-3">Equipped Items</h4>
                        <div class="grid grid-cols-4 gap-2">
                            {#each equipmentSlots as slot}
                                {@const equipped = inventory.equipped[slot.replace('_', '') as keyof typeof inventory.equipped]}
                                <div class="border rounded p-2 text-center {equipped ? getRarityBg(equipped.item.rarity) : ''}">
                                    <div class="text-lg">{getSlotIcon(slot)}</div>
                                    <div class="text-xs text-muted-foreground">{getSlotLabel(slot)}</div>
                                    {#if equipped}
                                        <div class="text-xs font-medium truncate {getRarityColor(equipped.item.rarity)}">
                                            {equipped.item.name}
                                        </div>
                                        <Button
                                            size="sm"
                                            variant="ghost"
                                            class="text-xs h-6 px-2 mt-1"
                                            onclick={() => handleUnequipItem(slot)}
                                        >
                                            Unequip
                                        </Button>
                                    {:else}
                                        <div class="text-xs text-muted-foreground">Empty</div>
                                    {/if}
                                </div>
                            {/each}
                        </div>
                    </div>

                    <!-- Inventory Items -->
                    <div class="border rounded-lg p-4">
                        <div class="flex items-center justify-between mb-3">
                            <h4 class="font-semibold">Inventory</h4>
                            <span class="text-sm text-muted-foreground">
                                {inventory.used_slots}/{inventory.total_slots} slots
                            </span>
                        </div>

                        {#if inventory.items.length === 0}
                            <p class="text-center text-muted-foreground py-4">No items in inventory</p>
                        {:else}
                            <div class="grid grid-cols-2 gap-2 max-h-[200px] overflow-y-auto">
                                {#each inventory.items.filter(i => !i.is_equipped) as item}
                                    <div class="border rounded p-2 {getRarityBg(item.item.rarity)}">
                                        <div class="flex items-center gap-2">
                                            <span>{getSlotIcon(item.item.slot)}</span>
                                            <span class="text-sm font-medium {getRarityColor(item.item.rarity)}">
                                                {item.item.name}
                                            </span>
                                            {#if item.quantity > 1}
                                                <span class="text-xs">x{item.quantity}</span>
                                            {/if}
                                        </div>
                                        <div class="flex gap-1 mt-2">
                                            {#if item.item.is_consumable}
                                                <Button
                                                    size="sm"
                                                    variant="outline"
                                                    class="text-xs h-6 px-2"
                                                    onclick={() => handleUseItem(item.id)}
                                                >
                                                    Use
                                                </Button>
                                            {:else}
                                                <Button
                                                    size="sm"
                                                    variant="outline"
                                                    class="text-xs h-6 px-2"
                                                    onclick={() => handleEquipItem(item.id)}
                                                >
                                                    Equip
                                                </Button>
                                            {/if}
                                        </div>
                                    </div>
                                {/each}
                            </div>
                        {/if}
                    </div>
                {:else}
                    <p class="text-center text-muted-foreground py-8">Select a hero to view inventory</p>
                {/if}
            </Tabs.Content>

            <!-- Adventures Tab -->
            <Tabs.Content value="adventures" class="space-y-4 mt-4">
                {#if activeAdventure}
                    <div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
                        <div class="flex items-center gap-2 mb-2">
                            <span class="text-xl">⚔️</span>
                            <h4 class="font-semibold">Active Adventure</h4>
                        </div>
                        <p class="text-sm">
                            {activeAdventure.difficulty} adventure in progress
                        </p>
                        <p class="text-xs text-muted-foreground">
                            Ends at: {new Date(activeAdventure.ends_at).toLocaleString()}
                        </p>
                    </div>
                {/if}

                <div class="border rounded-lg p-4">
                    <h4 class="font-semibold mb-3">Available Adventures</h4>
                    {#if availableAdventures.length === 0}
                        <p class="text-center text-muted-foreground py-4">
                            {loading ? 'Loading...' : 'No adventures available'}
                        </p>
                    {:else}
                        <div class="space-y-2">
                            {#each availableAdventures as adventure}
                                <div class="flex items-center justify-between p-3 border rounded">
                                    <div>
                                        <div class="font-medium capitalize">{adventure.difficulty} Adventure</div>
                                        <div class="text-sm text-muted-foreground">
                                            Duration: {adventure.duration_range}
                                        </div>
                                        {#if adventure.potential_reward}
                                            <div class="text-xs text-green-600">
                                                Potential: {adventure.potential_reward}
                                            </div>
                                        {/if}
                                    </div>
                                    <Button
                                        size="sm"
                                        onclick={() => handleStartAdventure(adventure.id)}
                                        disabled={loading || !selectedHero || selectedHero.status !== 'idle' || activeAdventure !== null}
                                    >
                                        Start
                                    </Button>
                                </div>
                            {/each}
                        </div>
                    {/if}
                </div>

                {#if !selectedHero}
                    <p class="text-sm text-muted-foreground text-center">
                        Select a hero from the Heroes tab to start adventures
                    </p>
                {:else if selectedHero.status !== 'idle'}
                    <p class="text-sm text-yellow-600 text-center">
                        Hero is currently {getStatusLabel(selectedHero.status).toLowerCase()}
                    </p>
                {/if}
            </Tabs.Content>

            <!-- Tavern Tab -->
            <Tabs.Content value="tavern" class="space-y-4 mt-4">
                {#if usedSlots >= totalSlots}
                    <div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4 text-center">
                        <p class="text-sm">All hero slots are full.</p>
                        {#if nextSlotCost}
                            <Button size="sm" class="mt-2" onclick={handleBuySlot} disabled={loading}>
                                Buy New Slot ({nextSlotCost} Gold)
                            </Button>
                        {/if}
                    </div>
                {:else}
                    <!-- Quick Create -->
                    <div class="border rounded-lg p-4">
                        <h4 class="font-semibold mb-3">Create Custom Hero</h4>
                        <div class="flex gap-2">
                            <Input
                                placeholder="Hero name (optional)"
                                bind:value={newHeroName}
                            />
                            <Button onclick={handleCreateHero} disabled={loading || !currentVillage}>
                                Create
                            </Button>
                        </div>
                    </div>

                    <!-- Named Heroes -->
                    <div class="border rounded-lg p-4">
                        <h4 class="font-semibold mb-3">Legendary Heroes</h4>
                        {#if tavernHeroes.length === 0}
                            <p class="text-center text-muted-foreground py-4">
                                {loading ? 'Loading...' : 'No heroes available in tavern'}
                            </p>
                        {:else}
                            <div class="space-y-2 max-h-[300px] overflow-y-auto">
                                {#each tavernHeroes as def}
                                    <div class="flex items-center gap-3 p-3 border rounded hover:border-primary transition-colors">
                                        <div class="w-10 h-10 rounded-full bg-muted flex items-center justify-center">
                                            {getTribeIcon(def.tribe)}
                                        </div>
                                        <div class="flex-1">
                                            <div class="flex items-center gap-2">
                                                <span class="font-medium">{def.name}</span>
                                                <span class="text-yellow-500 text-sm">{def.rarity_stars}</span>
                                            </div>
                                            {#if def.passive_bonuses.length > 0}
                                                <div class="text-xs text-muted-foreground">
                                                    {def.passive_bonuses[0].description}
                                                </div>
                                            {/if}
                                        </div>
                                        <Button
                                            size="sm"
                                            onclick={() => {
                                                selectedDefinitionId = def.id;
                                                handleCreateHero();
                                            }}
                                            disabled={loading}
                                        >
                                            Recruit
                                        </Button>
                                    </div>
                                {/each}
                            </div>
                        {/if}
                    </div>
                {/if}
            </Tabs.Content>
        </Tabs.Root>
    </Dialog.Content>
</Dialog.Root>
