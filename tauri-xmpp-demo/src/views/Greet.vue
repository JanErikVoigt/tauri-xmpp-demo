<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

const emit = defineEmits<{ error: [msg: string] }>();

interface Contact {
    jid: string;
    display_name: string;
}

interface ReceivedGreeting {
    name: string;
    from: string;
    sent_at: number; // Unix seconds
}

const contacts = ref<Contact[]>([]);
const to = ref("");
const name = ref("");
const sending = ref(false);
const greetings = ref<ReceivedGreeting[]>([]);

let unlistenMessage: UnlistenFn | null = null;

function formatTime(unixSeconds: number): string {
    return new Date(unixSeconds * 1000).toLocaleString();
}

async function loadContacts() {
    try {
        contacts.value = await invoke<Contact[]>("get_contacts");
        if (to.value && !contacts.value.some((c) => c.jid === to.value)) {
            to.value = "";
        }
    } catch (e) {
        emit("error", String(e));
    }
}

async function loadGreetings() {
    try {
        greetings.value = await invoke<ReceivedGreeting[]>("get_greetings");
    } catch (e) {
        emit("error", String(e));
    }
}

async function send() {
    if (!to.value || !name.value.trim()) return;
    sending.value = true;
    try {
        await invoke("send_greet", {
            to: to.value,
            name: name.value.trim(),
        });
        name.value = "";
    } catch (e) {
        emit("error", String(e));
    } finally {
        sending.value = false;
    }
}

onMounted(async () => {
    await loadContacts();
    await loadGreetings();
    unlistenMessage = await listen<void>("xmpp:message", () => {
        loadGreetings();
    });
});

onUnmounted(() => {
    unlistenMessage?.();
});
</script>

<template>
    <div class="page">
        <section class="send-section">
            <h2>Send Greeting</h2>
            <form @submit.prevent="send" class="form">
                <select
                    v-model="to"
                    :disabled="sending || contacts.length === 0"
                    :class="{ placeholder: !to }"
                >
                    <option value="" disabled>
                        {{
                            contacts.length === 0
                                ? "Add contacts first"
                                : "Select recipient…"
                        }}
                    </option>
                    <option v-for="c in contacts" :key="c.jid" :value="c.jid">
                        {{ c.display_name }} ({{ c.jid }})
                    </option>
                </select>
                <input
                    v-model="name"
                    placeholder="Your name"
                    :disabled="sending"
                />
                <button
                    type="submit"
                    :disabled="sending || !to || !name.trim()"
                >
                    {{ sending ? "Sending…" : "Send" }}
                </button>
            </form>
        </section>

        <section class="history-section">
            <div class="history-header">
                <h2>Received Greetings</h2>
                <button class="refresh" @click="loadGreetings" title="Refresh">
                    ↻
                </button>
            </div>
            <ul v-if="greetings.length" class="list">
                <li v-for="(g, i) in greetings" :key="i" class="item">
                    <div class="greeting-body">
                        <span class="greeting-name">👋 {{ g.name }}</span>
                        <span class="greeting-from">{{ g.from }}</span>
                    </div>
                    <span class="greeting-time">{{
                        formatTime(g.sent_at)
                    }}</span>
                </li>
            </ul>
            <p v-else class="empty">No greetings received yet.</p>
        </section>
    </div>
</template>

<style scoped>
.page {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 2rem;
}

h2 {
    font-size: 1.1rem;
    font-weight: 600;
    margin-bottom: 0.75rem;
}

.form {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.form select,
.form input {
    padding: 0.5rem 0.75rem;
    border: 1px solid #d4d4d8;
    border-radius: 6px;
    font-size: 0.9rem;
    background: #fff;
    color: inherit;
    width: 100%;
}

.form select.placeholder {
    color: #a1a1aa;
}

@media (prefers-color-scheme: dark) {
    .form select,
    .form input {
        background: #3f3f46;
        border-color: #52525b;
    }
    .form select.placeholder {
        color: #71717a;
    }
}

.form button {
    align-self: flex-start;
    padding: 0.5rem 1.25rem;
    border: none;
    border-radius: 6px;
    background: #2563eb;
    color: #fff;
    cursor: pointer;
    font-size: 0.9rem;
    box-shadow: none;
}

.form button:disabled {
    opacity: 0.5;
    cursor: default;
}
.form button:not(:disabled):hover {
    background: #1d4ed8;
}

.history-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.75rem;
}

.refresh {
    border: none;
    background: none;
    font-size: 1.2rem;
    cursor: pointer;
    color: #71717a;
    padding: 0.2rem 0.4rem;
    border-radius: 4px;
    box-shadow: none;
    line-height: 1;
}

.refresh:hover {
    color: #2563eb;
    background: #eff6ff;
}

@media (prefers-color-scheme: dark) {
    .refresh:hover {
        background: #1e3a5f;
        color: #60a5fa;
    }
}

.list {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.6rem 0.75rem;
    background: #fff;
    border: 1px solid #e4e4e7;
    border-radius: 8px;
    font-size: 0.9rem;
}

@media (prefers-color-scheme: dark) {
    .item {
        background: #27272a;
        border-color: #3f3f46;
    }
}

.greeting-body {
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
}

.greeting-from {
    font-size: 0.78rem;
    color: #a1a1aa;
}

.greeting-time {
    font-size: 0.78rem;
    color: #a1a1aa;
    white-space: nowrap;
    margin-left: 0.75rem;
    flex-shrink: 0;
}

.empty {
    color: #a1a1aa;
    font-size: 0.9rem;
}
</style>
