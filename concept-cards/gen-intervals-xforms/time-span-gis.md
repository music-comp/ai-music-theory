---
# === CORE IDENTIFICATION ===
concept: Time-Span GIS
slug: time-span-gis

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
  - "GIS 4.1.3"
  - "non-commutative time-span GIS"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - time-span
  - time-span-interval-group
  - generalized-interval-system
  - commutative-vs-noncommutative-gis
extends:
  - generalized-interval-system
related:
  - commutative-time-span-gis
  - time-span-gis-uniqueness
  - time-span-interval-independence
contrasts_with:
  - commutative-time-span-gis

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the time-span GIS?"
  - "How does the time-span GIS relate to simpler time-point and duration GIS structures?"
  - "What must I understand before non-commutative GIS structures?"
---

# Quick Definition
The time-span GIS is a non-commutative GIS where elements are time spans (a, x) and the interval int((a, x), (b, y)) = ((b-a)/x, y/x) measures how many of the first span's durations fit before the second begins, plus the ratio of their durations.

# Core Definition
Theorem 4.1.3.2: Let int map TMSPS x TMSPS into the group IVLS (from Lemma 4.1.3.1) by int((a, x), (b, y)) = ((b-a)/x, y/x). Then (TMSPS, IVLS, int) is a GIS. The interval tells us: (b, y) begins (b-a)/x x-lengths after (a, x), and lasts y/x times as long. This GIS is non-commutative and uniquely enjoys independence from referential time-unit and time-point zero (Lewin, Theorem 4.1.3.2, pp. 106-108).

# Prerequisites
- **Time Span** — The elements (a, x) of the space TMSPS
- **Time-Span Interval Group** — The non-commutative group IVLS with (i, p)(j, q) = (i + pj, pq)
- **Generalized Interval System** — The GIS framework and axioms
- **Commutative vs. Non-Commutative GIS** — Understanding what non-commutativity entails

# Key Properties
1. int((a, x), (b, y)) = ((b-a)/x, y/x)
2. The first component measures temporal offset in x-lengths (spans of the first time span)
3. The second component measures duration ratio
4. The GIS is non-commutative (IVLS is non-abelian)
5. Independent of referential time-unit (Theorem 4.1.4B)
6. Independent of referential zero time-point (Theorem 4.1.4A)
7. Essentially unique GIS with these independence properties (Theorem 4.1.5)

# Construction / Recognition
## To Construct:
1. Take S = TMSPS (all pairs (a, x) with a real, x positive)
2. Take IVLS from Lemma 4.1.3.1
3. Define int((a, x), (b, y)) = ((b-a)/x, y/x)
4. Verify Conditions (A) and (B) of Definition 2.3.1

## To Recognize:
1. Elements are time spans (attack + duration pairs)
2. The interval function divides temporal distance by the first span's duration
3. The interval group is non-commutative

# Context & Application
The time-span GIS uses the first time span itself as a measuring rod, eliminating dependence on arbitrary referential time units and zero time-points. This makes it particularly valuable for analyzing music without a fixed global beat: Carter's metric modulations, Stockhausen's Klavierstuck XI, Nancarrow's tempo canons, and Ligeti's Poeme symphonique.

# Examples
**Example 1** (p. 108): Basic interval:
- s = (0, 1), t = (4, 2): int(s, t) = (4/1, 2/1) = (4, 2)
- Meaning: t begins 4 s-lengths after s, lasts twice as long

**Example 2** (Figure 4.4, p. 74): Two pairs (s_1, t_1) and (s_2, t_2) at different tempi both satisfy int(s, t) = (4, 2). This demonstrates reference-independence: the intervallic relationship is the same regardless of the absolute tempo.

**Example 3** (p. 108): Non-commutativity:
- (1, 2)(0, 3) = (1, 6) but (0, 3)(1, 2) = (3, 6)

# Relationships
## Builds Upon
- **Generalized Interval System** — the time-span GIS is a specific GIS instance
- **Time Span** — the elements of the space
- **Time-Span Interval Group** — the non-commutative interval group

## Enables
- **Time-Span Transposition** — T_{(i,p)}(a, x) = (a + ix, px)
- **Time-Span Inversion** — non-commutative inversion operations on TMSPS
- **Time-Span Interval-Preserving Operation** — P_{(h,u)}(a, x) = (h + ua, ux)

## Related
- **Time-Span GIS Uniqueness** — essentially the only reference-independent time-span GIS

## Contrasts With
- **Commutative Time-Span GIS** — GIS 4.1.2 uses (b-a, y/x), is simpler but unit-dependent

# Common Errors
- **Error**: Using (b-a, y/x) instead of ((b-a)/x, y/x)
  **Correction**: The non-commutative GIS divides temporal distance by x (the first span's duration)

# Common Confusions
- **Confusion**: Thinking non-commutativity is a deficiency
  **Clarification**: Non-commutativity is a necessary consequence of reference-independence (Theorem 4.1.5). It reflects the genuine structure of rhythmic relationships.

- **Confusion**: Thinking the commutative and non-commutative GIS model the same relationships
  **Clarification**: GIS 4.1.2 measures absolute temporal distance; GIS 4.1.3 measures relative temporal distance in span-lengths. They capture different rhythmic intuitions.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Theorem 4.1.3.2, pp. 106-108. Independence properties: Theorem 4.1.4, pp. 108-110.

# Verification Notes
- Definition source: direct from Theorem 4.1.3.2
- Confidence rationale: high — explicit theorem with proof
- Re-extraction notes: Re-extracted from v2 card; preserved: three examples, contrast with GIS 4.1.2, reference-independence discussion
