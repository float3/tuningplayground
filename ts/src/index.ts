console.debug('imports');
import * as tuningplayground from 'tuningplayground';
import { playingTonesChanged, logToDiv, transpose, tuningSelect, volumeSlider } from './UI';
import { Tone } from './Tone';

console.debug('static');
tuningplayground.default();

export const playingTones: Record<number, Tone> = [];
let audioContext: AudioContext;
// let recording: boolean;

export function stopAllTones(): void {
	console.debug('stopAllTones');
	for (const key in playingTones) {
		playingTones[key].Oscillator.stop();
		delete playingTones[key];
	}
	playingTonesChanged();
}

export const heldKeys: Record<string, boolean> = {};

export function noteOn(tone_index: number, velocity?: number): void {
	console.debug('noteOn');
	tone_index += parseInt(transpose.value);
	const tone: Tone = tuningplayground.get_tone(tuningSelect.value, tone_index);
	playFrequencyNative(tone, parseFloat(volumeSlider.value), tone_index);
	logToDiv(tone.name);
}

export function noteOff(tone_index: number): void {
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