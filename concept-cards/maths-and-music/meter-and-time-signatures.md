---
concept: Meter and Time Signatures
category: theory
source: "Mathematics and Music"
chapter: "Horizontal Structure"
chapter_number: 2
pdf_page: 30
authors: "David Wright"
---

# Quick Definition
Meter organizes music into groups of beats (measures), and the time signature specifies the number of beats per measure and which note value receives one beat, with a special compound interpretation when the top number is divisible by 3 and greater than 3.

# Formal Definition
A piece of music is divided into measures (bars) of $n$ beats ($n \geq 1$). The time signature $\frac{n}{r}$ (written as stacked integers, not a fraction) specifies:

**Usual meaning**: $n$ = beats per measure; $r = 2^m$ designates that the $\frac{1}{2^m}$-th note gets one beat.

**Exceptional case** (compound time): when $3 \mid n$ and $n > 3$, the number of beats per measure is $n/3$, and one beat is signified by three $\frac{1}{2^m}$-th notes (i.e., a dotted $\frac{1}{2^{m-1}}$-th note).

# Mathematical Context
The bottom number $r$ is always a power of 2, reflecting the binary structure of the durational system. The time signature specifies a bijection between beat counts and note durations. In compound time, the beat unit is a dotted note (duration $\frac{3}{2} \cdot \frac{1}{2^m}$ of a whole note), introducing the factor 3 into the otherwise binary system. The integer $r$ in practice is nearly always 2, 4, or 8.

# Musical Context
Common time signatures include $\frac{4}{4}$ (4 beats, quarter note = 1 beat), $\frac{3}{4}$ (3 beats, quarter note = 1 beat, i.e. waltz time), $\frac{2}{4}$ (2 beats, quarter note = 1 beat), and $\frac{6}{8}$ (compound: 2 beats, dotted quarter = 1 beat). The time signature appears after the clef symbol and changes at subsequent positions if the meter changes. Tempo (beats per minute) determines the actual speed.

# Examples
- $\frac{4}{4}$: 4 beats per measure, quarter note gets one beat; whole note gets 4 beats
- $\frac{2}{4}$: 2 beats per measure, quarter note gets one beat
- $\frac{6}{8}$ (compound): $6/3 = 2$ beats per measure, one beat = three eighth notes = dotted quarter note
- $\frac{2}{2}$: 2 beats per measure, half note gets one beat; whole note gets 2 beats
- In $\frac{2}{2}$ time, a sixteenth note has duration $\frac{1}{16} \cdot 2 = \frac{1}{8}$ beats

# Related Concepts
- Note Durational Values
- Rhythm
- Dotted Note Duration Formula
- Horizontal Structure
- Tuplets

# Common Confusions
- The time signature $\frac{n}{r}$ is NOT a fraction, even though it looks like one -- it is two stacked integers
- $\frac{6}{8}$ is NOT 6 beats of eighth notes; it is 2 beats of dotted quarter notes (compound time)
- The distinction between $\frac{3}{4}$ (simple: 3 quarter-note beats) and $\frac{6}{8}$ (compound: 2 dotted-quarter-note beats) is fundamental despite both containing the same total duration
- $\frac{3}{4}$ is NOT compound even though 3 divides 3, because the rule requires $n > 3$

# Source Reference
Chapter 2, "Meter" section, pp. 36-37 (PDF)
