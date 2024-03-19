import { noteOn, noteOff } from ".";
import { Midi } from "@tonejs/midi";
import { midiMultiplier } from "./config";

export function requestMIDI(): void {
  if (navigator.requestMIDIAccess) {
    navigator.requestMIDIAccess().then(onMIDISuccess, onMIDIFailure);
  } else {
    alert("WebMIDI is not supported in this browser.");
  }
}

function onMIDISuccess(midiAccess: WebMidi.MIDIAccess): void {
  console.log("onMIDISuccess");
  const input: WebMidi.MIDIInput = midiAccess.inputs.values().next()
    .value as WebMidi.MIDIInput;

  if (input) {
    input.onmidimessage = onMIDIMessage;
  } else {
    alert("No MIDI input devices found.");
  }
}

function onMIDIFailure(error: DOMException): void {
  console.log("onMIDIFailure");
  console.error("MIDI Access failed:", error);
}

function onMIDIMessage(event: WebMidi.MIDIMessageEvent): void {
  console.log("onMIDIMessage");
  const [status, tone_index, velocity] = event.data;
  const is_note_on = (status & 240) === 144;
  const is_note_off = (status & 240) === 128;

  if (is_note_off) {
    noteOff(tone_index);
  }
  if (is_note_on) {
    noteOn(tone_index, velocity);
  }
}

let timeoutIds: NodeJS.Timeout[] = [];

export function stopMIDIFile(): void {
  console.log("stopMIDIFile");
  timeoutIds.forEach((id) => clearTimeout(id));
  timeoutIds = [];
}

export function playMIDIFile(midiFile: ArrayBuffer): void {
  console.log("playMIDIFile");

  const midi = new Midi(midiFile);

  // const tempo = midi.header.tempos[0].bpm;

  midi.tracks.forEach((track) => {
    const startTime: number = 0;
    // track.notes.forEach((note) => {
    //   startTime = note.time * midiMultiplier;
    //   return;
    // });
    track.notes.forEach((note) => {
      const noteOnTime = note.time * midiMultiplier - startTime;
      const noteOffTime =
        (note.time + note.duration) * midiMultiplier - startTime;
      const velocity = note.velocity;
      const midiNote = note.midi;

      timeoutIds.push(setTimeout(() => noteOn(midiNote, velocity), noteOnTime));
      timeoutIds.push(setTimeout(() => noteOff(midiNote), noteOffTime));
    });
  });
}
