console.debug('imports');
import * as tuningplayground from 'tuningplayground';
import * as abcjs from 'abcjs';

console.debug('static');
tuningplayground.default();

if (navigator.requestMIDIAccess) {
	navigator.requestMIDIAccess().then(onMIDISuccess, onMIDIFailure);
} else {
	alert('WebMIDI is not supported in this browser.');
}

const octaveSize = document.getElementById('octaveSize') as HTMLInputElement;
const tuningSelect = document.getElementById('tuningSelect') as HTMLSelectElement;
const volumeSlider = document.getElementById('volumeSlider') as HTMLInputElement;
const transpose = document.getElementById('transpose') as HTMLInputElement;
const logContainer = document.getElementById('logContainer') as HTMLDivElement;

class Tone {
	index: number;
	freq: number;
	cents: number;
	name: string;
	Oscillator: OscillatorNode;
	constructor(index: number, freq: number, cents: number, name: string, node: OscillatorNode) {
		this.index = index;
		this.freq = freq;
		this.cents = cents;
		this.name = name;
		this.Oscillator = node;
	}
}

declare global {
	interface Window {
		createTone: (index: number, freq: number, cents: number, name: string, oscillator: OscillatorNode) => Tone;
	}
}

window.createTone = function (index: number, freq: number, cents: number, name: string, oscillator: OscillatorNode) {
	console.debug('window.createTone');
	return new Tone(index, freq, cents, name, oscillator);
};

const playingTones: Record<number, Tone> = [];
let audioContext: AudioContext;
// let recording: boolean;

octaveSize.onchange = () => {
	console.debug('octaveSize.onchange');
	tuningplayground.set_octave_size(parseInt(octaveSize.value));
};

tuningSelect.onchange = () => {
	console.debug('tuningSelect.onchange');
	switch (tuningSelect.value) {
		case 'StepMethod':
		case 'EqualTemperament':
			octaveSize.readOnly = false;
			break;
		default:
			octaveSize.value = tuningplayground.get_tuning_size(tuningSelect.value).toString();
			octaveSize.readOnly = true;
			break;
	}

	stopAllTones();
};

window.addEventListener('blur', function () {
	console.debug('window.blur');
	stopAllTones();
});

document.addEventListener('visibilitychange', function () {
	console.debug('document.visibilitychange');
	if (document.hidden) {
		stopAllTones();
	}
});

function stopAllTones() {
	console.debug('stopAllTones');
	for (const key in playingTones) {
		playingTones[key].Oscillator.stop();
		delete playingTones[key];
	}
	playingTonesChanged();
}

function onMIDISuccess(midiAccess: WebMidi.MIDIAccess) {
	console.debug('onMIDISuccess');
	const input = midiAccess.inputs.values().next().value;

	if (input) {
		input.onmidimessage = onMIDIMessage;
	} else {
		alert('No MIDI input devices found.');
	}
}

function onMIDIFailure(error: DOMException) {
	console.debug('onMIDIFailure');
	console.error('MIDI Access failed:', error);
}

function onMIDIMessage(event: WebMidi.MIDIMessageEvent) {
	console.debug('onMIDIMessage');
	const [status, tone_index, velocity] = event.data;
	const is_note_on = (status & 0xf0) === 0x90;
	const is_note_off = (status & 0xf0) === 0x80;

	if (is_note_off) {
		noteOff(tone_index);
	}
	if (is_note_on) {
		noteOn(tone_index);
	}
}

const heldKeys: Record<string, boolean> = {};

document.addEventListener('keydown', function (event) {
	console.debug('keydown');
	if (!document.hasFocus()) return;
	if (event.repeat) return;
	if (event.code in heldKeys) return;

	if (document.activeElement?.tagName === 'BODY') {
		// if (recording) { }
		const tone_index: number = tuningplayground.from_keymap(event.code);
		noteOn(tone_index);
		heldKeys[event.code] = true;
	}
});

document.addEventListener('keyup', function (event) {
	console.debug('keyup');
	// if (recording) { }
	const tone_index: number = tuningplayground.from_keymap(event.code);
	noteOff(tone_index);
	delete heldKeys[event.code];
});

function noteOn(tone_index: number) {
	console.debug('noteOn');
	tone_index += parseInt(transpose.value);
	const tone: Tone = tuningplayground.get_tone(tuningSelect.value, tone_index);
	playFrequencyNative(tone, parseFloat(volumeSlider.value), tone_index);
	logToDiv(tone.name);
}

function noteOff(tone_index: number) {
	console.debug('noteOff');
	tone_index += parseInt(transpose.value);
	if (!(tone_index in playingTones)) return;
	playingTones[tone_index].Oscillator.stop();
	delete playingTones[tone_index];
	playingTonesChanged();
}

function playFrequencyNative(
	tone: Tone,
	volume: number,
	tone_index: number
): void {
	console.debug('playFrequencyNative');
	audioContext = new window.AudioContext();
	const oscillator = audioContext.createOscillator();
	const gainNode = audioContext.createGain();
	gainNode.gain.value = Math.pow(volume, 2);
	gainNode.connect(audioContext.destination);
	oscillator.type = 'square'; // TODO: make this configurable
	oscillator.frequency.setValueAtTime(tone.freq, audioContext.currentTime);
	oscillator.connect(gainNode);
	oscillator.start();
	tone.Oscillator = oscillator;
	if (tone_index in playingTones) playingTones[tone_index].Oscillator.stop();
	playingTones[tone_index] = tone;
	playingTonesChanged();
}

const output = document.getElementById('output') as HTMLDivElement;
const width = 150;
const height = 150;

output.style.width = width + 'px';
output.style.height = height + 'px';
output.style.backgroundColor = 'white';
output.style.color = 'black';

function playingTonesChanged() {
	console.debug('playingTonesChanged');
	if (octaveSize.value === '12' && Object.keys(playingTones).length > 0) {
		const notes = Object.values(playingTones).map((tone) => {
			return tone.name;
		});
		abcjs.renderAbc('output', tuningplayground.convert_notes(notes));
	}
}

function logToDiv(message: string): void {
	logContainer.innerHTML = '<p>' + message + '</p>' + logContainer.innerHTML;
}
