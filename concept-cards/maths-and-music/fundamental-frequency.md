---
concept: Fundamental Frequency
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
unit: null
---

# Quick Definition
The fundamental frequency F is the lowest frequency component of a periodic tone, equal to 1/P where P is the period. It determines the perceived pitch, while higher harmonics (2F, 3F, ...) determine the timbre.

# Formal Definition
For a periodic function g(t) with period P, the fundamental frequency is F = 1/P. In the Fourier series g(t) = C + sum d_k*sin(2*pi*F*k*t + beta_k), the term with k = 1 is the first harmonic or fundamental, having frequency F. All other harmonics have frequencies that are integer multiples of F.

# Mathematical Context
The fundamental frequency is the reciprocal of the period. All harmonics kF share the period 1/F (even though each individually has a shorter period 1/(kF)), which is why their sum is periodic with period 1/F. The fundamental is the greatest common divisor of all the harmonic frequencies.

# Musical Context
The fundamental determines the perceived pitch of a tone. A440 tuning means the fundamental frequency of the note A4 is 440 Hz. Even if the fundamental is weak or absent (as in some instruments or in telephony), the ear can often infer the pitch from the pattern of higher harmonics -- a phenomenon called "the missing fundamental."

# Examples
- A4: fundamental frequency 440 Hz, period 1/440 seconds
- sin(880*pi*t): frequency = 880*pi/(2*pi) = 440 Hz, so the fundamental is 440 Hz
- Starting from F2 as fundamental, the sequence of harmonics approximates: F2, F3, C4, F4, A4, C5, ...

# Related Concepts
- Harmonics and Overtones
- Overtone Series
- Periodic Functions
- Frequency and Period
- Fourier Series

# Common Confusions
Students sometimes confuse fundamental frequency with the lowest audible frequency. The fundamental is specific to a particular tone -- it is that tone's lowest harmonic component. Different notes have different fundamentals. Also, the fundamental need not be the strongest harmonic; in some instruments, higher harmonics can be louder.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," pp. 126-127 (PDF page 118).
