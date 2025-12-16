<script lang="ts">
  interface Resources {
    wood: number;
    clay: number;
    iron: number;
    crop: number;
    warehouseCapacity: number;
    granaryCapacity: number;
    woodProduction: number;
    clayProduction: number;
    ironProduction: number;
    cropProduction: number;
    cropConsumption: number;
  }

  interface Props {
    resources: Resources;
    gold?: number;
    compact?: boolean;
  }

  let { resources, gold = 0, compact = false }: Props = $props();

  function formatNumber(num: number): string {
    if (num >= 1000000) {
      return (num / 1000000).toFixed(1) + 'M';
    }
    if (num >= 1000) {
      return (num / 1000).toFixed(1) + 'K';
    }
    return Math.floor(num).toString();
  }

  function getPercentage(current: number, max: number): number {
    return Math.min((current / max) * 100, 100);
  }

  function isNearFull(current: number, max: number): boolean {
    return current / max >= 0.9;
  }

  const netCrop = $derived(resources.cropProduction - resources.cropConsumption);
</script>

<div class="bg-card border-b px-4 py-2">
  <div class="flex items-center justify-between gap-4 overflow-x-auto">
    <!-- Resources -->
    <div class="flex items-center gap-4 md:gap-6">
      <!-- Wood -->
      <div class="flex items-center gap-2 min-w-fit" title="Wood">
        <span class="text-lg">🪵</span>
        <div class="flex flex-col">
          <div class="flex items-center gap-1">
            <span class="font-medium {isNearFull(resources.wood, resources.warehouseCapacity) ? 'text-destructive' : ''}">
              {formatNumber(resources.wood)}
            </span>
            {#if !compact}
              <span class="text-xs text-muted-foreground">/ {formatNumber(resources.warehouseCapacity)}</span>
            {/if}
          </div>
          {#if !compact}
            <span class="text-xs text-emerald-600">+{formatNumber(resources.woodProduction)}/h</span>
          {/if}
        </div>
      </div>

      <!-- Clay -->
      <div class="flex items-center gap-2 min-w-fit" title="Clay">
        <span class="text-lg">🧱</span>
        <div class="flex flex-col">
          <div class="flex items-center gap-1">
            <span class="font-medium {isNearFull(resources.clay, resources.warehouseCapacity) ? 'text-destructive' : ''}">
              {formatNumber(resources.clay)}
            </span>
            {#if !compact}
              <span class="text-xs text-muted-foreground">/ {formatNumber(resources.warehouseCapacity)}</span>
            {/if}
          </div>
          {#if !compact}
            <span class="text-xs text-emerald-600">+{formatNumber(resources.clayProduction)}/h</span>
          {/if}
        </div>
      </div>

      <!-- Iron -->
      <div class="flex items-center gap-2 min-w-fit" title="Iron">
        <span class="text-lg">⛏️</span>
        <div class="flex flex-col">
          <div class="flex items-center gap-1">
            <span class="font-medium {isNearFull(resources.iron, resources.warehouseCapacity) ? 'text-destructive' : ''}">
              {formatNumber(resources.iron)}
            </span>
            {#if !compact}
              <span class="text-xs text-muted-foreground">/ {formatNumber(resources.warehouseCapacity)}</span>
            {/if}
          </div>
          {#if !compact}
            <span class="text-xs text-emerald-600">+{formatNumber(resources.ironProduction)}/h</span>
          {/if}
        </div>
      </div>

      <!-- Crop -->
      <div class="flex items-center gap-2 min-w-fit" title="Crop">
        <span class="text-lg">🌾</span>
        <div class="flex flex-col">
          <div class="flex items-center gap-1">
            <span class="font-medium {isNearFull(resources.crop, resources.granaryCapacity) ? 'text-destructive' : ''}">
              {formatNumber(resources.crop)}
            </span>
            {#if !compact}
              <span class="text-xs text-muted-foreground">/ {formatNumber(resources.granaryCapacity)}</span>
            {/if}
          </div>
          {#if !compact}
            <span class="text-xs {netCrop >= 0 ? 'text-emerald-600' : 'text-destructive'}">
              {netCrop >= 0 ? '+' : ''}{formatNumber(netCrop)}/h
            </span>
          {/if}
        </div>
      </div>
    </div>

    <!-- Gold & Population -->
    <div class="flex items-center gap-4">
      <!-- Gold -->
      <div class="flex items-center gap-2 min-w-fit" title="Gold">
        <span class="text-lg">💰</span>
        <span class="font-medium text-amber-500">{formatNumber(gold)}</span>
      </div>

      <!-- Free Crop (population capacity) -->
      {#if !compact}
        <div class="flex items-center gap-2 min-w-fit" title="Free Crop (Population Capacity)">
          <span class="text-lg">👥</span>
          <span class="font-medium {netCrop < 0 ? 'text-destructive' : ''}">
            {formatNumber(netCrop)}/h
          </span>
        </div>
      {/if}
    </div>
  </div>
</div>
