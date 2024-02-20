console.debug('imports');
import init, * as playground from 'playground';

init(); //init wasm IMPORTANT

console.debug('static');
if (navigator.requestMIDIAccess) {
	navigator.requestMIDIAccess().then(on_midi_success, onMIDIFailure);
} else {
	alert('Web MIDI is working.');
}

const octave_size = document.getElementById('octave_size') as HTMLInputElement;
const tuning_select = document.getElementById('tuning_select') as HTMLSelectElement;
const volumeSlider = document.getElementById('volumeSlider') as HTMLInputElement;

octave_size.onchange = () => {
	playground.set_octave_size(parseInt(octave_size.value));
};

function on_midi_success(midiAccess: WebMidi.MIDIAccess) {
	console.debug('onMIDISuccess');
	const input = midiAccess.inputs.values().next().value;

	if (input) {
		input.onmidimessage = on_midi_message;
	} else {
		alert('No MIDI input devices found.');
	}
}

function onMIDIFailure(error: DOMException) {
	console.debug('onMIDIFailure');
	console.error('MIDI Access failed:', error);
}

//let recording: boolean;

function on_midi_message(event: WebMidi.MIDIMessageEvent) {
	console.debug('onMIDIMessage');
	const [status, note, velocity] = event.data;
	const is_note_on = (status & 0xf0) === 0x90;
	const is_note_off = (status & 0xf0) === 0x80;

	if (is_note_off) {
		note_off(note);
	}
	if (is_note_on) {
		note_on(note);
	}
}

document.addEventListener('keydown', function (event) {
	console.debug('keydown');
	if (!document.hasFocus()) return;
	if (!(event.code in keyboard)) return;
	if (event.repeat) return;

	if (document.activeElement?.tagName === 'BODY') {
		// if (recording) { }
		const tone_index: number = keyboard[event.code] + 24 + 24;
		note_on(tone_index);
	}
});

document.addEventListener('keyup', function (event) {
	console.debug('keyup');
	if (!(event.code in keyboard)) return;

	// if (recording) { }
	const tone_index: number = keyboard[event.code] + 24 + 24;
	note_off(tone_index);
});

const playing_tones: Record<number, OscillatorNode> = [];

function note_on(tone_index: number) {
	console.debug('note_on');
	const freq = playground.get_frequency(tuning_select.value, tone_index);
	console.debug('tone_index', tone_index);
	console.debug('freq', freq);
	playFrequencyNative(freq, parseFloat(volumeSlider.value), tone_index);
}

function note_off(tone_index: number) {
	console.debug('note_off');
	playing_tones[tone_index].stop();
}

let audioContext: AudioContext;

function playFrequencyNative(
	frequency: number,
	volume: number,
	tone_index: number
): void {
	audioContext = new window.AudioContext();
	const oscillator = audioContext.createOscillator();
	const gainNode = audioContext.createGain();
	gainNode.gain.value = volume;
	gainNode.connect(audioContext.destination);
	oscillator.type = 'square'; // TODO: make this configurable
	oscillator.frequency.setValueAtTime(frequency, audioContext.currentTime);
	oscillator.connect(gainNode);
	oscillator.start();
	playing_tones[tone_index] = oscillator;
}


const keyboard: Record<string, number> = {
	//TODO: adjust this to match real DAW keymaps and maybe detect keymap and switch between different layouts
	IntlBackslash: -2,
	KeyA: -1,
	KeyZ: 0, // 24
	KeyS: 1,
	KeyX: 2,
	KeyC: 3,
	KeyF: 4,
	KeyV: 5,
	KeyG: 6,
	KeyB: 7,
	KeyN: 8,
	KeyJ: 9,
	KeyM: 10,
	KeyK: 11,
	Comma: 12,
	KeyL: 13,
	Period: 14,
	Slash: 15,
	Quote: 16,
	Digit1: 16,
	BackSlash: 17,
	KeyQ: 17, // 36
	Digit2: 18,
	KeyW: 19,
	KeyE: 20,
	Digit4: 21,
	KeyR: 22,
	Digit5: 23,
	KeyT: 24,
	Digit6: 25,
	KeyY: 26,
	KeyU: 27,
	Digit8: 28,
	KeyI: 29,
	Digit9: 30,
	KeyO: 31,
	KeyP: 32,
	Minus: 33,
	BracketLeft: 34,
	Equal: 35,
	BracketRight: 36,
};
