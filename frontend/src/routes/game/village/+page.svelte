<script lang="ts">
  import { Card } from '$lib/components/ui/card';
  import { Button } from '$lib/components/ui/button';
  import BuildingSlot, { type BuildingType } from '$lib/components/game/BuildingSlot.svelte';
  import BuildingDetailModal from '$lib/components/modals/BuildingDetailModal.svelte';
  import BuildMenuModal from '$lib/components/modals/BuildMenuModal.svelte';

  type ViewMode = 'village' | 'resources';

  interface Building {
    id: string;
    type: BuildingType;
    level: number;
    isUpgrading: boolean;
    upgradeEndsAt?: Date;
  }

  let viewMode = $state<ViewMode>('village');

  // Modal states
  let showBuildingDetail = $state(false);
  let showBuildMenu = $state(false);
  let selectedBuilding = $state<Building | null>(null);
  let selectedSlot = $state(0);
  let selectedIsResource = $state(false);

  // Mock data - Village center buildings (22 slots)
  const villageBuildingSlots: (Building | null)[] = [
    { id: '1', type: 'main_building', level: 5, isUpgrading: false },
    { id: '2', type: 'warehouse', level: 3, isUpgrading: true, upgradeEndsAt: new Date(Date.now() + 3600000) },
    { id: '3', type: 'granary', level: 3, isUpgrading: false },
    { id: '4', type: 'barracks', level: 2, isUpgrading: false },
    { id: '5', type: 'rally_point', level: 1, isUpgrading: false },
    { id: '6', type: 'market', level: 1, isUpgrading: false },
    null, // Empty slot
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null
  ];

  // Mock data - Resource fields (18 slots)
  const resourceFieldSlots: (Building | null)[] = [
    { id: 'r1', type: 'woodcutter', level: 5, isUpgrading: false },
    { id: 'r2', type: 'woodcutter', level: 4, isUpgrading: false },
    { id: 'r3', type: 'woodcutter', level: 3, isUpgrading: true, upgradeEndsAt: new Date(Date.now() + 1800000) },
    { id: 'r4', type: 'woodcutter', level: 3, isUpgrading: false },
    { id: 'r5', type: 'clay_pit', level: 4, isUpgrading: false },
    { id: 'r6', type: 'clay_pit', level: 4, isUpgrading: false },
    { id: 'r7', type: 'clay_pit', level: 3, isUpgrading: false },
    { id: 'r8', type: 'clay_pit', level: 3, isUpgrading: false },
    { id: 'r9', type: 'iron_mine', level: 3, isUpgrading: false },
    { id: 'r10', type: 'iron_mine', level: 3, isUpgrading: false },
    { id: 'r11', type: 'iron_mine', level: 2, isUpgrading: false },
    { id: 'r12', type: 'iron_mine', level: 2, isUpgrading: false },
    { id: 'r13', type: 'crop_field', level: 4, isUpgrading: false },
    { id: 'r14', type: 'crop_field', level: 4, isUpgrading: false },
    { id: 'r15', type: 'crop_field', level: 3, isUpgrading: false },
    { id: 'r16', type: 'crop_field', level: 3, isUpgrading: false },
    { id: 'r17', type: 'crop_field', level: 2, isUpgrading: false },
    { id: 'r18', type: 'crop_field', level: 2, isUpgrading: false }
  ];

  // Build queue mock
  const buildQueue = [
    { id: '2', name: 'Warehouse', level: 4, endsAt: new Date(Date.now() + 3600000) },
    { id: 'r3', name: 'Woodcutter', level: 4, endsAt: new Date(Date.now() + 5400000) }
  ];

  function formatTimeRemaining(endTime: Date): string {
    const diff = endTime.getTime() - Date.now();
    if (diff <= 0) return 'Done';

    const hours = Math.floor(diff / 3600000);
    const minutes = Math.floor((diff % 3600000) / 60000);
    const seconds = Math.floor((diff % 60000) / 1000);

    if (hours > 0) {
      return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    }
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
  }

  function handleBuildingClick(building: Building | null, slot: number, isResource: boolean) {
    selectedSlot = slot;
    selectedIsResource = isResource;

    if (building) {
      selectedBuilding = building;
      showBuildingDetail = true;
    } else {
      showBuildMenu = true;
    }
  }

  function handleUpgrade() {
    console.log('Upgrading building:', selectedBuilding);
    // TODO: Call API to upgrade building
    showBuildingDetail = false;
  }

  function handleDemolish() {
    console.log('Demolishing building:', selectedBuilding);
    // TODO: Call API to demolish building
    showBuildingDetail = false;
  }

  function handleBuild(type: BuildingType) {
    console.log('Building new:', type, 'at slot:', selectedSlot);
    // TODO: Call API to build new building
    showBuildMenu = false;
  }
</script>

<svelte:head>
  <title>Village - Tusk & Horn</title>
</svelte:head>

<div class="container mx-auto px-4 py-6 max-w-6xl">
  <!-- Header -->
  <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
    <div>
      <h1 class="text-2xl font-bold">Capital</h1>
      <p class="text-muted-foreground text-sm">Coordinates: (100|100)</p>
    </div>

    <!-- View Toggle -->
    <div class="flex items-center gap-2 bg-muted p-1 rounded-lg">
      <Button
        variant={viewMode === 'village' ? 'default' : 'ghost'}
        size="sm"
        onclick={() => viewMode = 'village'}
        class="gap-2"
      >
        <span>🏘️</span>
        <span>Village</span>
      </Button>
      <Button
        variant={viewMode === 'resources' ? 'default' : 'ghost'}
        size="sm"
        onclick={() => viewMode = 'resources'}
        class="gap-2"
      >
        <span>🌾</span>
        <span>Resources</span>
      </Button>
    </div>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
    <!-- Building Grid -->
    <div class="lg:col-span-3">
      {#if viewMode === 'village'}
        <!-- Village Center Grid - 22 slots in irregular layout -->
        <Card class="p-4">
          <h2 class="text-lg font-semibold mb-4">Village Center</h2>
          <div class="grid grid-cols-4 sm:grid-cols-5 md:grid-cols-6 gap-2 sm:gap-3">
            {#each villageBuildingSlots as building, index}
              <BuildingSlot
                {building}
                slot={index + 1}
                onclick={() => handleBuildingClick(building, index + 1, false)}
              />
            {/each}
          </div>
        </Card>
      {:else}
        <!-- Resource Fields Grid - 18 slots -->
        <Card class="p-4">
          <h2 class="text-lg font-semibold mb-4">Resource Fields</h2>

          <!-- Resource field layout - grouped by type -->
          <div class="space-y-6">
            <!-- Wood -->
            <div>
              <div class="flex items-center gap-2 mb-3">
                <span class="text-lg">🪵</span>
                <span class="font-medium">Woodcutters</span>
              </div>
              <div class="grid grid-cols-4 gap-2 sm:gap-3">
                {#each resourceFieldSlots.slice(0, 4) as building, index}
                  <BuildingSlot
                    {building}
                    slot={index + 1}
                    isResourceField={true}
                    onclick={() => handleBuildingClick(building, index + 1, true)}
                  />
                {/each}
              </div>
            </div>

            <!-- Clay -->
            <div>
              <div class="flex items-center gap-2 mb-3">
                <span class="text-lg">🧱</span>
                <span class="font-medium">Clay Pits</span>
              </div>
              <div class="grid grid-cols-4 gap-2 sm:gap-3">
                {#each resourceFieldSlots.slice(4, 8) as building, index}
                  <BuildingSlot
                    {building}
                    slot={index + 5}
                    isResourceField={true}
                    onclick={() => handleBuildingClick(building, index + 5, true)}
                  />
                {/each}
              </div>
            </div>

            <!-- Iron -->
            <div>
              <div class="flex items-center gap-2 mb-3">
                <span class="text-lg">⛏️</span>
                <span class="font-medium">Iron Mines</span>
              </div>
              <div class="grid grid-cols-4 gap-2 sm:gap-3">
                {#each resourceFieldSlots.slice(8, 12) as building, index}
                  <BuildingSlot
                    {building}
                    slot={index + 9}
                    isResourceField={true}
                    onclick={() => handleBuildingClick(building, index + 9, true)}
                  />
                {/each}
              </div>
            </div>

            <!-- Crop -->
            <div>
              <div class="flex items-center gap-2 mb-3">
                <span class="text-lg">🌾</span>
                <span class="font-medium">Crop Fields</span>
              </div>
              <div class="grid grid-cols-3 sm:grid-cols-6 gap-2 sm:gap-3">
                {#each resourceFieldSlots.slice(12, 18) as building, index}
                  <BuildingSlot
                    {building}
                    slot={index + 13}
                    isResourceField={true}
                    onclick={() => handleBuildingClick(building, index + 13, true)}
                  />
                {/each}
              </div>
            </div>
          </div>
        </Card>
      {/if}
    </div>

    <!-- Sidebar -->
    <div class="space-y-4">
      <!-- Build Queue -->
      <Card class="p-4">
        <h3 class="font-semibold mb-3 flex items-center gap-2">
          <span>🔨</span>
          Build Queue
        </h3>
        {#if buildQueue.length > 0}
          <div class="space-y-3">
            {#each buildQueue as item, index}
              <div class="flex items-center justify-between p-2 bg-muted rounded-lg">
                <div>
                  <p class="font-medium text-sm">{item.name}</p>
                  <p class="text-xs text-muted-foreground">Level {item.level}</p>
                </div>
                <div class="text-right">
                  <p class="text-sm font-mono text-amber-600">{formatTimeRemaining(item.endsAt)}</p>
                  {#if index === 0}
                    <span class="text-xs text-emerald-600">Building...</span>
                  {:else}
                    <span class="text-xs text-muted-foreground">Queued</span>
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        {:else}
          <p class="text-sm text-muted-foreground text-center py-4">
            No buildings in queue
          </p>
        {/if}
      </Card>

      <!-- Quick Actions -->
      <Card class="p-4">
        <h3 class="font-semibold mb-3 flex items-center gap-2">
          <span>⚡</span>
          Quick Actions
        </h3>
        <div class="space-y-2">
          <Button variant="outline" class="w-full justify-start gap-2" size="sm">
            <span>⚔️</span>
            Train Troops
          </Button>
          <Button variant="outline" class="w-full justify-start gap-2" size="sm">
            <span>🏪</span>
            Open Market
          </Button>
          <Button variant="outline" class="w-full justify-start gap-2" size="sm">
            <span>📜</span>
            Village Overview
          </Button>
        </div>
      </Card>

      <!-- Village Stats -->
      <Card class="p-4">
        <h3 class="font-semibold mb-3 flex items-center gap-2">
          <span>📊</span>
          Village Stats
        </h3>
        <div class="space-y-2 text-sm">
          <div class="flex justify-between">
            <span class="text-muted-foreground">Population</span>
            <span class="font-medium">127</span>
          </div>
          <div class="flex justify-between">
            <span class="text-muted-foreground">Culture Points</span>
            <span class="font-medium">45/day</span>
          </div>
          <div class="flex justify-between">
            <span class="text-muted-foreground">Loyalty</span>
            <span class="font-medium text-emerald-600">100%</span>
          </div>
          <div class="flex justify-between">
            <span class="text-muted-foreground">Troops</span>
            <span class="font-medium">52</span>
          </div>
        </div>
      </Card>
    </div>
  </div>
</div>

<!-- Modals -->
<BuildingDetailModal
  bind:open={showBuildingDetail}
  building={selectedBuilding}
  onUpgrade={handleUpgrade}
  onDemolish={handleDemolish}
/>

<BuildMenuModal
  bind:open={showBuildMenu}
  slot={selectedSlot}
  isResourceField={selectedIsResource}
  onBuild={handleBuild}
/>
