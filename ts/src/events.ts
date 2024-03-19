import * as wasm from "wasm";
import { heldKeys, noteOn, noteOff } from ".";
import { stopAllTones } from ".";
import { markKey, markedButtons } from "./UI";

export function visibilityChange(): void {
  if (document.hidden) {
    stopAllTones();
  }
}

export function onload(): void {
  const hash = window.location.hash.substring(1);
  if (hash) {
    const notes = hash.split(",");
    markedButtons.style.display = "flex";
    notes.forEach((note) => {
      const index = parseInt(note);
      // noteOn(index);
      markKey(index);
    });
  } else {
    markedButtons.style.display = "none";
  }
}

export function keydown(event: KeyboardEvent): void {
  if (!document.hasFocus()) return;
  if (event.repeat) return;
  if (event.code in heldKeys) return;

  if (document.activeElement?.tagName === "BODY") {
    // if (recording) { }
    const tone_index: number = wasm.from_keymap(event.code);
    if (tone_index === -1) return;
    noteOn(tone_index);
    heldKeys[event.code] = true;
  }
}

export function keyup(event: KeyboardEvent): void {
  // if (recording) { }
  const tone_index: number = wasm.from_keymap(event.code);
  if (tone_index === -1) return;
  noteOff(tone_index);
  delete heldKeys[event.code];
}
