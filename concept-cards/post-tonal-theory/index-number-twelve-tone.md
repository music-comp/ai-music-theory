---
concept: Index Number in Twelve-Tone Context
slug: index-number-twelve-tone
category: twelve-tone
subcategory: invariants
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Twelve-Tone Music"
chapter_number: 6
pdf_page: 320
section: "6.2.9 Twelve-count"
extraction_confidence: high
aliases:
  - "index number (serial)"
  - "inversional index"
prerequisites:
  - inversion-twelve-tone
  - twelve-tone-series
extends:
  - index-number
related:
  - invariants
  - invariant-dyads-between-series-forms
  - axis-of-symmetry-twelve-tone
  - series-form-identification
contrasts_with: []
answers_questions:
  - "What is the index number in twelve-tone music?"
  - "How does the index number relate series forms?"
---

# Quick Definition
The index number is the sum (mod 12) of corresponding pitch classes in two inversionally related series forms, determining which subsets are held invariant and which axis of symmetry governs the inversion.

# Core Definition
For two inversionally related series forms P_i and I_j, the index number n = i + j (mod 12). This sum is consistent for every pair of corresponding pitch classes between the two forms. The index number determines the axis of inversion, the specific dyads formed in note-against-note counterpoint, and which subsets map onto each other or onto themselves (Straus, pp. 320--321, 331--334).

# Prerequisites
- **Inversion (twelve-tone)** -- the index number relates inversionally paired forms
- **Twelve-tone series** -- the series forms being related

# Key Properties
1. n = i + j (mod 12) for P_i and I_j
2. Consistent for all corresponding pitch-class pairs
3. Determines the axis of symmetry
4. Constrains vertical dyads in note-against-note counterpoint
5. Predicts which subsets are held invariant

# Construction / Recognition
- For P_m and RI_x: sum corresponding pc pairs (first of P + last of RI, etc.)
- All sums should be equal (mod 12); that equal value is the index number
- Example: P7 paired with RI7: 7 + 7 = 14 = 2 (mod 12), index = 2

# Context & Application
The index number is essential for: identifying series forms during twelve-counting, predicting invariant relationships, and understanding combinatorial pairings. Maintaining a constant index throughout a passage creates consistent vertical dyads and a fixed axis of symmetry.

# Examples
**Example 1** (pp. 320--321): Webern, "Wie bin ich froh!" -- P7 and RI7 have index 2 (6+8 = 5+9 = 2 mod 12). The axis of inversion is C#-G.

**Example 2** (pp. 331--334): Webern, Piano Variations, op. 27 -- constant index 6 throughout the second movement. All P/I pairings maintain this index.

# Relationships
## Builds Upon
- **Index number** -- extends the general concept to serial context

## Enables
- **Invariant dyads between series forms** -- constrained by the index
- **Axis of symmetry** -- determined by the index
- **Series form identification** -- index helps identify RI-forms

# Common Errors
- Confusing index number with transposition level
- Forgetting to reduce to mod 12 when sums exceed 11

# Common Confusions
- **Index number vs. transposition number**: Index relates inversionally paired forms; transposition number relates prime-related forms

# Source Reference
Chapter 6, Sections 6.2.9 and 6.4.3, pp. 320--322, 331--334

# Verification Notes
Preserved from old card: Webern examples, index calculation methods. Added: v3 template, streamlined presentation, explicit construction steps.
