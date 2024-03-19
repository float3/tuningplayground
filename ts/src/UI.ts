import * as wasm from "wasm";
import * as abcjs from "abcjs";
import {
  _noteOn,
  markedKeys,
  noteOff,
  noteOn,
  playingTones,
  stopAllTones,
} from ".";
import { playMIDIFile, stopMIDIFile } from "./MIDI";

const octaveSize = document.getElementById("octaveSize") as HTMLInputElement;
const stepSize = document.getElementById("stepSize") as HTMLInputElement;
const fileInput = document.getElementById("fileInput") as HTMLInputElement;
export const soundMethod = document.getElementById(
  "soundMethod",
) as HTMLSelectElement;
// const linkInput = document.getElementById("linkInput") as HTMLInputElement;

const logContainer = document.getElementById("logContainer") as HTMLDivElement;
const stepSizeParent = stepSize.parentElement as HTMLDivElement;

export const playButton = document.getElementById(
  "playButton",
) as HTMLButtonElement;
export const playMarkedButton = document.getElementById(
  "playMarked",
) as HTMLButtonElement;
const stopButton = document.getElementById("stopButton") as HTMLButtonElement;

export const tuningSelect = document.getElementById(
  "tuningSelect",
) as HTMLSelectElement;

export const volumeSlider = document.getElementById(
  "volumeSlider",
) as HTMLInputElement;

const transpose = document.getElementById("transpose") as HTMLInputElement;

export const output = document.getElementById("output") as HTMLElement;

octaveSize.onchange = handleTuningSelectChange;
tuningSelect.onchange = handleTuningSelectChange;
stepSize.onchange = handleTuningSelectChange;
stopButton.onclick = stop;
fileInput.onchange = fileInputChange;
playMarkedButton.onclick = playMarkedKeys;
transpose.onchange = transposeChange;
// linkInput.onchange = linkInputChange;

export let tranposeValue = 0;
function transposeChange(): void {
  tranposeValue = parseInt(transpose.value);
}

let midiFile: ArrayBuffer;
let midiFilePromise: Promise<ArrayBuffer> | null = null;

function initOrGetMidiFile(): Promise<ArrayBuffer> {
  if (!midiFilePromise) {
    console.log("fetching sample.mid");
    midiFilePromise = fetch("sample.mid")
      .then((response) => response.arrayBuffer())
      .then((buffer) => {
        midiFile = buffer;
        return midiFile;
      })
      .catch((error) => {
        console.error(error);
        throw error;
      });
  }
  return midiFilePromise;
}

function fileInputChange(event: Event): Promise<void> {
  return new Promise((resolve, reject) => {
    const files = (event.target as HTMLInputElement).files;
    if (files && files.length > 0) {
      const reader = new FileReader();
      reader.onload = (e) => {
        midiFile = e.target!.result as ArrayBuffer;
        midiFilePromise = Promise.resolve(midiFile);
        resolve();
      };
      reader.onerror = reject;
      reader.readAsArrayBuffer(files[0]);
    } else {
      reject(new Error("No file selected"));
    }
  });
}

// export function linkInputChange(): void {
//   console.log("linkInputChange");
//   const link = linkInput.value;
//   fetch(link)
//     .then((response) => response.arrayBuffer())
//     .then((buffer) => {
//       midiFile = buffer;
//     });
// }

function playMarkedKeys(): void {
  console.log("playMarkedKeys");
  markedKeys.forEach((note) => _noteOn(note));
  playingTonesChanged;
}

function stop(): void {
  console.log("stop");
  stopMIDIFile();
}

export function play(): void {
  console.log("play");
  initOrGetMidiFile().then(playMIDIFile).catch(console.error);
}

export function DOMContentLoaded(): void {
  console.log("DOMContentLoaded");
  handleTuningSelectChange;
}

export function handleTuningSelectChange(): void {
  console.log("handleTuningSelectChange");
  switch (tuningSelect.value) {
    case "StepMethod":
      stepSizeParent.hidden = false;
      stepSize.readOnly = false;
      octaveSize.readOnly = false;
      break;
    case "EqualTemperament":
      stepSizeParent.hidden = true;
      stepSize.readOnly = true;
      octaveSize.readOnly = false;
      break;
    default:
      wasm.set_tuning_system(
        tuningSelect.value,
        parseInt(octaveSize.value),
        parseInt(stepSize.value),
      );
      octaveSize.value = wasm.get_tuning_size().toString();
      octaveSize.readOnly = true;
      stepSize.hidden = true;
      stepSize.readOnly = true;
      break;
  }
  stopAllTones();
}

function adjustOutputSize(): void {
  output.style.width = "300px";
  output.style.height = "150px";
}

export function playingTonesChanged(): void {
  console.log("playingTonesChanged");

  const notes = Object.keys(playingTones).map(Number);

  if (notes.length === 0) {
    abcjs.renderAbc("output", 'X: 1\nL: 1/1\n|""[u]|');
    adjustOutputSize();
    return;
  }

  let chordName;
  const tones = Object.values(playingTones)
    .map((tone) => tone.name)
    .join(" ");

  if (octaveSize.value === "12") {
    const formatted_notes = wasm.convert_notes(tones.split(" "));
    chordName = wasm.get_chord_name();
    abcjs.renderAbc("output", formatted_notes);
    adjustOutputSize();
  }

  logToDiv(`${tones} | ${chordName}`, notes);
}

export function logToDiv(message: string, notes: number[]): void {
  const p = document.createElement("p");
  p.textContent = message;

  const shareButton = document.createElement("button");
  shareButton.textContent = "Share";
  shareButton.onclick = function () {
    const url = `${window.location.origin + window.location.pathname}#${notes.join(",")} `;
    navigator.clipboard.writeText(url).catch(console.error);
  };

  const div = document.createElement("div");
  div.style.display = "flex";
  div.style.justifyContent = "space-between";
  div.style.alignItems = "center";
  div.appendChild(p);
  div.appendChild(shareButton);

  logContainer.insertBefore(div, logContainer.firstChild);
}

export function keyActive(tone_index: number, active: boolean) {
  const keyElement = document.querySelector(`div[data-note="${tone_index}"]`);
  if (keyElement) {
    if (active) keyElement.classList.add("key-active");
    else keyElement.classList.remove("key-active");
  }
}

export function keyMarked(tone_index: number) {
  markedKeys.push(tone_index);
  const keyElement = document.querySelector(`div[data-note="${tone_index}"]`);
  if (keyElement) {
    keyElement.classList.add("key-marked");
  }
}

export function addEvents(key: Element) {
  const note = parseInt(key.getAttribute("data-note")!) - tranposeValue;

  const addEvent = (eventName: string, callback: () => void) => {
    key.addEventListener(eventName, callback);
  };

  addEvent("mousedown", () => noteOn(note));
  addEvent("mouseup", () => noteOff(note));
  addEvent("mouseenter", () => noteOn(note));
  addEvent("mouseleave", () => noteOff(note));
  addEvent("touchstart", () => noteOn(note));
  addEvent("touchend", () => noteOff(note));
}
