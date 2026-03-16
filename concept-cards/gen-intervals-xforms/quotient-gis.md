---
# === CORE IDENTIFICATION ===
concept: Quotient GIS
slug: quotient-gis

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: formal-features
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
section: "3.2 Quotient GIS"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "GIS/CONG"
  - "modularized GIS"
  - "quotient interval system"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-interval-system
  - congruence-relation-gis
  - induced-equivalence
  - quotient-group
extends:
  - generalized-interval-system
related:
  - direct-product-gis
  - natural-map
contrasts_with:
  - direct-product-gis

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a quotient GIS?"
  - "How does pitch-class space relate to chromatic pitch space via quotient GIS?"
  - "How do I construct a quotient GIS from a congruence relation?"
---

# Quick Definition
A quotient GIS is a new GIS derived from an existing GIS by applying a congruence on the interval group, which induces an equivalence on the space, creating a "modularized" system whose elements are equivalence classes and whose intervals are congruence classes.

# Core Definition
Given GIS_1 = (S_1, IVLS_1, int_1) and a congruence CONG on IVLS_1, the quotient GIS GIS_2 = GIS_1/CONG is constructed as follows: S_2 = S_1/EQUIV (the family of equivalence classes under the induced equivalence); IVLS_2 = IVLS_1/CONG (the quotient group of congruence classes); int_2(p, q) is the congruence class containing int_1(s, t) for any s in p, t in q. Theorem 3.2.3 proves this is well-defined and satisfies the GIS axioms (Lewin, Theorem 3.2.3 and Definition 3.2.4, pp. 66-68).

# Prerequisites
- **Generalized Interval System** — The starting GIS that is being quotiented
- **Congruence Relation in GIS** — The congruence on IVLS that drives the construction
- **Induced Equivalence** — The equivalence on S derived from the congruence
- **Quotient Group** — IVLS/CONG must form a group for the quotient GIS to have an interval group

# Key Properties
1. S_2 = S_1/EQUIV: elements are equivalence classes from S_1
2. IVLS_2 = IVLS_1/CONG: intervals are congruence classes from IVLS_1
3. int_2(p, q) = [int_1(s, t)] for any representatives s in p, t in q
4. int_2 is well-defined by Lemma 3.2.2 (representative-independence)
5. The quotient GIS satisfies both Conditions (A) and (B) of the GIS definition
6. Notation: GIS_2 = GIS_1/CONG

# Construction / Recognition
## To Construct:
1. Start with GIS_1 = (S_1, IVLS_1, int_1)
2. Choose a congruence CONG on IVLS_1
3. Form the induced equivalence EQUIV on S_1
4. Set S_2 = S_1/EQUIV, IVLS_2 = IVLS_1/CONG
5. Define int_2(p, q) = congruence class of int_1(s, t) for any s in p, t in q
6. Verify GIS axioms (guaranteed by Theorem 3.2.3)

## To Recognize:
1. Identify a GIS whose elements are equivalence classes from a larger space
2. Verify that the interval group is a quotient of a larger interval group
3. Check that int on the quotient is induced by int on the original

# Context & Application
The quotient GIS formalizes "modularization" — a ubiquitous operation in music theory. Whenever we pass from pitches to pitch classes, from time-points to beat-classes, or from just-intonation pitches to pitch classes on a game board, we are constructing a quotient GIS. This is one of the two principal methods (along with direct product) for constructing new GIS structures from existing ones.

# Examples
**Example 1** (pp. 63-66): Chromatic scale to 12 pitch classes:
- GIS_1: S_1 = infinite chromatic scale, IVLS_1 = Z, int_1 = semitones
- CONG: i ~ i' when they differ by a multiple of 12
- GIS_2: S_2 = 12 pitch classes, IVLS_2 = Z/12Z, int_2(C, F) = 5

**Example 2** (pp. 67-68): Just-intonation to modular harmonic space:
- GIS_1: pitches with ratios 2^a * 3^b * 5^c
- CONG: i ~ i' when i'/i is a power of 2
- GIS_2: pitch classes on the game board of figure 2.2, intervals = pairs (b, c)

**Example 3** (p. 68): Diatonic scale to scale degrees:
- GIS_1: diatonic pitches, IVLS_1 = Z (scale steps)
- CONG: i ~ i' when they differ by a multiple of 7
- GIS_2: 7 scale degrees, IVLS_2 = Z/7Z

**Example 4** (p. 68): Time-points to beat-classes:
- GIS_1: time-points, IVLS_1 = Z
- CONG: i ~ i' when they differ by a multiple of N
- GIS_2: N beat-classes, IVLS_2 = Z/NZ

# Relationships
## Builds Upon
- **Generalized Interval System** — the quotient GIS is itself a GIS, derived from another
- **Congruence Relation in GIS** — the congruence on IVLS drives the construction
- **Induced Equivalence** — determines which elements of S are identified

## Enables
- **Pitch-Class Space** — the 12-tone pitch-class GIS is a quotient of chromatic pitch space
- **Beat-Class Space** — beat-class GIS is a quotient of time-point GIS

## Related
- **Natural Map** — the canonical projection from GIS_1 to GIS_2

## Contrasts With
- **Direct-Product GIS** — direct product combines two spaces; quotient reduces one space by identification

# Common Errors
- **Error**: Choosing representatives carelessly when computing int_2
  **Correction**: While int_2 is independent of representatives (by Lemma 3.2.2), one must still apply the correct congruence class to the result

- **Error**: Attempting to construct a quotient GIS from an arbitrary equivalence on S
  **Correction**: The equivalence must be induced by a congruence on IVLS; otherwise int_2 may not be well-defined

# Common Confusions
- **Confusion**: Thinking the quotient GIS loses essential information
  **Clarification**: The quotient GIS captures a specific level of structural abstraction (e.g., pitch-class relationships rather than specific-pitch relationships). Both the original and quotient GIS are valid models for different analytical purposes.

- **Confusion**: Confusing the congruence on IVLS with the equivalence on S
  **Clarification**: The congruence is on IVLS and the equivalence is on S. They are related (the congruence induces the equivalence) but live in different domains.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.2.3 and Definition 3.2.4, pp. 66-68. Examples on pp. 63-69.

# Verification Notes
- Definition source: direct from Theorem 3.2.3 and Definition 3.2.4
- Confidence rationale: high — explicit definition with proof and multiple examples
- Re-extraction notes: Re-extracted from v2 card; preserved: four concrete examples (chromatic/pitch-class, just-intonation, diatonic, time-point), confusion about well-definedness
