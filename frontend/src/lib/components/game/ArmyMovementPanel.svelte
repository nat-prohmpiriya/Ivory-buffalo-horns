<script lang="ts">
    import { onMount } from 'svelte';
    import { Card } from '$lib/components/ui/card';
    import {
        armyStore,
        type Army,
        getMissionIcon,
        getMissionLabel,
        getTotalTroops,
    } from '$lib/stores/army';

    interface Props {
        villageId: string;
    }

    let { villageId }: Props = $props();

    // Store state
    let armyState = $state(armyStore);
    let outgoingArmies = $derived($armyState.outgoingArmies);
    let incomingArmies = $derived($armyState.incomingArmies);
    let loading = $derived($armyState.loading);

    // Timer for countdown updates
    let now = $state(Date.now());

    // Load armies when villageId changes
    $effect(() => {
        if (villageId) {
            loadArmies();
        }
    });

    async function loadArmies() {
        try {
            await Promise.all([
                armyStore.loadOutgoing(villageId),
                armyStore.loadIncoming(villageId),
            ]);
        } catch (error) {
            console.error('Failed to load armies:', error);
        }
    }

    // Update timer every second
    onMount(() => {
        const interval = setInterval(() => {
            now = Date.now();
        }, 1000);

        return () => clearInterval(interval);
    });

    // Format countdown time
    function formatCountdown(targetTime: string): string {
        const target = new Date(targetTime).getTime();
        const diff = target - now;

        if (diff <= 0) return 'Arrived';

        const hours = Math.floor(diff / (1000 * 60 * 60));
        const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
        const seconds = Math.floor((diff % (1000 * 60)) / 1000);

        if (hours > 0) {
            return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
        }
        return `${minutes}:${seconds.toString().padStart(2, '0')}`;
    }

    // Get arrival time based on direction
    function getArrivalTime(army: Army): string {
        if (army.is_returning && army.returns_at) {
            return army.returns_at;
        }
        return army.arrives_at;
    }

    // Check if army has arrived
    function hasArrived(army: Army): boolean {
        const targetTime = getArrivalTime(army);
        return new Date(targetTime).getTime() <= now;
    }

    // Get status text
    function getStatusText(army: Army): string {
        if (army.is_returning) {
            return 'Returning';
        }
        return 'En route';
    }

    // Filter active armies (not yet arrived)
    const activeOutgoing = $derived(
        outgoingArmies.filter(a => !hasArrived(a))
    );
    const activeIncoming = $derived(
        incomingArmies.filter(a => !hasArrived(a))
    );

    const totalActive = $derived(activeOutgoing.length + activeIncoming.length);
</script>

<Card class="p-4">
    <h3 class="font-semibold mb-3 flex items-center gap-2">
        <span>⚔️</span>
        Army Movements
        {#if totalActive > 0}
            <span class="px-1.5 py-0.5 text-xs bg-primary text-primary-foreground rounded-full">
                {totalActive}
            </span>
        {/if}
    </h3>

    {#if loading && outgoingArmies.length === 0 && incomingArmies.length === 0}
        <div class="text-center py-4">
            <span class="animate-spin text-lg">⏳</span>
        </div>
    {:else if totalActive === 0}
        <p class="text-sm text-muted-foreground text-center py-4">
            No armies in movement
        </p>
    {:else}
        <div class="space-y-4">
            <!-- Outgoing Armies -->
            {#if activeOutgoing.length > 0}
                <div>
                    <p class="text-xs text-muted-foreground mb-2 flex items-center gap-1">
                        <span>📤</span>
                        Outgoing ({activeOutgoing.length})
                    </p>
                    <div class="space-y-2">
                        {#each activeOutgoing as army (army.id)}
                            <div class="p-2 bg-muted rounded-lg">
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center gap-2">
                                        <span class="text-lg">{getMissionIcon(army.mission)}</span>
                                        <div>
                                            <p class="text-sm font-medium">
                                                {getMissionLabel(army.mission)}
                                                {#if army.is_returning}
                                                    <span class="text-xs text-muted-foreground">(returning)</span>
                                                {/if}
                                            </p>
                                            <p class="text-xs text-muted-foreground">
                                                → ({army.to_x}|{army.to_y})
                                            </p>
                                        </div>
                                    </div>
                                    <div class="text-right">
                                        <p class="text-sm font-mono text-amber-600">
                                            {formatCountdown(getArrivalTime(army))}
                                        </p>
                                        <p class="text-xs text-muted-foreground">
                                            🪖 {getTotalTroops(army.troops)}
                                        </p>
                                    </div>
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}

            <!-- Incoming Armies -->
            {#if activeIncoming.length > 0}
                <div>
                    <p class="text-xs text-muted-foreground mb-2 flex items-center gap-1">
                        <span>📥</span>
                        Incoming ({activeIncoming.length})
                    </p>
                    <div class="space-y-2">
                        {#each activeIncoming as army (army.id)}
                            <div class="p-2 bg-muted rounded-lg border-l-2 border-l-destructive">
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center gap-2">
                                        <span class="text-lg">{getMissionIcon(army.mission)}</span>
                                        <div>
                                            <p class="text-sm font-medium text-destructive">
                                                {getMissionLabel(army.mission)}
                                            </p>
                                            <p class="text-xs text-muted-foreground">
                                                ← from ({army.to_x}|{army.to_y})
                                            </p>
                                        </div>
                                    </div>
                                    <div class="text-right">
                                        <p class="text-sm font-mono text-red-600">
                                            {formatCountdown(army.arrives_at)}
                                        </p>
                                        <p class="text-xs text-muted-foreground">
                                            🪖 ?
                                        </p>
                                    </div>
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}
        </div>
    {/if}
</Card>
