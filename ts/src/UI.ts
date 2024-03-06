import * as wasm from 'tuningplayground';
import * as abcjs from 'abcjs';
import { playingTones, stopAllTones } from '.';

const logContainer = document.getElementById('logContainer') as HTMLDivElement;
const output = document.getElementById('output') as HTMLDivElement;

const octaveSize = document.getElementById('octaveSize') as HTMLInputElement;
const stepSize = document.getElementById('stepSize') as HTMLInputElement;
export const tuningSelect = document.getElementById('tuningSelect') as HTMLSelectElement;

export const volumeSlider = document.getElementById('volumeSlider') as HTMLInputElement;
export const transpose = document.getElementById('transpose') as HTMLInputElement;
const width = 150;
const height = 150;

output.style.width = width + 'px';
output.style.height = height + 'px';
output.style.backgroundColor = 'white';
output.style.color = 'black';

octaveSize.onchange = handleTuningSelectChange;
tuningSelect.onchange = handleTuningSelectChange;
stepSize.onchange = handleTuningSelectChange;

export function handleTuningSelectChange(): void {
	console.debug('handleTuningSelectChange');
	switch (tuningSelect.value) {
	case 'StepMethod':
		stepSize.hidden = false;
		stepSize.readOnly = false;
		octaveSize.readOnly = false;
		break;
	case 'EqualTemperament':
		stepSize.hidden = true;
		stepSize.readOnly = true;
		octaveSize.readOnly = false;
		break;
	default:
		wasm.set_tuning_system(tuningSelect.value, parseInt(octaveSize.value), parseInt(stepSize.value));
		octaveSize.value = wasm.get_tuning_size().toString();
		octaveSize.readOnly = true;
		stepSize.hidden = true;
		stepSize.readOnly = true;
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
		abcjs.renderAbc('output', wasm.convert_notes(notes));
	}
}

export function logToDiv(message: string): void {
	logContainer.innerHTML = '<p>' + message + '</p>' + logContainer.innerHTML;
}