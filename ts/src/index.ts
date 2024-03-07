console.log("imports");
import * as wasm from "wasm";
import { playingTonesChanged, logToDiv, transpose, volumeSlider } from "./UI";
import { Tone, createTone } from "./Tone";
import { requestMIDI } from "./MIDI";
import { keydown, keyup, visibilityChange } from "./events";

console.log("static");

requestMIDI();

document.addEventListener("keydown", keydown);
document.addEventListener("keyup", keyup);
document.addEventListener("visibilitychange", visibilityChange);

window.addEventListener("blur", stopAllTones);
window.createTone = createTone;

wasm.default();

export const playingTones: Record<number, Tone> = [];
let audioContext: AudioContext;
// let recording: boolean;

export function stopAllTones(): void {
  console.log("stopAllTones");
  for (const key in playingTones) {
    playingTones[key].Oscillator.stop();
    delete playingTones[key];
  }
  playingTonesChanged();
}

export const heldKeys: Record<string, boolean> = {};

export async function noteOn(
  tone_index: number,
  velocity?: number,
): Promise<void> {
  console.log("noteOn");
  console.log("velocity: ", velocity);
  tone_index += parseInt(transpose.value);
  const tone: Tone = wasm.get_tone(tone_index);
  playFrequencyNative(tone, parseFloat(volumeSlider.value), tone_index);
  logToDiv(tone.name);
}

export async function noteOff(tone_index: number): Promise<void> {
  console.log("noteOff");
  tone_index += parseInt(transpose.value);
  if (!(tone_index in playingTones)) return;
  playingTones[tone_index].Oscillator.stop();
  delete playingTones[tone_index];
  playingTonesChanged();
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
