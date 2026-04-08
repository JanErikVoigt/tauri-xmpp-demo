<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits<{ error: [msg: string] }>();

const jid = ref("");
const password = ref("");
const saving = ref<"jid" | "password" | null>(null);
const savedJid = ref(false);
const savedPassword = ref(false);

async function loadJid() {
    try {
        const current = await invoke<string | null>("get_my_jid");
        if (current) jid.value = current;
    } catch (e) {
        emit("error", String(e));
    }
}

async function saveJid() {
    if (!jid.value.trim()) return;
    saving.value = "jid";
    try {
        await invoke("set_jid", { jid: jid.value.trim() });
        savedJid.value = true;
        setTimeout(() => (savedJid.value = false), 2000);
    } catch (e) {
        emit("error", String(e));
    } finally {
        saving.value = null;
    }
}

async function savePassword() {
    if (!password.value) return;
    saving.value = "password";
    try {
        await invoke("set_password", { password: password.value });
        password.value = "";
        savedPassword.value = true;
        setTimeout(() => (savedPassword.value = false), 2000);
    } catch (e) {
        emit("error", String(e));
    } finally {
        saving.value = null;
    }
}

onMounted(loadJid);
</script>

<template>
    <div class="page">
        <h2>Settings</h2>

        <div class="group">
            <label for="jid-input">XMPP JID</label>
            <div class="row">
                <input
                    id="jid-input"
                    v-model="jid"
                    placeholder="you@example.com"
                    :disabled="saving === 'jid'"
                    @keydown.enter="saveJid"
                />
                <button
                    @click="saveJid"
                    :disabled="saving === 'jid' || !jid.trim()"
                >
                    {{ savedJid ? "Saved ✓" : "Save" }}
                </button>
            </div>
        </div>

        <div class="group">
            <label for="pw-input">Password</label>
            <div class="row">
                <input
                    id="pw-input"
                    v-model="password"
                    type="password"
                    placeholder="••••••••"
                    :disabled="saving === 'password'"
                    @keydown.enter="savePassword"
                />
                <button
                    @click="savePassword"
                    :disabled="saving === 'password' || !password"
                >
                    {{ savedPassword ? "Saved ✓" : "Save" }}
                </button>
            </div>
            <p class="hint">
                Stored in the system keyring. Leave blank to keep current value.
            </p>
        </div>
    </div>
</template>

<style scoped>
.page {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.75rem;
}

h2 {
    font-size: 1.1rem;
    font-weight: 600;
}

.group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

label {
    font-size: 0.85rem;
    font-weight: 500;
    color: #52525b;
}

@media (prefers-color-scheme: dark) {
    label {
        color: #a1a1aa;
    }
}

.row {
    display: flex;
    gap: 0.5rem;
}

.row input {
    flex: 1;
    padding: 0.5rem 0.75rem;
    border: 1px solid #d4d4d8;
    border-radius: 6px;
    font-size: 0.9rem;
    background: #fff;
    color: inherit;
}

@media (prefers-color-scheme: dark) {
    .row input {
        background: #3f3f46;
        border-color: #52525b;
    }
}

.row button {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 6px;
    background: #2563eb;
    color: #fff;
    cursor: pointer;
    font-size: 0.9rem;
    white-space: nowrap;
    box-shadow: none;
    min-width: 70px;
}

.row button:disabled {
    opacity: 0.5;
    cursor: default;
}
.row button:not(:disabled):hover {
    background: #1d4ed8;
}

.hint {
    font-size: 0.78rem;
    color: #a1a1aa;
}
</style>
