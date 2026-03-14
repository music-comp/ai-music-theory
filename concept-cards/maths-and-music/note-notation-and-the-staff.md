---
concept: Note Notation and the Staff
category: theory
source: "Mathematics and Music"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
authors: "David Wright"
unit: null
---

# Quick Definition
The system for representing specific pitches using notes placed on a five-line staff with treble and bass clefs, employing letters A through G and integer subscripts to identify exact pitches.

# Formal Definition
Specific pitches are notated as notes on a staff using treble and bass clefs. Notes are labeled with letters A through G. A subscript notation uniquely identifies each keyboard note: $C_0$ is the C four octaves below middle C; for any integer $n$, $C_n$ is the C lying $n$ octaves above $C_0$. Other notes receive the subscript of the highest C that is lower than or equal to that note (ignoring accidentals). Middle C is $C_4$.

# Mathematical Context
The subscript system establishes a bijection between keyboard notes and a discrete, ordered subset of $\mathbb{R}^+$ (via frequency). The naming convention creates a systematic enumeration: within each octave from $C_n$ to $B_n$, the notes form an ordered sequence. This convention interacts with accidentals: $F^\sharp$ below $C_4$ is $F^\sharp_3$, while $F^\sharp$ above $C_4$ is $F^\sharp_4$.

# Musical Context
The treble clef (G clef) and bass clef (F clef) are the standard clefs. Middle C ($C_4$) appears on a ledger line below the treble clef or above the bass clef. The vertical position on the staff determines pitch. Notes on lines and spaces follow the letter sequence A-G in ascending order, wrapping around.

# Examples
- Middle C is $C_4$; the C below middle C is $C_3$
- The lowest C on the piano keyboard is $C_1$
- $F^\sharp$ below $C_4$ is $F^\sharp_3$; $F^\sharp$ above $C_4$ is $F^\sharp_4$
- $B^\sharp_3$ and $C^\flat_4$ both coincide with $C_4$
- The lowest $B^\flat$ on the piano is $B^\flat_0$

# Related Concepts
- Pitch and Frequency
- Keyboard Layout
- Accidentals
- Note Classes
- Musical Intervals

# Common Confusions
- The subscript changes at C, not at A: $B_3$ is immediately below $C_4$, even though B comes after A alphabetically
- Accidentals are stripped before determining the subscript: find the highest C below the natural version of the note, then reattach the accidental
- $B^\sharp_3 = C_4$ and $C^\flat_4 = B_3$ illustrate the enharmonic overlap at octave boundaries

# Source Reference
Chapter 1, "Notes" section, pp. 18-19 (PDF)
