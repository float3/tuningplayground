#!/usr/bin/env python3
if __name__ == "__main__":
    import itertools
    import json
    import sys
    from collections import deque

    sys.path.append("./music21")
    from music21 import chord

    def rotate_left(lst):
        d = deque(lst)
        d.rotate(-1)
        return list(d)

    original_notes = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]
    notes = deque(["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"])

    outer_dict = {}
    for i in range(12):
        inner_dict = {}
        inner_dict[0] = ""
        for r in range(1, 13):
            combinations = itertools.combinations(range(12), r)
            for combination in combinations:
                chord_notes = [notes[i] for i in combination]
                c = chord.Chord(chord_notes).pitchedCommonName
                bitmask = sum(1 << i for i in combination)
                inner_dict[bitmask] = c

        outer_dict[original_notes[i]] = inner_dict
        notes = rotate_left(notes)

    with open("../ts/src/chords.json", "w") as outfile:
        json.dump(outer_dict, outfile)
