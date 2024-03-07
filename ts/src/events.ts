import * as wasm from "wasm";
import { heldKeys, noteOn, noteOff } from ".";
import { stopAllTones } from ".";
import { logToDiv } from "./UI";

export function visibilityChange(): void {
  console.log("visibilityChange");
  if (document.hidden) {
    stopAllTones();
  }
}

export function keydown(event: KeyboardEvent): void {
  console.log("keydown");
  if (!document.hasFocus()) {
    return;
  }
  if (event.repeat) {
    return;
  }
  if (event.code in heldKeys) {
    return;
  }

  if (document.activeElement?.tagName === "BODY") {
    // if (recording) { }
    const tone_index: number = wasm.from_keymap(event.code);
    noteOn(tone_index);
    heldKeys[event.code] = true;
  }
}

export function keyup(event: KeyboardEvent): void {
  console.log("keyup");
  // if (recording) { }
  const tone_index: number = wasm.from_keymap(event.code);
  noteOff(tone_index);
  delete heldKeys[event.code];
}
