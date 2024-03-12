#!/usr/bin/env python3
if __name__ == "__main__":
    import itertools
    import json
    import sys

    sys.path.append("./music21")
    from music21 import chord

    notes = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]

    chords_dict = {}
    chords_dict[0] = ""

    for r in range(1, 13):
        combinations = itertools.combinations(range(12), r)
        for combination in combinations:
            chord_notes = [notes[i] for i in combination]
            c = chord.Chord(chord_notes).pitchedCommonName
            # print(f"{chord_notes}: {c}")
            bitflag = sum(1 << i for i in combination)
            chords_dict[bitflag] = c

    with open("../tuningplayground/chords.json", "w") as outfile:
        json.dump(chords_dict, outfile)
