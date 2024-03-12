#!/usr/bin/env python3
import sys

sys.path.append('./music21')

import music21

# here are a few common chords
chord = music21.chord.Chord("C E G")
print(chord.pitchedCommonName)
chord = music21.chord.Chord("C E G B")
print(chord.pitchedCommonName)
chord = music21.chord.Chord("C E G B D")
print(chord.pitchedCommonName)
chord = music21.chord.Chord("C E G B D F")
print(chord.pitchedCommonName)
chord = music21.chord.Chord("C E G B D F A")
print(chord.pitchedCommonName)
chord = music21.chord.Chord("C E G B D F A C")
print(chord.pitchedCommonName)
chord = music21.chord.Chord("C E G B D F A C E")
print(chord.pitchedCommonName)
