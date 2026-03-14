---
concept: Chords as Note Class Collections
category: theory
source: "Mathematics and Music"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
authors: "David Wright"
---

# Quick Definition
A chord is a collection of note classes (usually three or more) sounded simultaneously, defined by its constituent pitch classes rather than by specific octave placements of the notes.

# Formal Definition
A chord is defined by its note classes and the modular intervals (elements of Z_12) between them, rather than by specific pitches. Since chords are defined by note classes, any note in a chord may be displaced and/or doubled by one or more octaves without changing the chord's identity. The chord's type is determined by the sequence of intervals modulo octave between the notes.

# Mathematical Context
The move from specific pitches to note classes involves the quotient Z -> Z_12, where intervals measured in semitones are reduced modulo 12. A chord is essentially a subset of Z_12 (the set of note classes), together with a distinguished ordering that determines its interval sequence. The equivalence relation of octave identification (k ≡ l mod 12) underlies this abstraction.

# Musical Context
In practice, a C major chord consists of the note classes C, E, and G regardless of which specific octave each note occupies. A pianist might play C3, E4, and G4, or C4, G4, and E5 -- both are the same chord (C major) in different voicings. This abstraction allows musicians to speak of chord identity independent of register.

# Examples
- The C major chord consists of note classes {C, E, G} regardless of voicing
- Playing C2-E3-G3-C4-E4 is still a C major chord, with doubled and octave-displaced notes
- The interval sequence (4, 3, 5) in semitones defines the major chord type in Z_12

# Related Concepts
- Harmony
- Chord Types and Interval Sequences
- Voicing
- Modular Arithmetic in Z_12

# Common Confusions
- A chord is not defined by specific pitches but by note classes -- the same chord can appear in many different voicings
- Doubling a note at the octave does not add a new note class to the chord
- The number of distinct notes sounded may exceed the number of note classes in the chord

# Source Reference
Chapter 3: "Harmony and Related Numerology," pp. 44-46.
