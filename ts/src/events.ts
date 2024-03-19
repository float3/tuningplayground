import * as wasm from "wasm";
import { heldKeys, noteOn, noteOff } from ".";
import { stopAllTones } from ".";
import { keyMarked, transpose } from "./UI";

export function visibilityChange(): void {
  console.log("visibilityChange");
  if (document.hidden) {
    stopAllTones();
  }
}

export function onload(): void {
  console.log("onload");
  const hash = window.location.hash.substring(1);
  if (hash) {
    const notes = hash.split(",");
    notes.forEach((note) => {
      const index = parseInt(note);
      // noteOn(index);
      keyMarked(index);
    });
  }
}

export function keydown(event: KeyboardEvent): void {
  console.log("keydown");
  if (!document.hasFocus()) return;
  if (event.repeat) return;
  if (event.code in heldKeys) return;

  if (document.activeElement?.tagName === "BODY") {
    // if (recording) { }
    let tone_index: number = wasm.from_keymap(event.code);
    if (tone_index === -1) return;
    tone_index += parseInt(transpose.value);
    noteOn(tone_index);
    heldKeys[event.code] = true;
  }
}

export function keyup(event: KeyboardEvent): void {
  console.log("keyup");
  // if (recording) { }
  let tone_index: number = wasm.from_keymap(event.code);
  if (tone_index === -1) return;
  tone_index += parseInt(transpose.value);
  noteOff(tone_index);
  delete heldKeys[event.code];
}
