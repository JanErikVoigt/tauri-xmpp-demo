<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits<{ error: [msg: string] }>();

interface Contact {
  jid: string;
  display_name: string;
}

const contacts = ref<Contact[]>([]);
const jid = ref("");
const displayName = ref("");
const adding = ref(false);

async function load() {
  try {
    contacts.value = await invoke<Contact[]>("cmd_get_contacts");
  } catch (e) {
    emit("error", String(e));
  }
}

async function add() {
  if (!jid.value.trim() || !displayName.value.trim()) return;
  adding.value = true;
  try {
    await invoke("cmd_add_contact", {
      jid: jid.value.trim(),
      displayName: displayName.value.trim(),
    });
    jid.value = "";
    displayName.value = "";
    await load();
  } catch (e) {
    emit("error", String(e));
  } finally {
    adding.value = false;
  }
}

async function remove(contactJid: string) {
  try {
    await invoke("cmd_remove_contact", { jid: contactJid });
    contacts.value = contacts.value.filter((c) => c.jid !== contactJid);
  } catch (e) {
    emit("error", String(e));
  }
}

onMounted(load);
</script>

<template>
  <div class="page">
    <h2>Contacts</h2>

    <form class="form" @submit.prevent="add">
      <input
        v-model="jid"
        placeholder="JID (user@example.com)"
        :disabled="adding"
      />
      <input
        v-model="displayName"
        placeholder="Display name"
        :disabled="adding"
      />
      <button type="submit" :disabled="adding || !jid.trim() || !displayName.trim()">
        Add
      </button>
    </form>

    <ul v-if="contacts.length" class="list">
      <li v-for="c in contacts" :key="c.jid" class="item">
        <div class="info">
          <span class="name">{{ c.display_name }}</span>
          <span class="jid">{{ c.jid }}</span>
        </div>
        <button class="remove" @click="remove(c.jid)" title="Remove">✕</button>
      </li>
    </ul>

    <p v-else class="empty">No contacts yet.</p>
  </div>
</template>

<style scoped>
.page {
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

h2 { font-size: 1.1rem; font-weight: 600; }

.form {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.form input {
  flex: 1;
  min-width: 140px;
  padding: 0.5rem 0.75rem;
  border: 1px solid #d4d4d8;
  border-radius: 6px;
  font-size: 0.9rem;
  background: #fff;
  color: inherit;
}

@media (prefers-color-scheme: dark) {
  .form input { background: #3f3f46; border-color: #52525b; }
}

.form button {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 6px;
  background: #2563eb;
  color: #fff;
  cursor: pointer;
  font-size: 0.9rem;
  box-shadow: none;
  white-space: nowrap;
}

.form button:disabled { opacity: 0.5; cursor: default; }
.form button:not(:disabled):hover { background: #1d4ed8; }

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
}

@media (prefers-color-scheme: dark) {
  .item { background: #27272a; border-color: #3f3f46; }
}

.info {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

.name { font-size: 0.9rem; font-weight: 500; }

.jid { font-size: 0.78rem; color: #a1a1aa; }

.remove {
  border: none;
  background: none;
  cursor: pointer;
  color: #a1a1aa;
  font-size: 0.85rem;
  padding: 0.2rem 0.4rem;
  border-radius: 4px;
  box-shadow: none;
  line-height: 1;
  flex-shrink: 0;
}

.remove:hover { color: #dc2626; background: #fee2e2; }

@media (prefers-color-scheme: dark) {
  .remove:hover { background: #450a0a; color: #f87171; }
}

.empty { color: #a1a1aa; font-size: 0.9rem; }
</style>
