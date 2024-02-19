console.debug("imports");
import init, * as playground from "playground";
import * as Tone from "tone";

init(); //init wasm IMPORTANT

var octave_size: HTMLInputElement;
var tuning_select: HTMLSelectElement;

console.debug("static");
if (navigator.requestMIDIAccess) {
    navigator.requestMIDIAccess().then(onMIDISuccess, onMIDIFailure);
} else {
    alert("Web MIDI is working.");
}

octave_size = document.getElementById("octave_size") as HTMLInputElement;
tuning_select = document.getElementById("tuning_select") as HTMLSelectElement;

octave_size.onchange = () => {
    playground.set_octave_size(parseInt(octave_size.value));
}

function onMIDISuccess(midiAccess: WebMidi.MIDIAccess) {
    console.debug("onMIDISuccess");
    const input = midiAccess.inputs.values().next().value;

    if (input) {
        input.onmidimessage = onMIDIMessage;
    } else {
        alert("No MIDI input devices found.");
    }
}

function onMIDIFailure(error: DOMException) {
    console.debug("onMIDIFailure");
    console.error("MIDI Access failed:", error);
}

var recording: boolean;

function onMIDIMessage(event: WebMidi.MIDIMessageEvent) {
    console.debug("onMIDIMessage");
    const [status, note, velocity] = event.data;
    const isNoteOn = (status & 0xf0) === 0x90;
    const isNoteOff = (status & 0xf0) === 0x80;
    if (recording) { }
}

document.addEventListener("keydown", function (event) {
    console.debug("keydown");
    if (document.activeElement?.tagName === 'BODY') {
        if (recording) { }
        let tone_index: number = keyboard[event.code] + 24;
        console.log(playground.get_frequency(tuning_select.value, tone_index));
    }
});

document.addEventListener("keyup", function (event) {
    console.debug("keyup");
    if (document.activeElement?.tagName === 'BODY') {
        if (recording) { }
    }
});

const keyboard: Record<string, number> = {
    //TODO: adjust this to match real DAW keymaps and maybe detect keymap and switch between different layouts
    "IntlBackslash": -2,
    "KeyA": -1,
    "KeyZ": 0, // 24
    "KeyS": 1,
    "KeyX": 2,
    "KeyC": 3,
    "KeyF": 4,
    "KeyV": 5,
    "KeyG": 6,
    "KeyB": 7,
    "KeyN": 8,
    "KeyJ": 9,
    "KeyM": 10,
    "KeyK": 11,
    "Comma": 12,
    "KeyL": 13,
    "Period": 14,
    "Slash": 15,
    "Quote": 16,
    "Digit1": 16,
    "BackSlash": 17,
    "KeyQ": 17, // 36
    "Digit2": 18,
    "KeyW": 19,
    "KeyE": 20,
    "Digit4": 21,
    "KeyR": 22,
    "Digit5": 23,
    "KeyT": 24,
    "Digit6": 25,
    "KeyY": 26,
    "KeyU": 27,
    "Digit8": 28,
    "KeyI": 29,
    "Digit9": 30,
    "KeyO": 31,
    "KeyP": 32,
    "Minus": 33,
    "BracketLeft": 34,
    "Equal": 35,
    "BracketRight": 36,
};