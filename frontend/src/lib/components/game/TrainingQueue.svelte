<script lang="ts">
    import { onMount } from 'svelte';
    import { Card } from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import {
        type TroopQueueItem,
        formatTroopName,
        getTroopIcon,
    } from '$lib/stores/troop';

    interface Props {
        queue: TroopQueueItem[];
        onCancel?: (queueId: string) => void;
        loading?: boolean;
    }

    let { queue, onCancel, loading = false }: Props = $props();

    let now = $state(Date.now());

    // Update timer every second
    onMount(() => {
        const interval = setInterval(() => {
            now = Date.now();
        }, 1000);

        return () => clearInterval(interval);
    });

    function formatTimeRemaining(endsAt: string): string {
        const endTime = new Date(endsAt).getTime();
        const diff = Math.max(0, endTime - now);

        if (diff <= 0) return 'Done!';

        const hours = Math.floor(diff / 3600000);
        const minutes = Math.floor((diff % 3600000) / 60000);
        const seconds = Math.floor((diff % 60000) / 1000);

        if (hours > 0) {
            return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
        }
        return `${minutes}:${seconds.toString().padStart(2, '0')}`;
    }

    function getProgress(startedAt: string, endsAt: string): number {
        const start = new Date(startedAt).getTime();
        const end = new Date(endsAt).getTime();
        const total = end - start;
        const elapsed = now - start;
        return Math.min(100, Math.max(0, (elapsed / total) * 100));
    }

    function canCancel(startedAt: string): boolean {
        // Can only cancel if not yet started
        return new Date(startedAt).getTime() > now;
    }
</script>

<Card class="p-4">
    <h3 class="font-semibold mb-3 flex items-center gap-2">
        <span>⚔️</span>
        Training Queue
    </h3>

    {#if queue.length > 0}
        <div class="space-y-3">
            {#each queue as item, index}
                {@const icon = getTroopIcon(item.troop_type)}
                {@const progress = getProgress(item.started_at, item.ends_at)}
                {@const timeRemaining = formatTimeRemaining(item.ends_at)}
                {@const isActive = index === 0 || new Date(item.started_at).getTime() <= now}

                <div class="flex items-center gap-3 p-2 bg-muted rounded-lg">
                    <div class="w-10 h-10 rounded-lg bg-background flex items-center justify-center text-xl">
                        {icon}
                    </div>

                    <div class="flex-1 min-w-0">
                        <div class="flex items-center justify-between">
                            <p class="font-medium text-sm truncate">
                                {item.count}x {formatTroopName(item.troop_type)}
                            </p>
                            <span class="text-sm font-mono {isActive ? 'text-amber-600' : 'text-muted-foreground'}">
                                {timeRemaining}
                            </span>
                        </div>

                        {#if isActive}
                            <!-- Progress bar -->
                            <div class="w-full bg-muted-foreground/20 rounded-full h-1.5 mt-1">
                                <div
                                    class="bg-amber-500 h-1.5 rounded-full transition-all duration-1000"
                                    style="width: {progress}%"
                                ></div>
                            </div>
                            <p class="text-xs text-emerald-600 mt-1">Training...</p>
                        {:else}
                            <p class="text-xs text-muted-foreground mt-1">Queued</p>
                        {/if}
                    </div>

                    {#if canCancel(item.started_at) && onCancel}
                        <Button
                            variant="ghost"
                            size="sm"
                            class="text-destructive hover:text-destructive"
                            disabled={loading}
                            onclick={() => onCancel(item.id)}
                        >
                            Cancel
                        </Button>
                    {/if}
                </div>
            {/each}
        </div>
    {:else}
        <p class="text-sm text-muted-foreground text-center py-4">
            No troops in training
        </p>
    {/if}
</Card>
