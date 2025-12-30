<script lang="ts">
    import * as Dialog from '$lib/components/ui/dialog';
    import { Button } from '$lib/components/ui/button';
    import { Separator } from '$lib/components/ui/separator';
    import { Input } from '$lib/components/ui/input';
    import { Label } from '$lib/components/ui/label';
    import {
        messageStore,
        type Message,
        type MessageListItem,
        formatMessageDate,
        truncateText,
    } from '$lib/stores/message';
    import { allianceStore } from '$lib/stores/alliance';

    interface Props {
        open: boolean;
    }

    let { open = $bindable(false) }: Props = $props();

    // Tabs
    type TabType = 'inbox' | 'sent' | 'compose' | 'alliance';
    let activeTab = $state<TabType>('inbox');

    // View state
    type ViewMode = 'list' | 'read' | 'compose';
    let viewMode = $state<ViewMode>('list');

    // Store state
    let msgState = $state(messageStore);
    let inbox = $derived($msgState.inbox);
    let sent = $derived($msgState.sent);
    let allianceMessages = $derived($msgState.allianceMessages);
    let currentMessage = $derived($msgState.currentMessage);
    let loading = $derived($msgState.loading);

    let allianceState = $state(allianceStore);
    let myAlliance = $derived($allianceState.myAlliance);

    // Compose form
    let composeRecipientId = $state('');
    let composeSubject = $state('');
    let composeBody = $state('');

    // Reply form
    let replyBody = $state('');

    // Unread counts
    const inboxUnreadCount = $derived(inbox.filter(m => !m.is_read).length);
    const allianceUnreadCount = $derived(allianceMessages.filter(m => !m.is_read).length);

    // Load data when modal opens
    $effect(() => {
        if (open) {
            loadData();
        }
    });

    async function loadData() {
        try {
            await Promise.all([
                messageStore.loadInbox(),
                messageStore.loadSent(),
                messageStore.loadAllianceMessages(),
                messageStore.loadUnreadCount(),
            ]);
        } catch (error) {
            console.error('Failed to load messages:', error);
        }
    }

    // Open message
    async function handleOpenMessage(messageId: string, isAlliance: boolean = false) {
        try {
            if (isAlliance) {
                await messageStore.loadAllianceMessage(messageId);
            } else {
                await messageStore.loadMessage(messageId);
            }
            viewMode = 'read';
        } catch {
            // Error handled in store
        }
    }

    // Back to list
    function handleBackToList() {
        viewMode = 'list';
        messageStore.clearCurrentMessage();
        replyBody = '';
    }

    // Delete message
    async function handleDeleteMessage(messageId: string) {
        if (!confirm('Delete this message?')) return;

        try {
            await messageStore.deleteMessage(messageId);
            viewMode = 'list';
        } catch {
            // Error handled in store
        }
    }

    // Send message
    async function handleSendMessage() {
        if (!composeRecipientId.trim() || !composeSubject.trim() || !composeBody.trim()) return;

        try {
            await messageStore.sendMessage({
                recipient_id: composeRecipientId.trim(),
                subject: composeSubject.trim(),
                body: composeBody.trim(),
            });
            // Reset form and go to sent
            composeRecipientId = '';
            composeSubject = '';
            composeBody = '';
            activeTab = 'sent';
            viewMode = 'list';
        } catch {
            // Error handled in store
        }
    }

    // Send alliance message
    async function handleSendAllianceMessage() {
        if (!composeSubject.trim() || !composeBody.trim()) return;

        try {
            await messageStore.sendAllianceMessage({
                subject: composeSubject.trim(),
                body: composeBody.trim(),
            });
            // Reset form and go to alliance
            composeSubject = '';
            composeBody = '';
            activeTab = 'alliance';
            viewMode = 'list';
        } catch {
            // Error handled in store
        }
    }

    // Reply to message
    async function handleReply() {
        if (!currentMessage || !replyBody.trim()) return;

        try {
            await messageStore.sendMessage({
                recipient_id: currentMessage.sender_id,
                subject: `Re: ${currentMessage.subject}`,
                body: replyBody.trim(),
            });
            replyBody = '';
            handleBackToList();
            // Reload sent
            messageStore.loadSent();
        } catch {
            // Error handled in store
        }
    }

    // Start compose
    function handleStartCompose(recipientId?: string) {
        composeRecipientId = recipientId || '';
        composeSubject = '';
        composeBody = '';
        activeTab = 'compose';
        viewMode = 'compose';
    }

    // Handle tab change
    function handleTabChange(tab: TabType) {
        activeTab = tab;
        viewMode = tab === 'compose' ? 'compose' : 'list';
        messageStore.clearCurrentMessage();
    }
</script>

<Dialog.Root bind:open>
    <Dialog.Content class="sm:max-w-2xl max-h-[90vh] flex flex-col">
        <Dialog.Header>
            <div class="flex items-center gap-3">
                <div class="w-12 h-12 rounded-xl bg-muted flex items-center justify-center text-2xl">
                    ✉️
                </div>
                <div>
                    <Dialog.Title class="text-xl">Messages</Dialog.Title>
                    <Dialog.Description>
                        Send and receive messages
                    </Dialog.Description>
                </div>
            </div>
        </Dialog.Header>

        <!-- Tabs (only show when in list mode) -->
        {#if viewMode === 'list' || viewMode === 'compose'}
            <div class="flex gap-2 mt-4 flex-wrap">
                <Button
                    variant={activeTab === 'inbox' ? 'default' : 'outline'}
                    size="sm"
                    onclick={() => handleTabChange('inbox')}
                >
                    <span class="mr-1">📥</span> Inbox
                    {#if inboxUnreadCount > 0}
                        <span class="ml-1 px-1.5 py-0.5 text-xs bg-destructive text-destructive-foreground rounded-full">
                            {inboxUnreadCount}
                        </span>
                    {/if}
                </Button>
                <Button
                    variant={activeTab === 'sent' ? 'default' : 'outline'}
                    size="sm"
                    onclick={() => handleTabChange('sent')}
                >
                    <span class="mr-1">📤</span> Sent
                </Button>
                <Button
                    variant={activeTab === 'compose' ? 'default' : 'outline'}
                    size="sm"
                    onclick={() => handleTabChange('compose')}
                >
                    <span class="mr-1">✏️</span> Compose
                </Button>
                {#if myAlliance}
                    <Button
                        variant={activeTab === 'alliance' ? 'default' : 'outline'}
                        size="sm"
                        onclick={() => handleTabChange('alliance')}
                    >
                        <span class="mr-1">🏰</span> Alliance
                        {#if allianceUnreadCount > 0}
                            <span class="ml-1 px-1.5 py-0.5 text-xs bg-destructive text-destructive-foreground rounded-full">
                                {allianceUnreadCount}
                            </span>
                        {/if}
                    </Button>
                {/if}
            </div>
        {/if}

        <!-- Content -->
        <div class="flex-1 overflow-y-auto mt-4 min-h-[400px]">
            {#if loading && inbox.length === 0 && viewMode === 'list'}
                <div class="flex items-center justify-center py-12">
                    <span class="animate-spin text-2xl mr-2">⏳</span>
                    <span>Loading...</span>
                </div>

            {:else if viewMode === 'read' && currentMessage}
                <!-- Read Message View -->
                <div class="space-y-4">
                    <!-- Back button -->
                    <Button variant="ghost" size="sm" onclick={handleBackToList}>
                        <span class="mr-1">←</span> Back
                    </Button>

                    <!-- Message header -->
                    <div class="p-4 rounded-lg bg-muted">
                        <h3 class="font-semibold text-lg">{currentMessage.subject}</h3>
                        <div class="flex items-center gap-2 mt-2 text-sm text-muted-foreground">
                            <span>From: <span class="font-medium text-foreground">{currentMessage.sender_name}</span></span>
                            <span>•</span>
                            <span>{formatMessageDate(currentMessage.created_at)}</span>
                        </div>
                        {#if currentMessage.recipient_name}
                            <div class="text-sm text-muted-foreground mt-1">
                                To: <span class="font-medium text-foreground">{currentMessage.recipient_name}</span>
                            </div>
                        {/if}
                    </div>

                    <!-- Message body -->
                    <div class="p-4 border rounded-lg whitespace-pre-wrap">
                        {currentMessage.body}
                    </div>

                    <!-- Actions -->
                    <div class="flex gap-2">
                        {#if currentMessage.message_type === 'private'}
                            <Button
                                variant="outline"
                                onclick={() => handleStartCompose(currentMessage.sender_id)}
                            >
                                <span class="mr-1">↩️</span> Reply
                            </Button>
                        {/if}
                        <Button
                            variant="ghost"
                            class="text-destructive"
                            onclick={() => handleDeleteMessage(currentMessage.id)}
                        >
                            <span class="mr-1">🗑️</span> Delete
                        </Button>
                    </div>

                    <!-- Quick Reply (for private messages) -->
                    {#if currentMessage.message_type === 'private'}
                        <Separator />
                        <div class="space-y-2">
                            <Label>Quick Reply</Label>
                            <textarea
                                bind:value={replyBody}
                                placeholder="Write your reply..."
                                rows={3}
                                class="w-full px-3 py-2 border rounded-md bg-background resize-none"
                            ></textarea>
                            <Button
                                onclick={handleReply}
                                disabled={loading || !replyBody.trim()}
                            >
                                {#if loading}
                                    <span class="animate-spin mr-2">⏳</span>
                                {/if}
                                Send Reply
                            </Button>
                        </div>
                    {/if}
                </div>

            {:else if viewMode === 'compose' || activeTab === 'compose'}
                <!-- Compose View -->
                <div class="space-y-4 max-w-lg mx-auto">
                    <div class="text-center mb-4">
                        <div class="text-3xl mb-2">✏️</div>
                        <h3 class="font-semibold">New Message</h3>
                    </div>

                    <!-- Toggle private/alliance -->
                    {#if myAlliance}
                        <div class="flex gap-2 justify-center">
                            <Button
                                variant={!composeRecipientId ? 'outline' : 'default'}
                                size="sm"
                                onclick={() => composeRecipientId = ''}
                            >
                                👤 Private
                            </Button>
                            <Button
                                variant={composeRecipientId === '' ? 'outline' : 'default'}
                                size="sm"
                                onclick={() => composeRecipientId = 'alliance'}
                                disabled={!myAlliance}
                            >
                                🏰 Alliance
                            </Button>
                        </div>
                    {/if}

                    {#if composeRecipientId !== 'alliance'}
                        <div>
                            <Label for="recipient">Recipient (Player ID)</Label>
                            <Input
                                id="recipient"
                                bind:value={composeRecipientId}
                                placeholder="Enter player ID..."
                                class="mt-1"
                            />
                        </div>
                    {/if}

                    <div>
                        <Label for="subject">Subject</Label>
                        <Input
                            id="subject"
                            bind:value={composeSubject}
                            placeholder="Message subject..."
                            maxlength={100}
                            class="mt-1"
                        />
                    </div>

                    <div>
                        <Label for="body">Message</Label>
                        <textarea
                            id="body"
                            bind:value={composeBody}
                            placeholder="Write your message..."
                            rows={6}
                            maxlength={2000}
                            class="w-full mt-1 px-3 py-2 border rounded-md bg-background resize-none"
                        ></textarea>
                        <div class="text-xs text-muted-foreground text-right mt-1">
                            {composeBody.length}/2000
                        </div>
                    </div>

                    <Button
                        class="w-full"
                        onclick={composeRecipientId === 'alliance' ? handleSendAllianceMessage : handleSendMessage}
                        disabled={loading || !composeSubject.trim() || !composeBody.trim() || (composeRecipientId !== 'alliance' && !composeRecipientId.trim())}
                    >
                        {#if loading}
                            <span class="animate-spin mr-2">⏳</span>
                        {/if}
                        {composeRecipientId === 'alliance' ? 'Send to Alliance' : 'Send Message'}
                    </Button>
                </div>

            {:else if activeTab === 'inbox'}
                <!-- Inbox List -->
                {#if inbox.length === 0}
                    <div class="text-center py-12 text-muted-foreground">
                        <p class="text-4xl mb-2">📥</p>
                        <p>Your inbox is empty</p>
                    </div>
                {:else}
                    <div class="space-y-2">
                        {#each inbox as message (message.id)}
                            <button
                                class="w-full p-3 rounded-lg border text-left hover:bg-muted/50 transition-colors
                                       {!message.is_read ? 'bg-primary/5 border-primary/20' : ''}"
                                onclick={() => handleOpenMessage(message.id)}
                            >
                                <div class="flex items-start gap-3">
                                    <div class="w-8 h-8 rounded-full bg-muted flex items-center justify-center text-sm shrink-0">
                                        {!message.is_read ? '🔵' : '👤'}
                                    </div>
                                    <div class="flex-1 min-w-0">
                                        <div class="flex items-center gap-2">
                                            <span class="font-medium truncate {!message.is_read ? 'text-primary' : ''}">
                                                {message.sender_name}
                                            </span>
                                            <span class="text-xs text-muted-foreground shrink-0">
                                                {formatMessageDate(message.created_at)}
                                            </span>
                                        </div>
                                        <p class="text-sm truncate {!message.is_read ? 'font-medium' : 'text-muted-foreground'}">
                                            {message.subject}
                                        </p>
                                    </div>
                                </div>
                            </button>
                        {/each}
                    </div>
                {/if}

            {:else if activeTab === 'sent'}
                <!-- Sent List -->
                {#if sent.length === 0}
                    <div class="text-center py-12 text-muted-foreground">
                        <p class="text-4xl mb-2">📤</p>
                        <p>No sent messages</p>
                        <Button variant="outline" size="sm" class="mt-2" onclick={() => handleStartCompose()}>
                            Compose Message
                        </Button>
                    </div>
                {:else}
                    <div class="space-y-2">
                        {#each sent as message (message.id)}
                            <button
                                class="w-full p-3 rounded-lg border text-left hover:bg-muted/50 transition-colors"
                                onclick={() => handleOpenMessage(message.id)}
                            >
                                <div class="flex items-start gap-3">
                                    <div class="w-8 h-8 rounded-full bg-muted flex items-center justify-center text-sm shrink-0">
                                        📤
                                    </div>
                                    <div class="flex-1 min-w-0">
                                        <div class="flex items-center gap-2">
                                            <span class="text-xs text-muted-foreground shrink-0">
                                                {formatMessageDate(message.created_at)}
                                            </span>
                                        </div>
                                        <p class="text-sm truncate text-muted-foreground">
                                            {message.subject}
                                        </p>
                                    </div>
                                </div>
                            </button>
                        {/each}
                    </div>
                {/if}

            {:else if activeTab === 'alliance'}
                <!-- Alliance Messages -->
                <div class="mb-4">
                    <Button size="sm" onclick={() => { composeRecipientId = 'alliance'; activeTab = 'compose'; viewMode = 'compose'; }}>
                        <span class="mr-1">✏️</span> New Alliance Message
                    </Button>
                </div>

                {#if allianceMessages.length === 0}
                    <div class="text-center py-12 text-muted-foreground">
                        <p class="text-4xl mb-2">🏰</p>
                        <p>No alliance messages</p>
                    </div>
                {:else}
                    <div class="space-y-2">
                        {#each allianceMessages as message (message.id)}
                            <button
                                class="w-full p-3 rounded-lg border text-left hover:bg-muted/50 transition-colors
                                       {!message.is_read ? 'bg-primary/5 border-primary/20' : ''}"
                                onclick={() => handleOpenMessage(message.id, true)}
                            >
                                <div class="flex items-start gap-3">
                                    <div class="w-8 h-8 rounded-full bg-muted flex items-center justify-center text-sm shrink-0">
                                        {!message.is_read ? '🔵' : '🏰'}
                                    </div>
                                    <div class="flex-1 min-w-0">
                                        <div class="flex items-center gap-2">
                                            <span class="font-medium truncate {!message.is_read ? 'text-primary' : ''}">
                                                {message.sender_name}
                                            </span>
                                            <span class="text-xs text-muted-foreground shrink-0">
                                                {formatMessageDate(message.created_at)}
                                            </span>
                                        </div>
                                        <p class="text-sm truncate {!message.is_read ? 'font-medium' : 'text-muted-foreground'}">
                                            {message.subject}
                                        </p>
                                    </div>
                                </div>
                            </button>
                        {/each}
                    </div>
                {/if}
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
