---
# === CORE IDENTIFICATION ===
concept: Hexachordal Combinatoriality
slug: hexachordal-combinatoriality
# === CLASSIFICATION ===
category: analysis
subcategory: twelve-tone-theory
tier: advanced
# === PROVENANCE ===
source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "Row Properties"
chapter_number: 9
pdf_page: null
section: "IX.3"
# === CONFIDENCE ===
extraction_confidence: high
# === VARIANTS ===
aliases:
  - "combinatoriality"
# === TYPED RELATIONSHIPS ===
prerequisites:
  - twelve-tone-row
  - aggregate
extends: []
related:
  - magic-hexachord
  - all-trichord-row
contrasts_with: []
# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is hexachordal combinatoriality?"
  - "What is the difference between semi-combinatorial and all-combinatorial?"
  - "Why is combinatoriality compositionally useful?"
---

# Quick Definition
Hexachordal combinatoriality occurs when the first hexachord of one row form and the first hexachord of a related row form (by P-P, P-I, or P-RI transformation) are complementary -- together completing the twelve-tone aggregate. This ensures no pitch-class repetition when two combinatorial row forms sound simultaneously.

# Core Definition
By definition, the first hexachord of P0 complements the second hexachord of P0, and also complements the first hexachord of R0. Combinatoriality becomes significant when it holds between other transformation pairs. Using Babbitt's terminology: semi-combinatorial rows have this property for one transformation type (P-P, P-I, or P-RI); all-combinatorial rows have it for all transformation types. Only six distinct all-combinatorial hexachords exist. Combinatoriality enables composers to stack two row forms vertically without pitch-class duplication within each hexachord span.

# Prerequisites
- Twelve-tone row and aggregate concepts

# Key Properties
1. First hexachords of two row forms together complete the aggregate
2. By definition, P0-R0 first hexachords are always complementary
3. Semi-combinatorial: holds for one additional transformation type
4. All-combinatorial: holds for P-P, P-I, and P-RI simultaneously
5. Only 6 distinct all-combinatorial hexachord types exist
6. Enables pitch-class-complete vertical combinations

# Context & Application
Combinatoriality was central to Babbitt's compositional practice and is a key structural property in many twelve-tone works. Webern's Symphonie Op. 21 uses hexachordal combinatoriality extensively.

# Examples
**Example 1** (Webern, Symphonie Op. 21): The row's hexachords are both chromatic hexachords (set class 6-1), enabling combinatorial pairing.
**Example 2**: If P0's first hexachord is {0,1,2,3,4,5} and I6's first hexachord is {6,7,8,9,10,11}, they are combinatorial -- together they complete the aggregate.

# Relationships
## Builds Upon
- **twelve-tone-row** -- Combinatoriality is a row property
- **aggregate** -- Combinatorial pairs complete the aggregate
## Related
- **magic-hexachord** -- A hexachord with exceptional combinatorial properties

# Common Confusions
- **Confusion**: All twelve-tone rows are combinatorial
  **Clarification**: P-R combinatoriality holds by definition, but P-P, P-I, or P-RI combinatoriality requires special hexachord properties
- **Confusion**: Combinatoriality only applies to hexachords
  **Clarification**: While hexachordal is most common, the concept can extend to other segment sizes

# Source Reference
Open Music Theory, Part IX, Chapter 3: "Row Properties," section on Hexachordal Combinatoriality.

# Verification Notes
- Definition source: From 09-03
- Confidence rationale: High
- Preserved from v2: Babbitt terminology, Webern Op. 21 example, all-combinatorial count
- Cross-reference status: Verified
