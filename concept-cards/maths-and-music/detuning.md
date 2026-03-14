---
concept: Detuning
category: technique
source: "Mathematics and Music"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
authors: "David Wright"
---

# Quick Definition
The practice of adjusting individual notes on a synthesizer by a specified number of cents from their standard 12-chromatic tuning, enabling performance in non-standard chromatic scales.

# Formal Definition
Detuning modifies the pitch of individual keyboard notes by a specified offset in cents from their standard equal-temperament values. To play an n-chromatic scale on a keyboard, each key is detuned so that adjacent used keys are separated by exactly $1200/n$ cents. The detuning offset for each key is calculated cumulatively from a reference pitch.

# Mathematical Context
If the n-chromatic unit is $1200/n$ cents, and the standard keyboard interval between two adjacent keys is $s$ cents (100 for semitones, 200 for whole steps), then the $k$th key above the reference requires a cumulative detuning of $k \cdot (1200/n) - d_k$ cents, where $d_k$ is the default interval from the reference in standard tuning.

# Musical Context
Many synthesizers allow individual note detuning in cents, making it possible to explore the sound of non-standard chromatic scales. This is the practical gateway to microtonal music. Detuning transforms a standard keyboard into an instrument capable of playing in any equal temperament with $n \leq 12$ (using a subset of keys).

# Examples
- 5-chromatic scale from G (unit = 240 cents): A detuned +40, B detuned +80, C detuned +220, D detuned +260 cents
- 7-chromatic scale from C (unit $\approx$ 171.43 cents): D = -28.57, E = -57.14, F = +114.29, G = +85.71, A = +57.14, B = +28.57 cents
- For $n = 4$ (unit = 300 cents): no detuning needed; use G, Bb, Db, E

# Related Concepts
- Non-Standard Chromatic Scales
- N-Chromatic Scale
- N-Tone Row Chart

# Common Confusions
- Detuning is measured in cents relative to standard equal temperament, not relative to just intonation or any other reference
- Only a subset of keyboard keys is used for the n-chromatic scale when $n < 12$; the other keys remain at their standard pitch but are not used
- Detuning offsets accumulate; each successive key's offset depends on all previous intervals

# Source Reference
Chapter 6, "Detuning" section, p. 74 (PDF)
