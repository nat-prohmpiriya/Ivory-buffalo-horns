<script lang="ts">
    import { onMount } from 'svelte';
    import { Button } from '$lib/components/ui/button';
    import { Input } from '$lib/components/ui/input';
    import {
        adminStore,
        formatDate,
        getStatusBadge,
        getProviderIcon,
        type AdminUserResponse,
    } from '$lib/stores/admin';

    let adminState = $state(adminStore);
    let users = $derived($adminState.users);
    let stats = $derived($adminState.stats);
    let selectedUser = $derived($adminState.selectedUser);
    let loading = $derived($adminState.loading);
    let currentPage = $derived($adminState.currentPage);

    let searchQuery = $state('');
    let showUserDetail = $state(false);

    // Resource adjustment form
    let adjustVillageId = $state('');
    let adjustWood = $state<number | undefined>(undefined);
    let adjustClay = $state<number | undefined>(undefined);
    let adjustIron = $state<number | undefined>(undefined);
    let adjustCrop = $state<number | undefined>(undefined);
    let adjustReason = $state('');

    onMount(() => {
        adminStore.getServerStats();
        adminStore.loadUsers();
    });

    async function handleSearch() {
        if (searchQuery.trim()) {
            await adminStore.searchUsers(searchQuery);
        } else {
            await adminStore.loadUsers();
        }
    }

    async function handleViewUser(userId: string) {
        await adminStore.getPlayerDetail(userId);
        showUserDetail = true;
    }

    async function handleBanUser(user: AdminUserResponse) {
        const reason = prompt('Ban reason (optional):');
        if (reason !== null) {
            await adminStore.banUser(user.id, reason || undefined);
        }
    }

    async function handleUnbanUser(user: AdminUserResponse) {
        if (confirm(`Unban ${user.display_name || user.email}?`)) {
            await adminStore.unbanUser(user.id);
        }
    }

    async function handleToggleAdmin(user: AdminUserResponse) {
        const action = user.is_admin ? 'revoke admin from' : 'grant admin to';
        if (confirm(`${action} ${user.display_name || user.email}?`)) {
            await adminStore.setAdmin(user.id, !user.is_admin);
        }
    }

    async function handleAdjustResources() {
        if (!adjustVillageId || !adjustReason) {
            alert('Village ID and reason are required');
            return;
        }
        await adminStore.adjustResources(adjustVillageId, {
            wood: adjustWood,
            clay: adjustClay,
            iron: adjustIron,
            crop: adjustCrop,
            reason: adjustReason,
        });
        // Reset form
        adjustVillageId = '';
        adjustWood = undefined;
        adjustClay = undefined;
        adjustIron = undefined;
        adjustCrop = undefined;
        adjustReason = '';
    }

    function closeUserDetail() {
        showUserDetail = false;
        adminStore.clearSelectedUser();
    }
</script>

<div class="space-y-6">
    <!-- Stats Cards -->
    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
        <div class="bg-card border rounded-lg p-4">
            <div class="text-2xl font-bold">{stats?.total_users ?? '-'}</div>
            <div class="text-sm text-muted-foreground">Total Users</div>
        </div>
        <div class="bg-card border rounded-lg p-4">
            <div class="text-2xl font-bold text-green-600">{stats?.active_users_24h ?? '-'}</div>
            <div class="text-sm text-muted-foreground">Active (24h)</div>
        </div>
        <div class="bg-card border rounded-lg p-4">
            <div class="text-2xl font-bold text-red-600">{stats?.banned_users ?? '-'}</div>
            <div class="text-sm text-muted-foreground">Banned</div>
        </div>
        <div class="bg-card border rounded-lg p-4">
            <div class="text-2xl font-bold">{stats?.total_villages ?? '-'}</div>
            <div class="text-sm text-muted-foreground">Villages</div>
        </div>
        <div class="bg-card border rounded-lg p-4">
            <div class="text-2xl font-bold">{stats?.total_alliances ?? '-'}</div>
            <div class="text-sm text-muted-foreground">Alliances</div>
        </div>
        <div class="bg-card border rounded-lg p-4">
            <div class="text-2xl font-bold text-orange-600">{stats?.total_battles_today ?? '-'}</div>
            <div class="text-sm text-muted-foreground">Battles Today</div>
        </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Users Table -->
        <div class="lg:col-span-2 bg-card border rounded-lg">
            <div class="p-4 border-b">
                <h2 class="text-lg font-semibold mb-3">Users</h2>
                <div class="flex gap-2">
                    <Input
                        placeholder="Search users..."
                        bind:value={searchQuery}
                        onkeydown={(e) => e.key === 'Enter' && handleSearch()}
                    />
                    <Button onclick={handleSearch} disabled={loading}>Search</Button>
                </div>
            </div>

            <div class="overflow-x-auto">
                <table class="w-full">
                    <thead class="bg-muted">
                        <tr>
                            <th class="px-4 py-2 text-left text-sm font-medium">User</th>
                            <th class="px-4 py-2 text-left text-sm font-medium">Status</th>
                            <th class="px-4 py-2 text-center text-sm font-medium">Villages</th>
                            <th class="px-4 py-2 text-right text-sm font-medium">Actions</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#if loading && users.length === 0}
                            <tr>
                                <td colspan="4" class="px-4 py-8 text-center text-muted-foreground">
                                    Loading...
                                </td>
                            </tr>
                        {:else if users.length === 0}
                            <tr>
                                <td colspan="4" class="px-4 py-8 text-center text-muted-foreground">
                                    No users found
                                </td>
                            </tr>
                        {:else}
                            {#each users as user}
                                {@const status = getStatusBadge(user)}
                                <tr class="border-t hover:bg-muted/50">
                                    <td class="px-4 py-3">
                                        <div class="flex items-center gap-3">
                                            {#if user.photo_url}
                                                <img
                                                    src={user.photo_url}
                                                    alt=""
                                                    class="w-8 h-8 rounded-full"
                                                />
                                            {:else}
                                                <div class="w-8 h-8 rounded-full bg-muted flex items-center justify-center">
                                                    {getProviderIcon(user.provider)}
                                                </div>
                                            {/if}
                                            <div>
                                                <div class="font-medium">
                                                    {user.display_name || 'No name'}
                                                </div>
                                                <div class="text-xs text-muted-foreground">
                                                    {user.email || user.firebase_uid.slice(0, 8)}
                                                </div>
                                            </div>
                                        </div>
                                    </td>
                                    <td class="px-4 py-3">
                                        <span class="px-2 py-0.5 rounded-full text-xs {status.class}">
                                            {status.label}
                                        </span>
                                    </td>
                                    <td class="px-4 py-3 text-center">
                                        {user.village_count}
                                    </td>
                                    <td class="px-4 py-3 text-right">
                                        <div class="flex items-center justify-end gap-1">
                                            <Button
                                                size="sm"
                                                variant="ghost"
                                                onclick={() => handleViewUser(user.id)}
                                            >
                                                View
                                            </Button>
                                            {#if user.banned_at}
                                                <Button
                                                    size="sm"
                                                    variant="ghost"
                                                    class="text-green-600"
                                                    onclick={() => handleUnbanUser(user)}
                                                >
                                                    Unban
                                                </Button>
                                            {:else}
                                                <Button
                                                    size="sm"
                                                    variant="ghost"
                                                    class="text-red-600"
                                                    onclick={() => handleBanUser(user)}
                                                >
                                                    Ban
                                                </Button>
                                            {/if}
                                        </div>
                                    </td>
                                </tr>
                            {/each}
                        {/if}
                    </tbody>
                </table>
            </div>

            <div class="p-4 border-t flex items-center justify-between">
                <Button
                    size="sm"
                    variant="outline"
                    disabled={currentPage <= 1 || loading}
                    onclick={() => adminStore.loadUsers(currentPage - 1)}
                >
                    Previous
                </Button>
                <span class="text-sm text-muted-foreground">Page {currentPage}</span>
                <Button
                    size="sm"
                    variant="outline"
                    disabled={users.length < 20 || loading}
                    onclick={() => adminStore.loadUsers(currentPage + 1)}
                >
                    Next
                </Button>
            </div>
        </div>

        <!-- Quick Actions -->
        <div class="space-y-4">
            <!-- Adjust Resources -->
            <div class="bg-card border rounded-lg p-4">
                <h3 class="font-semibold mb-3">Adjust Resources</h3>
                <div class="space-y-3">
                    <div>
                        <label class="text-xs text-muted-foreground">Village ID</label>
                        <Input
                            placeholder="Village UUID"
                            bind:value={adjustVillageId}
                        />
                    </div>
                    <div class="grid grid-cols-2 gap-2">
                        <div>
                            <label class="text-xs text-muted-foreground">Wood</label>
                            <Input type="number" placeholder="+/-" bind:value={adjustWood} />
                        </div>
                        <div>
                            <label class="text-xs text-muted-foreground">Clay</label>
                            <Input type="number" placeholder="+/-" bind:value={adjustClay} />
                        </div>
                        <div>
                            <label class="text-xs text-muted-foreground">Iron</label>
                            <Input type="number" placeholder="+/-" bind:value={adjustIron} />
                        </div>
                        <div>
                            <label class="text-xs text-muted-foreground">Crop</label>
                            <Input type="number" placeholder="+/-" bind:value={adjustCrop} />
                        </div>
                    </div>
                    <div>
                        <label class="text-xs text-muted-foreground">Reason *</label>
                        <Input
                            placeholder="Reason for adjustment"
                            bind:value={adjustReason}
                        />
                    </div>
                    <Button
                        class="w-full"
                        onclick={handleAdjustResources}
                        disabled={loading || !adjustVillageId || !adjustReason}
                    >
                        Apply
                    </Button>
                </div>
            </div>

            <!-- Recent Activity (placeholder) -->
            <div class="bg-card border rounded-lg p-4">
                <h3 class="font-semibold mb-3">Server Info</h3>
                <div class="space-y-2 text-sm">
                    <div class="flex justify-between">
                        <span class="text-muted-foreground">Version</span>
                        <span>1.0.0</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-muted-foreground">Environment</span>
                        <span>Development</span>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

<!-- User Detail Modal -->
{#if showUserDetail && selectedUser}
    <div class="fixed inset-0 z-50 bg-black/50 flex items-center justify-center p-4">
        <div class="bg-background rounded-lg max-w-2xl w-full max-h-[80vh] overflow-y-auto">
            <div class="p-4 border-b flex items-center justify-between">
                <h2 class="text-lg font-semibold">Player Detail</h2>
                <Button variant="ghost" size="sm" onclick={closeUserDetail}>Close</Button>
            </div>

            <div class="p-4 space-y-4">
                <!-- User Info -->
                <div class="flex items-center gap-4">
                    {#if selectedUser.user.photo_url}
                        <img
                            src={selectedUser.user.photo_url}
                            alt=""
                            class="w-16 h-16 rounded-full"
                        />
                    {:else}
                        <div class="w-16 h-16 rounded-full bg-muted flex items-center justify-center text-2xl">
                            {getProviderIcon(selectedUser.user.provider)}
                        </div>
                    {/if}
                    <div>
                        <h3 class="text-xl font-semibold">
                            {selectedUser.user.display_name || 'No name'}
                        </h3>
                        <p class="text-muted-foreground">{selectedUser.user.email}</p>
                        <span class="px-2 py-0.5 rounded-full text-xs {getStatusBadge(selectedUser.user).class}">
                            {getStatusBadge(selectedUser.user).label}
                        </span>
                    </div>
                </div>

                <!-- User Details -->
                <div class="grid grid-cols-2 gap-4 text-sm">
                    <div>
                        <span class="text-muted-foreground">User ID</span>
                        <p class="font-mono text-xs">{selectedUser.user.id}</p>
                    </div>
                    <div>
                        <span class="text-muted-foreground">Provider</span>
                        <p>{selectedUser.user.provider}</p>
                    </div>
                    <div>
                        <span class="text-muted-foreground">Created</span>
                        <p>{formatDate(selectedUser.user.created_at)}</p>
                    </div>
                    <div>
                        <span class="text-muted-foreground">Last Login</span>
                        <p>{formatDate(selectedUser.user.last_login_at)}</p>
                    </div>
                </div>

                {#if selectedUser.user.banned_at}
                    <div class="bg-red-50 border border-red-200 rounded p-3">
                        <p class="text-red-800 font-medium">Banned</p>
                        <p class="text-sm text-red-700">
                            {selectedUser.user.banned_reason || 'No reason provided'}
                        </p>
                        <p class="text-xs text-red-600 mt-1">
                            {formatDate(selectedUser.user.banned_at)}
                        </p>
                    </div>
                {/if}

                <!-- Alliance -->
                {#if selectedUser.alliance}
                    <div class="border rounded p-3">
                        <h4 class="font-medium mb-2">Alliance</h4>
                        <p>[{selectedUser.alliance.tag}] {selectedUser.alliance.name}</p>
                        <p class="text-sm text-muted-foreground">Role: {selectedUser.alliance.role}</p>
                    </div>
                {/if}

                <!-- Villages -->
                <div class="border rounded p-3">
                    <h4 class="font-medium mb-2">Villages ({selectedUser.villages.length})</h4>
                    <div class="space-y-2 max-h-40 overflow-y-auto">
                        {#each selectedUser.villages as village}
                            <div class="flex items-center justify-between text-sm bg-muted/50 rounded p-2">
                                <div>
                                    <span class="font-medium">{village.name}</span>
                                    {#if village.is_capital}
                                        <span class="text-yellow-600 ml-1">👑</span>
                                    {/if}
                                    <span class="text-muted-foreground ml-2">
                                        ({village.x}|{village.y})
                                    </span>
                                </div>
                                <div class="text-muted-foreground">
                                    Pop: {village.population}
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>

                <!-- Heroes -->
                {#if selectedUser.heroes.length > 0}
                    <div class="border rounded p-3">
                        <h4 class="font-medium mb-2">Heroes ({selectedUser.heroes.length})</h4>
                        <div class="space-y-2">
                            {#each selectedUser.heroes as hero}
                                <div class="flex items-center justify-between text-sm bg-muted/50 rounded p-2">
                                    <div>
                                        <span class="font-medium">{hero.name}</span>
                                        <span class="text-muted-foreground ml-2">Lv.{hero.level}</span>
                                    </div>
                                    <div class="text-muted-foreground">
                                        HP: {hero.health}% | {hero.status}
                                    </div>
                                </div>
                            {/each}
                        </div>
                    </div>
                {/if}

                <!-- Actions -->
                <div class="flex gap-2 pt-4 border-t">
                    <Button
                        variant="outline"
                        onclick={() => handleToggleAdmin(selectedUser.user)}
                    >
                        {selectedUser.user.is_admin ? 'Revoke Admin' : 'Grant Admin'}
                    </Button>
                    {#if selectedUser.user.banned_at}
                        <Button
                            variant="outline"
                            class="text-green-600"
                            onclick={() => handleUnbanUser(selectedUser.user)}
                        >
                            Unban User
                        </Button>
                    {:else}
                        <Button
                            variant="outline"
                            class="text-red-600"
                            onclick={() => handleBanUser(selectedUser.user)}
                        >
                            Ban User
                        </Button>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}
