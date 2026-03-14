---
concept: REL Transformation
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
unit: null
authors: David Lewin
---

# Quick Definition
A Klang transformation that takes any Klang into its relative minor/major: (C, +)REL = (A, -) and (C, -)REL = (Eb, +).

# Formal Definition
REL is defined as the operation taking any Klang to its relative:
- (p, +)REL = (p - 3, -): Major to relative minor (down 3 semitones)
- (p, -)REL = (p + 3, +): Minor to relative major (up 3 semitones)

REL is its own inverse: (REL)(REL) = identity

# Mathematical Formulation
REL transformation:
- (p, +)REL = (p + 9, -) mod 12
- (p, -)REL = (p + 3, +) mod 12
- REL^2 = identity (REL is an involution)

REL differs from both MED and SUBM:
- (C, +)REL = (A, -) = (C, +)MED
- (C, -)REL = (Eb, +) but (C, -)MED = (Ab, +)
- (C, +)SUBM = (E, -) [not the same as REL]

# Musical Context/Application
REL captures the relative major/minor relationship that preserves key signature. Like PAR, REL is not expressible as a power of MED or SUBM, contributing to the genuinely non-intervallic character of general Klang transformation networks.

# Examples
Applications:
- (C, +)REL = (A, -): C major to A minor
- (A, -)REL = (C, +): A minor to C major
- (G, +)REL = (E, -): G major to E minor
- (F, -)REL = (Ab, +): F minor to Ab major

Contrast with MED:
- (C, +)MED = (A, -) [same as REL in this case]
- (C, -)MED = (Ab, +) but (C, -)REL = (Eb, +) [different]

# Related Concepts
- Klang Representation
- PAR Transformation
- MED Transformation
- SUBM Transformation
- Non-Intervallic Transformations

# Common Confusions
- REL changes both root and mode
- REL is NOT the same as MED (they differ on minor Klangs)
- REL preserves key signature; MED does not
- REL is an involution (self-inverse)

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.1.1
