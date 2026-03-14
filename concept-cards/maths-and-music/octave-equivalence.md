---
concept: Octave Equivalence
category: theory
source: "Mathematics and Music"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
authors: "David Wright"
---

# Quick Definition
The principle that notes separated by one or more octaves are considered equivalent, reducing the infinite set of keyboard notes to just twelve note classes.

# Formal Definition
Octave equivalence is an equivalence relation on the set of chromatic scale notes: two notes are related if the interval between them is $n$ octaves (i.e., $12n$ semitones) for some integer $n \in \mathbb{Z}$. This relation satisfies reflexivity ($n = 0$), symmetry (if the interval is $12n$, the reverse is $-12n = 12(-n)$), and transitivity (if intervals are $12n$ and $12m$, their sum is $12(n+m)$). The term "modulo octave" refers to this equivalence relation.

# Mathematical Context
Octave equivalence is a concrete instance of modular arithmetic: identifying notes modulo 12 semitones. Each equivalence class of intervals has a unique representative that is positive and strictly less than an octave (i.e., between 0 and 11 semitones inclusive). This connects to the Division Algorithm: given an interval of $n$ semitones, $n = 12q + r$ with $0 \leq r < 12$, and $r$ identifies the interval class.

# Musical Context
Under octave equivalence, there are only 12 distinct note classes on the piano. A note written without a subscript (e.g., "A" rather than "$A_4$") denotes an equivalence class. Music notation and terminology routinely assume octave equivalence: key signatures, chord names, and scale patterns all operate modulo octave. The equivalence also applies to intervals: a whole step and a ninth are equivalent modulo octave.

# Examples
- $B^\flat_2$ and $B^\flat_5$ are equivalent modulo octave
- The note class $B^\flat$ is the equivalence class $\{B^\flat_n \mid n \in \mathbb{Z}\}$
- A whole step (2 semitones) and a ninth (14 semitones) are equivalent modulo octave, since $14 = 1 \cdot 12 + 2$
- Modulo octave, there are exactly 12 note classes: C, C$^\sharp$/D$^\flat$, D, D$^\sharp$/E$^\flat$, E, F, F$^\sharp$/G$^\flat$, G, G$^\sharp$/A$^\flat$, A, A$^\sharp$/B$^\flat$, B

# Related Concepts
- Equivalence Relations
- Equivalence Classes
- Note Classes
- Division Algorithm
- Musical Intervals

# Common Confusions
- Octave equivalence is a mathematical choice, not a physical fact: $A_2$ (110 Hz) and $A_5$ (880 Hz) are perceptually similar but physically distinct
- Under octave equivalence, only 7 scale numbers are needed ($\hat{1}$ through $\hat{7}$), though $\hat{9}$ is sometimes used when octave identification is suspended
- The standard scale ends on the same note it begins (e.g., C D E F G A B C), but the final C is redundant under octave equivalence

# Source Reference
Chapter 1, "Octave Equivalence" section, pp. 20-21 (PDF)
