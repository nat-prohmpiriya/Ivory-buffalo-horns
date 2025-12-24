<script lang="ts">
    import { Card } from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import { Input } from '$lib/components/ui/input';
    import {
        type TroopDefinition,
        type Troop,
        formatTroopName,
        getTroopIcon,
        getTribeColor,
        formatTime
    } from '$lib/stores/troop';

    interface VillageResources {
        wood: number;
        clay: number;
        iron: number;
        crop: number;
    }

    interface Props {
        definition: TroopDefinition;
        troop?: Troop;
        villageResources: VillageResources;
        canTrain: boolean;
        onTrain?: (count: number) => void;
        loading?: boolean;
    }

    let { definition, troop, villageResources, canTrain, onTrain, loading = false }: Props = $props();

    let trainCount = $state(1);

    const icon = $derived(getTroopIcon(definition.troop_type));
    const tribeColor = $derived(getTribeColor(definition.tribe));
    const currentCount = $derived(troop?.count || 0);
    const inVillage = $derived(troop?.in_village || 0);
    const onMission = $derived(troop?.on_mission || 0);

    const totalCost = $derived({
        wood: definition.wood_cost * trainCount,
        clay: definition.clay_cost * trainCount,
        iron: definition.iron_cost * trainCount,
        crop: definition.crop_cost * trainCount,
        time: definition.training_time_seconds * trainCount,
    });

    const canAfford = $derived(
        villageResources.wood >= totalCost.wood &&
        villageResources.clay >= totalCost.clay &&
        villageResources.iron >= totalCost.iron &&
        villageResources.crop >= totalCost.crop
    );

    const maxTrainable = $derived(
        Math.min(
            Math.floor(villageResources.wood / definition.wood_cost),
            Math.floor(villageResources.clay / definition.clay_cost),
            Math.floor(villageResources.iron / definition.iron_cost),
            Math.floor(villageResources.crop / definition.crop_cost)
        )
    );

    function handleTrain() {
        if (onTrain && trainCount > 0 && canAfford && canTrain) {
            onTrain(trainCount);
            trainCount = 1;
        }
    }

    function setMax() {
        trainCount = Math.max(1, maxTrainable);
    }
</script>

<Card class="p-4 {!canTrain ? 'opacity-60' : ''}">
    <!-- Header -->
    <div class="flex items-center gap-3 mb-3">
        <div class="w-12 h-12 rounded-lg bg-muted flex items-center justify-center text-2xl">
            {icon}
        </div>
        <div class="flex-1">
            <h3 class="font-semibold">{formatTroopName(definition.troop_type)}</h3>
            <p class="text-xs {tribeColor} capitalize">{definition.tribe}</p>
        </div>
        {#if currentCount > 0}
            <div class="text-right">
                <p class="font-bold text-lg">{currentCount}</p>
                <p class="text-xs text-muted-foreground">
                    {inVillage} here {#if onMission > 0}/ {onMission} away{/if}
                </p>
            </div>
        {/if}
    </div>

    <!-- Stats -->
    <div class="grid grid-cols-3 gap-2 text-xs mb-3">
        <div class="flex items-center gap-1" title="Attack">
            <span>⚔️</span>
            <span class="font-medium">{definition.attack}</span>
        </div>
        <div class="flex items-center gap-1" title="Defense vs Infantry">
            <span>🛡️</span>
            <span class="font-medium">{definition.defense_infantry}</span>
        </div>
        <div class="flex items-center gap-1" title="Defense vs Cavalry">
            <span>🐎</span>
            <span class="font-medium">{definition.defense_cavalry}</span>
        </div>
        <div class="flex items-center gap-1" title="Speed">
            <span>💨</span>
            <span class="font-medium">{definition.speed}/h</span>
        </div>
        <div class="flex items-center gap-1" title="Carry Capacity">
            <span>📦</span>
            <span class="font-medium">{definition.carry_capacity}</span>
        </div>
        <div class="flex items-center gap-1" title="Crop Consumption">
            <span>🌾</span>
            <span class="font-medium">{definition.crop_consumption}/h</span>
        </div>
    </div>

    {#if canTrain}
        <!-- Training Cost -->
        <div class="border-t pt-3 mt-3">
            <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-medium">Train</span>
                <div class="flex items-center gap-2">
                    <Input
                        type="number"
                        min={1}
                        max={maxTrainable}
                        bind:value={trainCount}
                        class="w-16 h-8 text-center"
                    />
                    <Button variant="outline" size="sm" onclick={setMax} class="h-8 px-2">
                        Max
                    </Button>
                </div>
            </div>

            <!-- Cost display -->
            <div class="grid grid-cols-4 gap-1 text-xs mb-3">
                <div class="flex items-center gap-1 {villageResources.wood < totalCost.wood ? 'text-destructive' : ''}">
                    <span>🪵</span>
                    <span>{totalCost.wood}</span>
                </div>
                <div class="flex items-center gap-1 {villageResources.clay < totalCost.clay ? 'text-destructive' : ''}">
                    <span>🧱</span>
                    <span>{totalCost.clay}</span>
                </div>
                <div class="flex items-center gap-1 {villageResources.iron < totalCost.iron ? 'text-destructive' : ''}">
                    <span>⛏️</span>
                    <span>{totalCost.iron}</span>
                </div>
                <div class="flex items-center gap-1 {villageResources.crop < totalCost.crop ? 'text-destructive' : ''}">
                    <span>🌾</span>
                    <span>{totalCost.crop}</span>
                </div>
            </div>

            <div class="flex items-center justify-between">
                <span class="text-xs text-muted-foreground">
                    ⏱️ {formatTime(totalCost.time)}
                </span>
                <Button
                    size="sm"
                    disabled={!canAfford || loading || trainCount < 1}
                    onclick={handleTrain}
                >
                    {#if loading}
                        Training...
                    {:else if canAfford}
                        Train {trainCount}
                    {:else}
                        Not enough resources
                    {/if}
                </Button>
            </div>
        </div>
    {:else}
        <div class="border-t pt-3 mt-3">
            <p class="text-xs text-muted-foreground text-center">
                Requires {definition.required_building} Level {definition.required_building_level}
            </p>
        </div>
    {/if}
</Card>
