import { writable } from "svelte/store";
import { toast } from "svelte-sonner";
import { api } from "../api/client";

// Types
export type MessageType = 'private' | 'alliance';

export interface Message {
    id: string;
    message_type: MessageType;
    sender_id: string;
    sender_name: string;
    recipient_id: string | null;
    recipient_name: string | null;
    alliance_id: string | null;
    alliance_name: string | null;
    subject: string;
    body: string;
    is_read: boolean;
    created_at: string;
}

export interface MessageListItem {
    id: string;
    sender_id: string;
    sender_name: string;
    subject: string;
    is_read: boolean;
    created_at: string;
}

export interface Conversation {
    id: string;
    other_user_id: string;
    other_user_name: string;
    last_message_subject: string | null;
    last_message_preview: string | null;
    last_message_at: string;
    unread_count: number;
}

export interface SendMessageRequest {
    recipient_id: string;
    subject: string;
    body: string;
}

export interface SendAllianceMessageRequest {
    subject: string;
    body: string;
}

interface MessageState {
    inbox: MessageListItem[];
    sent: MessageListItem[];
    allianceMessages: MessageListItem[];
    conversations: Conversation[];
    currentMessage: Message | null;
    unreadCount: number;
    loading: boolean;
    error: string | null;
}

function createMessageStore() {
    const { subscribe, set, update } = writable<MessageState>({
        inbox: [],
        sent: [],
        allianceMessages: [],
        conversations: [],
        currentMessage: null,
        unreadCount: 0,
        loading: false,
        error: null,
    });

    return {
        subscribe,

        // Load inbox
        loadInbox: async (limit = 20, offset = 0) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const messages = await api.get<MessageListItem[]>(
                    `/api/messages/inbox?limit=${limit}&offset=${offset}`
                );
                update(state => ({
                    ...state,
                    inbox: messages,
                    loading: false,
                }));
                return messages;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                return [];
            }
        },

        // Load sent messages
        loadSent: async (limit = 20, offset = 0) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const messages = await api.get<MessageListItem[]>(
                    `/api/messages/sent?limit=${limit}&offset=${offset}`
                );
                update(state => ({
                    ...state,
                    sent: messages,
                    loading: false,
                }));
                return messages;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                return [];
            }
        },

        // Load single message
        loadMessage: async (messageId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const message = await api.get<Message>(`/api/messages/${messageId}`);
                update(state => ({
                    ...state,
                    currentMessage: message,
                    // Mark as read in inbox
                    inbox: state.inbox.map(m =>
                        m.id === messageId ? { ...m, is_read: true } : m
                    ),
                    loading: false,
                }));
                return message;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                throw error;
            }
        },

        // Send private message
        sendMessage: async (request: SendMessageRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const message = await api.post<Message>('/api/messages', request);
                update(state => ({
                    ...state,
                    sent: [
                        {
                            id: message.id,
                            sender_id: message.sender_id,
                            sender_name: message.sender_name,
                            subject: message.subject,
                            is_read: true,
                            created_at: message.created_at,
                        },
                        ...state.sent,
                    ],
                    loading: false,
                }));
                toast.success('Message Sent');
                return message;
            } catch (error: any) {
                const errorMsg = error.message || 'Failed to send message';
                update(state => ({
                    ...state,
                    loading: false,
                    error: errorMsg,
                }));
                toast.error('Failed', { description: errorMsg });
                throw error;
            }
        },

        // Delete message
        deleteMessage: async (messageId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                await api.delete(`/api/messages/${messageId}`);
                update(state => ({
                    ...state,
                    inbox: state.inbox.filter(m => m.id !== messageId),
                    sent: state.sent.filter(m => m.id !== messageId),
                    currentMessage: state.currentMessage?.id === messageId ? null : state.currentMessage,
                    loading: false,
                }));
                toast.success('Message Deleted');
                return true;
            } catch (error: any) {
                const errorMsg = error.message || 'Failed to delete message';
                update(state => ({
                    ...state,
                    loading: false,
                    error: errorMsg,
                }));
                toast.error('Failed', { description: errorMsg });
                throw error;
            }
        },

        // Load unread count
        loadUnreadCount: async () => {
            try {
                const response = await api.get<{ unread_count: number }>('/api/messages/unread-count');
                update(state => ({
                    ...state,
                    unreadCount: response.unread_count,
                }));
                return response.unread_count;
            } catch {
                return 0;
            }
        },

        // Load conversations
        loadConversations: async (limit = 20, offset = 0) => {
            try {
                const conversations = await api.get<Conversation[]>(
                    `/api/conversations?limit=${limit}&offset=${offset}`
                );
                update(state => ({
                    ...state,
                    conversations,
                }));
                return conversations;
            } catch (error: any) {
                console.error('Failed to load conversations:', error);
                return [];
            }
        },

        // Reply to conversation
        replyToConversation: async (conversationId: string, body: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const message = await api.post<Message>(
                    `/api/conversations/${conversationId}/reply`,
                    { body }
                );
                update(state => ({ ...state, loading: false }));
                toast.success('Reply Sent');
                return message;
            } catch (error: any) {
                const errorMsg = error.message || 'Failed to send reply';
                update(state => ({
                    ...state,
                    loading: false,
                    error: errorMsg,
                }));
                toast.error('Failed', { description: errorMsg });
                throw error;
            }
        },

        // Load alliance messages
        loadAllianceMessages: async (limit = 20, offset = 0) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const messages = await api.get<MessageListItem[]>(
                    `/api/alliance-messages?limit=${limit}&offset=${offset}`
                );
                update(state => ({
                    ...state,
                    allianceMessages: messages,
                    loading: false,
                }));
                return messages;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                return [];
            }
        },

        // Load single alliance message
        loadAllianceMessage: async (messageId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const message = await api.get<Message>(`/api/alliance-messages/${messageId}`);
                update(state => ({
                    ...state,
                    currentMessage: message,
                    allianceMessages: state.allianceMessages.map(m =>
                        m.id === messageId ? { ...m, is_read: true } : m
                    ),
                    loading: false,
                }));
                return message;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message,
                }));
                throw error;
            }
        },

        // Send alliance message
        sendAllianceMessage: async (request: SendAllianceMessageRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const message = await api.post<Message>('/api/alliance-messages', request);
                update(state => ({
                    ...state,
                    allianceMessages: [
                        {
                            id: message.id,
                            sender_id: message.sender_id,
                            sender_name: message.sender_name,
                            subject: message.subject,
                            is_read: true,
                            created_at: message.created_at,
                        },
                        ...state.allianceMessages,
                    ],
                    loading: false,
                }));
                toast.success('Alliance Message Sent');
                return message;
            } catch (error: any) {
                const errorMsg = error.message || 'Failed to send alliance message';
                update(state => ({
                    ...state,
                    loading: false,
                    error: errorMsg,
                }));
                toast.error('Failed', { description: errorMsg });
                throw error;
            }
        },

        // Clear current message
        clearCurrentMessage: () => {
            update(state => ({ ...state, currentMessage: null }));
        },

        // Clear error
        clearError: () => {
            update(state => ({ ...state, error: null }));
        },

        // Reset store
        reset: () => {
            set({
                inbox: [],
                sent: [],
                allianceMessages: [],
                conversations: [],
                currentMessage: null,
                unreadCount: 0,
                loading: false,
                error: null,
            });
        },
    };
}

export const messageStore = createMessageStore();

// Helper functions
export function formatMessageDate(dateString: string): string {
    const date = new Date(dateString);
    const now = new Date();
    const diff = now.getTime() - date.getTime();

    // Less than 24 hours ago
    if (diff < 24 * 60 * 60 * 1000) {
        return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    }

    // Less than 7 days ago
    if (diff < 7 * 24 * 60 * 60 * 1000) {
        const days = Math.floor(diff / (24 * 60 * 60 * 1000));
        return `${days}d ago`;
    }

    // Otherwise show date
    return date.toLocaleDateString();
}

export function truncateText(text: string, maxLength: number): string {
    if (text.length <= maxLength) return text;
    return text.substring(0, maxLength) + '...';
}
