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
  tranposeValue,
  // linkInputChange,
} from "./UI";

console.log("static");

document.addEventListener("DOMContentLoaded", DOMContentLoaded);
document.addEventListener("visibilitychange", visibilityChange);
window.addEventListener("blur", stopAllTones);
window.addEventListener("hashchange", onload);
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
    playingTonesChanged();
    // linkInputChange();
  })
  .catch(console.error);

export const playingTones: Record<number, Tone> = [];
export const heldKeys: Record<string, boolean> = {};
export const markedKeys: number[] = [];
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

export function noteOn(
  tone_index: number,
  velocity?: number,
  cancel?: boolean,
): void {
  console.log("noteOn");
  console.log("velocity: ", velocity);

  _noteOn(tone_index, velocity, cancel);
  playingTonesChanged();
}

export function _noteOn(
  tone_index: number,
  velocity?: number,
  cancel?: boolean,
) {
  tone_index += tranposeValue;
  const tone: Tone = wasm.get_tone(tone_index) as Tone;
  const volume = Math.pow(parseFloat(volumeSlider.value), 2);
  switch (soundMethod.value) {
    case "native":
      playFrequencyNative(tone, volume).catch(console.error);
      break;
    case "sample":
      playFrequencySample(tone, volume, cancel).catch(console.error);
      break;
  }
  keyActive(tone_index, true);
}

export function noteOff(tone_index: number): void {
  console.log("noteOff");
  tone_index += tranposeValue;
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

let audioContext: AudioContext | null = null;
function initOrGetAudioContext(): Promise<AudioContext> {
  return new Promise((resolve, reject) => {
    try {
      if (!audioContext) {
        audioContext = new window.AudioContext();
      }
      resolve(audioContext);
    } catch (error) {
      reject(error);
    }
  });
}

let audioBuffer: AudioBuffer | null = null;
function initOrGetAudioBuffer(): Promise<AudioBuffer> {
  if (!audioBuffer) {
    return fetch("a1.wav")
      .then((response) => response.arrayBuffer())
      .then((arrayBuffer) =>
        initOrGetAudioContext().then((context) =>
          context.decodeAudioData(arrayBuffer),
        ),
      )
      .then((newAudioBuffer) => {
        audioBuffer = newAudioBuffer;
        return audioBuffer;
      });
  } else {
    return Promise.resolve(audioBuffer);
  }
}

async function playFrequencySample(
  tone: Tone,
  volume: number,
  cancel?: boolean,
): Promise<void> {
  const localAudioContext = await initOrGetAudioContext();
  const source = localAudioContext.createBufferSource();
  source.buffer = await initOrGetAudioBuffer();
  const gainNode = localAudioContext.createGain();
  gainNode.gain.value = volume;
  source.connect(gainNode);
  gainNode.connect(localAudioContext.destination);
  source.playbackRate.value = tone.freq / 220;
  source.start();
  tone.node = source;
  playingTones[tone.index] = tone;
  playingTonesChanged();
  if (cancel) {
    source.onended = () => {
      noteOff(tone.index);
    };
  }
}

async function playFrequencyNative(tone: Tone, volume: number): Promise<void> {
  const localAudioContext = await initOrGetAudioContext();
  const oscillator = localAudioContext.createOscillator();
  const gainNode = localAudioContext.createGain();
  gainNode.gain.value = volume;
  gainNode.connect(localAudioContext.destination);
  oscillator.type = "square"; // TODO: make this configurable
  oscillator.frequency.setValueAtTime(tone.freq, localAudioContext.currentTime);
  oscillator.connect(gainNode);
  oscillator.start();
  tone.node = oscillator;
  if (tone.index in playingTones) playingTones[tone.index].node.stop();
  playingTones[tone.index] = tone;
  playingTonesChanged();
}
