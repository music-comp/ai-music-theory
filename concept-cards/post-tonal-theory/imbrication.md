---
concept: Imbrication
slug: imbrication
category: analysis
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.9 Segmentation and Analysis"
extraction_confidence: high
aliases:
  - overlapping segmentation
prerequisites:
  - pitch-class-set
  - segmentation
extends:
  - segmentation
related:
  - set-class
contrasts_with: []
answers_questions:
  - "What is imbrication?"
  - "How do I systematically segment a melodic line?"
---

# Quick Definition
Imbrication is a systematic segmentation method that considers all overlapping note groups of a given size within a melodic line.

# Core Definition
Imbrication is a technique for generating melodic segments by extracting all consecutive groups of n notes from a melody. For a six-note melody, the imbricated trichords would be notes 1-2-3, 2-3-4, 3-4-5, and 4-5-6. Groups may span across rests or phrasing boundaries. "A rich interaction between phrase structure and set-class structure is a familiar feature of post-tonal music" (Straus, Ch. 2).

# Prerequisites
- **Pitch-class set** -- each imbricated group is a pitch-class set
- **Segmentation** -- imbrication is one segmentation strategy

# Key Properties
1. Systematic: considers every consecutive group of a given size
2. Overlapping: adjacent groups share (n-1) elements
3. For a melody of m notes extracting groups of size n: produces (m - n + 1) groups
4. May cross rests and phrase boundaries
5. Useful for exhaustive discovery of set-class patterns

# Construction / Recognition
For a melody of m notes, imbricated groups of size n:
```
Notes:    1  2  3  4  5  6
Group 1: [1  2  3]
Group 2:    [2  3  4]
Group 3:       [3  4  5]
Group 4:          [4  5  6]
```
Number of groups = m - n + 1 = 6 - 3 + 1 = 4

# Context & Application
Imbrication reveals hidden relationships within melodic lines. A melody may contain multiple members of the same set class that overlap and interlock. The technique ensures no contiguous grouping is overlooked. It is particularly revealing in Webern's melodies, which often use a single trichord type throughout.

# Examples
In the analytical discussions of Chapter 2, imbrication is applied to melodic lines to discover:
- Which set classes recur within a melody
- How set-class structure interacts with phrase structure
- Hidden continuities across phrase boundaries

For example, in Webern's *Concerto*, op. 24 (Model Analysis), the melody can be parsed into overlapping trichords, all members of sc(014).

# Relationships
## Builds Upon
- **Segmentation** -- imbrication is one segmentation strategy
## Enables
- **Set-class saturation analysis** -- revealing how thoroughly a set class permeates a melody
## Related
- **Set class** -- imbricated groups are compared by set-class membership

# Common Errors
- **Error**: Skipping groups that span rests. **Correction**: Imbrication considers all contiguous groupings regardless of rests or phrase boundaries.

# Common Confusions
- **Confusion**: Imbrication is exhaustive of all segmentations. **Clarification**: Imbrication covers only contiguous melodic groups. Other strategies (registral, rhythmic, timbral) reveal additional groupings.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.9, page 86.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: explicitly named and defined in segmentation strategies
- Re-extraction notes: preserved old card's formula and ASCII diagram; upgraded to v3 template
