---
concept: Mapping
slug: mapping
category: operations
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.3.4 Levels of transposition"
extraction_confidence: high
aliases:
  - map
  - maps onto
prerequisites:
  - transposition
  - inversion
extends:
  - transposition
related:
  - nodes-and-arrows
  - isography
contrasts_with: []
answers_questions:
  - "What is a mapping in pitch-class set theory?"
  - "How does transposition or inversion establish correspondences between elements?"
---

# Quick Definition
A mapping describes how a transformation (Tn or In) sends each element of one set onto a corresponding element of another set.

# Core Definition
In set theory, a mapping describes the one-to-one correspondence established by an operation between elements of one set and elements of another. When a set is transposed or inverted, each pitch class maps onto a specific pitch class in the resulting set. The language used is: "Tn maps x onto y" or "x maps to y under Tn."

# Prerequisites
- **Transposition (Tn)** -- one of the operations that creates mappings
- **Inversion (In)** -- the other operation that creates mappings

# Key Properties
1. Transposition mapping: Tn maps x onto (x + n) mod 12
2. Inversion mapping: In maps x onto (n - x) mod 12
3. Every element has exactly one image (bijective / one-to-one)
4. Under transposition, first-to-first, second-to-second correspondence (in normal form)
5. Under inversion, first-to-last, second-to-second-to-last correspondence (in normal form)

# Construction / Recognition
To trace a mapping:
1. For Tn: add n to each element of the original set
2. For In: subtract each element from n
3. Arrows drawn between corresponding elements visualize the mapping

# Context & Application
Understanding mappings helps analysts trace how musical ideas transform. The choice of transposition or inversion level is often compositionally significant because a mapping may replicate an interval found within the set being transformed. When the same operation maps both note-to-note and set-to-set, this creates multi-level structural coherence.

# Examples
**Example 2-8** (p. 65, Webern, *Concerto for Nine Instruments*, op. 24): T8 maps the first melodic fragment onto the second:
- G maps onto Eb (7 + 8 = 3 mod 12)
- D# maps onto B (3 + 8 = 11)
- E maps onto C (4 + 8 = 0)

The same T8 that connects the first two notes within the fragment (G to D#) also connects the entire fragment to its transposition.

**Example 2-22** (p. 73): Under I6, G maps onto B (7 + 11 = 6 mod 12... actually G + B = 7 + 11 = 18 = 6 mod 12). Sets [G, G#, B] and [G, Bb, B] are related by I6 with correspondences: G<->B, G#<->Bb.

# Relationships
## Builds Upon
- **Transposition (Tn)** -- creates additive mappings
- **Inversion (In)** -- creates subtractive mappings
## Enables
- **Nodes and arrows** -- visual representation of mappings
- **Isography** -- comparison of mapping structures
## Related
- **Levels of transposition** -- mappings reveal multi-level structural connections

# Common Errors
- **Error**: Assuming mappings imply physical motion. **Correction**: "Maps onto" describes a correspondence, not literal movement of notes.

# Common Confusions
- **Confusion**: Correspondence order under inversion. **Clarification**: In normal form, inversion maps first to last, second to second-to-last, etc. -- not first to first as in transposition.

# Source Reference
Chapter 2: Pitch-Class Sets, Sections 2.3.4--2.3.6, pages 65--67.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: explicitly described with arrow diagrams
- Re-extraction notes: preserved old card's Webern example; corrected I6 example arithmetic; upgraded to v3 template
