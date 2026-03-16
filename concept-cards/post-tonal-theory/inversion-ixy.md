---
concept: "Inversion (Ixy Notation)"
slug: inversion-ixy
category: operations
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.5 Inversion (Ixy)"
extraction_confidence: high
aliases:
  - Ixy
  - Lewin inversion notation
prerequisites:
  - inversion
  - index-number-sum
extends:
  - inversion
related:
  - mapping
  - twelve-inversions
contrasts_with: []
answers_questions:
  - "What is Ixy notation for inversion?"
  - "How does Ixy relate to In?"
  - "Why are there multiple valid Ixy labels for the same inversion?"
---

# Quick Definition
Ixy notation identifies an inversion by naming two pitch classes (x and y) that map onto each other, providing a musically intuitive label for inversional relationships.

# Core Definition
In Ixy notation (David Lewin's formulation), an inversion is identified by specifying any pair of pitch classes that exchange under that inversion. The notation Ixy (x as superscript, y as subscript) indicates the inversion mapping x onto y and y onto x. Multiple Ixy labels can describe the same inversion, since every inversion creates six exchange pairs (plus possible self-mappings). The relationship to index-number notation is: x + y = n (mod 12), so Ixy corresponds to In where n = x + y.

# Prerequisites
- **Inversion (In)** -- Ixy is an alternative notation for In
- **Index number (sum)** -- x + y = n connects the two notations

# Key Properties
1. Ixy = Iyx (order of superscript/subscript does not matter)
2. Each of the twelve inversions has multiple valid Ixy labels
3. The same inversion maps six pairs of pitch classes onto each other
4. When n is even, one pitch class maps to itself and one other maps to itself
5. Conversion: x + y = n, so Ixy = I(x+y)
6. There are exactly twelve possible inversions, each with a unique set of mappings

# Construction / Recognition
**To convert Ixy to In**: Add x + y (mod 12) to get n.
- Example: I(G/B) = I(7+11) = I(18 mod 12) = I6

**To find Ixy labels for a given In**: List all pairs (x, y) where x + y = n (mod 12).
- Example for I9: I(A/A), I(G#/Bb), I(G/B), I(F#/C), I(F/C#), I(E/D), I(D#/D#)

**Choosing an Ixy label**: Select the pair most musically relevant in context -- typically notes that are prominent in the passage.

# Context & Application
The Ixy notation is "particularly useful when an analysis wants to emphasize a specific pitch-class mapping that has musical significance" (Straus, Ch. 2). Rather than the abstract index number, Ixy highlights which actual notes exchange, making the relationship more musically concrete and analytically descriptive.

# Examples
**Examples 2-24, 2-25** (pp. 75--76): G and B are related by inversion. The same inversion maps C onto F#, C# onto F, D onto E, Bb onto G#, and maps D# and A each onto themselves. This can be called I(G/B), I(F#/C), I(E/D), I(A/A), I(D#/D#), etc. -- all equivalent to I6.

**Example 2-26** (p. 76, Schoenberg, *Three Piano Pieces*, op. 11, no. 1): [G, G#, B] and [G, Bb, B] related by I(G/B), emphasizing that G and B exchange. I(G#/Bb) also valid.

**Example 2-27** (p. 76, Berio, *Sequenza for Solo Flute*): Three sets. Sets 1 and 2 related by I(D/G) (= I3), emphasizing the registral midpoint. Sets 2 and 3 related by I(D/C) (= I2).

**Example 2-28** (p. 77): All twelve inversions displayed on clockfaces, with their exchange pairs and corresponding index numbers.

# Relationships
## Builds Upon
- **Inversion (In)** -- Ixy is an alternative notation
- **Index number (sum)** -- x + y = n provides the conversion
## Enables
- **Contextual analysis** -- Ixy labels emphasize musically relevant pitch-class exchanges
## Related
- **Mapping** -- Ixy notation highlights specific pitch-class mappings
- **Twelve inversions** -- Example 2-28 catalogs all twelve

# Common Errors
- **Error**: Thinking different Ixy labels mean different inversions. **Correction**: Many Ixy labels describe the same inversion; choose whichever is most musically relevant.

# Common Confusions
- **Confusion**: Which note goes on top vs. bottom. **Clarification**: It does not matter: Ixy = Iyx. The choice of superscript vs. subscript is purely conventional.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.5, pages 75--78.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: explicitly defined with comprehensive example catalog
- Re-extraction notes: preserved old card's Berio and Schoenberg examples; added reference to Lewin's origin and Example 2-28; upgraded to v3 template
