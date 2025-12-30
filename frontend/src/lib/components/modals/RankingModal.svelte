<script lang="ts">
    import * as Dialog from '$lib/components/ui/dialog';
    import * as Tabs from '$lib/components/ui/tabs';
    import { Button } from '$lib/components/ui/button';
    import {
        rankingStore,
        getRankIcon,
        getRankClass,
        formatNumber,
        type RankingType,
    } from '$lib/stores/ranking';

    interface Props {
        open?: boolean;
    }

    let { open = $bindable(false) }: Props = $props();

    let rankingState = $state(rankingStore);
    let populationRankings = $derived($rankingState.populationRankings);
    let attackRankings = $derived($rankingState.attackRankings);
    let defenseRankings = $derived($rankingState.defenseRankings);
    let heroRankings = $derived($rankingState.heroRankings);
    let allianceRankings = $derived($rankingState.allianceRankings);
    let totals = $derived($rankingState.totals);
    let currentPage = $derived($rankingState.currentPage);
    let perPage = $derived($rankingState.perPage);
    let loading = $derived($rankingState.loading);

    let activeTab = $state<RankingType>('population');

    $effect(() => {
        if (open && populationRankings.length === 0) {
            rankingStore.loadPopulationRankings();
        }
    });

    function handleTabChange(tab: string) {
        activeTab = tab as RankingType;
        loadRankingForTab(tab as RankingType);
    }

    function loadRankingForTab(type: RankingType, page = 1) {
        switch (type) {
            case 'population':
                if (populationRankings.length === 0 || page !== currentPage.population) {
                    rankingStore.loadPopulationRankings(page, perPage);
                }
                break;
            case 'attackers':
                if (attackRankings.length === 0 || page !== currentPage.attackers) {
                    rankingStore.loadAttackRankings(page, perPage);
                }
                break;
            case 'defenders':
                if (defenseRankings.length === 0 || page !== currentPage.defenders) {
                    rankingStore.loadDefenseRankings(page, perPage);
                }
                break;
            case 'heroes':
                if (heroRankings.length === 0 || page !== currentPage.heroes) {
                    rankingStore.loadHeroRankings(page, perPage);
                }
                break;
            case 'alliances':
                if (allianceRankings.length === 0 || page !== currentPage.alliances) {
                    rankingStore.loadAllianceRankings(page, perPage);
                }
                break;
        }
    }

    function handlePageChange(type: RankingType, newPage: number) {
        loadRankingForTab(type, newPage);
    }

    function getTotalPages(type: RankingType): number {
        return Math.ceil(totals[type] / perPage);
    }
</script>

<Dialog.Root bind:open>
    <Dialog.Content class="max-w-2xl max-h-[85vh] overflow-y-auto">
        <Dialog.Header>
            <Dialog.Title class="flex items-center gap-2">
                <span class="text-xl">🏆</span>
                Leaderboard
            </Dialog.Title>
            <Dialog.Description>
                See how you rank against other players
            </Dialog.Description>
        </Dialog.Header>

        <Tabs.Root value={activeTab} onValueChange={handleTabChange} class="mt-4">
            <Tabs.List class="grid w-full grid-cols-5">
                <Tabs.Trigger value="population">👥 Pop</Tabs.Trigger>
                <Tabs.Trigger value="attackers">⚔️ ATK</Tabs.Trigger>
                <Tabs.Trigger value="defenders">🛡️ DEF</Tabs.Trigger>
                <Tabs.Trigger value="heroes">🦸 Heroes</Tabs.Trigger>
                <Tabs.Trigger value="alliances">🏰 Allies</Tabs.Trigger>
            </Tabs.List>

            <!-- Population Rankings -->
            <Tabs.Content value="population" class="mt-4">
                <div class="border rounded-lg overflow-hidden">
                    <table class="w-full">
                        <thead class="bg-muted">
                            <tr>
                                <th class="px-4 py-2 text-left text-sm font-medium w-16">Rank</th>
                                <th class="px-4 py-2 text-left text-sm font-medium">Player</th>
                                <th class="px-4 py-2 text-right text-sm font-medium">Villages</th>
                                <th class="px-4 py-2 text-right text-sm font-medium">Population</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#if loading && populationRankings.length === 0}
                                <tr>
                                    <td colspan="4" class="px-4 py-8 text-center text-muted-foreground">
                                        Loading...
                                    </td>
                                </tr>
                            {:else if populationRankings.length === 0}
                                <tr>
                                    <td colspan="4" class="px-4 py-8 text-center text-muted-foreground">
                                        No rankings available
                                    </td>
                                </tr>
                            {:else}
                                {#each populationRankings as player}
                                    <tr class="border-t hover:bg-muted/50">
                                        <td class="px-4 py-2 {getRankClass(player.rank)}">
                                            {getRankIcon(player.rank)}
                                        </td>
                                        <td class="px-4 py-2">
                                            <div class="font-medium">{player.display_name || 'Unknown'}</div>
                                            {#if player.alliance_tag}
                                                <div class="text-xs text-muted-foreground">[{player.alliance_tag}]</div>
                                            {/if}
                                        </td>
                                        <td class="px-4 py-2 text-right text-muted-foreground">
                                            {player.village_count}
                                        </td>
                                        <td class="px-4 py-2 text-right font-semibold">
                                            {formatNumber(player.population)}
                                        </td>
                                    </tr>
                                {/each}
                            {/if}
                        </tbody>
                    </table>
                </div>

                <!-- Pagination -->
                {#if getTotalPages('population') > 1}
                    <div class="flex items-center justify-center gap-2 mt-4">
                        <Button
                            size="sm"
                            variant="outline"
                            disabled={currentPage.population <= 1 || loading}
                            onclick={() => handlePageChange('population', currentPage.population - 1)}
                        >
                            Previous
                        </Button>
                        <span class="text-sm text-muted-foreground">
                            Page {currentPage.population} of {getTotalPages('population')}
                        </span>
                        <Button
                            size="sm"
                            variant="outline"
                            disabled={currentPage.population >= getTotalPages('population') || loading}
                            onclick={() => handlePageChange('population', currentPage.population + 1)}
                        >
                            Next
                        </Button>
                    </div>
                {/if}
            </Tabs.Content>

            <!-- Attack Rankings -->
            <Tabs.Content value="attackers" class="mt-4">
                <div class="border rounded-lg overflow-hidden">
                    <table class="w-full">
                        <thead class="bg-muted">
                            <tr>
                                <th class="px-4 py-2 text-left text-sm font-medium w-16">Rank</th>
                                <th class="px-4 py-2 text-left text-sm font-medium">Player</th>
                                <th class="px-4 py-2 text-right text-sm font-medium">Battles</th>
                                <th class="px-4 py-2 text-right text-sm font-medium">Points</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#if loading && attackRankings.length === 0}
                                <tr>
                                    <td colspan="4" class="px-4 py-8 text-center text-muted-foreground">
                                        Loading...
                                    </td>
                                </tr>
                            {:else if attackRankings.length === 0}
                                <tr>
                                    <td colspan="4" class="px-4 py-8 text-center text-muted-foreground">
                                        No rankings available
                                    </td>
                                </tr>
                            {:else}
                                {#each attackRankings as player}
                                    <tr class="border-t hover:bg-muted/50">
                                        <td class="px-4 py-2 {getRankClass(player.rank)}">
                                            {getRankIcon(player.rank)}
                                        </td>
                                        <td class="px-4 py-2">
                                            <div class="font-medium">{player.display_name || 'Unknown'}</div>
                                            {#if player.alliance_tag}
                                                <div class="text-xs text-muted-foreground">[{player.alliance_tag}]</div>
                                            {/if}
                                        </td>
                                        <td class="px-4 py-2 text-right text-muted-foreground">
                                            {player.battles_won} won
                                        </td>
                                        <td class="px-4 py-2 text-right font-semibold text-red-600">
                                            {formatNumber(player.attack_points)}
                                        </td>
                                    </tr>
                                {/each}
                            {/if}
                        </tbody>
                    </table>
                </div>

                {#if getTotalPages('attackers') > 1}
                    <div class="flex items-center justify-center gap-2 mt-4">
                        <Button
                            size="sm"
                            variant="outline"
                            disabled={currentPage.attackers <= 1 || loading}
                            onclick={() => handlePageChange('attackers', currentPage.attackers - 1)}
                        >
                            Previous
                        </Button>
                        <span class="text-sm text-muted-foreground">
                            Page {currentPage.attackers} of {getTotalPages('attackers')}
                        </span>
                        <Button
                            size="sm"
                            variant="outline"
                            disabled={currentPage.attackers >= getTotalPages('attackers') || loading}
                            onclick={() => handlePageChange('attackers', currentPage.attackers + 1)}
                        >
                            Next
                        </Button>
                    </div>
                {/if}
            </Tabs.Content>

            <!-- Defense Rankings -->
            <Tabs.Content value="defenders" class="mt-4">
                <div class="border rounded-lg overflow-hidden">
                    <table class="w-full">
                        <thead class="bg-muted">
                            <tr>
                                <th class="px-4 py-2 text-left text-sm font-medium w-16">Rank</th>
                                <th class="px-4 py-2 text-left text-sm font-medium">Player</th>
                                <th class="px-4 py-2 text-right text-sm font-medium">Defended</th>
                                <th class="px-4 py-2 text-right text-sm font-medium">Points</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#if loading && defenseRankings.length === 0}
                                <tr>
                                    <td colspan="4" class="px-4 py-8 text-center text-muted-foreground">
                                        Loading...
                                    </td>
                                </tr>
                            {:else if defenseRankings.length === 0}
                                <tr>
                                    <td colspan="4" class="px-4 py-8 text-center text-muted-foreground">
                                        No rankings available
                                    </td>
                                </tr>
                            {:else}
                                {#each defenseRankings as player}
                                    <tr class="border-t hover:bg-muted/50">
                                        <td class="px-4 py-2 {getRankClass(player.rank)}">
                                            {getRankIcon(player.rank)}
                                        </td>
                                        <td class="px-4 py-2">
                                            <div class="font-medium">{player.display_name || 'Unknown'}</div>
                                            {#if player.alliance_tag}
                                                <div class="text-xs text-muted-foreground">[{player.alliance_tag}]</div>
                                            {/if}
                                        </td>
                                        <td class="px-4 py-2 text-right text-muted-foreground">
                                            {player.battles_defended}
                                        </td>
                                        <td class="px-4 py-2 text-right font-semibold text-blue-600">
                                            {formatNumber(player.defense_points)}
                                        </td>
                                    </tr>
                                {/each}
                            {/if}
                        </tbody>
                    </table>
                </div>

                {#if getTotalPages('defenders') > 1}
                    <div class="flex items-center justify-center gap-2 mt-4">
                        <Button
                            size="sm"
                            variant="outline"
                            disabled={currentPage.defenders <= 1 || loading}
                            onclick={() => handlePageChange('defenders', currentPage.defenders - 1)}
                        >
                            Previous
                        </Button>
                        <span class="text-sm text-muted-foreground">
                            Page {currentPage.defenders} of {getTotalPages('defenders')}
                        </span>
                        <Button
                            size="sm"
                            variant="outline"
                            disabled={currentPage.defenders >= getTotalPages('defenders') || loading}
                            onclick={() => handlePageChange('defenders', currentPage.defenders + 1)}
                        >
                            Next
                        </Button>
                    </div>
                {/if}
            </Tabs.Content>

            <!-- Hero Rankings -->
            <Tabs.Content value="heroes" class="mt-4">
                <div class="border rounded-lg overflow-hidden">
                    <table class="w-full">
                        <thead class="bg-muted">
                            <tr>
                                <th class="px-4 py-2 text-left text-sm font-medium w-16">Rank</th>
                                <th class="px-4 py-2 text-left text-sm font-medium">Hero</th>
                                <th class="px-4 py-2 text-left text-sm font-medium">Owner</th>
                                <th class="px-4 py-2 text-right text-sm font-medium">Level</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#if loading && heroRankings.length === 0}
                                <tr>
                                    <td colspan="4" class="px-4 py-8 text-center text-muted-foreground">
                                        Loading...
                                    </td>
                                </tr>
                            {:else if heroRankings.length === 0}
                                <tr>
                                    <td colspan="4" class="px-4 py-8 text-center text-muted-foreground">
                                        No rankings available
                                    </td>
                                </tr>
                            {:else}
                                {#each heroRankings as hero}
                                    <tr class="border-t hover:bg-muted/50">
                                        <td class="px-4 py-2 {getRankClass(hero.rank)}">
                                            {getRankIcon(hero.rank)}
                                        </td>
                                        <td class="px-4 py-2">
                                            <div class="font-medium">{hero.hero_name}</div>
                                            <div class="text-xs text-muted-foreground">
                                                EXP: {formatNumber(hero.experience)}
                                            </div>
                                        </td>
                                        <td class="px-4 py-2 text-muted-foreground">
                                            {hero.owner_name || 'Unknown'}
                                        </td>
                                        <td class="px-4 py-2 text-right">
                                            <span class="font-semibold text-purple-600">Lv.{hero.level}</span>
                                        </td>
                                    </tr>
                                {/each}
                            {/if}
                        </tbody>
                    </table>
                </div>

                {#if getTotalPages('heroes') > 1}
                    <div class="flex items-center justify-center gap-2 mt-4">
                        <Button
                            size="sm"
                            variant="outline"
                            disabled={currentPage.heroes <= 1 || loading}
                            onclick={() => handlePageChange('heroes', currentPage.heroes - 1)}
                        >
                            Previous
                        </Button>
                        <span class="text-sm text-muted-foreground">
                            Page {currentPage.heroes} of {getTotalPages('heroes')}
                        </span>
                        <Button
                            size="sm"
                            variant="outline"
                            disabled={currentPage.heroes >= getTotalPages('heroes') || loading}
                            onclick={() => handlePageChange('heroes', currentPage.heroes + 1)}
                        >
                            Next
                        </Button>
                    </div>
                {/if}
            </Tabs.Content>

            <!-- Alliance Rankings -->
            <Tabs.Content value="alliances" class="mt-4">
                <div class="border rounded-lg overflow-hidden">
                    <table class="w-full">
                        <thead class="bg-muted">
                            <tr>
                                <th class="px-4 py-2 text-left text-sm font-medium w-16">Rank</th>
                                <th class="px-4 py-2 text-left text-sm font-medium">Alliance</th>
                                <th class="px-4 py-2 text-right text-sm font-medium">Members</th>
                                <th class="px-4 py-2 text-right text-sm font-medium">Population</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#if loading && allianceRankings.length === 0}
                                <tr>
                                    <td colspan="4" class="px-4 py-8 text-center text-muted-foreground">
                                        Loading...
                                    </td>
                                </tr>
                            {:else if allianceRankings.length === 0}
                                <tr>
                                    <td colspan="4" class="px-4 py-8 text-center text-muted-foreground">
                                        No rankings available
                                    </td>
                                </tr>
                            {:else}
                                {#each allianceRankings as alliance}
                                    <tr class="border-t hover:bg-muted/50">
                                        <td class="px-4 py-2 {getRankClass(alliance.rank)}">
                                            {getRankIcon(alliance.rank)}
                                        </td>
                                        <td class="px-4 py-2">
                                            <div class="font-medium">{alliance.name}</div>
                                            <div class="text-xs text-muted-foreground">[{alliance.tag}]</div>
                                        </td>
                                        <td class="px-4 py-2 text-right text-muted-foreground">
                                            {alliance.member_count}
                                        </td>
                                        <td class="px-4 py-2 text-right font-semibold text-green-600">
                                            {formatNumber(alliance.total_population)}
                                        </td>
                                    </tr>
                                {/each}
                            {/if}
                        </tbody>
                    </table>
                </div>

                {#if getTotalPages('alliances') > 1}
                    <div class="flex items-center justify-center gap-2 mt-4">
                        <Button
                            size="sm"
                            variant="outline"
                            disabled={currentPage.alliances <= 1 || loading}
                            onclick={() => handlePageChange('alliances', currentPage.alliances - 1)}
                        >
                            Previous
                        </Button>
                        <span class="text-sm text-muted-foreground">
                            Page {currentPage.alliances} of {getTotalPages('alliances')}
                        </span>
                        <Button
                            size="sm"
                            variant="outline"
                            disabled={currentPage.alliances >= getTotalPages('alliances') || loading}
                            onclick={() => handlePageChange('alliances', currentPage.alliances + 1)}
                        >
                            Next
                        </Button>
                    </div>
                {/if}
            </Tabs.Content>
        </Tabs.Root>
    </Dialog.Content>
</Dialog.Root>
