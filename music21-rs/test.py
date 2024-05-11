#!/usr/bin/env python3
if __name__ == "__main__":
    import sys

    sys.path.append("./music21")
    from music21 import chord

    chord_definitions = [
        "E G# B",
        "E Ab B",
        "E Ab Cb",
        "E G# Cb",
        # "C G F E B",
        # "E G F C B",
        # "E C F G B",
        # "G E F C B",
        # "G C F E B",
        # "C E G B",
        # "C E G B D",
        # "C E G B D F",
        # "C E G B D F A",
        # "F A C E",
        # "G B D F",
        # "A C E G",
        # "D F A C",
        # "E G# B D#",
        # "A C# E G#",
        # "B D# F# A#",
        # "F# A# C# E# G#",
        # "D F# A",
        # "E G B",
        # "A C# E",
        # "B D# F#",
        # "C# E# G#",
        # "F# A# C#",
        # "G# B# D#",
        # "A# C## E#",
        # "D G B",
        # "E A C#",
        # "F# B D#",
        # "G# C# F",
        # "A# D# G",
        # "C F A",
    ]

    list = [0]

    c = chord.Chord(list)
    print(c.pitchedCommonName)

    for definition in chord_definitions:
        c = chord.Chord(definition)
        print(f"{definition}: {c.pitchedCommonName}")
