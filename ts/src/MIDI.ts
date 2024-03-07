import { noteOn, noteOff } from ".";
import { Midi } from "@tonejs/midi";

export function requestMIDI(): void {
  if (navigator.requestMIDIAccess) {
    navigator.requestMIDIAccess().then(onMIDISuccess, onMIDIFailure);
  } else {
    alert("WebMIDI is not supported in this browser.");
  }
  if (navigator.requestMIDIAccess) {
    navigator.requestMIDIAccess().then(onMIDISuccess, onMIDIFailure);
  } else {
    alert("WebMIDI is not supported in this browser.");
  }
}

function onMIDISuccess(midiAccess: WebMidi.MIDIAccess): void {
  console.log("onMIDISuccess");
  const input = midiAccess.inputs.values().next().value;

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

async function playMIDIFile(midiFile: string): Promise<void> {
  console.log("playMIDIFile");

  const response = await fetch(midiFile);
  const arrayBuffer = await response.arrayBuffer();
  const midi = new Midi(arrayBuffer);

  const track = midi.tracks[0];

  track.notes.forEach((note) => {
    const noteOnTime = note.time;
    const noteOffTime = note.time + note.duration;
    const velocity = note.velocity;
    const midiNote = note.midi;

    setTimeout(() => noteOn(midiNote, velocity), noteOnTime * 1000);
    setTimeout(() => noteOff(midiNote), noteOffTime * 1000);
  });
}
