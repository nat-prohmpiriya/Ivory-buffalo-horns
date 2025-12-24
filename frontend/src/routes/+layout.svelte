<script lang="ts">
  import '../app.css';
  import '$lib/i18n';
  import { isLoading } from 'svelte-i18n';
  import { Toaster } from 'svelte-sonner';
  import type { Snippet } from 'svelte';

  let { children }: { children: Snippet } = $props();
</script>

<svelte:head>
  <title>Tusk & Horn - Southeast Asian Strategy Game</title>
  <meta name="description" content="Build your kingdom, train your army, conquer the land. A real-time strategy game inspired by Southeast Asian ancient kingdoms." />
</svelte:head>

{#if $isLoading}
  <div class="flex h-screen items-center justify-center">
    <p class="text-muted-foreground">Loading...</p>
  </div>
{:else}
  {@render children()}
{/if}

<Toaster richColors position="top-right" />
