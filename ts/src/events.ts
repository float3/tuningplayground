import * as tuningplayground from 'tuningplayground';
import { heldKeys, noteOn, noteOff } from '.';
import { stopAllTones } from '.';

document.addEventListener('keydown', keydown);
document.addEventListener('keyup', keyup);
document.addEventListener('visibilitychange', visibilityChange);
window.addEventListener('blur', stopAllTones);


function visibilityChange(): void {
	console.debug('visibilityChange');
	if (document.hidden) {
		stopAllTones();
	}
}

function keydown(event: KeyboardEvent): void {
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
}

function keyup(event: KeyboardEvent): void {
	console.debug('keyup');
	// if (recording) { }
	const tone_index: number = tuningplayground.from_keymap(event.code);
	noteOff(tone_index);
	delete heldKeys[event.code];
}