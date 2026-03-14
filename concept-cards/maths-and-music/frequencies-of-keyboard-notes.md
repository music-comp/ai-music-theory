---
concept: Frequencies of Keyboard Notes
category: application
source: "Mathematics and Music"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
authors: "David Wright"
---

# Quick Definition
Given a reference pitch (A4 = 440 Hz) and the semitone ratio 2^(1/12), the frequency of any keyboard note can be calculated by multiplying 440 by the appropriate power of 2^(1/12).

# Formal Definition
If a note N has frequency f and an interval has ratio r, the note lying the interval r from N has frequency rf. Given the standard tuning A4 = 440 Hz, any note that is k semitones above A4 has frequency:

f = 440 * 2^(k/12)

For notes below A4, k is negative. More generally, given any note's frequency, applying an interval of ratio r yields a new frequency of rf.

# Mathematical Context
This is a direct application of the semitone ratio formula. The frequency mapping is an exponential function of the chromatic pitch number: if we assign A4 the number 0, then pitch number k maps to frequency 440 * 2^(k/12). This is a strictly increasing function, confirming the one-to-one correspondence between pitch numbers and frequencies. The non-equidistance of chromatic frequencies on a linear axis reflects the exponential nature of this mapping.

# Musical Context
Standard tuning (A4 = 440 Hz, also called A440 or concert pitch) is the international standard. From this single reference, the entire keyboard is tuned. The computed frequencies are the basis for electronic instrument tuning, synthesizer design, and acoustic instrument manufacturing.

# Examples
- A4 = 440 Hz (reference)
- A3 = 220 Hz (one octave below: 440 * 2^(-1))
- A5 = 880 Hz (one octave above: 440 * 2^1)
- C#4 = 220 * 2^(1/3) ~ 277.18 Hz (major third above A3)
- The chromatic pitches from C4 to C5, when plotted on a number line, are not equidistant

# Related Concepts
- Semitone Ratio
- Interval as Frequency Ratio
- Multiplicative Composition of Intervals

# Common Confusions
- Equal temperament means equal *ratio* spacing, not equal *frequency* spacing -- higher notes have larger frequency differences between adjacent semitones
- A4 = 440 Hz is a convention, not a physical law; historically and in some modern practices, other reference frequencies are used
- The frequency formula works for fractional semitone values too, not just integers

# Source Reference
Chapter 4: "Ratios and Musical Intervals," pp. 60-61.
