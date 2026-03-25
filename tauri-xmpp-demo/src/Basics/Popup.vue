<script setup lang="ts">
import { popups } from "./popup";
</script>

<template>
    <div class="popup-container">
        <div v-for="p in popups" :key="p.id" class="popup" :class="p.severity">
            {{ p.text }}
        </div>
    </div>
</template>

<style scoped>
.popup-container {
    position: fixed;
    top: 15%;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    z-index: 9999;
    pointer-events: none;
}

.popup {
    padding: 10px 20px;
    border-radius: 6px;
    font-size: 0.95em;
    color: #fff;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}
.popup.Success {
    background: #49cc69;
}
.popup.Info {
    background: #2196f3;
}
.popup.Warning {
    background: #ff9800;
}
.popup.Error {
    background: #f44336;
}
</style>
