---
concept: Modular Clock
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
unit: null
---

# Quick Definition
A circular diagram with $n$ equally spaced positions representing the elements of $\mathbb{Z}_n$, where addition corresponds to clockwise rotation, used to visualize modular arithmetic and chromatic intervals.

# Formal Definition
The group $\mathbb{Z}_n$ is realized as the group of rotations of a regular $n$-gon, or equivalently as a clock with $n$ positions dividing a circle into $n$ equal arcs. The top position is labeled $[0]$, and positions are labeled $[1], [2], \ldots, [n-1]$ clockwise. Addition $[k] + [\ell]$ is computed by rotating the top position clockwise by $k$ positions, then by $\ell$ positions.

# Mathematical Context
The modular clock establishes a bijection between elements of $\mathbb{Z}_n$ and positions on the clock. Addition in $\mathbb{Z}_n$ becomes rotation: $[k] + [\ell]$ is where the top position lands after rotating by $k$ then by $\ell$ positions (counterclockwise if negative). This geometric realization makes the group structure visually apparent and aids in converting between modular integers and note classes.

# Musical Context
The 12-position modular clock is the "chromatic clock" used extensively in twelve-tone theory. Labeling each position with both its modular integer $[k]$ and the corresponding note class (starting from a designated note at $[0]$) allows rapid conversion between numerical and musical representations. This is essential for constructing and reading row charts.

# Examples
- $\mathbb{Z}_4$ is the group of rotations of a square, with four clock positions
- $\mathbb{Z}_{12}$ labeled from E: $[0]$ = E, $[1]$ = F, $[2]$ = F$\sharp$, ..., $[11]$ = D$\sharp$, used to translate row chart entries to note names
- $\mathbb{Z}_7$ labeled from C: $[0]$ = C, $[1]$ = D, $[2]$ = E, $[3]$ = F, $[4]$ = G, $[5]$ = A, $[6]$ = B, for 7-tone composition
- Adding $[5] + [7]$ in $\mathbb{Z}_{12}$: rotate 5 then 7 positions clockwise, landing on $[0]$

# Related Concepts
- Modular Integers
- Modular Arithmetic
- Modular Chromatic Intervals
- Wrapping Real Line Around Circle
- Row Chart

# Common Confusions
- The modular clock is not the same as an ordinary clock, despite the analogy; it has $n$ positions (not necessarily 12) and the "hours" start at 0
- Clockwise rotation corresponds to positive addition; counterclockwise corresponds to negative
- The starting position $[0]$ can be assigned to any note class, depending on the context (C, E, etc.)

# Source Reference
Chapter 7, "Modular clock" section, p. 82 (PDF)
