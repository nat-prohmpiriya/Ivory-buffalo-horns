<script lang="ts">
    import { onMount } from 'svelte';
    import * as Dialog from '$lib/components/ui/dialog';
    import { Button } from '$lib/components/ui/button';
    import { Separator } from '$lib/components/ui/separator';
    import TroopCard from '$lib/components/game/TroopCard.svelte';
    import TrainingQueue from '$lib/components/game/TrainingQueue.svelte';
    import {
        troopStore,
        type TroopDefinition,
        type Troop,
        type TroopQueueItem,
        type TroopType,
    } from '$lib/stores/troop';
    import type { BuildingType } from '$lib/stores/village';

    interface VillageResources {
        wood: number;
        clay: number;
        iron: number;
        crop: number;
    }

    interface Props {
        open: boolean;
        villageId: string;
        buildingType: BuildingType;
        buildingLevel: number;
        villageResources: VillageResources;
    }

    let { open = $bindable(false), villageId, buildingType, buildingLevel, villageResources }: Props = $props();

    let definitions = $state<TroopDefinition[]>([]);
    let troops = $state<Troop[]>([]);
    let queue = $state<TroopQueueItem[]>([]);
    let loading = $state(false);
    let trainingTroopType = $state<TroopType | null>(null);

    // Building info for header
    const buildingInfo: Record<string, { name: string; icon: string; description: string }> = {
        barracks: {
            name: 'Barracks',
            icon: '⚔️',
            description: 'Train infantry units to defend and attack.',
        },
        stable: {
            name: 'Stable',
            icon: '🐎',
            description: 'Train cavalry and mounted units.',
        },
        workshop: {
            name: 'Workshop',
            icon: '🔧',
            description: 'Build siege weapons and special units.',
        },
    };

    // Map building type to required building name for filtering
    const buildingTypeToRequired: Record<string, string[]> = {
        barracks: ['barracks'],
        stable: ['stable'],
        workshop: ['workshop'],
    };

    // Filter definitions based on building type and level
    const availableDefinitions = $derived(
        definitions.filter((def) => {
            const requiredBuildings = buildingTypeToRequired[buildingType] || [];
            return (
                requiredBuildings.includes(def.required_building) &&
                def.required_building_level <= buildingLevel
            );
        })
    );

    // Get troop count for a specific type
    function getTroopByType(type: TroopType): Troop | undefined {
        return troops.find((t) => t.troop_type === type);
    }

    // Check if can train (has required building level)
    function canTrainType(def: TroopDefinition): boolean {
        return def.required_building_level <= buildingLevel;
    }

    // Load data when modal opens
    async function loadData() {
        if (!villageId) return;

        loading = true;
        try {
            await Promise.all([
                troopStore.loadDefinitions(),
                troopStore.loadTroops(villageId),
                troopStore.loadQueue(villageId),
            ]);

            // Subscribe to store updates
            troopStore.subscribe((state) => {
                definitions = state.definitions;
                troops = state.troops;
                queue = state.queue;
            });
        } finally {
            loading = false;
        }
    }

    // Handle training
    async function handleTrain(troopType: TroopType, count: number) {
        if (!villageId) return;

        trainingTroopType = troopType;
        try {
            await troopStore.train(villageId, troopType, count);
        } finally {
            trainingTroopType = null;
        }
    }

    // Handle cancel training
    async function handleCancelTraining(queueId: string) {
        if (!villageId) return;

        try {
            await troopStore.cancelTraining(villageId, queueId);
        } catch {
            // Error handled in store
        }
    }

    // Load data when modal opens
    $effect(() => {
        if (open) {
            loadData();
        }
    });

    const info = $derived(buildingInfo[buildingType] || buildingInfo.barracks);
</script>

<Dialog.Root bind:open>
    <Dialog.Content class="sm:max-w-2xl max-h-[90vh] overflow-y-auto">
        <Dialog.Header>
            <div class="flex items-center gap-3">
                <div class="w-12 h-12 rounded-xl bg-muted flex items-center justify-center text-2xl">
                    {info.icon}
                </div>
                <div>
                    <Dialog.Title class="text-xl">{info.name}</Dialog.Title>
                    <Dialog.Description>
                        Level {buildingLevel} - {info.description}
                    </Dialog.Description>
                </div>
            </div>
        </Dialog.Header>

        <div class="space-y-4 py-4">
            {#if loading}
                <div class="flex items-center justify-center py-8">
                    <span class="animate-spin text-2xl mr-2">⏳</span>
                    <span>Loading troops...</span>
                </div>
            {:else}
                <!-- Training Queue Section -->
                {#if queue.length > 0}
                    <TrainingQueue
                        {queue}
                        onCancel={handleCancelTraining}
                        loading={loading}
                    />
                    <Separator />
                {/if}

                <!-- Available Troops -->
                <div>
                    <h3 class="font-semibold mb-3">Available Troops</h3>

                    {#if availableDefinitions.length > 0}
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            {#each availableDefinitions as definition}
                                <TroopCard
                                    {definition}
                                    troop={getTroopByType(definition.troop_type)}
                                    {villageResources}
                                    canTrain={canTrainType(definition)}
                                    onTrain={(count) => handleTrain(definition.troop_type, count)}
                                    loading={trainingTroopType === definition.troop_type}
                                />
                            {/each}
                        </div>
                    {:else}
                        <div class="text-center py-8 text-muted-foreground">
                            <p class="text-4xl mb-2">🔒</p>
                            <p>No troops available at this level.</p>
                            <p class="text-sm mt-1">Upgrade the {info.name} to unlock more troops.</p>
                        </div>
                    {/if}
                </div>

                <!-- Locked Troops Preview -->
                {#if definitions.length > 0}
                    {@const lockedDefinitions = definitions.filter((def) => {
                        const requiredBuildings = buildingTypeToRequired[buildingType] || [];
                        return (
                            requiredBuildings.includes(def.required_building) &&
                            def.required_building_level > buildingLevel
                        );
                    })}

                    {#if lockedDefinitions.length > 0}
                        <Separator />
                        <div>
                            <h3 class="font-semibold mb-3 text-muted-foreground">Locked Troops</h3>
                            <div class="grid grid-cols-2 md:grid-cols-3 gap-2">
                                {#each lockedDefinitions as def}
                                    <div class="p-3 rounded-lg bg-muted/50 opacity-60">
                                        <div class="flex items-center gap-2">
                                            <span class="text-xl">🔒</span>
                                            <div>
                                                <p class="text-sm font-medium">
                                                    {def.name || def.troop_type.split('_').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ')}
                                                </p>
                                                <p class="text-xs text-muted-foreground">
                                                    Level {def.required_building_level} required
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                {/each}
                            </div>
                        </div>
                    {/if}
                {/if}
            {/if}
        </div>

        <Dialog.Footer>
            <Button variant="outline" onclick={() => (open = false)}>Close</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
