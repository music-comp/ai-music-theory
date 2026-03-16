---
concept: Segmental Invariance
slug: segmental-invariance
category: analysis
subcategory: twelve-tone-theory
tier: advanced
source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "Row Properties"
chapter_number: 9
pdf_page: null
section: "IX.3"
extraction_confidence: high
aliases:
  - "invariant segments"
prerequisites:
  - twelve-tone-row
  - derived-row
extends: []
related:
  - hexachordal-combinatoriality
contrasts_with: []
answers_questions:
  - "What is segmental invariance?"
  - "How do you find invariant segments between row forms?"
---

# Quick Definition
Segmental invariance occurs when a pitch-class segment of a row remains in place (same absolute pitch classes) when the row is transformed by transposition or inversion, because the relationship between the segments within the row matches the transformation applied to the whole row.

# Core Definition
When a pitch-class segment of a row shares the same transpositional or inversional relationship with another segment in the same row, transforming the whole row by that relationship "holds" those segments invariant -- they swap positions but retain their absolute pitch-class content. To find invariant segments: (1) find equivalent set classes within the row; (2) determine their Tn or In relationship; (3) when the whole row is transformed by that same relationship, those segments are held invariant.

# Prerequisites
- Twelve-tone row and derived row concepts

# Key Properties
1. Segments are preserved as absolute pitch-class collections (not just set class)
2. Occurs when internal segment relationships match the row transformation
3. Algorithm: find equivalent segments, determine their Tn/In, apply that to whole row
4. Intervallic invariance (preserving interval content) is common; segmental (pitch-class) invariance is rarer

# Context & Application
Segmental invariance is compositionally valuable because it creates continuity across row-form changes -- certain harmonic sonorities persist even as the overall pitch material transforms.

# Examples
**Example 1** (Webern, String Quartet Op. 28): P0 and P4 share the same tetrachords (in different positions). The three discrete tetrachords in P0 are related by T4/T8, so transposing the row by T4 swaps their positions while preserving their pitch-class content.

# Relationships
## Builds Upon
- **twelve-tone-row** and **derived-row** -- Invariance relates to row segment properties
## Related
- **hexachordal-combinatoriality** -- Related hexachord-level property

# Common Confusions
- **Confusion**: Segmental invariance means the intervals stay the same
  **Clarification**: Intervallic invariance preserves intervals; segmental invariance preserves actual pitch classes

# Source Reference
Open Music Theory, Part IX, Chapter 3: "Row Properties," section on Segmental Invariance.

# Verification Notes
- Definition source: From 09-03
- Confidence rationale: High
- Preserved from v2: Webern Op. 28 example, algorithm steps
- Cross-reference status: Verified
