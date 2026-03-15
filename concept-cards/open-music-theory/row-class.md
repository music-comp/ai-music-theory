---
# === CORE IDENTIFICATION ===
concept: Row Class
slug: row-class
# === CLASSIFICATION ===
category: analysis
subcategory: twelve-tone-theory
tier: advanced
# === PROVENANCE ===
source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "Basics of Twelve-Tone Theory"
chapter_number: 9
pdf_page: null
section: "IX.1"
# === CONFIDENCE ===
extraction_confidence: high
# === VARIANTS ===
aliases: []
# === TYPED RELATIONSHIPS ===
prerequisites:
  - twelve-tone-row
  - row-operations
extends: []
related:
  - row-matrix
contrasts_with: []
# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a row class?"
  - "How many forms are in a row class?"
---

# Quick Definition
A row class is the complete collection of row forms generated from a single twelve-tone row through transposition, inversion, retrograde, and retrograde inversion -- normally 48 forms (P0-P11, I0-I11, R0-R11, RI0-RI11), though rows with special symmetry properties may produce fewer distinct forms.

# Core Definition
A row class contains all forms derivable from a given row through the four standard operations, each at twelve transposition levels: 4 x 12 = 48 forms maximum. Symmetric rows reduce this count. For example, Webern's Op. 21 row is retrograde-equivalent (P and R are related by transposition), yielding 24 distinct forms. His Op. 24 row has further symmetries, yielding only 12. The row class represents the complete pitch material available to the composer.

# Prerequisites
- Twelve-tone row and row operations

# Key Properties
1. Maximum 48 forms: 12 P + 12 R + 12 I + 12 RI
2. Symmetric rows have fewer distinct forms
3. All forms in a class share fundamental intervallic properties
4. Displayed compactly in a row matrix
5. Represents the total pitch-class vocabulary of a twelve-tone work

# Context & Application
The row class defines the complete set of pitch materials for a twelve-tone composition. Composers select which forms to use and in what order, creating large-scale formal plans analogous to key schemes in tonal music.

# Examples
**Example 1**: Standard asymmetric row: 48 distinct forms.
**Example 2** (Webern, Op. 21): Retrograde equivalence yields 24 forms.
**Example 3** (Webern, Op. 24): Multiple symmetries yield only 12 forms.

# Relationships
## Builds Upon
- **twelve-tone-row** -- A row class is generated from a single row
- **row-operations** -- The four operations generate the row class
## Related
- **row-matrix** -- Displays all forms of a row class

# Common Confusions
- **Confusion**: Every row class has exactly 48 members
  **Clarification**: Symmetric rows reduce the count (24, 12, etc.)

# Source Reference
Open Music Theory, Part IX, Chapter 1: "Basics of Twelve-Tone Theory."

# Verification Notes
- Definition source: From 09-01 source chapter
- Confidence rationale: High
- Preserved from v2: Webern symmetry examples
- Cross-reference status: Verified against row properties chapter
