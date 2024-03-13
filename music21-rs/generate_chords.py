#!/usr/bin/env python3
if __name__ == "__main__":
    import itertools
    import json
    import sys
    from collections import deque

    sys.path.append("./music21")
    from music21 import chord

    notes = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]

    dict = {}
    dict[0] = ""
    for r in range(1, 13):
        combinations = itertools.combinations(range(12), r)
        for combination in combinations:
            chord_notes = [notes[i] for i in combination]
            c = chord.Chord(
                chord_notes
            ).pitchedCommonName  # check if pitchedCommonName contains enharmonic equivalent
            # also look into keycontext
            bitmask = sum(1 << i for i in combination)
            dict[bitmask] = c

    with open("../ts/src/chords.json", "w") as outfile:
        json.dump(dict, outfile)
