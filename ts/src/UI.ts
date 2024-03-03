import * as tuningplayground from 'tuningplayground';
import * as abcjs from 'abcjs';
import { playingTones, stopAllTones } from '.';

const logContainer = document.getElementById('logContainer') as HTMLDivElement;
const output = document.getElementById('output') as HTMLDivElement;
const octaveSize = document.getElementById('octaveSize') as HTMLInputElement;
export const tuningSelect = document.getElementById('tuningSelect') as HTMLSelectElement;
export const volumeSlider = document.getElementById('volumeSlider') as HTMLInputElement;
export const transpose = document.getElementById('transpose') as HTMLInputElement;
const width = 150;
const height = 150;

output.style.width = width + 'px';
output.style.height = height + 'px';
output.style.backgroundColor = 'white';
output.style.color = 'black';

octaveSize.onchange = handleOctaveSizeChange;
tuningSelect.onchange = handleTuningSelectChange;

export function handleOctaveSizeChange(): void {
	console.debug('octaveSize.onchange');
	tuningplayground.set_octave_size(parseInt(octaveSize.value));
}

export function handleTuningSelectChange(): void {
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
}

export function playingTonesChanged(): void {
	console.debug('playingTonesChanged');

	if (octaveSize.value === '12' && Object.keys(playingTones).length > 0) {
		const notes = Object.values(playingTones).map((tone) => {
			return tone.name;
		});
		abcjs.renderAbc('output', tuningplayground.convert_notes(notes));
	}
}

export function logToDiv(message: string): void {
	logContainer.innerHTML = '<p>' + message + '</p>' + logContainer.innerHTML;
}