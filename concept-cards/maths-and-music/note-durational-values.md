---
concept: Note Durational Values
category: theory
source: "Mathematics and Music"
chapter: "Horizontal Structure"
chapter_number: 2
pdf_page: 30
authors: "David Wright"
unit: null
---

# Quick Definition
The system of note durations based on powers of 2, where each successive subdivision halves the duration: whole note, half note, quarter note, eighth note, and so on.

# Formal Definition
The durational names of notes are based on the whole note, whose duration in beats is dictated by the time signature. Notes with duration in proportion $1/2^n$ ($n$ a non-negative integer) to the whole note are named accordingly: the $\frac{1}{2^n}$-th note. The term "durational note" (Wright's non-standard term) means a note distinguished by its duration, independent of pitch -- formally, an equivalence class of all notes having the same duration.

# Mathematical Context
The duration system is built on powers of 2: the $\frac{1}{2^n}$-th note has duration $\frac{1}{2^n}$ relative to the whole note. This gives the sequence $1, \frac{1}{2}, \frac{1}{4}, \frac{1}{8}, \frac{1}{16}, \frac{1}{32}, \frac{1}{64}, \ldots$ The system is "highly oriented around the prime number 2 and its powers." To divide into non-power-of-2 parts requires tuplets. The concept of durational note as equivalence class parallels octave equivalence: grouping by duration rather than by pitch.

# Musical Context
The notation system encodes duration through:
1. Whether the notehead interior is filled (unfilled for whole and half notes, filled for $n \geq 2$)
2. Presence/absence of a stem and the number of flags or beams ($n - 2$ flags for $n \geq 3$)
3. Number of dots following the note
4. Tuplet designation

All notes except the whole note ($n = 0$) have stems. The eighth note ($n = 3$) has 1 flag, sixteenth ($n = 4$) has 2 flags, etc. Adjacent flagged notes may use beams instead.

# Examples
- Whole note ($n = 0$): duration 1 (relative to whole note)
- Half note ($n = 1$): duration $\frac{1}{2}$
- Quarter note ($n = 2$): duration $\frac{1}{4}$
- Eighth note ($n = 3$): 1 flag, duration $\frac{1}{8}$
- Sixty-fourth note ($n = 6$): 4 flags, duration $\frac{1}{64}$
- If the whole note gets 4 beats, the sixty-fourth note represents $\frac{1}{16}$ of a beat

# Related Concepts
- Dotted Note Duration Formula
- Tuplets
- Meter and Time Signatures
- Ties and Slurs
- Equivalence Classes
- Horizontal Structure

# Common Confusions
- "Durational note" is Wright's non-standard term for the equivalence class of notes having the same duration (e.g., "half note" regardless of pitch)
- This durational equivalence is distinct from octave equivalence; the former classifies by duration, the latter by pitch class
- The number of flags is $n - 2$ (not $n$) for the $\frac{1}{2^n}$-th note, starting at $n = 3$

# Source Reference
Chapter 2, "Duration of Notes" and "Noteheads, Stems, Flags, and Beams" sections, pp. 30-32 (PDF)
