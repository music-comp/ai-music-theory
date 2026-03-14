---
concept: N-Chromatic Scale
category: theory
source: "Mathematics and Music"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
authors: "David Wright"
---

# Quick Definition
A scale that divides the octave into n equal intervals, generalizing the standard 12-tone chromatic scale to arbitrary equal divisions of the octave.

# Formal Definition
For a positive integer $n$, the n-chromatic scale is the scale obtained by dividing the octave into $n$ equal intervals. The smallest interval (the n-chromatic unit) has ratio $2^{1/n} = \sqrt[n]{2}$ and measures $1200/n$ cents. The standard chromatic scale is the special case $n = 12$.

# Mathematical Context
The n-chromatic unit is defined as $\frac{1}{n}$ of an octave. In terms of frequency ratio, if a note has frequency $f$, the next note in the n-chromatic scale has frequency $f \cdot 2^{1/n}$. The $k$th note above the starting pitch has frequency $f \cdot 2^{k/n}$. The set of intervals modulo octave in this scale is identified with the group $\mathbb{Z}_n$.

# Musical Context
The 12-chromatic scale is the foundation of Western equal temperament. Non-standard values of $n$ (such as 5, 7, 14, 19, 24, or 48) produce alternative tuning systems that have been explored by microtonal composers. Some values of $n$ produce scales whose intervals approximate standard keyboard intervals better than others.

# Examples
- $n = 12$: the standard chromatic scale, with unit = 100 cents (the semitone)
- $n = 4$: unit = 300 cents (minor third); playable on a keyboard as G, Bb, Db, E
- $n = 3$: unit = 400 cents (major third)
- $n = 6$: unit = 200 cents (whole step)
- $n = 5$: unit = 240 cents; requires detuning to play on a keyboard
- $n = 14$: unit $\approx$ 85.714 cents

# Related Concepts
- Twelve-Chromatic Scale
- Non-Standard Chromatic Scales
- Generating Interval
- Modular Integers

# Common Confusions
- The n-chromatic scale divides the octave equally by frequency ratio, not by frequency difference; the intervals are equal in logarithmic (cents) measure
- Not every n-chromatic scale can be played on a standard keyboard without detuning; only those where $n$ divides 12 (i.e., $n = 1, 2, 3, 4, 6, 12$) can use standard tuning

# Source Reference
Chapter 6, "Chromatic Scales," p. 74 (PDF)
