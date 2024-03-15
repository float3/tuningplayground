console.log("imports");
import * as wasm from "wasm";
import { Tone, createTone } from "./Tone";
import { requestMIDI } from "./MIDI";
import { keydown, keyup, visibilityChange, onload } from "./events";
import {
  playingTonesChanged,
  volumeSlider,
  keyActive,
  DOMContentLoaded,
  addEvents,
  playButton,
  play,
  soundMethod,
  // linkInputChange,
} from "./UI";

console.log("static");

document.addEventListener("DOMContentLoaded", DOMContentLoaded);
document.addEventListener("visibilitychange", visibilityChange);
window.addEventListener("blur", stopAllTones);
window.createTone = createTone;
wasm
  .default()
  .then(() => {
    //make sure do anything that can call wasm after wasm has finished importing
    requestMIDI();
    playButton.onclick = play;
    document.addEventListener("keydown", keydown);
    document.addEventListener("keyup", keyup);
    document.querySelectorAll(".white-key, .black-key").forEach((key) => {
      addEvents(key);
    });
    onload();
    // linkInputChange();
  })
  .catch(console.error);

export const playingTones: Record<number, Tone> = [];
let audioContext: AudioContext;
// let recording: boolean;

export function stopAllTones(): void {
  console.log("stopAllTones");
  Object.keys(playingTones).forEach((key) => {
    const tone_index: number = parseInt(key);
    playingTones[tone_index].node.stop();
    delete playingTones[tone_index];
    keyActive(tone_index, false);
  });
  playingTonesChanged();
}

export const heldKeys: Record<string, boolean> = {};

export function noteOn(tone_index: number, velocity?: number): void {
  console.log("noteOn");
  console.log("velocity: ", velocity);
  const tone: Tone = wasm.get_tone(tone_index) as Tone;
  const volume = Math.pow(parseFloat(volumeSlider.value), 2);
  switch (soundMethod.value) {
    case "native":
      playFrequencyNative(tone, volume);
      break;
    case "sample":
      playFrequencySample(tone, volume);
      break;
  }
  playingTonesChanged();
  keyActive(tone_index, true);
}

export function noteOff(tone_index: number): void {
  console.log("noteOff");
  if (!(tone_index in playingTones)) return;

  switch (soundMethod.value) {
    case "native":
      playingTones[tone_index].node.stop();
      break;
    case "sample":
      break;
  }
  delete playingTones[tone_index];
  playingTonesChanged();
  keyActive(tone_index, false);
}

function playFrequencySample(tone: Tone, volume: number): void {
  console.log("playFrequencySample");
  audioContext = new window.AudioContext();
  fetch("a1.wav")
    .then((response) =>
      response
        .arrayBuffer()
        .then((arrayBuffer) => audioContext.decodeAudioData(arrayBuffer)),
    )
    .then((audioBuffer) => {
      const source = audioContext.createBufferSource();
      source.buffer = audioBuffer;
      const gainNode = audioContext.createGain();
      gainNode.gain.value = volume;
      source.connect(gainNode);
      gainNode.connect(audioContext.destination);
      source.playbackRate.value = tone.freq / 220;
      source.start();
      tone.node = source;
      playingTones[tone.index] = tone;
      playingTonesChanged();
    })
    .catch(console.error);
}

function playFrequencyNative(tone: Tone, volume: number): void {
  console.log("playFrequencyNative");
  audioContext = new window.AudioContext();
  const oscillator = audioContext.createOscillator();
  const gainNode = audioContext.createGain();
  gainNode.gain.value = volume;
  gainNode.connect(audioContext.destination);
  oscillator.type = "square"; // TODO: make this configurable
  oscillator.frequency.setValueAtTime(tone.freq, audioContext.currentTime);
  oscillator.connect(gainNode);
  oscillator.start();
  tone.node = oscillator;
  if (tone.index in playingTones) playingTones[tone.index].node.stop();
  playingTones[tone.index] = tone;
  playingTonesChanged();
}
