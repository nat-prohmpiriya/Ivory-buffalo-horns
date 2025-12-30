<script lang="ts">
    import * as Dialog from '$lib/components/ui/dialog';
    import { Button } from '$lib/components/ui/button';
    import { Separator } from '$lib/components/ui/separator';
    import { Input } from '$lib/components/ui/input';
    import { Label } from '$lib/components/ui/label';
    import {
        allianceStore,
        type Alliance,
        type AllianceMember,
        type AllianceInvitation,
        type AllianceDiplomacy,
        type AllianceListItem,
        type AllianceRole,
        type DiplomacyStatus,
        getRoleLabel,
        getRoleIcon,
        getRoleColor,
        getDiplomacyLabel,
        getDiplomacyIcon,
        getDiplomacyColor,
        formatPopulation,
    } from '$lib/stores/alliance';
    import { authStore } from '$lib/stores/auth';

    interface Props {
        open: boolean;
    }

    let { open = $bindable(false) }: Props = $props();

    // Tabs
    type TabType = 'overview' | 'members' | 'invitations' | 'diplomacy' | 'browse';
    let activeTab = $state<TabType>('overview');

    // Store state
    let allianceState = $state(allianceStore);
    let myAlliance = $derived($allianceState.myAlliance);
    let members = $derived($allianceState.members);
    let invitations = $derived($allianceState.invitations);
    let diplomacy = $derived($allianceState.diplomacy);
    let allAlliances = $derived($allianceState.allAlliances);
    let loading = $derived($allianceState.loading);

    let authState = $state(authStore);
    let backendUser = $derived($authState.backendUser);

    // Create alliance form
    let createName = $state('');
    let createTag = $state('');
    let createDescription = $state('');

    // Invite player form
    let invitePlayerId = $state('');
    let inviteMessage = $state('');

    // Diplomacy form
    let diplomacyTargetId = $state('');
    let diplomacyStatus = $state<DiplomacyStatus>('neutral');

    // Permissions
    const isLeader = $derived(myAlliance && backendUser?.id === myAlliance.leader_id);
    const isOfficer = $derived(
        members.find(m => m.user_id === backendUser?.id)?.role === 'officer'
    );
    const canManage = $derived(isLeader || isOfficer);

    // Load data when modal opens
    $effect(() => {
        if (open) {
            loadData();
        }
    });

    async function loadData() {
        try {
            await Promise.all([
                allianceStore.loadMyAlliance(),
                allianceStore.loadInvitations(),
                allianceStore.loadAllAlliances(),
            ]);

            // If has alliance, load members and diplomacy
            const alliance = $allianceState.myAlliance;
            if (alliance) {
                await Promise.all([
                    allianceStore.loadMembers(alliance.id),
                    allianceStore.loadDiplomacy(alliance.id),
                ]);
            }
        } catch (error) {
            console.error('Failed to load alliance data:', error);
        }
    }

    // Create alliance
    async function handleCreateAlliance() {
        if (!createName.trim() || !createTag.trim()) return;

        try {
            await allianceStore.createAlliance({
                name: createName.trim(),
                tag: createTag.trim().toUpperCase(),
                description: createDescription.trim() || undefined,
            });
            // Reset form
            createName = '';
            createTag = '';
            createDescription = '';
            // Reload data
            await loadData();
        } catch {
            // Error handled in store
        }
    }

    // Leave alliance
    async function handleLeaveAlliance() {
        if (!confirm('Are you sure you want to leave this alliance?')) return;

        try {
            await allianceStore.leaveAlliance();
        } catch {
            // Error handled in store
        }
    }

    // Disband alliance
    async function handleDisbandAlliance() {
        if (!myAlliance) return;
        if (!confirm('Are you sure you want to DISBAND this alliance? This cannot be undone!')) return;

        try {
            await allianceStore.disbandAlliance(myAlliance.id);
        } catch {
            // Error handled in store
        }
    }

    // Kick member
    async function handleKickMember(userId: string) {
        if (!myAlliance) return;
        if (!confirm('Kick this member?')) return;

        try {
            await allianceStore.kickMember(myAlliance.id, userId);
        } catch {
            // Error handled in store
        }
    }

    // Update member role
    async function handleUpdateRole(userId: string, newRole: AllianceRole) {
        if (!myAlliance) return;

        try {
            await allianceStore.updateMemberRole(myAlliance.id, userId, newRole);
        } catch {
            // Error handled in store
        }
    }

    // Invite player
    async function handleInvitePlayer() {
        if (!myAlliance || !invitePlayerId.trim()) return;

        try {
            await allianceStore.invitePlayer(myAlliance.id, {
                player_id: invitePlayerId.trim(),
                message: inviteMessage.trim() || undefined,
            });
            // Reset form
            invitePlayerId = '';
            inviteMessage = '';
        } catch {
            // Error handled in store
        }
    }

    // Respond to invitation
    async function handleRespondInvitation(invitationId: string, accept: boolean) {
        try {
            await allianceStore.respondInvitation(invitationId, accept);
            if (accept) {
                await loadData();
            }
        } catch {
            // Error handled in store
        }
    }

    // Set diplomacy
    async function handleSetDiplomacy() {
        if (!myAlliance || !diplomacyTargetId) return;

        try {
            await allianceStore.setDiplomacy(myAlliance.id, {
                target_alliance_id: diplomacyTargetId,
                status: diplomacyStatus,
            });
            // Reset form
            diplomacyTargetId = '';
            diplomacyStatus = 'neutral';
        } catch {
            // Error handled in store
        }
    }

    const diplomacyOptions: DiplomacyStatus[] = ['neutral', 'ally', 'nap', 'enemy'];
</script>

<Dialog.Root bind:open>
    <Dialog.Content class="sm:max-w-3xl max-h-[90vh] flex flex-col">
        <Dialog.Header>
            <div class="flex items-center gap-3">
                <div class="w-12 h-12 rounded-xl bg-muted flex items-center justify-center text-2xl">
                    🏰
                </div>
                <div>
                    <Dialog.Title class="text-xl">
                        {#if myAlliance}
                            [{myAlliance.tag}] {myAlliance.name}
                        {:else}
                            Alliance
                        {/if}
                    </Dialog.Title>
                    <Dialog.Description>
                        {#if myAlliance}
                            {myAlliance.member_count} / {myAlliance.max_members} members
                        {:else}
                            Join or create an alliance
                        {/if}
                    </Dialog.Description>
                </div>
            </div>
        </Dialog.Header>

        <!-- Tabs -->
        <div class="flex gap-2 mt-4 flex-wrap">
            <Button
                variant={activeTab === 'overview' ? 'default' : 'outline'}
                size="sm"
                onclick={() => activeTab = 'overview'}
            >
                <span class="mr-1">🏠</span> Overview
            </Button>
            {#if myAlliance}
                <Button
                    variant={activeTab === 'members' ? 'default' : 'outline'}
                    size="sm"
                    onclick={() => activeTab = 'members'}
                >
                    <span class="mr-1">👥</span> Members
                </Button>
                <Button
                    variant={activeTab === 'diplomacy' ? 'default' : 'outline'}
                    size="sm"
                    onclick={() => activeTab = 'diplomacy'}
                >
                    <span class="mr-1">🤝</span> Diplomacy
                </Button>
            {/if}
            <Button
                variant={activeTab === 'invitations' ? 'default' : 'outline'}
                size="sm"
                onclick={() => activeTab = 'invitations'}
            >
                <span class="mr-1">📨</span> Invitations
                {#if invitations.length > 0}
                    <span class="ml-1 px-1.5 py-0.5 text-xs bg-destructive text-destructive-foreground rounded-full">
                        {invitations.length}
                    </span>
                {/if}
            </Button>
            <Button
                variant={activeTab === 'browse' ? 'default' : 'outline'}
                size="sm"
                onclick={() => activeTab = 'browse'}
            >
                <span class="mr-1">🔍</span> Browse
            </Button>
        </div>

        <!-- Content -->
        <div class="flex-1 overflow-y-auto mt-4 min-h-[400px]">
            {#if loading && !myAlliance && activeTab === 'overview'}
                <div class="flex items-center justify-center py-12">
                    <span class="animate-spin text-2xl mr-2">⏳</span>
                    <span>Loading...</span>
                </div>

            {:else if activeTab === 'overview'}
                {#if myAlliance}
                    <!-- Alliance Info -->
                    <div class="space-y-4">
                        <div class="p-4 rounded-lg bg-muted">
                            <div class="flex items-center gap-3 mb-3">
                                <div class="w-16 h-16 rounded-lg bg-primary/20 flex items-center justify-center text-2xl font-bold">
                                    {myAlliance.tag}
                                </div>
                                <div>
                                    <h3 class="text-lg font-semibold">{myAlliance.name}</h3>
                                    <p class="text-sm text-muted-foreground">
                                        {myAlliance.member_count} members
                                    </p>
                                </div>
                            </div>
                            {#if myAlliance.description}
                                <p class="text-sm">{myAlliance.description}</p>
                            {/if}
                        </div>

                        <!-- Quick Stats -->
                        <div class="grid grid-cols-3 gap-3">
                            <div class="p-3 rounded-lg border text-center">
                                <div class="text-2xl font-bold">{myAlliance.member_count}</div>
                                <div class="text-xs text-muted-foreground">Members</div>
                            </div>
                            <div class="p-3 rounded-lg border text-center">
                                <div class="text-2xl font-bold">{diplomacy.filter(d => d.status === 'ally').length}</div>
                                <div class="text-xs text-muted-foreground">Allies</div>
                            </div>
                            <div class="p-3 rounded-lg border text-center">
                                <div class="text-2xl font-bold">{diplomacy.filter(d => d.status === 'enemy').length}</div>
                                <div class="text-xs text-muted-foreground">Enemies</div>
                            </div>
                        </div>

                        <Separator />

                        <!-- Actions -->
                        <div class="space-y-2">
                            <Button
                                variant="outline"
                                class="w-full justify-start"
                                onclick={handleLeaveAlliance}
                                disabled={loading || isLeader}
                            >
                                <span class="mr-2">🚪</span>
                                Leave Alliance
                                {#if isLeader}
                                    <span class="text-xs text-muted-foreground ml-auto">(Transfer leadership first)</span>
                                {/if}
                            </Button>
                            {#if isLeader}
                                <Button
                                    variant="destructive"
                                    class="w-full justify-start"
                                    onclick={handleDisbandAlliance}
                                    disabled={loading}
                                >
                                    <span class="mr-2">💥</span>
                                    Disband Alliance
                                </Button>
                            {/if}
                        </div>
                    </div>
                {:else}
                    <!-- Create Alliance Form -->
                    <div class="max-w-md mx-auto space-y-4">
                        <div class="text-center mb-6">
                            <div class="text-4xl mb-2">🏰</div>
                            <h3 class="text-lg font-semibold">Create an Alliance</h3>
                            <p class="text-sm text-muted-foreground">
                                Unite with other players to conquer the world
                            </p>
                        </div>

                        <div>
                            <Label for="alliance-name">Alliance Name</Label>
                            <Input
                                id="alliance-name"
                                bind:value={createName}
                                placeholder="Enter alliance name"
                                maxlength={50}
                                class="mt-1"
                            />
                        </div>

                        <div>
                            <Label for="alliance-tag">Tag (2-4 characters)</Label>
                            <Input
                                id="alliance-tag"
                                bind:value={createTag}
                                placeholder="ABC"
                                maxlength={4}
                                class="mt-1 uppercase"
                            />
                        </div>

                        <div>
                            <Label for="alliance-desc">Description (optional)</Label>
                            <textarea
                                id="alliance-desc"
                                bind:value={createDescription}
                                placeholder="Describe your alliance..."
                                maxlength={500}
                                rows={3}
                                class="w-full mt-1 px-3 py-2 border rounded-md bg-background resize-none"
                            ></textarea>
                        </div>

                        <Button
                            class="w-full"
                            onclick={handleCreateAlliance}
                            disabled={loading || !createName.trim() || createTag.length < 2}
                        >
                            {#if loading}
                                <span class="animate-spin mr-2">⏳</span>
                            {/if}
                            Create Alliance
                        </Button>
                    </div>
                {/if}

            {:else if activeTab === 'members'}
                <div class="space-y-3">
                    {#if canManage}
                        <!-- Invite Form -->
                        <div class="p-4 rounded-lg border bg-muted/50">
                            <h4 class="font-medium mb-3">Invite Player</h4>
                            <div class="flex gap-2">
                                <Input
                                    bind:value={invitePlayerId}
                                    placeholder="Player ID"
                                    class="flex-1"
                                />
                                <Button
                                    onclick={handleInvitePlayer}
                                    disabled={loading || !invitePlayerId.trim()}
                                >
                                    Invite
                                </Button>
                            </div>
                        </div>
                        <Separator />
                    {/if}

                    <!-- Members List -->
                    {#if members.length === 0}
                        <div class="text-center py-8 text-muted-foreground">
                            <p class="text-4xl mb-2">👥</p>
                            <p>No members yet</p>
                        </div>
                    {:else}
                        {#each members as member (member.id)}
                            <div class="p-3 rounded-lg border flex items-center gap-3">
                                <div class="w-10 h-10 rounded-full bg-muted flex items-center justify-center text-lg">
                                    {getRoleIcon(member.role)}
                                </div>
                                <div class="flex-1">
                                    <div class="flex items-center gap-2">
                                        <span class="font-medium">{member.player_name}</span>
                                        <span class="text-xs px-2 py-0.5 rounded-full bg-muted {getRoleColor(member.role)}">
                                            {getRoleLabel(member.role)}
                                        </span>
                                    </div>
                                    <div class="text-xs text-muted-foreground">
                                        {member.villages_count} villages · Pop: {formatPopulation(member.population)}
                                    </div>
                                </div>

                                {#if isLeader && member.user_id !== backendUser?.id}
                                    <div class="flex gap-1">
                                        <select
                                            class="text-xs px-2 py-1 border rounded bg-background"
                                            value={member.role}
                                            onchange={(e) => handleUpdateRole(member.user_id, e.currentTarget.value as AllianceRole)}
                                        >
                                            <option value="member">Member</option>
                                            <option value="officer">Officer</option>
                                        </select>
                                        <Button
                                            variant="ghost"
                                            size="sm"
                                            onclick={() => handleKickMember(member.user_id)}
                                        >
                                            ❌
                                        </Button>
                                    </div>
                                {/if}
                            </div>
                        {/each}
                    {/if}
                </div>

            {:else if activeTab === 'diplomacy'}
                <div class="space-y-3">
                    {#if canManage}
                        <!-- Set Diplomacy Form -->
                        <div class="p-4 rounded-lg border bg-muted/50">
                            <h4 class="font-medium mb-3">Set Diplomacy</h4>
                            <div class="flex gap-2">
                                <select
                                    class="flex-1 px-3 py-2 border rounded-md bg-background"
                                    bind:value={diplomacyTargetId}
                                >
                                    <option value="">Select alliance...</option>
                                    {#each allAlliances.filter(a => a.id !== myAlliance?.id) as alliance}
                                        <option value={alliance.id}>[{alliance.tag}] {alliance.name}</option>
                                    {/each}
                                </select>
                                <select
                                    class="px-3 py-2 border rounded-md bg-background"
                                    bind:value={diplomacyStatus}
                                >
                                    {#each diplomacyOptions as status}
                                        <option value={status}>{getDiplomacyIcon(status)} {getDiplomacyLabel(status)}</option>
                                    {/each}
                                </select>
                                <Button
                                    onclick={handleSetDiplomacy}
                                    disabled={loading || !diplomacyTargetId}
                                >
                                    Set
                                </Button>
                            </div>
                        </div>
                        <Separator />
                    {/if}

                    <!-- Diplomacy List -->
                    {#if diplomacy.length === 0}
                        <div class="text-center py-8 text-muted-foreground">
                            <p class="text-4xl mb-2">🤝</p>
                            <p>No diplomatic relations</p>
                        </div>
                    {:else}
                        {#each diplomacy as rel (rel.id)}
                            <div class="p-3 rounded-lg border flex items-center gap-3">
                                <div class="w-10 h-10 rounded-lg bg-muted flex items-center justify-center text-xl">
                                    {getDiplomacyIcon(rel.status)}
                                </div>
                                <div class="flex-1">
                                    <div class="font-medium">
                                        {#if rel.target_alliance_name}
                                            [{rel.target_alliance_tag}] {rel.target_alliance_name}
                                        {:else}
                                            Alliance #{rel.target_alliance_id.slice(0, 8)}
                                        {/if}
                                    </div>
                                    <div class="text-sm {getDiplomacyColor(rel.status)}">
                                        {getDiplomacyLabel(rel.status)}
                                    </div>
                                </div>
                            </div>
                        {/each}
                    {/if}
                </div>

            {:else if activeTab === 'invitations'}
                <div class="space-y-3">
                    {#if invitations.length === 0}
                        <div class="text-center py-8 text-muted-foreground">
                            <p class="text-4xl mb-2">📨</p>
                            <p>No pending invitations</p>
                        </div>
                    {:else}
                        {#each invitations as invitation (invitation.id)}
                            <div class="p-4 rounded-lg border">
                                <div class="flex items-start justify-between gap-3">
                                    <div>
                                        <div class="font-medium">
                                            {#if invitation.alliance_name}
                                                [{invitation.alliance_tag}] {invitation.alliance_name}
                                            {:else}
                                                Alliance Invitation
                                            {/if}
                                        </div>
                                        {#if invitation.message}
                                            <p class="text-sm text-muted-foreground mt-1">
                                                "{invitation.message}"
                                            </p>
                                        {/if}
                                        <div class="text-xs text-muted-foreground mt-2">
                                            Expires: {new Date(invitation.expires_at).toLocaleDateString()}
                                        </div>
                                    </div>
                                    <div class="flex gap-2">
                                        <Button
                                            size="sm"
                                            onclick={() => handleRespondInvitation(invitation.id, true)}
                                            disabled={loading || !!myAlliance}
                                        >
                                            Accept
                                        </Button>
                                        <Button
                                            variant="outline"
                                            size="sm"
                                            onclick={() => handleRespondInvitation(invitation.id, false)}
                                            disabled={loading}
                                        >
                                            Decline
                                        </Button>
                                    </div>
                                </div>
                            </div>
                        {/each}
                    {/if}
                </div>

            {:else if activeTab === 'browse'}
                <div class="space-y-3">
                    <div class="flex justify-between items-center mb-4">
                        <h4 class="font-medium">All Alliances</h4>
                        <Button
                            variant="outline"
                            size="sm"
                            onclick={() => allianceStore.loadAllAlliances()}
                        >
                            Refresh
                        </Button>
                    </div>

                    {#if allAlliances.length === 0}
                        <div class="text-center py-8 text-muted-foreground">
                            <p class="text-4xl mb-2">🔍</p>
                            <p>No alliances found</p>
                        </div>
                    {:else}
                        <div class="overflow-x-auto">
                            <table class="w-full text-sm">
                                <thead>
                                    <tr class="border-b">
                                        <th class="text-left py-2">Alliance</th>
                                        <th class="text-right py-2">Members</th>
                                        <th class="text-right py-2">Population</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {#each allAlliances as alliance (alliance.id)}
                                        <tr class="border-b hover:bg-muted/50">
                                            <td class="py-2">
                                                <span class="font-medium">[{alliance.tag}]</span>
                                                <span class="ml-1">{alliance.name}</span>
                                            </td>
                                            <td class="text-right py-2">{alliance.member_count}</td>
                                            <td class="text-right py-2">{formatPopulation(alliance.total_population)}</td>
                                        </tr>
                                    {/each}
                                </tbody>
                            </table>
                        </div>
                    {/if}
                </div>
            {/if}
        </div>

        <!-- Footer -->
        <Dialog.Footer class="mt-4">
            <Button variant="outline" onclick={() => open = false}>
                Close
            </Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
