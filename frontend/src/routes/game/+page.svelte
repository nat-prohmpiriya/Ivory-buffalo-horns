<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  onMount(() => {
    // Redirect to village page by default
    goto('/game/village', { replaceState: true });
  });
</script>

<div class="flex items-center justify-center h-64">
  <p class="text-muted-foreground">Redirecting to village...</p>
</div>
