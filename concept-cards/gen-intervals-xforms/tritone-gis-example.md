---
concept: Tritone GIS Example
slug: tritone-gis-example

category: transformation-theory
subcategory: graph-network-mappings
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.5.4"

extraction_confidence: high

aliases: []

prerequisites:
  - generalized-interval-system
  - simply-transitive-group
  - graph-homomorphism
extends: []
related:
  - gis-from-simply-transitive-group
contrasts_with: []

answers_questions:
  - "How can a GIS be constructed on tritone set-classes?"
  - "How does graph homomorphism work when NODEMAP is not 1-to-1?"
---

# Quick Definition
A GIS constructed on the six tritones (unordered pitch-class pairs spanning a tritone), with six "transposition operations" forming a simply transitive group, used to demonstrate graph homomorphism where both NODEMAP and SGMAP are onto but not 1-to-1.

# Core Definition
The tritone GIS has S = the six tritones {(C,F#), (C#,G), (D,Ab), (Eb,A), (E,Bb), (F,B)}. Transposing a tritone by interval i has the same effect as transposing by i+6, so six formal transposition operations are defined: 0-or-6, 1-or-7, 2-or-8, 3-or-9, 4-or-10, 5-or-11. These form a simply transitive group. The map SGMAP(i) = i-or-(i+6) is a group homomorphism from pitch-class intervals onto tritone intervals (Lewin, Section 9.5.4, pp. 236-237).

# Prerequisites
- **Generalized interval system** — the tritone GIS is a specific GIS
- **Simply transitive group** — the six operations act simply transitively
- **Graph homomorphism** — the example demonstrates non-bijective homomorphism

# Key Properties
1. Six tritones form the space S
2. Six transposition operations form a simply transitive group (cyclic group of order 6)
3. SGMAP: Z12 -> Z6 via i -> i-or-(i+6) is a group homomorphism
4. NODEMAP collapses pairs of pitch-class nodes into single tritone nodes (not 1-to-1)
5. Both NODEMAP and SGMAP are onto but not 1-to-1

# Construction / Recognition
## To Construct:
1. Identify the six tritones as objects
2. Define transposition operations that collapse i and i+6
3. Verify simple transitivity
## To Recognize:
1. Look for interval systems on tritone pairs
2. Check that the group has order 6 (not 12)

# Context & Application
This example demonstrates a graph homomorphism where NODEMAP is onto but not 1-to-1 (collapsing two pitch-class nodes into one tritone node) and SGMAP is also onto but not 1-to-1. It contrasts with Example 9.5.3 (where NODEMAP was an isomorphism) and Example 9.5.5 (where SGMAP was an isomorphism).

# Examples
**Example 1** (Figure 9.7, pp. 236-237): Network (a) is a pitch-class network; network (b) is a tritone network. Two top nodes of (a) both map to the top node of (b); two bottom nodes map to the bottom node. SGMAP collapses intervals: int = 5-or-11 in the tritone domain.

# Relationships
## Builds Upon
- **Simply transitive group** — the tritone transposition group
- **Graph homomorphism** — the formal relationship between pc and tritone networks
## Related
- **GIS from simply transitive group** — the construction method

# Common Errors
- **Error**: Confusing tritone (the interval) with tritone (the unordered pair)
  **Correction**: Here "tritone" means an unordered pair of pitch classes spanning that interval

# Common Confusions
- **Confusion**: Thinking the tritone GIS has 12 intervals
  **Clarification**: It has 6 intervals because transposition by i and by i+6 are identified

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.5.4, pp. 236-237. See Figure 9.7.

# Verification Notes
- Definition source: direct from Section 9.5.4
- Confidence rationale: high -- explicitly worked example
- Re-extracted from v2 card; preserved: six tritones enumerated, SGMAP definition, contrast with other examples
