<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits<{ error: [msg: string] }>();

const friends = ref<string[]>([]);
const input = ref("");
const loading = ref(false);

async function load() {
  try {
    friends.value = await invoke<string[]>("cmd_get_friends");
  } catch (e) {
    emit("error", String(e));
  }
}

async function add() {
  const jid = input.value.trim();
  if (!jid) return;
  loading.value = true;
  try {
    await invoke("cmd_befriend", { jid });
    input.value = "";
    await load();
  } catch (e) {
    emit("error", String(e));
  } finally {
    loading.value = false;
  }
}

async function remove(jid: string) {
  try {
    await invoke("cmd_unfriend", { jid });
    friends.value = friends.value.filter(f => f !== jid);
  } catch (e) {
    emit("error", String(e));
  }
}

onMounted(load);
</script>

<template>
  <div class="page">
    <h2>Friends</h2>

    <form class="add-row" @submit.prevent="add">
      <input v-model="input" placeholder="user@example.com" :disabled="loading" />
      <button type="submit" :disabled="loading || !input.trim()">Add</button>
    </form>

    <ul v-if="friends.length" class="list">
      <li v-for="jid in friends" :key="jid" class="item">
        <span class="jid">{{ jid }}</span>
        <button class="remove" @click="remove(jid)" title="Remove">✕</button>
      </li>
    </ul>

    <p v-else class="empty">No friends yet.</p>
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

.add-row {
  display: flex;
  gap: 0.5rem;
}

.add-row input {
  flex: 1;
  padding: 0.5rem 0.75rem;
  border: 1px solid #d4d4d8;
  border-radius: 6px;
  font-size: 0.9rem;
  background: #fff;
  color: inherit;
}

@media (prefers-color-scheme: dark) {
  .add-row input { background: #3f3f46; border-color: #52525b; }
}

.add-row button {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 6px;
  background: #2563eb;
  color: #fff;
  cursor: pointer;
  font-size: 0.9rem;
  box-shadow: none;
}

.add-row button:disabled { opacity: 0.5; cursor: default; }
.add-row button:not(:disabled):hover { background: #1d4ed8; }

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

.jid { font-size: 0.9rem; }

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
}

.remove:hover { color: #dc2626; background: #fee2e2; }

@media (prefers-color-scheme: dark) {
  .remove:hover { background: #450a0a; color: #f87171; }
}

.empty { color: #a1a1aa; font-size: 0.9rem; }
</style>
