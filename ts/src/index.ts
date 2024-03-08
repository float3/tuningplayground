console.log("imports");
import * as wasm from "wasm";
import { Tone, createTone } from "./Tone";
import { requestMIDI } from "./MIDI";
import { keydown, keyup, visibilityChange } from "./events";
import {
  playingTonesChanged,
  logToDiv,
  transpose,
  volumeSlider,
  keyActive,
} from "./UI";

console.log("static");

requestMIDI();

document.addEventListener("keydown", keydown);
document.addEventListener("keyup", keyup);
document.addEventListener("visibilitychange", visibilityChange);

window.addEventListener("blur", stopAllTones);
window.createTone = createTone;

void wasm.default(); // TODO: don't allow stuff to happen before this finishes

export const playingTones: Record<number, Tone> = [];
let audioContext: AudioContext;
// let recording: boolean;

export function stopAllTones(): void {
  console.log("stopAllTones");
  Object.keys(playingTones).forEach((key) => {
    const tone_index: number = parseInt(key);
    playingTones[tone_index].Oscillator.stop();
    delete playingTones[tone_index];
    keyActive(tone_index, false);
  });
  playingTonesChanged();
}

export const heldKeys: Record<string, boolean> = {};

export function noteOn(tone_index: number, velocity?: number): void {
  console.log("noteOn");
  console.log("velocity: ", velocity);
  tone_index += parseInt(transpose.value);
  const tone: Tone = wasm.get_tone(tone_index) as Tone;
  playFrequencyNative(tone, parseFloat(volumeSlider.value), tone_index);
  keyActive(tone_index, true);
  logToDiv(tone.name);
}

export function noteOff(tone_index: number): void {
  console.log("noteOff");
  tone_index += parseInt(transpose.value);
  if (!(tone_index in playingTones)) return;
  playingTones[tone_index].Oscillator.stop();
  delete playingTones[tone_index];
  playingTonesChanged();
  keyActive(tone_index, false);
}

function playFrequencyNative(
  tone: Tone,
  volume: number,
  tone_index: number,
): void {
  console.log("playFrequencyNative");
  audioContext = new window.AudioContext();
  const oscillator = audioContext.createOscillator();
  const gainNode = audioContext.createGain();
  gainNode.gain.value = Math.pow(volume, 2);
  gainNode.connect(audioContext.destination);
  oscillator.type = "square"; // TODO: make this configurable
  oscillator.frequency.setValueAtTime(tone.freq, audioContext.currentTime);
  oscillator.connect(gainNode);
  oscillator.start();
  tone.Oscillator = oscillator;
  if (tone_index in playingTones) playingTones[tone_index].Oscillator.stop();
  playingTones[tone_index] = tone;
  playingTonesChanged();
}
