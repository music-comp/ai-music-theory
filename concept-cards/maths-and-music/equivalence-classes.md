---
concept: Equivalence Classes
category: theory
source: "Mathematics and Music"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
authors: "David Wright"
unit: null
---

# Quick Definition
The set of all elements equivalent to a given element under an equivalence relation, forming one block of the partition induced by that relation.

# Formal Definition
Given an equivalence relation $\sim$ on a set $S$, the equivalence class of $s \in S$ is the set $\{t \in S \mid t \sim s\}$. The equivalence classes form a partition of $S$, meaning $S$ is the disjoint union of all equivalence classes and every element of $S$ belongs to exactly one class.

# Mathematical Context
Equivalence classes allow us to "collapse" a set by treating equivalent elements as identical. The set of all equivalence classes is called the quotient set, denoted $S/\!\sim$. Each equivalence class can be represented by any of its members (a "representative"). The partition property ensures no element belongs to more than one class.

# Musical Context
Wright identifies several musical equivalence classes:
- **Note classes**: equivalence classes under octave equivalence (e.g., the class "B$\flat$" contains all $B^\flat_n$ for $n \in \mathbb{Z}$)
- **Durational notes**: equivalence classes of notes having the same duration (e.g., "half note" regardless of pitch)
- **Interval classes**: equivalence classes of intervals modulo octave (e.g., whole step and ninth are equivalent)
- **Enharmonic classes**: equivalence classes under enharmonic equivalence (e.g., $\{F^\sharp, G^\flat\}$)

# Examples
- Under octave equivalence, the equivalence class of $B^\flat$ is $\{\ldots, B^\flat_1, B^\flat_2, B^\flat_3, B^\flat_4, B^\flat_5, \ldots\}$
- Under octave equivalence of intervals, each class has a unique representative that is positive and strictly less than an octave
- The equivalence class of $(2, 3)$ under the relation $(a,b) \sim (a',b')$ iff $ab' - a'b = 0$ corresponds to the rational number $2/3$

# Related Concepts
- Equivalence Relations
- Octave Equivalence
- Note Classes
- Enharmonic Equivalence

# Common Confusions
- A note class (like "C") is an equivalence class containing infinitely many notes ($C_0, C_1, C_2, \ldots$), not a single note
- Durational equivalence classes vs. octave equivalence classes: the former groups by duration regardless of pitch, the latter groups by pitch regardless of octave

# Source Reference
Chapter 1, "Equivalence relations" section, p. 17 (PDF)
