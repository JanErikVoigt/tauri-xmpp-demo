<script setup lang="ts">
defineProps<{ message: string | null }>();
defineEmits<{ close: [] }>();
</script>

<template>
  <Transition name="popup">
    <div v-if="message" class="overlay" @click.self="$emit('close')">
      <div class="popup">
        <p class="popup-msg">{{ message }}</p>
        <button class="popup-close" @click="$emit('close')">Dismiss</button>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.popup {
  background: #fff;
  border-radius: 10px;
  padding: 1.5rem 2rem;
  max-width: 360px;
  width: 90%;
  box-shadow: 0 8px 32px rgba(0,0,0,0.18);
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

@media (prefers-color-scheme: dark) {
  .popup { background: #27272a; }
}

.popup-msg {
  font-size: 0.95rem;
  color: #dc2626;
  word-break: break-word;
}

.popup-close {
  align-self: flex-end;
  padding: 0.4rem 1rem;
  border: none;
  border-radius: 6px;
  background: #2563eb;
  color: #fff;
  cursor: pointer;
  font-size: 0.85rem;
}

.popup-close:hover { background: #1d4ed8; }

.popup-enter-active, .popup-leave-active { transition: opacity 0.15s; }
.popup-enter-from, .popup-leave-to { opacity: 0; }
</style>
