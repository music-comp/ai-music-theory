---
# === CORE IDENTIFICATION ===
concept: Referential Time-Unit Problem
slug: referential-time-unit-problem

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
  - "absolute time-unit problem"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - time-span
extends: []
related:
  - referential-zero-time-point
  - time-span-interval-independence
  - time-span-gis
  - local-time-unit
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "Why is a fixed referential time-unit problematic?"
  - "What motivates the non-commutative time-span GIS?"
---

# Quick Definition
The referential time-unit problem asks: what is the absolute conceptual time-unit by which we measure durations? No such unit has privileged status; this motivates developing a GIS independent of unit choice.

# Core Definition
When we write a time span (a, x), both numbers are measured relative to some implicit time unit. Lewin argues that no unit has absolute conceptual priority — minutes derive from astronomical periods, notational units presuppose structured theories, and "the contextual beat" may not exist uniquely in all music. This problem motivates the non-commutative time-span GIS 4.1.3, which is independent of unit choice (Lewin, pp. 92-95).

# Prerequisites
- **Time Span** — The concept whose measurement depends on the problematic unit

# Key Properties
1. No time-unit has absolute conceptual priority
2. Changing the unit transforms time spans: (a, x) becomes (au, xu)
3. In commutative GIS 4.1.2, changing the unit changes intervals
4. In GIS 4.1.3, changing the unit does NOT change intervals (Property B)
5. The problem is especially acute for music with multiple simultaneous tempi

# Context & Application
The problem is practical for: Carter's metric modulations (mm. 22-32 of String Quartet No. 1 show four instruments at MM36, MM96, MM180, MM120 simultaneously), Stockhausen's Klavierstuck XI (19 groups at any of 6 tempi), Nancarrow's tempo canons (including irrational ratios like pi), and Ligeti's Poeme symphonique (100 metronomes at different tempi).

# Examples
**Example 1** (pp. 93-95): Carter String Quartet No. 1, mm. 22-32: First violin at MM36, second violin at MM96, viola at MM180, cello at MM120 — no single beat governs all instruments.

**Example 2** (p. 98): Stockhausen's Klavierstuck XI: each of 19 groups may be played at any of 6 tempi; the same group may occur at different tempi in one performance.

# Relationships
## Enables
- **Time-Span GIS** — the problem motivates developing the reference-independent GIS
- **Time-Span Interval Independence** — Property (B) resolves the unit problem

## Related
- **Referential Zero Time-Point** — the companion problem about the choice of time origin
- **Local Time Unit** — the concept of locally referential time-units in multi-tempo music

# Common Errors
- **Error**: Choosing a notational beat as "the" unit for all analytical purposes
  **Correction**: In multi-tempo music, no single notational beat governs all parts

# Common Confusions
- **Confusion**: Thinking the problem is merely philosophical
  **Clarification**: The problem has practical consequences: GIS 4.1.2 gives different intervals when the unit changes, making analysis unit-dependent

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Section 4.1 discussion, pp. 92-95.

# Verification Notes
- Definition source: synthesized from extended discussion in Section 4.1
- Confidence rationale: high — extensive philosophical argument with musical examples
- Re-extraction notes: Re-extracted from v2 card; preserved: Carter/Stockhausen/Nancarrow/Ligeti examples
