#!/usr/bin/env python3
import itertools
import sys

if __name__ == "__main__":

    sys.path.append("./music21")
    from music21 import chord

    data = []
    for r in range(1, 13):
        combinations = itertools.combinations(range(12), r)
        for combination in combinations:
            c = chord.Chord(combination).pitchedCommonName
            bitmask = sum(1 << i for i in combination)
            data.append((bitmask, c))

    data.sort()

    with open("../ts/src/chords.txt", "w") as outfile:
        for item in data:
            outfile.write(f"{item[1]};")
