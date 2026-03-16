---
# === CORE IDENTIFICATION ===
concept: "W-Group (Weitzmann Group)"
slug: w-group

# === CLASSIFICATION ===
category: transformations
subcategory: transformation-groups
tier: intermediate

# === PROVENANCE ===
source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Weitzmann Regions"
chapter_number: 4
pdf_page: 79
section: "Weitzmann Transformations and N/R Cycles"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Weitzmann group"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - weitzmann-region
  - r-transformation
  - n-transformation
  - s-transformation
extends: []
related:
  - h-group
  - weitzmann-transformations
contrasts_with:
  - h-group

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the W-group?"
  - "How do W-group and H-group complement each other?"
---

# Quick Definition
The group of three transformations (R, N, S) that preserve Weitzmann region membership while shifting hexatonic region, complementary to the H-group (L, P, H).

# Core Definition
The W-group consists of R (Relative), N (Nebenverwandt), and S (Slide). All three require 2 units of voice-leading work, preserve Weitzmann region membership, shift hexatonic region membership, and change mode. The W-group complements the H-group: "alternating W and H moves between adjacent regions" (cf. Chapter 5). Any chromatic sequence results from alternating one H-group and one W-group transformation. The W-group contains two classically normative transformations (R, N) and one maverick (S), paralleling the H-group's structure (L, P normative; H maverick).

# Prerequisites
- **Weitzmann region**: The space preserved by W-group operations
- **R, N, S transformations**: The three member operations

# Key Properties
1. Closed under combination: any sequence of W-group operations stays in the same Weitzmann region
2. Complementary to H-group: alternating W and H generates inter-regional motion
3. All members require 2 voice-leading units
4. All members are involutions (self-inverse)
5. R and N are classically normative; S is maverick (parallel to L/P vs. H)

# Construction / Recognition
W-group combined with H-group generates chromatic sequences:
| H\W | R | N | S |
|-----|-----|-----|-----|
| L | T5 | T1 | T3 |
| P | T3 | T5 | T1 |
| H | T1 | T3 | T5 |

# Context & Application
The W-group/H-group distinction is fundamental to understanding chromatic sequences in 19th-century music. Any chromatic sequence results from alternating one H-group and one W-group transformation, producing one of three odd transpositions (T1, T3, T5).

# Examples
**N/R cycles** (Schubert symphonies): Use W-group members R and N.
**S progressions** (Schubert, Liszt, Wagner): Use W-group member S.
**Chromatic sequences**: Combine W-group and H-group members.

# Relationships
## Builds Upon
- weitzmann-region: The space preserved by the group

## Enables
- weitzmann-transformations: The W-group defines which transformations are "Weitzmann"

## Related
- h-group: The complementary group

## Contrasts With
- h-group: H-group preserves hexatonic region; W-group preserves Weitzmann region

# Common Errors
- **Error**: Treating W-group and H-group as opposites
  **Correction**: They are complements, not opposites; together they generate all chromatic sequences

# Common Confusions
- **Confusion**: R is in the W-group despite being a common tonal operation
  **Clarification**: R's membership reflects geometric classification (region preservation), not functional classification

# Source Reference
Cohn, Richard. *Audacious Euphony*, Chapter 4: Weitzmann Regions, pp. 79-83; Chapter 5, pp. 107-109.

# Verification Notes
Re-extracted from v2 card; preserved: combination table, normative/maverick parallel, complementary relationship. High confidence: systematically defined.
