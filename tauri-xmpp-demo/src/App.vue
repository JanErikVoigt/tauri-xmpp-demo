<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import Contacts from "./views/Contacts.vue";
import Greet from "./views/Greet.vue";
import Settings from "./views/Settings.vue";
import ErrorPopup from "./components/ErrorPopup.vue";

type View = "contacts" | "greet" | "settings";
const view = ref<View>("greet");

const errorMsg = ref<string | null>(null);
function showError(msg: string) {
  errorMsg.value = msg;
}

type ConnectionStatus = "online" | "offline" | null;
const connectionStatus = ref<ConnectionStatus>(null);

let unlistenOnline: UnlistenFn | null = null;
let unlistenOffline: UnlistenFn | null = null;
let unlistenError: UnlistenFn | null = null;

onMounted(async () => {
  unlistenOnline = await listen<void>("xmpp:online", () => {
    connectionStatus.value = "online";
    // Clear any previous disconnect error when we reconnect
    if (errorMsg.value?.startsWith("Disconnected")) {
      errorMsg.value = null;
    }
  });

  unlistenOffline = await listen<string>("xmpp:offline", (event) => {
    connectionStatus.value = "offline";
    const reason = event.payload?.trim();
    showError(reason ? `Disconnected: ${reason}` : "Disconnected");
  });

  unlistenError = await listen<string>("xmpp:error", (event) => {
    showError(event.payload ?? "Unknown XMPP error");
  });
});

onUnmounted(() => {
  unlistenOnline?.();
  unlistenOffline?.();
  unlistenError?.();
});
</script>

<template>
  <div class="app">
    <nav class="tabs">
      <button :class="{ active: view === 'contacts' }" @click="view = 'contacts'">
        Contacts
      </button>
      <button :class="{ active: view === 'greet' }" @click="view = 'greet'">
        Greet
      </button>
      <button :class="{ active: view === 'settings' }" @click="view = 'settings'">
        Settings
      </button>

      <div v-if="connectionStatus !== null" class="status" :class="connectionStatus">
        <span class="dot" />
        {{ connectionStatus === "online" ? "Connected" : "Disconnected" }}
      </div>
    </nav>

    <main class="view">
      <Contacts v-if="view === 'contacts'" @error="showError" />
      <Greet v-if="view === 'greet'" @error="showError" />
      <Settings v-if="view === 'settings'" @error="showError" />
    </main>

    <ErrorPopup :message="errorMsg" @close="errorMsg = null" />
  </div>
</template>

<style>
*,
*::before,
*::after {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 15px;
  line-height: 1.5;
  color: #1a1a1a;
  background: #f4f4f5;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #e4e4e7;
    background: #18181b;
  }
}
</style>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.tabs {
  display: flex;
  align-items: center;
  border-bottom: 1px solid #d4d4d8;
  background: #ffffff;
}

@media (prefers-color-scheme: dark) {
  .tabs {
    background: #27272a;
    border-color: #3f3f46;
  }
}

.tabs button {
  flex: 1;
  padding: 0.75rem 1rem;
  border: none;
  border-bottom: 2px solid transparent;
  background: none;
  cursor: pointer;
  font-size: 0.9rem;
  font-weight: 500;
  color: #71717a;
  border-radius: 0;
  box-shadow: none;
  transition: color 0.15s, border-color 0.15s;
}

.tabs button.active {
  color: #2563eb;
  border-bottom-color: #2563eb;
}

.status {
  display: flex;
  align-items: center;
  gap: 0.35rem;
  font-size: 0.75rem;
  font-weight: 500;
  padding: 0 0.75rem;
  white-space: nowrap;
  margin-bottom: 2px; /* align with tab underlines */
}

.status.online  { color: #16a34a; }
.status.offline { color: #dc2626; }

@media (prefers-color-scheme: dark) {
  .status.online  { color: #4ade80; }
  .status.offline { color: #f87171; }
}

.dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: currentColor;
  flex-shrink: 0;
}

.view {
  flex: 1;
  overflow-y: auto;
}
</style>
