import * as wasm from "wasm";
import * as abcjs from "abcjs";
import { playingTones, stopAllTones } from ".";
import { playMIDIFile } from "./MIDI";

const octaveSize = document.getElementById("octaveSize") as HTMLInputElement;
const stepSize = document.getElementById("stepSize") as HTMLInputElement;
const fileInput = document.getElementById("fileInput") as HTMLInputElement;
const linkInput = document.getElementById("linkInput") as HTMLInputElement;

const logContainer = document.getElementById("logContainer") as HTMLDivElement;
const output = document.getElementById("output") as HTMLDivElement;
const stepSizeParent = stepSize.parentElement as HTMLDivElement;

const playButton = document.getElementById("playButton") as HTMLButtonElement;
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
playButton.onclick = play;
stopButton.onclick = stop;
fileInput.onchange = fileInputChange;
linkInput.onchange = linkInputChange;

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

function linkInputChange(): void {}

function stop(): void {
  console.log("stop");
}

function play(): void {
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

  if (octaveSize.value === "12") {
    const notes = Object.values(playingTones).map((tone) => {
      return tone.name;
    });

    const formatted_notes = wasm.convert_notes(notes);

    console.log(formatted_notes);

    abcjs.renderAbc("output", formatted_notes);
  }

  let tones: string = "";
  Object.values(playingTones).forEach((element) => {
    tones += element.name + " ";
  });

  logToDiv(tones);
}

export function logToDiv(message: string): void {
  logContainer.innerHTML = "<p>" + message + "</p>" + logContainer.innerHTML;
}

const keyboardOffset = 24;

export function keyActive(tone_index: number, active: boolean) {
  const keyElement = document.querySelector(
    `div[data-note="${tone_index - keyboardOffset}"]`,
  );
  if (keyElement) {
    if (active) keyElement.classList.add("key-active");
    else keyElement.classList.remove("key-active");
  }
}
