<script lang="ts">
    import { onMount } from 'svelte';
    import { Button } from '$lib/components/ui/button';
    import { Input } from '$lib/components/ui/input';
    import * as Dialog from '$lib/components/ui/dialog';
    import {
        adminStore,
        formatDate,
        getStatusBadge,
        getProviderIcon,
        type AdminUserResponse,
    } from '$lib/stores/admin';

    let adminState = $state(adminStore);
    let users = $derived($adminState.users);
    let selectedUser = $derived($adminState.selectedUser);
    let loading = $derived($adminState.loading);
    let currentPage = $derived($adminState.currentPage);

    let searchQuery = $state('');
    let showUserDetail = $state(false);
    let filterStatus = $state<'all' | 'active' | 'banned' | 'admin'>('all');

    onMount(() => {
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

    function closeUserDetail() {
        showUserDetail = false;
        adminStore.clearSelectedUser();
    }

    const filteredUsers = $derived(
        users.filter(user => {
            if (filterStatus === 'all') return true;
            if (filterStatus === 'banned') return user.banned_at !== null;
            if (filterStatus === 'admin') return user.is_admin;
            if (filterStatus === 'active') return !user.banned_at && !user.is_admin;
            return true;
        })
    );
</script>

<div class="space-y-4">
    <div class="flex items-center justify-between">
        <h1 class="text-2xl font-bold">User Management</h1>
    </div>

    <!-- Filters -->
    <div class="flex items-center gap-4 flex-wrap">
        <div class="flex-1 max-w-md flex gap-2">
            <Input
                placeholder="Search by name or email..."
                bind:value={searchQuery}
                onkeydown={(e) => e.key === 'Enter' && handleSearch()}
            />
            <Button onclick={handleSearch} disabled={loading}>Search</Button>
        </div>

        <div class="flex items-center gap-2">
            <span class="text-sm text-muted-foreground">Filter:</span>
            <select
                bind:value={filterStatus}
                class="h-9 rounded-md border border-input bg-background px-3 text-sm"
            >
                <option value="all">All Users</option>
                <option value="active">Active</option>
                <option value="banned">Banned</option>
                <option value="admin">Admins</option>
            </select>
        </div>
    </div>

    <!-- Users Table -->
    <div class="bg-card border rounded-lg overflow-hidden">
        <table class="w-full">
            <thead class="bg-muted">
                <tr>
                    <th class="px-4 py-3 text-left text-sm font-medium">User</th>
                    <th class="px-4 py-3 text-left text-sm font-medium">Email</th>
                    <th class="px-4 py-3 text-left text-sm font-medium">Status</th>
                    <th class="px-4 py-3 text-center text-sm font-medium">Villages</th>
                    <th class="px-4 py-3 text-left text-sm font-medium">Last Login</th>
                    <th class="px-4 py-3 text-right text-sm font-medium">Actions</th>
                </tr>
            </thead>
            <tbody>
                {#if loading && users.length === 0}
                    <tr>
                        <td colspan="6" class="px-4 py-12 text-center text-muted-foreground">
                            Loading users...
                        </td>
                    </tr>
                {:else if filteredUsers.length === 0}
                    <tr>
                        <td colspan="6" class="px-4 py-12 text-center text-muted-foreground">
                            No users found
                        </td>
                    </tr>
                {:else}
                    {#each filteredUsers as user}
                        {@const status = getStatusBadge(user)}
                        <tr class="border-t hover:bg-muted/50">
                            <td class="px-4 py-3">
                                <div class="flex items-center gap-3">
                                    {#if user.photo_url}
                                        <img
                                            src={user.photo_url}
                                            alt=""
                                            class="w-10 h-10 rounded-full"
                                        />
                                    {:else}
                                        <div class="w-10 h-10 rounded-full bg-muted flex items-center justify-center text-lg">
                                            {getProviderIcon(user.provider)}
                                        </div>
                                    {/if}
                                    <div>
                                        <div class="font-medium">
                                            {user.display_name || 'No name'}
                                        </div>
                                        <div class="text-xs text-muted-foreground font-mono">
                                            {user.id.slice(0, 8)}...
                                        </div>
                                    </div>
                                </div>
                            </td>
                            <td class="px-4 py-3 text-sm">
                                {user.email || '-'}
                            </td>
                            <td class="px-4 py-3">
                                <span class="px-2 py-0.5 rounded-full text-xs {status.class}">
                                    {status.label}
                                </span>
                            </td>
                            <td class="px-4 py-3 text-center">
                                {user.village_count}
                            </td>
                            <td class="px-4 py-3 text-sm text-muted-foreground">
                                {formatDate(user.last_login_at)}
                            </td>
                            <td class="px-4 py-3">
                                <div class="flex items-center justify-end gap-1">
                                    <Button
                                        size="sm"
                                        variant="ghost"
                                        onclick={() => handleViewUser(user.id)}
                                    >
                                        View
                                    </Button>
                                    <Button
                                        size="sm"
                                        variant="ghost"
                                        onclick={() => handleToggleAdmin(user)}
                                    >
                                        {user.is_admin ? '👑' : '👤'}
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

    <!-- Pagination -->
    <div class="flex items-center justify-between">
        <div class="text-sm text-muted-foreground">
            Showing {filteredUsers.length} users
        </div>
        <div class="flex items-center gap-2">
            <Button
                size="sm"
                variant="outline"
                disabled={currentPage <= 1 || loading}
                onclick={() => adminStore.loadUsers(currentPage - 1)}
            >
                Previous
            </Button>
            <span class="text-sm px-3">Page {currentPage}</span>
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
</div>

<!-- User Detail Dialog -->
<Dialog.Root bind:open={showUserDetail}>
    <Dialog.Content class="max-w-2xl max-h-[85vh] overflow-y-auto">
        {#if selectedUser}
            <Dialog.Header>
                <Dialog.Title>Player Detail</Dialog.Title>
            </Dialog.Header>

            <div class="space-y-4 mt-4">
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

                <!-- Details Grid -->
                <div class="grid grid-cols-2 gap-4 text-sm border rounded-lg p-4">
                    <div>
                        <span class="text-muted-foreground">User ID</span>
                        <p class="font-mono text-xs break-all">{selectedUser.user.id}</p>
                    </div>
                    <div>
                        <span class="text-muted-foreground">Firebase UID</span>
                        <p class="font-mono text-xs break-all">{selectedUser.user.firebase_uid}</p>
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
                    <div>
                        <span class="text-muted-foreground">Villages</span>
                        <p>{selectedUser.villages.length}</p>
                    </div>
                </div>

                {#if selectedUser.user.banned_at}
                    <div class="bg-red-50 border border-red-200 rounded-lg p-4">
                        <p class="text-red-800 font-medium">Banned</p>
                        <p class="text-sm text-red-700">
                            Reason: {selectedUser.user.banned_reason || 'No reason provided'}
                        </p>
                        <p class="text-xs text-red-600 mt-1">
                            Since: {formatDate(selectedUser.user.banned_at)}
                        </p>
                    </div>
                {/if}

                <!-- Alliance -->
                {#if selectedUser.alliance}
                    <div class="border rounded-lg p-4">
                        <h4 class="font-medium mb-2">Alliance</h4>
                        <div class="flex items-center justify-between">
                            <div>
                                <span class="text-muted-foreground">[{selectedUser.alliance.tag}]</span>
                                <span class="ml-1">{selectedUser.alliance.name}</span>
                            </div>
                            <span class="text-sm capitalize">{selectedUser.alliance.role}</span>
                        </div>
                    </div>
                {/if}

                <!-- Villages -->
                <div class="border rounded-lg p-4">
                    <h4 class="font-medium mb-2">Villages ({selectedUser.villages.length})</h4>
                    <div class="space-y-2 max-h-48 overflow-y-auto">
                        {#each selectedUser.villages as village}
                            <div class="flex items-center justify-between text-sm bg-muted/50 rounded p-3">
                                <div class="flex-1">
                                    <div class="flex items-center gap-2">
                                        <span class="font-medium">{village.name}</span>
                                        {#if village.is_capital}
                                            <span class="text-yellow-600">👑</span>
                                        {/if}
                                        <span class="text-muted-foreground">({village.x}|{village.y})</span>
                                    </div>
                                    <div class="text-xs text-muted-foreground mt-1">
                                        🪵 {village.wood} | 🧱 {village.clay} | ⛏️ {village.iron} | 🌾 {village.crop}
                                    </div>
                                </div>
                                <div class="text-right">
                                    <div class="font-medium">Pop: {village.population}</div>
                                    <div class="text-xs text-muted-foreground font-mono">
                                        {village.id.slice(0, 8)}
                                    </div>
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>

                <!-- Heroes -->
                {#if selectedUser.heroes.length > 0}
                    <div class="border rounded-lg p-4">
                        <h4 class="font-medium mb-2">Heroes ({selectedUser.heroes.length})</h4>
                        <div class="space-y-2">
                            {#each selectedUser.heroes as hero}
                                <div class="flex items-center justify-between text-sm bg-muted/50 rounded p-3">
                                    <div>
                                        <span class="font-medium">{hero.name}</span>
                                        <span class="text-purple-600 ml-2">Lv.{hero.level}</span>
                                    </div>
                                    <div class="text-right">
                                        <span class="text-muted-foreground">HP: {hero.health}%</span>
                                        <span class="ml-2 capitalize">{hero.status}</span>
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
                            class="text-green-600 border-green-600"
                            onclick={() => handleUnbanUser(selectedUser.user)}
                        >
                            Unban User
                        </Button>
                    {:else}
                        <Button
                            variant="outline"
                            class="text-red-600 border-red-600"
                            onclick={() => handleBanUser(selectedUser.user)}
                        >
                            Ban User
                        </Button>
                    {/if}
                    <Button variant="ghost" class="ml-auto" onclick={closeUserDetail}>
                        Close
                    </Button>
                </div>
            </div>
        {/if}
    </Dialog.Content>
</Dialog.Root>
