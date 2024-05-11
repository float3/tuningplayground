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
export const markedButtons = document.getElementById(
  "markedButtons",
) as HTMLDivElement;

export const playButton = document.getElementById(
  "playButton",
) as HTMLButtonElement;
export const shareMarked = document.getElementById(
  "shareMarked",
) as HTMLButtonElement;
export const playMarked = document.getElementById(
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
fileInput.onchange = fileInputChange;
transpose.onchange = transposeChange;
volumeSlider.onchange = volumeChange;
// linkInput.onchange = linkInputChange;

stopButton.onclick = stop;
playMarked.onclick = playMarkedKeys;
shareMarked.onclick = sharedMarkedKeys;

export let tranposeValue = 0;
function transposeChange(): void {
  tranposeValue = parseInt(transpose.value);
}

export let volumeValue = 0.25;
function volumeChange(): void {
  volumeValue = parseFloat(volumeSlider.value);
}

let midiFile: ArrayBuffer;
let midiFilePromise: Promise<ArrayBuffer> | null = null;

function initOrGetMidiFile(): Promise<ArrayBuffer> {
  if (!midiFilePromise) {
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
//   const link = linkInput.value;
//   fetch(link)
//     .then((response) => response.arrayBuffer())
//     .then((buffer) => {
//       midiFile = buffer;
//     });
// }

function playMarkedKeys(): void {
  markedKeys.forEach((note) => _noteOn(note, undefined, true));
  playingTonesChanged;
}

function sharedMarkedKeys(): void {
  createAndCopyUrl(markedKeys)();
}

function stop(): void {
  stopMIDIFile();
}

export function play(): void {
  initOrGetMidiFile().then(playMIDIFile).catch(console.error);
}

export function DOMContentLoaded(): void {
  handleTuningSelectChange;
}

export function handleTuningSelectChange(): void {
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
  output.style.height = "200px";
}

export function playingTonesChanged(): void {
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

function createAndCopyUrl(keys: number[]): () => void {
  const hash = generateHash(keys);
  const url = `${window.location.origin + window.location.pathname}#${hash}`;
  return function () {
    navigator.clipboard.writeText(url).catch(console.error);
  };
}

function generateHash(keys: number[]) {
  const hash = keys.join(",");
  return hash;
}

export function logToDiv(message: string, notes: number[]): void {
  const p = document.createElement("p");
  p.textContent = message;

  const shareButton = document.createElement("button");
  shareButton.textContent = "Share";
  shareButton.onclick = createAndCopyUrl(notes);

  shareButton.style.marginRight = "10px";
  p.style.marginLeft = "10px";

  const div = document.createElement("div");
  div.style.display = "flex";
  div.style.justifyContent = "left";
  div.style.alignItems = "center";
  div.appendChild(shareButton);
  div.appendChild(p);
  logContainer.insertBefore(div, logContainer.firstChild);
}

export function keyActive(tone_index: number, active: boolean) {
  const keyElement = document.querySelector(`div[data-note="${tone_index}"]`);
  if (keyElement) {
    if (active) keyElement.classList.add("key-active");
    else keyElement.classList.remove("key-active");
  }
}

export function markKey(tone_index: number) {
  if (markedKeys.includes(tone_index)) return;
  markedKeys.push(tone_index);
  const keyElement = document.querySelector(`div[data-note="${tone_index}"]`);
  if (keyElement) {
    keyElement.classList.add("key-marked");
  }
  markedButtons.style.display = "block";
}

export function unmarkKey(tone_index: number): void {
  const index = markedKeys.indexOf(tone_index);
  if (index > -1) {
    markedKeys.splice(index, 1);
  }
  const keyElement = document.querySelector(`div[data-note="${tone_index}"]`);
  if (keyElement) {
    keyElement.classList.remove("key-marked");
  }
  if (markedKeys.length === 0) {
    markedButtons.style.display = "none";
  }
}

export function markOrUnmarkKey(tone_index: number) {
  const index = markedKeys.indexOf(tone_index);
  if (index > -1) {
    unmarkKey(tone_index);
  } else {
    markKey(tone_index);
  }
  markedKeys.sort((a, b) => a - b);
  window.location.hash = generateHash(markedKeys);
}

export function addEvents(key: Element) {
  const note = parseInt(key.getAttribute("data-note")!);

  const addEvent = (eventName: string, callback: () => void) => {
    key.addEventListener(eventName, callback);
  };

  key.addEventListener("mousedown", (event) => {
    const mouseEvent = event as MouseEvent;
    if (mouseEvent.ctrlKey) {
      markOrUnmarkKey(note);
    } else {
      noteOn(note);
    }
  });

  // key.addEventListener("mouseup", (event) => {
  //   let mouseEvent = event as MouseEvent;
  //   if (mouseEvent.ctrlKey) {
  //     unmarkKey(note);
  //   } else {
  //     noteOff(note);
  //   }
  // });

  addEvent("mouseup", () => noteOff(note));

  key.addEventListener("mouseenter", (event) => {
    const mouseEvent = event as MouseEvent;
    if (mouseEvent.ctrlKey) {
      return;
    }
    noteOn(note);
  });

  addEvent("mouseleave", () => noteOff(note));
  addEvent("touchstart", () => noteOn(note));
  addEvent("touchend", () => noteOff(note));
}
