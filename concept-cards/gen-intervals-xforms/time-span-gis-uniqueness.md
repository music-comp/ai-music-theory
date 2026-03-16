---
# === CORE IDENTIFICATION ===
concept: Time-Span GIS Uniqueness
slug: time-span-gis-uniqueness

# === CLASSIFICATION ===
category: timbral-temporal-systems
subcategory: rhythmic-structures
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS; Some Timbral GIS Models"
chapter_number: 4
pdf_page: 91
section: "4.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Theorem 4.1.5"
  - "essential uniqueness of time-span GIS"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - time-span-gis
  - time-span-interval-independence
extends:
  - time-span-gis
related:
  - isomorphism
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "Is the time-span GIS the only reference-independent time-span GIS?"
  - "Why is non-commutativity forced by reference-independence?"
---

# Quick Definition
The time-span GIS 4.1.3 is essentially unique: any GIS on time spans with both zero-point independence and unit independence must be isomorphic to GIS 4.1.3 via a map preserving the int function.

# Core Definition
Theorem 4.1.5: Let GIS' = (TMSPS, IVLS', int') be any GIS with time spans for its objects that enjoys Properties (A) and (B) of Theorem 4.1.4. Then IVLS and IVLS' are isomorphic via a map f such that int'(s, t) = f(int(s, t)) for all time spans s and t. The isomorphism is constructed using the LABEL' function of GIS': f(i, p) = int'((0, 1), (i, p)) (Lewin, Theorem 4.1.5, pp. 110-112).

# Prerequisites
- **Time-Span GIS** — The GIS whose uniqueness is being established
- **Time-Span Interval Independence** — The two properties that characterize GIS 4.1.3

# Key Properties
1. Any reference-independent time-span GIS is isomorphic to GIS 4.1.3
2. The isomorphism f maps IVLS to IVLS' and satisfies int'(s,t) = f(int(s,t))
3. "Essentially unique" means unique up to isomorphism — different presentations encode the same structure
4. Non-commutativity is not imposed but forced by the independence properties

# Construction / Recognition
## To Verify:
1. Propose any alternative GIS' on TMSPS with Properties (A) and (B)
2. Define f(i, p) = int'((0, 1), (i, p))
3. Show f is an isomorphism from IVLS to IVLS' satisfying int' = f(int)

# Context & Application
This uniqueness result validates the non-commutative structure as not merely an arbitrary choice but a mathematical necessity. If you want reference-independence for time-span intervals, you must (essentially) use this group structure. The non-commutativity is a consequence, not a premise.

# Examples
**Example 1** (p. 112): If someone proposes a different GIS' with Properties (A) and (B), Theorem 4.1.5 guarantees IVLS' is isomorphic to IVLS and int' is just int composed with the isomorphism.

**Example 2** (p. 110): GIS 4.1.2 does NOT have Property (B), so the uniqueness theorem does not apply to it — it is a genuinely different structure.

# Relationships
## Builds Upon
- **Time-Span GIS** — the GIS whose uniqueness is proved
- **Time-Span Interval Independence** — the characterizing properties

## Enables
- Confidence that the non-commutative structure is the "right" one for reference-independent rhythm analysis

# Common Errors
- **Error**: Thinking uniqueness means there is only one way to write the group
  **Correction**: Different presentations (notation, coordinates) are allowed; the algebraic structure is unique up to isomorphism

# Common Confusions
- **Confusion**: Thinking non-commutativity was assumed from the start
  **Clarification**: Lewin derives non-commutativity as a consequence of requiring reference-independence. The independence properties force IVLS to be non-commutative.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Theorem 4.1.5 and Lemmas 4.1.6.1-4.1.6.2, pp. 110-112.

# Verification Notes
- Definition source: direct from Theorem 4.1.5
- Confidence rationale: high — explicit theorem with proof
- Re-extraction notes: Re-extracted from v2 card; preserved: significance discussion, contrast with GIS 4.1.2, proof outline
