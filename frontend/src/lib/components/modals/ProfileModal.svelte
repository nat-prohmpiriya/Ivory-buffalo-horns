<script lang="ts">
    import { t } from 'svelte-i18n';
    import * as Dialog from '$lib/components/ui/dialog';
    import * as Tabs from '$lib/components/ui/tabs';
    import { Button } from '$lib/components/ui/button';
    import { Input } from '$lib/components/ui/input';
    import { Label } from '$lib/components/ui/label';
    import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
    import { toast } from 'svelte-sonner';
    import { authStore } from '$lib/stores/auth';
    import { villageStore } from '$lib/stores/village';
    import { allianceStore } from '$lib/stores/alliance';
    import { api } from '$lib/api/client';
    import { goto } from '$app/navigation';

    interface Props {
        open?: boolean;
    }

    let { open = $bindable(false) }: Props = $props();

    let authState = $state(authStore);
    let user = $derived($authState.user);
    let backendUser = $derived($authState.backendUser);
    let villages = $derived($authState.villages);

    let villageState = $state(villageStore);
    let allVillages = $derived($villageState.villages);

    let allianceState = $state(allianceStore);
    let myAlliance = $derived($allianceState.myAlliance);

    let activeTab = $state('profile');

    // Edit name state
    let isEditingName = $state(false);
    let newDisplayName = $state('');

    // Delete account state
    let showDeleteConfirm = $state(false);
    let deleteConfirmText = $state('');

    // Stats (would come from ranking API)
    let stats = $state({
        attackPoints: 0,
        defensePoints: 0,
        raidedResources: 0,
    });

    $effect(() => {
        if (open && backendUser) {
            newDisplayName = backendUser.display_name || '';
            loadStats();
        }
    });

    async function loadStats() {
        try {
            // Try to get ranking stats for current user
            const response = await api.get<any[]>('/api/rankings/players/population?limit=100');
            const myRank = response.find((r: any) => r.user_id === backendUser?.id);
            if (myRank) {
                stats = {
                    attackPoints: myRank.attack_points || 0,
                    defensePoints: myRank.defense_points || 0,
                    raidedResources: myRank.raided_resources || 0,
                };
            }
        } catch (error) {
            console.error('Failed to load stats:', error);
        }
    }

    
    function formatDate(dateStr: string | undefined): string {
        if (!dateStr) return 'Unknown';
        return new Date(dateStr).toLocaleDateString();
    }

    function getTotalPopulation(): number {
        return allVillages.reduce((sum, v) => sum + (v.population || 0), 0);
    }

    async function handleSaveName() {
        if (!newDisplayName.trim()) {
            toast.error('Name cannot be empty');
            return;
        }

        try {
            await api.put('/api/auth/profile', {
                display_name: newDisplayName.trim(),
            });

            // Re-sync with backend to update the store
            await authStore.syncWithBackend();

            isEditingName = false;
            toast.success('Name updated successfully');
        } catch (error: any) {
            toast.error('Failed to update name', {
                description: error.message,
            });
        }
    }

    async function handleDeleteAccount() {
        if (deleteConfirmText !== 'DELETE') {
            toast.error('Please type DELETE to confirm');
            return;
        }

        try {
            await api.delete('/api/auth/account');
            await authStore.logout();
            toast.success('Account deleted');
            open = false;
            goto('/');
        } catch (error: any) {
            toast.error('Failed to delete account', {
                description: error.message,
            });
        }
    }

    async function handleLogout() {
        try {
            await authStore.logout();
            open = false;
            goto('/');
        } catch (error: any) {
            toast.error('Failed to logout', {
                description: error.message,
            });
        }
    }
</script>

<Dialog.Root bind:open>
    <Dialog.Content class="max-w-2xl max-h-[85vh] overflow-hidden flex flex-col">
        <Dialog.Header>
            <Dialog.Title class="flex items-center gap-2">
                <span class="text-2xl">👤</span>
                {$t('profile.title')}
            </Dialog.Title>
        </Dialog.Header>

        <Tabs.Root bind:value={activeTab} class="flex-1 overflow-hidden flex flex-col">
            <Tabs.List class="grid w-full grid-cols-3">
                <Tabs.Trigger value="profile">{$t('profile.title')}</Tabs.Trigger>
                <Tabs.Trigger value="stats">{$t('profile.stats')}</Tabs.Trigger>
                <Tabs.Trigger value="settings">{$t('nav.settings')}</Tabs.Trigger>
            </Tabs.List>

            <div class="flex-1 overflow-y-auto mt-4">
                <!-- Profile Tab -->
                <Tabs.Content value="profile" class="space-y-4">
                    <!-- User Info Card -->
                    <Card>
                        <CardContent class="pt-6">
                            <div class="flex items-start gap-4">
                                <!-- Avatar -->
                                <div class="flex-shrink-0">
                                    {#if user?.photoURL}
                                        <img
                                            src={user.photoURL}
                                            alt="Profile"
                                            class="w-20 h-20 rounded-full border-2 border-primary"
                                        />
                                    {:else}
                                        <div class="w-20 h-20 rounded-full bg-muted flex items-center justify-center text-3xl">
                                            👤
                                        </div>
                                    {/if}
                                </div>

                                <!-- Info -->
                                <div class="flex-1 space-y-2">
                                    <div class="flex items-center gap-2">
                                        {#if isEditingName}
                                            <Input
                                                bind:value={newDisplayName}
                                                class="max-w-xs"
                                                placeholder={$t('profile.playerName')}
                                            />
                                            <Button size="sm" onclick={handleSaveName}>
                                                {$t('common.save')}
                                            </Button>
                                            <Button size="sm" variant="ghost" onclick={() => isEditingName = false}>
                                                {$t('common.cancel')}
                                            </Button>
                                        {:else}
                                            <h3 class="text-xl font-bold">
                                                {backendUser?.display_name || user?.displayName || 'Player'}
                                            </h3>
                                            <Button size="sm" variant="ghost" onclick={() => isEditingName = true}>
                                                ✏️
                                            </Button>
                                        {/if}
                                    </div>

                                    <p class="text-sm text-muted-foreground">
                                        {user?.email || backendUser?.email}
                                    </p>

                                    <div class="flex flex-wrap gap-4 text-sm">
                                        <div>
                                            <span class="text-muted-foreground">{$t('profile.joined')}:</span>
                                            <span class="ml-1 font-medium">{formatDate(backendUser?.created_at)}</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </CardContent>
                    </Card>

                    <!-- Villages Info -->
                    <Card>
                        <CardHeader class="pb-2">
                            <CardTitle class="text-lg flex items-center gap-2">
                                <span>🏘️</span> {$t('profile.villages')}
                            </CardTitle>
                        </CardHeader>
                        <CardContent>
                            <div class="grid grid-cols-2 gap-4">
                                <div class="text-center p-4 bg-muted rounded-lg">
                                    <div class="text-3xl font-bold text-primary">{allVillages.length}</div>
                                    <div class="text-sm text-muted-foreground">{$t('profile.villages')}</div>
                                </div>
                                <div class="text-center p-4 bg-muted rounded-lg">
                                    <div class="text-3xl font-bold text-primary">{getTotalPopulation()}</div>
                                    <div class="text-sm text-muted-foreground">{$t('profile.population')}</div>
                                </div>
                            </div>

                            {#if allVillages.length > 0}
                                <div class="mt-4 space-y-2">
                                    {#each allVillages as village}
                                        <div class="flex items-center justify-between p-2 bg-muted/50 rounded">
                                            <div class="flex items-center gap-2">
                                                <span>{village.is_capital ? '👑' : '🏠'}</span>
                                                <span class="font-medium">{village.name}</span>
                                                <span class="text-xs text-muted-foreground">
                                                    ({village.x}|{village.y})
                                                </span>
                                            </div>
                                            <span class="text-sm">Pop: {village.population || 0}</span>
                                        </div>
                                    {/each}
                                </div>
                            {/if}
                        </CardContent>
                    </Card>

                    <!-- Alliance Info -->
                    <Card>
                        <CardHeader class="pb-2">
                            <CardTitle class="text-lg flex items-center gap-2">
                                <span>🏰</span> {$t('profile.alliance')}
                            </CardTitle>
                        </CardHeader>
                        <CardContent>
                            {#if myAlliance}
                                <div class="flex items-center gap-4">
                                    <div class="text-2xl font-bold text-primary">[{myAlliance.tag}]</div>
                                    <div>
                                        <div class="font-medium">{myAlliance.name}</div>
                                        <div class="text-sm text-muted-foreground">
                                            {myAlliance.member_count} members
                                        </div>
                                    </div>
                                </div>
                            {:else}
                                <p class="text-muted-foreground">{$t('profile.noAlliance')}</p>
                            {/if}
                        </CardContent>
                    </Card>
                </Tabs.Content>

                <!-- Stats Tab -->
                <Tabs.Content value="stats" class="space-y-4">
                    <Card>
                        <CardHeader>
                            <CardTitle class="flex items-center gap-2">
                                <span>📊</span> {$t('profile.stats')}
                            </CardTitle>
                        </CardHeader>
                        <CardContent>
                            <div class="grid grid-cols-3 gap-4">
                                <div class="text-center p-4 bg-muted rounded-lg">
                                    <div class="text-2xl font-bold text-red-500">⚔️ {stats.attackPoints}</div>
                                    <div class="text-sm text-muted-foreground">{$t('profile.attackPoints')}</div>
                                </div>
                                <div class="text-center p-4 bg-muted rounded-lg">
                                    <div class="text-2xl font-bold text-blue-500">🛡️ {stats.defensePoints}</div>
                                    <div class="text-sm text-muted-foreground">{$t('profile.defensePoints')}</div>
                                </div>
                                <div class="text-center p-4 bg-muted rounded-lg">
                                    <div class="text-2xl font-bold text-yellow-500">💰 {stats.raidedResources}</div>
                                    <div class="text-sm text-muted-foreground">{$t('profile.raidedResources')}</div>
                                </div>
                            </div>
                        </CardContent>
                    </Card>
                </Tabs.Content>

                <!-- Settings Tab -->
                <Tabs.Content value="settings" class="space-y-4">
                    <!-- Logout -->
                    <Card>
                        <CardContent class="pt-6">
                            <Button variant="outline" class="w-full" onclick={handleLogout}>
                                🚪 {$t('nav.logout')}
                            </Button>
                        </CardContent>
                    </Card>

                    <!-- Danger Zone -->
                    <Card class="border-destructive/50">
                        <CardHeader>
                            <CardTitle class="text-destructive flex items-center gap-2">
                                <span>⚠️</span> Danger Zone
                            </CardTitle>
                        </CardHeader>
                        <CardContent class="space-y-4">
                            <p class="text-sm text-muted-foreground">
                                {$t('profile.deleteAccountWarning')}
                            </p>

                            {#if !showDeleteConfirm}
                                <Button
                                    variant="destructive"
                                    class="w-full"
                                    onclick={() => showDeleteConfirm = true}
                                >
                                    {$t('profile.deleteAccount')}
                                </Button>
                            {:else}
                                <div class="space-y-2">
                                    <Label>{$t('profile.confirmDelete')}</Label>
                                    <Input
                                        bind:value={deleteConfirmText}
                                        placeholder="DELETE"
                                    />
                                    <div class="flex gap-2">
                                        <Button
                                            variant="destructive"
                                            onclick={handleDeleteAccount}
                                            disabled={deleteConfirmText !== 'DELETE'}
                                        >
                                            {$t('common.confirm')}
                                        </Button>
                                        <Button
                                            variant="ghost"
                                            onclick={() => {
                                                showDeleteConfirm = false;
                                                deleteConfirmText = '';
                                            }}
                                        >
                                            {$t('common.cancel')}
                                        </Button>
                                    </div>
                                </div>
                            {/if}
                        </CardContent>
                    </Card>
                </Tabs.Content>
            </div>
        </Tabs.Root>
    </Dialog.Content>
</Dialog.Root>
