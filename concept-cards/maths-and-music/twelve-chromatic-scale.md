---
concept: Twelve-Chromatic Scale
category: theory
source: "Mathematics and Music"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
authors: "David Wright"
---

# Quick Definition
The standard Western chromatic scale, formed by dividing the octave into 12 equal semitone intervals, each measuring 100 cents.

# Formal Definition
The 12-chromatic scale is the n-chromatic scale with $n = 12$. Its smallest interval, the semitone, has ratio $2^{1/12} = \sqrt[12]{2} \approx 1.05946$ and measures exactly 100 cents. The set of modular chromatic intervals is identified with $\mathbb{Z}_{12}$.

# Mathematical Context
Since the chromatic unit is $1200/12 = 100$ cents, any keyboard interval can be expressed as an integer multiple of 100 cents: a semitone is 1 unit, a whole step is 2 units, a minor third is 3, a major third is 4, a fourth is 5, a tritone is 6, a fifth is 7, and so on up to 11 units (major seventh). Modular arithmetic in $\mathbb{Z}_{12}$ captures the full algebra of chromatic interval composition under octave equivalence.

# Musical Context
The subdivision of the octave into 12 equal intervals became standard in Western music only within the last 200 years. It is not universal across musical traditions, and its adoption was influenced by the desire to approximate just intervals while enabling free modulation between keys. The number 12! = 479,001,600 gives the number of possible orderings of all 12 note classes, relevant to twelve-tone composition.

# Examples
- The semitone (100 cents) is the 12-chromatic unit
- A fourth (5 semitones) composed with a fifth (7 semitones) gives $5 + 7 = 12 \equiv 0 \pmod{12}$, the octave/unison
- Two fifths: $7 + 7 = 14 \equiv 2 \pmod{12}$, a whole step
- The generating intervals are those $[m] \in \mathbb{Z}_{12}$ with $\gcd(m, 12) = 1$: the semitone [1], fourth [5], fifth [7], and major seventh [11]

# Related Concepts
- N-Chromatic Scale
- Modular Chromatic Intervals
- Generating Interval
- Twelve-Tone Technique

# Common Confusions
- Equal temperament (12-chromatic scale) is an approximation of just intonation, not identical to it; fifths are slightly flat and thirds are slightly sharp compared to just intervals
- The choice of 12 is not purely arbitrary but relates to how well powers of $2^{1/12}$ approximate simple frequency ratios like 3/2 and 5/4

# Source Reference
Chapter 6, "Chromatic Scales," p. 74 (PDF)
