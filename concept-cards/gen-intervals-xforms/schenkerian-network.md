---
concept: Schenkerian Network
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A transformation network that incorporates Schenkerian analytical concepts by having nodes contain triples (Klang, degree, level), with transformations including PROJ operations that move between structural levels.

# Formal Definition
In a Schenkerian network:
- Node contents are triples (Klang, degree, level)
- Klang: a (pitch-class, mode) pair
- degree: scale degree in structural voice (e.g., 5 for fifth degree)
- level: structural level (1 = deepest background, higher = closer to foreground)

Transformations include:
- Klang transformations (DOM, PAR, etc.)
- Degree transformations (SUST = sustain, N+ = upper neighbor, etc.)
- PROJ+ and PROJ- (increment/decrement level)

# Mathematical Formulation
Contents: (Kng, deg, lev) in Klangs x Degrees x Levels

PROJ+ transformation:
- (Kng, deg, lev) -> (Kng, deg, lev + 1)
- Projects content one level toward foreground

PROJ- transformation (inverse of PROJ+):
- (Kng, deg, lev) -> (Kng, deg, lev - 1)
- Projects content one level toward background

Within-level transformations:
- (Klangtrans, degtrans) pairs
- Example: (DOM, SUST) means Klang is dominant of result, degree sustains

# Musical Context/Application
Schenkerian networks model hierarchical tonal structure. PROJ arrows connect different structural levels, while within-level arrows show voice-leading and harmonic relationships at a single level. This allows:
- Multiple levels of structure in one network
- "Input at level n" and "output at level n" distinctions
- Integration of Schenkerian insights with transformational methodology

# Examples
From Figure 9.16 (Beethoven Appassionata slow movement):
- Level 1: (Db, 5, 1) - deep background Db with fifth degree
- Level 2: Db expands to (Db, 5, 2), (Ab, 5, 2), (Db, 5, 2)
- Level 3: Further elaboration with Gb Klangs
- PROJ arrows (two-way) connect corresponding events across levels
- Within-level arrows labeled with (Klangtrans, degtrans) pairs

The Gb nodes are "input at level 3" but not input at lower levels, refining the input/output analysis.

# Related Concepts
- Transformation Network Definition
- PROJ Transformation
- Structural Level
- Klang Representation
- Beethoven Appassionata Analysis

# Common Confusions
- Schenkerian networks are not identical to Schenkerian graphs (different formalism)
- PROJ operations are context-free (always increment/decrement by 1)
- The network represents analytical claims, not Schenker's own notation
- Full Schenkerian analysis involves voice-leading details not captured here

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.7.6, Figure 9.16
