<script setup lang="ts">
import { ref } from "vue";
import Greet from "./views/Greet.vue";
import Settings from "./views/Settings.vue";
import ErrorPopup from "./components/ErrorPopup.vue";

type View = "greet" | "settings";
const view = ref<View>("greet");

const errorMsg = ref<string | null>(null);
function showError(msg: string) {
  errorMsg.value = msg;
}
</script>

<template>
  <div class="app">
    <nav class="tabs">
      <button :class="{ active: view === 'greet' }" @click="view = 'greet'">
        Greet
      </button>
      <button :class="{ active: view === 'settings' }" @click="view = 'settings'">
        Settings
      </button>
    </nav>

    <main class="view">
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

.view {
  flex: 1;
  overflow-y: auto;
}
</style>
