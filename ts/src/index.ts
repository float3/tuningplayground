import * as tuning_systems from "tuning_systems-wasm";
import * as Tone from "tone";

document.addEventListener("DOMContentLoaded", () => {
    if (navigator.requestMIDIAccess) {
        navigator.requestMIDIAccess().then(onMIDISuccess, onMIDIFailure);
    } else {
        alert("Web MIDI is working.");
    }
});

function onMIDISuccess(midiAccess: WebMidi.MIDIAccess) {
    const input = midiAccess.inputs.values().next().value;

    if (input) {
        input.onmidimessage = onMIDIMessage;
    } else {
        alert("No MIDI input devices found.");
    }
}

function onMIDIFailure(error: DOMException) {
    console.error("MIDI Access failed:", error);
}

function onMIDIMessage(event: WebMidi.MIDIMessageEvent) {
    const [status, note, velocity] = event.data;
    const isNoteOn = (status & 0xf0) === 0x90;
    const isNoteOff = (status & 0xf0) === 0x80;
}

document.addEventListener("keydown", function (event) {
});

document.addEventListener("keyup", function (event) {
});