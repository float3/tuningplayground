console.debug('imports');
import * as tuningplayground from 'tuningplayground';
import * as abcjs from 'abcjs';

console.debug('static');
tuningplayground.default();

if (navigator.requestMIDIAccess) {
	navigator.requestMIDIAccess().then(on_midi_success, onMIDIFailure);
} else {
	alert('WebMIDI is not supported in this browser.');
}

const octave_size = document.getElementById('octave_size') as HTMLInputElement;
const tuning_select = document.getElementById('tuning_select') as HTMLSelectElement;
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
	return new Tone(index, freq, cents, name, oscillator);
};

const playing_tones: Record<number, Tone> = [];
let audioContext: AudioContext;
// let recording: boolean;

octave_size.onchange = () => {
	console.debug('octave_size.onchange');
	tuningplayground.set_octave_size(parseInt(octave_size.value));
};

tuning_select.onchange = () => {
	console.debug('tuning_select.onchange');
	switch (tuning_select.value) {
		case 'StepMethod':
		case 'EqualTemperament':
			octave_size.readOnly = false;
			break;
		default:
			octave_size.value = tuningplayground.get_tuning_size(tuning_select.value).toString();
			octave_size.readOnly = true;
			break;
	}

	for (const key in playing_tones) {
		playing_tones[key].Oscillator.stop();
		delete playing_tones[key];
	}
	playingTonesChanged();
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

function on_midi_message(event: WebMidi.MIDIMessageEvent) {
	console.debug('onMIDIMessage');
	const [status, tone_index, velocity] = event.data;
	const is_note_on = (status & 0xf0) === 0x90;
	const is_note_off = (status & 0xf0) === 0x80;

	if (is_note_off) {
		note_off(tone_index);
	}
	if (is_note_on) {
		note_on(tone_index);
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
		note_on(tone_index);
		heldKeys[event.code] = true;
	}
});

document.addEventListener('keyup', function (event) {
	console.debug('keyup');
	// if (recording) { }
	const tone_index: number = tuningplayground.from_keymap(event.code);
	note_off(tone_index);
	delete heldKeys[event.code];
});

function note_on(tone_index: number) {
	console.debug('note_on');
	tone_index += parseInt(transpose.value);
	const tone: Tone = tuningplayground.get_tone(tuning_select.value, tone_index);
	playFrequencyNative(tone, parseFloat(volumeSlider.value), tone_index);
}

function note_off(tone_index: number) {
	console.debug('note_off');
	tone_index += parseInt(transpose.value);
	if (!(tone_index in playing_tones)) return;
	playing_tones[tone_index].Oscillator.stop();
	delete playing_tones[tone_index];
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
	if (tone_index in playing_tones) playing_tones[tone_index].Oscillator.stop();
	playing_tones[tone_index] = tone;
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

	if (octave_size.value === '12' && Object.keys(playing_tones).length > 0) {

		const notes = Object.values(playing_tones).map((tone) => {
			return tone.name;
		});
		console.log(notes);
		const abcnotes = `L: 1/1 \n[${convertNotes(notes)}]`;
		abcjs.renderAbc('output', abcnotes);
	}
}

function convertNotes(notes: string[]): string {
	return notes.map(note => {
		const match = note.match(/([A-G])([#b]*)(N1|\d+)/);
		if (!match) return note;
		const [, pitch, accidental, octaveStr] = match;
		const formattedAccidental = accidental.replace(/#/g, '^').replace(/b/g, '_');
		let formattedOctave = '';
		const octaveNumber = parseInt(octaveStr, 10);
		if (octaveNumber === 4) {
			formattedOctave = '';
		} else if (octaveNumber < 4) {
			formattedOctave = ','.repeat(4 - octaveNumber);
		} else if (octaveNumber > 4) {
			formattedOctave = '\''.repeat(octaveNumber - 4);
		} else if (octaveStr === 'N1') {
			formattedOctave = ',,,,,';
		}
		return `${formattedAccidental}${pitch}${formattedOctave}`;
	}).join('');
}

function logToDiv(message: string): void {
	logContainer.innerHTML = '<p>' + message + '</p>' + logContainer.innerHTML;
}
