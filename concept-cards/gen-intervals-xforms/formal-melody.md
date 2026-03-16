---
# === CORE IDENTIFICATION ===
concept: Formal Melody
slug: formal-melody

# === CLASSIFICATION ===
category: transformation-theory
subcategory: graph-network-structure
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.7.7"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "series as transformation network"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - precedence-ordered-system
  - transformation-network-definition
extends:
  - transformation-network-definition
related:
  - scholica-enchiriadis-analysis
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can a melody be formally modeled as a transformation network?"
  - "What makes a transformation network a formal 'melody'?"
---

# Quick Definition
A formal "melody" is a transformation network whose node/arrow system is precedence-ordered and linearly ordered under that ordering, giving a unique sequential arrangement of nodes that models a series of objects with transformational transitions.

# Core Definition
A formal "melody" is defined as a transformation network whose node/arrow system is precedence-ordered and linearly ordered under that ordering. This means there is one and only one way of labeling the J nodes with numbers 1 through J compatible with the one-way arrows. The concept carries within it the idea of transforming earlier events to later ones, along the arrows, by transformations from a specified semigroup (Lewin, Section 9.7.7, p. 250).

# Prerequisites
- **Precedence-ordered system** — the node/arrow system must be precedence-ordered
- **Transformation network** — a formal melody is a special type of network

# Key Properties
1. PRECEDENCE is a linear ordering: every pair of distinct nodes is comparable
2. Nodes can be uniquely ordered: N1 < N2 < ... < NJ under precedence
3. Different arrow configurations can yield the same linear precedence ordering but different "melodies"
4. The concept encodes not just what objects occur but how each transforms into the next
5. Connects to Ernst Kurth's idea of melody as "the impetus of transition between the tones"

# Construction / Recognition
## To Construct:
1. Create a transformation network with linearly precedence-ordered nodes
2. Ensure every pair of distinct nodes is comparable under precedence
## To Recognize:
1. Verify the node/arrow system is precedence-ordered
2. Verify PRECEDENCE gives a linear (total) ordering on NODES

# Context & Application
This concept enriches the idea of "series" with transformational content. Earlier models (protocol pairs, etc.) treated a series as a sequence of objects. The formal melody adds the idea that specific transformations link successive elements. Different arrow patterns on the same precedence ordering give formally different "melodies" -- capturing different "transition structures" in Kurth's sense.

# Examples
**Example 1** (Figure 9.17, p. 250): Two node/arrow systems (a) and (b) give the same linear precedence ordering. Network (a) has arrows connecting only consecutive nodes. Network (b) has additional arrows connecting non-consecutive nodes. Both are formally different "melodies" despite having the same precedence ordering and the same node contents.

# Relationships
## Builds Upon
- **Precedence-ordered system** — melody requires linear precedence ordering
- **Transformation network** — melody is a specialized network
## Enables
- **Scholica Enchiriadis analysis** — "Nos qui vivimus" modeled as a formal melody
## Related
- **Series modeling** — formal melody is one way to model series (cf. protocol pairs in Section 6.2.4)

# Common Errors
- **Error**: Assuming the same precedence ordering gives the same melody
  **Correction**: Different arrow configurations (e.g., with or without skip-arrows) yield formally different melodies

# Common Confusions
- **Confusion**: Thinking "melody" here means ordinary musical melody
  **Clarification**: "Melody" is a formal term for a linearly precedence-ordered transformation network; Lewin suggests "a better word" might be needed
- **Confusion**: Thinking a formal melody is just a sequence of pitches
  **Clarification**: It includes the transformational transitions between elements, not just the elements themselves

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.7.7, pp. 249-251. See Figure 9.17. Cf. Ernst Kurth, Grundlagen des linearen Kontrapunkts, p. 2.

# Verification Notes
- Definition source: direct from Section 9.7.7
- Confidence rationale: explicit definition with examples and discussion
- Re-extracted from v2 card; preserved: Figure 9.17 example, Kurth reference
