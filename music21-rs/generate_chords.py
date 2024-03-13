#!/usr/bin/env python3
if __name__ == "__main__":
    import itertools
    import json
    import sys

    sys.path.append("./music21")
    from music21 import chord

    dict = {}
    dict[0] = ""
    for r in range(1, 13):
        combinations = itertools.combinations(range(12), r)
        for combination in combinations:
            c = chord.Chord(combination).pitchedCommonName
            bitmask = sum(1 << i for i in combination)
            dict[bitmask] = c

    with open("../ts/src/chords.json", "w") as outfile:
        json.dump(dict, outfile)