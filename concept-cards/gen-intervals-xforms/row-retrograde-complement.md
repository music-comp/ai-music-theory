---
concept: Row-Retrograde as Complement
slug: row-retrograde-complement

category: generalized-set-theory
subcategory: serial-theory
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: "Example 6.6.2"

extraction_confidence: high

aliases:
  - "retrograde as complement in PROT"

prerequisites:
  - protocol-pairs
  - inj-function
  - set-complementation
extends: []
related:
  - generalized-hexachord-theorem
  - retrograde-operation
  - inj-complement-theorem
contrasts_with: []

answers_questions:
  - "How are a twelve-tone row and its retrograde related as subsets of PROT?"
  - "Why does the Generalized Hexachord Theorem apply to rows and retrogrades?"
  - "What does it mean for a row to be a 'complement' of its retrograde in PROT?"
---

# Quick Definition
In the PROT (protocol pairs) model of twelve-tone rows, a row L and its retrograde R(L) are set-theoretic complements: L-bar = R(L). This places the row/retrograde relationship in the same formal position as hexachord/complement, and the Generalized Hexachord Theorem (6.6.1E) applies.

# Core Definition
For a row L modeled as a subset of PROT (the 132 ordered pairs of distinct pitch classes): L contains the 66 protocol pairs (p, q) where p precedes q in the row. Its complement L-bar contains the remaining 66 pairs. Since (p, q) is in L-bar if and only if q precedes p in L, which is if and only if (p, q) is in R(L), we have L-bar = R(L) (Lewin, Example 6.6.2, p. 154).

# Prerequisites
- **Protocol pairs (PROT)** — The space of 132 ordered pairs of distinct pitch classes
- **INJ function** — The injection function that generalizes IFUNC
- **Set complementation** — L-bar is the complement of L within PROT

# Key Properties
1. A row L as a subset of PROT contains 66 protocol pairs (all pairs where p precedes q)
2. PROT has 132 pairs total, so L-bar also has 66 pairs
3. L-bar = R(L): the complement equals the retrograde
4. The retrograde operation R on PROT is R(p, q) = (q, p)
5. cardL = 66 = (1/2) * cardPROT, placing L in the formal position of a "hexachord"
6. Theorem 6.6.1(E) applies: INJ(L, L)(OP) = INJ(R(L), R(L))(OP) for any operation OP

# Construction / Recognition
## To Construct:
1. Represent a row L as the set of all (p, q) pairs where p precedes q
2. The complement L-bar = {(p, q) : (q, p) in L} = R(L)
3. Apply Theorem 6.6.1(E) to deduce INJ equalities
## To Recognize:
1. A row and its retrograde sharing the same INJ internal structure
2. The complement relationship holding in PROT, not in pitch-class space

# Context & Application
The complement relationship extends Babbitt's hexachord theorem to serial structure. Just as the hexachord theorem says a hexachord and its complement have the same interval content, the generalized version says a row and its retrograde have the same "internal INJ structure." The formal analogy suggests deep connections between set-class theory and serial theory, unified through the PROT model and INJ function.

# Examples
**Example 1** (Example 6.6.2, Moses und Aron):
- Row L = A-Bb-E-D-Eb-C#-G-F-F#-G#-B-C
- L contains 66 pairs: (A, Bb), (A, E), (A, D), ..., (B, C)
- Retrograde R(L) = C-B-G#-F#-F-G-C#-Eb-D-E-Bb-A
- R(L) contains 66 pairs: (C, B), (C, G#), ..., (Bb, A)
- L-bar = R(L): e.g., (Bb, A) is in L-bar because A precedes Bb in L (so Bb does NOT precede A), and (Bb, A) is in R(L) because Bb precedes A in the retrograde.

**Application of Theorem 6.6.1(E)**: For any operation OP on PROT:
INJ(L, L)(OP) = INJ(L-bar, L-bar)(OP) = INJ(R(L), R(L))(OP)

# Relationships
## Builds Upon
- **Protocol pairs** — The space within which rows are modeled as sets
- **INJ function** — Measures internal structure of rows
## Enables
- **Generalized hexachord theorem** — Applied to rows via the complement relationship
## Related
- **Retrograde operation** — R(p, q) = (q, p) on PROT
- **INJ complement theorem** — The general theorem (6.6.1) instantiated here

# Common Errors
- **Error**: Thinking the complement relationship holds in pitch-class space
  **Correction**: A row and its retrograde have the same pitch-class content. The complementarity is in PROT (ordering relations), not in pitch classes.

# Common Confusions
- **Confusion**: Conflating "complement in PROT" with "pitch-class complement"
  **Clarification**: In PROT, L and R(L) partition the 132 protocol pairs into two equal halves. This is about ordering relationships, not about which pitch classes are present.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, Example 6.6.2, pages 154-155.

# Verification Notes
- Definition source: Direct from Example 6.6.2
- Confidence rationale: Explicitly constructed with proof and application
- Re-extraction notes: Re-extracted from v2 card; preserved: Moses und Aron row, complement proof, Theorem 6.6.1(E) application
