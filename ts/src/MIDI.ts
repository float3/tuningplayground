import { noteOn, noteOff } from ".";

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
  console.debug("onMIDISuccess");
  const input = midiAccess.inputs.values().next().value;

  if (input) {
    input.onmidimessage = onMIDIMessage;
  } else {
    alert("No MIDI input devices found.");
  }
}

function onMIDIFailure(error: DOMException): void {
  console.debug("onMIDIFailure");
  console.error("MIDI Access failed:", error);
}

function onMIDIMessage(event: WebMidi.MIDIMessageEvent): void {
  console.debug("onMIDIMessage");
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
