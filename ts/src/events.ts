import * as wasm from "wasm";
import { heldKeys, noteOn, noteOff } from ".";
import { stopAllTones } from ".";

export function visibilityChange(): void {
  console.debug("visibilityChange");
  if (document.hidden) {
    stopAllTones();
  }
}

export function keydown(event: KeyboardEvent): void {
  console.debug("keydown");
  if (!document.hasFocus()) return;
  if (event.repeat) return;
  if (event.code in heldKeys) return;

  if (document.activeElement?.tagName === "BODY") {
    // if (recording) { }
    const tone_index: number = wasm.from_keymap(event.code);
    noteOn(tone_index);
    heldKeys[event.code] = true;
  }
}

export function keyup(event: KeyboardEvent): void {
  console.debug("keyup");
  // if (recording) { }
  const tone_index: number = wasm.from_keymap(event.code);
  noteOff(tone_index);
  delete heldKeys[event.code];
}
