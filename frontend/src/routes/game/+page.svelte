<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { Button } from '$lib/components/ui/button';
  import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
  import { api } from '$lib/api/client';
  import { villageStore, formatBuildingType } from '$lib/stores/village';

  interface ProductionRates {
    wood_per_hour: number;
    clay_per_hour: number;
    iron_per_hour: number;
    crop_per_hour: number;
    crop_consumption: number;
    net_crop_per_hour: number;
  }

  interface BuildingQueueItem {
    id: string;
    building_type: string;
    slot: number;
    level: number;
    ends_at: string;
  }

  interface TroopQueueItem {
    id: string;
    troop_type: string;
    count: number;
    ends_at: string;
  }

  interface DashboardVillage {
    id: string;
    name: string;
    x: number;
    y: number;
    is_capital: boolean;
    wood: number;
    clay: number;
    iron: number;
    crop: number;
    warehouse_capacity: number;
    granary_capacity: number;
    population: number;
    production: ProductionRates | null;
    building_queue: BuildingQueueItem[];
    troop_queue: TroopQueueItem[];
  }

  interface IncomingArmy {
    id: string;
    from_village_name: string | null;
    from_player_name: string | null;
    to_village_id: string;
    to_village_name: string;
    mission: string;
    arrives_at: string;
  }

  interface DashboardData {
    villages: DashboardVillage[];
    incoming_attacks: IncomingArmy[];
    unread_reports: number;
  }

  let dashboard = $state<DashboardData | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  // Format time remaining
  function formatTimeRemaining(endsAt: string): string {
    const end = new Date(endsAt);
    const now = new Date();
    const diff = end.getTime() - now.getTime();

    if (diff <= 0) return 'Done';

    const hours = Math.floor(diff / (1000 * 60 * 60));
    const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
    const seconds = Math.floor((diff % (1000 * 60)) / 1000);

    if (hours > 0) {
      return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    }
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
  }

  // Format troop type for display
  function formatTroopType(type: string): string {
    return type
      .split('_')
      .map(word => word.charAt(0).toUpperCase() + word.slice(1))
      .join(' ');
  }

  // Get resource bar width percentage
  function getResourcePercent(current: number, max: number): number {
    return Math.min(100, (current / max) * 100);
  }

  // Get resource bar color
  function getResourceColor(current: number, max: number): string {
    const percent = (current / max) * 100;
    if (percent >= 95) return 'bg-red-500';
    if (percent >= 80) return 'bg-yellow-500';
    return 'bg-green-500';
  }

  // Navigate to village
  async function goToVillage(villageId: string) {
    await villageStore.loadVillage(villageId);
    goto('/game/village');
  }

  // Load dashboard data
  async function loadDashboard() {
    loading = true;
    error = null;
    try {
      dashboard = await api.get<DashboardData>('/api/dashboard');
    } catch (e: any) {
      error = e.message || 'Failed to load dashboard';
      console.error('Dashboard error:', e);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadDashboard();

    // Refresh every 30 seconds
    const interval = setInterval(loadDashboard, 30000);
    return () => clearInterval(interval);
  });

  // Update times every second
  let now = $state(new Date());
  onMount(() => {
    const timer = setInterval(() => {
      now = new Date();
    }, 1000);
    return () => clearInterval(timer);
  });
</script>

<svelte:head>
  <title>Dashboard - Tusk & Horn</title>
</svelte:head>

<div class="container mx-auto px-4 py-6 max-w-6xl">
  <div class="flex items-center justify-between mb-6">
    <h1 class="text-2xl font-bold">📊 Dashboard</h1>
    <Button variant="outline" size="sm" onclick={loadDashboard} disabled={loading}>
      {loading ? '⏳' : '🔄'} Refresh
    </Button>
  </div>

  {#if loading && !dashboard}
    <div class="flex items-center justify-center min-h-[40vh]">
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto mb-4"></div>
        <p class="text-muted-foreground">Loading dashboard...</p>
      </div>
    </div>
  {:else if error}
    <Card class="p-6 text-center">
      <p class="text-destructive mb-4">{error}</p>
      <Button onclick={loadDashboard}>Try Again</Button>
    </Card>
  {:else if dashboard}
    <!-- Alerts Section -->
    {#if dashboard.incoming_attacks.length > 0 || dashboard.unread_reports > 0}
      <Card class="mb-6 border-destructive/50 bg-destructive/5">
        <CardHeader class="pb-2">
          <CardTitle class="text-lg flex items-center gap-2">
            <span>⚠️</span> Alerts
          </CardTitle>
        </CardHeader>
        <CardContent class="space-y-2">
          {#if dashboard.incoming_attacks.length > 0}
            {#each dashboard.incoming_attacks as attack}
              <div class="flex items-center justify-between p-2 bg-destructive/10 rounded-md">
                <div class="flex items-center gap-2">
                  <span class="text-lg">🗡️</span>
                  <div>
                    <span class="font-medium capitalize">{attack.mission}</span>
                    <span class="text-muted-foreground"> from </span>
                    <span class="font-medium">{attack.from_village_name || 'Unknown'}</span>
                    <span class="text-muted-foreground"> → </span>
                    <span class="font-medium">{attack.to_village_name}</span>
                  </div>
                </div>
                <span class="text-sm font-mono text-destructive">
                  {formatTimeRemaining(attack.arrives_at)}
                </span>
              </div>
            {/each}
          {/if}
          {#if dashboard.unread_reports > 0}
            <div class="flex items-center gap-2 p-2 bg-muted rounded-md">
              <span class="text-lg">📨</span>
              <span>{dashboard.unread_reports} unread report{dashboard.unread_reports > 1 ? 's' : ''}</span>
            </div>
          {/if}
        </CardContent>
      </Card>
    {/if}

    <!-- Villages Grid -->
    <div class="grid gap-4 md:grid-cols-2">
      {#each dashboard.villages as village}
        <Card class="hover:border-primary/50 transition-colors">
          <CardHeader class="pb-2">
            <CardTitle class="flex items-center justify-between">
              <button
                class="flex items-center gap-2 hover:text-primary transition-colors text-left"
                onclick={() => goToVillage(village.id)}
              >
                <span>{village.is_capital ? '👑' : '🏠'}</span>
                <span>{village.name}</span>
                <span class="text-sm text-muted-foreground font-normal">
                  ({village.x}|{village.y})
                </span>
              </button>
              <span class="text-sm text-muted-foreground font-normal">
                Pop: {village.population}
              </span>
            </CardTitle>
          </CardHeader>
          <CardContent class="space-y-4">
            <!-- Resources -->
            <div class="grid grid-cols-4 gap-2 text-sm">
              <!-- Wood -->
              <div>
                <div class="flex items-center gap-1 mb-1">
                  <span>🪵</span>
                  <span class="truncate">{Math.floor(village.wood)}</span>
                </div>
                <div class="h-1.5 bg-muted rounded-full overflow-hidden">
                  <div
                    class="h-full {getResourceColor(village.wood, village.warehouse_capacity)} transition-all"
                    style="width: {getResourcePercent(village.wood, village.warehouse_capacity)}%"
                  ></div>
                </div>
              </div>
              <!-- Clay -->
              <div>
                <div class="flex items-center gap-1 mb-1">
                  <span>🧱</span>
                  <span class="truncate">{Math.floor(village.clay)}</span>
                </div>
                <div class="h-1.5 bg-muted rounded-full overflow-hidden">
                  <div
                    class="h-full {getResourceColor(village.clay, village.warehouse_capacity)} transition-all"
                    style="width: {getResourcePercent(village.clay, village.warehouse_capacity)}%"
                  ></div>
                </div>
              </div>
              <!-- Iron -->
              <div>
                <div class="flex items-center gap-1 mb-1">
                  <span>⛏️</span>
                  <span class="truncate">{Math.floor(village.iron)}</span>
                </div>
                <div class="h-1.5 bg-muted rounded-full overflow-hidden">
                  <div
                    class="h-full {getResourceColor(village.iron, village.warehouse_capacity)} transition-all"
                    style="width: {getResourcePercent(village.iron, village.warehouse_capacity)}%"
                  ></div>
                </div>
              </div>
              <!-- Crop -->
              <div>
                <div class="flex items-center gap-1 mb-1">
                  <span>🌾</span>
                  <span class="truncate">{Math.floor(village.crop)}</span>
                </div>
                <div class="h-1.5 bg-muted rounded-full overflow-hidden">
                  <div
                    class="h-full {getResourceColor(village.crop, village.granary_capacity)} transition-all"
                    style="width: {getResourcePercent(village.crop, village.granary_capacity)}%"
                  ></div>
                </div>
              </div>
            </div>

            <!-- Production rates -->
            {#if village.production}
              <div class="flex flex-wrap gap-2 text-xs text-muted-foreground">
                <span>+{village.production.wood_per_hour}/h 🪵</span>
                <span>+{village.production.clay_per_hour}/h 🧱</span>
                <span>+{village.production.iron_per_hour}/h ⛏️</span>
                <span class={village.production.net_crop_per_hour < 0 ? 'text-destructive' : ''}>
                  {village.production.net_crop_per_hour > 0 ? '+' : ''}{village.production.net_crop_per_hour}/h 🌾
                </span>
              </div>
            {/if}

            <!-- Queues -->
            <div class="space-y-2">
              <!-- Building Queue -->
              {#if village.building_queue.length > 0}
                {#each village.building_queue as building}
                  <div class="flex items-center justify-between p-2 bg-muted/50 rounded-md text-sm">
                    <div class="flex items-center gap-2">
                      <span>🔨</span>
                      <span>{formatBuildingType(building.building_type as any)} Lv.{building.level}</span>
                    </div>
                    <span class="font-mono text-primary">{formatTimeRemaining(building.ends_at)}</span>
                  </div>
                {/each}
              {:else}
                <div class="flex items-center gap-2 p-2 text-sm text-muted-foreground">
                  <span>🔨</span>
                  <span>No construction</span>
                </div>
              {/if}

              <!-- Troop Queue -->
              {#if village.troop_queue.length > 0}
                {#each village.troop_queue as troop}
                  <div class="flex items-center justify-between p-2 bg-muted/50 rounded-md text-sm">
                    <div class="flex items-center gap-2">
                      <span>⚔️</span>
                      <span>{troop.count}x {formatTroopType(troop.troop_type)}</span>
                    </div>
                    <span class="font-mono text-primary">{formatTimeRemaining(troop.ends_at)}</span>
                  </div>
                {/each}
              {:else}
                <div class="flex items-center gap-2 p-2 text-sm text-muted-foreground">
                  <span>⚔️</span>
                  <span>No training</span>
                </div>
              {/if}
            </div>
          </CardContent>
        </Card>
      {/each}
    </div>

    {#if dashboard.villages.length === 0}
      <Card class="p-6 text-center">
        <p class="text-muted-foreground mb-4">No villages found.</p>
        <Button onclick={() => goto('/onboarding')}>Create Your First Village</Button>
      </Card>
    {/if}
  {/if}
</div>
