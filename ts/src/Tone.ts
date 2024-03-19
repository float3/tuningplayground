export class Tone {
  index: number;
  freq: number;
  cents: number;
  name: string;
  node: OscillatorNode | AudioBufferSourceNode;
  constructor(
    index: number,
    freq: number,
    cents: number,
    name: string,
    node: OscillatorNode | AudioBufferSourceNode,
  ) {
    this.index = index;
    this.freq = freq;
    this.cents = cents;
    this.name = name;
    this.node = node;
  }
}

declare global {
  interface Window {
    createTone: (
      index: number,
      freq: number,
      cents: number,
      name: string,
      oscillator: OscillatorNode | AudioBufferSourceNode,
    ) => Tone;
  }
}

export function createTone(
  index: number,
  freq: number,
  cents: number,
  name: string,
  oscillator: OscillatorNode | AudioBufferSourceNode,
): Tone {
  return new Tone(index, freq, cents, name, oscillator);
}
