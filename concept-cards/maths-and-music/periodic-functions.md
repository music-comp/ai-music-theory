---
concept: Periodic Functions
category: theory
source: "Mathematics and Music"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
authors: "David Wright"
---

# Quick Definition
A periodic function repeats its values at regular intervals. The smallest such interval is the period P, and the function's frequency is 1/P.

# Formal Definition
A function f(x) whose domain is all of R is periodic if there is a positive number P such that f(x + P) = f(x) for all x in R. The number P is the period. The behavior of the function is completely determined by its values on any half-open interval of length P, such as [0, P).

# Mathematical Context
Any function defined on [0, P) can be uniquely extended to a periodic function on all of R by setting g(x) = f(x - nP) for x in [nP, (n+1)P) for all integers n. This procedure is called "extending from [0, P) to R by periodicity." If f(t) has period P, then f(t) also has period nP for any positive integer n.

# Musical Context
Sound waves are periodic functions of time: the repeating pattern of air pressure variation is what the ear perceives as a sustained musical tone. The period P (in seconds) determines the pitch, with frequency F = 1/P Hz. The shape of one period determines the timbre.

# Examples
- sin(x) and cos(x) have period 2*pi
- A square wave alternating between 1 and -1 every pi units has period 2*pi
- A vibrating string producing A4 (440 Hz) has period P = 1/440 seconds

# Related Concepts
- Frequency and Period
- Geometric Transformations on Periodic Functions
- Fourier Series
- Vibrations and Sound Waves

# Common Confusions
The period is not necessarily the smallest positive number P for which f(x + P) = f(x). The formal definition allows any such P to be called "a period." However, in practice we typically refer to the smallest such P as "the period." If f has period P, it also has period 2P, 3P, etc.

# Source Reference
Chapter 10: "Timbre and Periodic Functions," p. 120 (PDF page 118).
