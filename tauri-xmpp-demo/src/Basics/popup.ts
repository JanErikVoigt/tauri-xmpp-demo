import { ref } from "vue";

export type Severity = "Success" | "Info" | "Warning" | "Error";

export interface PopupMessage {
  id: number;
  text: string;
  severity: Severity;
  lifetime: number;
}

export const popups = ref<PopupMessage[]>([]);
let nextId = 0;

export function showPopup(
  text: string,
  severity: Severity,
  lifetime_secs: number,
) {
  const id = nextId++;
  popups.value.push({ id, text, severity, lifetime: lifetime_secs });
  setTimeout(() => {
    popups.value = popups.value.filter((p) => p.id !== id);
  }, lifetime_secs * 1000);
}
