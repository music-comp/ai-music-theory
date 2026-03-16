---
concept: RICH Transformation
slug: rich-transformation

category: transformation-theory
subcategory: serial-operations
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
section: "8.2.1"

extraction_confidence: high

aliases:
  - "RI-chaining operation"

prerequisites:
  - klang-representation
extends: []
related:
  - tch-transformation
  - ri-chaining
  - structural-sequencing
  - much-transformation
contrasts_with:
  - tch-transformation

answers_questions:
  - "How do I apply RI-chaining (RICH) to a series?"
  - "What distinguishes RICH from TCH?"
  - "What distinguishes RICH from generic RI operations?"
---

# Quick Definition
A serial transformation that produces the retrograde-inverted form of a series whose first two elements are the last two elements of the original series (in order), enabling RI-chaining. RICH(RICH(s)) always produces a transposed form of s.

# Core Definition
"Given a series s of pitches or pitch-classes s_1, s_2, ..., s_N, we can apply to s the RI-chaining operation RICH. RICH(s) is that retrograde-inverted form of s whose first two elements are s_{N-1} and s_N, in that order" (Lewin, 8.2.1, p. 180). Key property: "RICH(RICH(s)) is always some transposed form of s" with transposition interval i = int(s_1, s_N) + int(s_2, s_{N-1}) (p. 181).

# Prerequisites
- **Klang representation** — Context for understanding serial operations on abstract spaces (though RICH operates on pitch/pc series primarily)

# Key Properties
1. RICH(s) begins with s_{N-1}, s_N (the last two elements of s)
2. RICH(s) is a retrograde-inverted form of s
3. RICH(RICH(s)) = T_i(s) where i = int(s_1, s_N) + int(s_2, s_{N-1})
4. This i is the "TCH-interval for s"
5. The TCH interval for an RI form of s equals the TCH interval for s
6. The TCH interval for a retrograde or inverted form of s is the negative of i

# Construction / Recognition
## To Construct:
1. Take series s = s_1, s_2, ..., s_N
2. Find the RI form that begins with s_{N-1}, s_N
3. This RI form is RICH(s)
## To Recognize:
1. Two successive series share their last two / first two elements
2. The second is an RI of the first

# Context & Application
RICH is a non-intervallic serial transformation: it is not a transposition, but its double application (TCH) always is. Different series yield different TCH intervals, so writing "TCH" rather than "T_i" preserves isographic relationships across different musical contexts. RICH appears in Wagner, Webern, Bach, and other composers.

# Examples
**Example 1** (p. 180): Zauber series s = A-C-Eb-E: RICH(s) = Eb-E-G-Bb; RICH(RICH(s)) = G-Bb-Db-D = T_{10}(s). TCH interval = int(A,E) + int(C,Eb) = 7 + 3 = 10.

**Example 2** (p. 180): Webern's op. 27 row s = Eb-B-Bb-D-C#-C-F#-E-G-F-A-G#: TCH interval = int(Eb,G#) + int(B,A) = 5 + 10 = 3.

# Relationships
## Builds Upon
- (operates on series in any GIS)
## Enables
- **TCH transformation** — TCH = RICH composed with RICH
- **RI-chaining** — RICH is the generating operation
- **Structural sequencing** — Repeated RICH creates sequences
## Related
- **MUCH transformation** — Another serial chaining operation
## Contrasts With
- **TCH transformation** — RICH produces an RI form; TCH produces a transposition. RICH is non-intervallic; TCH is intervallic.

# Common Errors
- **Error**: Confusing RICH with generic RI operations
  **Correction**: RICH specifies WHICH RI form (the one overlapping by two elements); generic RI does not specify

# Common Confusions
- **Confusion**: Thinking RICH applied twice returns to the original series
  **Clarification**: RICH(RICH(s)) = T_i(s), a transposition, not the identity
- **Confusion**: Believing the TCH interval is fixed regardless of series
  **Clarification**: Different series have different TCH intervals; that is why "TCH" is preferred over "T_i"

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.2.1, pages 180-181.

# Verification Notes
- Definition source: Direct quotation from 8.2.1
- Confidence rationale: Explicit definition with formula and examples
- Re-extraction notes: Re-extracted from v2 card; preserved: both examples, TCH interval formula, RI form distinction
