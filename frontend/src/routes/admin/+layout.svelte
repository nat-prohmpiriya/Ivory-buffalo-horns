<script lang="ts">
    import { onMount } from 'svelte';
    import type { Snippet } from 'svelte';
    import { goto } from '$app/navigation';
    import { page } from '$app/state';
    import { Button } from '$lib/components/ui/button';
    import { authStore } from '$lib/stores/auth';

    let { children }: { children: Snippet } = $props();

    let authState = $state(authStore);
    let user = $derived($authState.user);
    let isAdmin = $derived($authState.isAdmin);
    let loading = $derived($authState.loading);

    const navItems = [
        { href: '/admin', label: 'Dashboard', icon: '📊' },
        { href: '/admin/users', label: 'Users', icon: '👥' },
    ];

    function isActive(href: string): boolean {
        if (href === '/admin') {
            return page.url.pathname === '/admin';
        }
        return page.url.pathname.startsWith(href);
    }

    onMount(() => {
        // Check admin access
        const checkAccess = () => {
            if (!loading && !isAdmin) {
                goto('/game/village');
            }
        };
        checkAccess();
    });
</script>

<svelte:head>
    <title>Admin Panel - Tusk & Horn</title>
</svelte:head>

{#if loading}
    <div class="min-h-screen flex items-center justify-center">
        <p class="text-muted-foreground">Loading...</p>
    </div>
{:else if !isAdmin}
    <div class="min-h-screen flex items-center justify-center">
        <div class="text-center">
            <h1 class="text-2xl font-bold mb-2">Access Denied</h1>
            <p class="text-muted-foreground mb-4">You don't have admin privileges.</p>
            <Button onclick={() => goto('/game/village')}>Go to Game</Button>
        </div>
    </div>
{:else}
    <div class="min-h-screen bg-background">
        <!-- Header -->
        <header class="sticky top-0 z-50 bg-background border-b">
            <div class="flex items-center justify-between px-6 h-14">
                <div class="flex items-center gap-4">
                    <a href="/admin" class="flex items-center gap-2">
                        <span class="text-xl">🛡️</span>
                        <span class="font-bold">Admin Panel</span>
                    </a>

                    <!-- Nav -->
                    <nav class="flex items-center gap-1 ml-8">
                        {#each navItems as item}
                            <a
                                href={item.href}
                                class="flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors
                                       {isActive(item.href)
                                         ? 'bg-primary text-primary-foreground'
                                         : 'hover:bg-muted'}"
                            >
                                <span>{item.icon}</span>
                                <span>{item.label}</span>
                            </a>
                        {/each}
                    </nav>
                </div>

                <div class="flex items-center gap-4">
                    <span class="text-sm text-muted-foreground">
                        {user?.displayName || user?.email}
                    </span>
                    <Button variant="outline" size="sm" onclick={() => goto('/game/village')}>
                        Back to Game
                    </Button>
                </div>
            </div>
        </header>

        <!-- Main Content -->
        <main class="p-6">
            {@render children()}
        </main>
    </div>
{/if}
