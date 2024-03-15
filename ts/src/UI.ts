import * as wasm from "wasm";
import * as abcjs from "abcjs";
import { noteOff, noteOn, playingTones, stopAllTones } from ".";
import { playMIDIFile, stopMIDIFile } from "./MIDI";
import { keyboardOffset } from "./config";

const octaveSize = document.getElementById("octaveSize") as HTMLInputElement;
const stepSize = document.getElementById("stepSize") as HTMLInputElement;
const fileInput = document.getElementById("fileInput") as HTMLInputElement;
export const soundMethod = document.getElementById(
  "soundMethod",
) as HTMLSelectElement;
// const linkInput = document.getElementById("linkInput") as HTMLInputElement;

const logContainer = document.getElementById("logContainer") as HTMLDivElement;
const output = document.getElementById("output") as HTMLDivElement;
const stepSizeParent = stepSize.parentElement as HTMLDivElement;

export const playButton = document.getElementById(
  "playButton",
) as HTMLButtonElement;
const stopButton = document.getElementById("stopButton") as HTMLButtonElement;

export const tuningSelect = document.getElementById(
  "tuningSelect",
) as HTMLSelectElement;

export const volumeSlider = document.getElementById(
  "volumeSlider",
) as HTMLInputElement;

export const transpose = document.getElementById(
  "transpose",
) as HTMLInputElement;

const width = 150;
const height = 150;

output.style.width = width + "px";
output.style.height = height + "px";
output.style.backgroundColor = "white";
output.style.color = "black";

octaveSize.onchange = handleTuningSelectChange;
tuningSelect.onchange = handleTuningSelectChange;
stepSize.onchange = handleTuningSelectChange;
stopButton.onclick = stop;
fileInput.onchange = fileInputChange;
// linkInput.onchange = linkInputChange;

let midiFile: ArrayBuffer;

function fileInputChange(event: Event): void {
  const files = (event.target as HTMLInputElement).files;
  if (files && files.length > 0) {
    const reader = new FileReader();
    reader.onload = (e) => {
      midiFile = e.target!.result as ArrayBuffer;
    };
    reader.readAsArrayBuffer(files[0]);
  }
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

function stop(): void {
  console.log("stop");
  stopMIDIFile();
}

export function play(): void {
  console.log("play");
  playMIDIFile(midiFile);
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

export function playingTonesChanged(): void {
  console.log("playingTonesChanged");

  const notes = Object.keys(playingTones).map(Number);

  if (notes.length === 0) return;

  let chordName;
  const tones = Object.values(playingTones)
    .map((tone) => tone.name)
    .join(" ");

  if (octaveSize.value === "12") {
    const formatted_notes = wasm.convert_notes(tones.split(" "));
    chordName = wasm.get_chord_name();

    console.log(formatted_notes);

    abcjs.renderAbc("output", formatted_notes);
  }

  logToDiv(`${tones} | ${chordName}`, notes);
}

export function logToDiv(message: string, notes: number[]): void {
  const p = document.createElement("p");
  p.textContent = message;

  const shareButton = document.createElement("button");
  shareButton.textContent = "Share";
  shareButton.onclick = function () {
    const url = `${window.location.href}#${encodeURIComponent(notes.join("&"))}`;
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
  const keyElement = document.querySelector(
    `div[data-note="${tone_index - keyboardOffset}"]`,
  );
  if (keyElement) {
    if (active) keyElement.classList.add("key-active");
    else keyElement.classList.remove("key-active");
  }
}

export function keyMarked(tone_index: number) {
  const keyElement = document.querySelector(
    `div[data-note="${tone_index - keyboardOffset}"]`,
  );
  if (keyElement) {
    keyElement.classList.add("key-marked");
  }
}

export function addEvents(key: Element) {
  const note = parseInt(key.getAttribute("data-note")!) + keyboardOffset;

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
