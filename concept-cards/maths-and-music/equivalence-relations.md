---
concept: Equivalence Relations
category: theory
source: "Mathematics and Music"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
authors: "David Wright"
---

# Quick Definition
A relation on a set that satisfies reflexivity, symmetry, and transitivity, providing the mathematical framework for identifying objects that are "the same" in some specified sense.

# Formal Definition
Let $S$ be a set and $\sim$ a relation on $S$. The relation $\sim$ is an equivalence relation if for all $s, t, u \in S$:
1. $s \sim s$ (reflexivity)
2. If $s \sim t$, then $t \sim s$ (symmetry)
3. If $s \sim t$ and $t \sim u$, then $s \sim u$ (transitivity)

# Mathematical Context
Equivalence relations partition a set into disjoint equivalence classes. The equivalence class of $s \in S$ is the set $\{t \in S \mid t \sim s\}$. The equivalence classes form a partition of $S$, meaning $S$ is the disjoint union of its equivalence classes. Every element belongs to exactly one equivalence class.

# Musical Context
Several fundamental musical concepts are formalized as equivalence relations:
- **Octave equivalence**: two notes are equivalent if the interval between them is $n$ octaves for some $n \in \mathbb{Z}$
- **Enharmonic equivalence**: two notes are equivalent if they produce the same pitch (e.g., $F^\sharp$ and $G^\flat$)
- **Durational equivalence**: notes are equivalent if they have the same duration, giving rise to "durational notes" (e.g., "half note" as an equivalence class)

# Examples
- "Same color" on a set of solid-colored objects satisfies all three properties
- On $\mathbb{Z}$, the relation $k \equiv \ell$ iff $n \mid (k - \ell)$ for a fixed positive integer $n$ is an equivalence relation (Exercise 3c)
- On piano notes, "interval is a major third" is NOT an equivalence relation (fails transitivity -- Exercise 3d)
- The relation $(a,b) \sim (a',b')$ iff $ab' - a'b = 0$ on $\{(a,b) \in \mathbb{Z}^2 \mid b \neq 0\}$ gives equivalence classes corresponding to $\mathbb{Q}$ (Exercise 4)

# Related Concepts
- Equivalence Classes
- Octave Equivalence
- Note Classes
- Enharmonic Equivalence

# Common Confusions
- Not every relation is an equivalence relation: $\leq$ on $\mathbb{R}$ fails symmetry
- The "major third" relation on piano notes fails transitivity (three major thirds span 12 semitones = octave, but the starting and ending notes are not a major third apart)
- An equivalence class is a set of elements, not a single element

# Source Reference
Chapter 1, "Equivalence relations" section, p. 17 (PDF)
