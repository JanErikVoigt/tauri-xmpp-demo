<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits<{ error: [msg: string] }>();

type MyMessage =
  | { Greet: string }
  | { Befriend: string }
  | { Unfriend: string };

const history = ref<MyMessage[]>([]);

function label(msg: MyMessage): string {
  if ("Greet" in msg) return `👋 Greeted by "${msg.Greet}"`;
  if ("Befriend" in msg) return `🤝 Befriended ${msg.Befriend}`;
  if ("Unfriend" in msg) return `👋 Unfriended ${msg.Unfriend}`;
  return JSON.stringify(msg);
}

async function load() {
  try {
    history.value = await invoke<MyMessage[]>("cmd_get_history");
  } catch (e) {
    emit("error", String(e));
  }
}

onMounted(load);
</script>

<template>
  <div class="page">
    <div class="header">
      <h2>Message History</h2>
      <button class="refresh" @click="load" title="Refresh">↻</button>
    </div>

    <ul v-if="history.length" class="list">
      <li v-for="(msg, i) in history" :key="i" class="item">
        {{ label(msg) }}
      </li>
    </ul>

    <p v-else class="empty">No messages received yet.</p>
  </div>
</template>

<style scoped>
.page {
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

h2 { font-size: 1.1rem; font-weight: 600; }

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

.refresh:hover { color: #2563eb; background: #eff6ff; }

@media (prefers-color-scheme: dark) {
  .refresh:hover { background: #1e3a5f; color: #60a5fa; }
}

.list {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.item {
  padding: 0.6rem 0.75rem;
  background: #fff;
  border: 1px solid #e4e4e7;
  border-radius: 8px;
  font-size: 0.9rem;
}

@media (prefers-color-scheme: dark) {
  .item { background: #27272a; border-color: #3f3f46; }
}

.empty { color: #a1a1aa; font-size: 0.9rem; }
</style>
