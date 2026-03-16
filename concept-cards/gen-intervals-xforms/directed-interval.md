---
# === CORE IDENTIFICATION ===
concept: Directed Interval
slug: directed-interval

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: interval-concepts
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - directed measurement
  - directed motion

# === TYPED RELATIONSHIPS ===
prerequisites:
  - function
  - group
extends: []
related:
  - generalized-interval-system
  - gis-condition-a
  - gis-theorem-2-3-2
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the LABEL function?"
  - "How does the interval function int relate to the group IVLS?"
---

# Quick Definition

A directed interval is an interval that specifies not just the distance between two elements but also the direction -- the interval FROM s TO t, denoted int(s, t), where the order of arguments matters.

# Core Definition

In the GIS framework, "int(s, t)" denotes "a directed measurement, distance, or motion behaving like an 'interval from s to t'" (Lewin, p. 47). The directedness is fundamental: int(s, t) and int(t, s) are generally different, in fact inverses of each other (Theorem 2.3.2). Lewin uses the arrow notation (Figure 0.1) to visualize this directedness: an arrow marked i extends from point s to point t.

# Prerequisites

- **Function** — int is a function from S x S into IVLS
- **Group** — intervals live in a group, which provides the inverse operation

# Key Properties

1. int(s, t) is the interval FROM s TO t (order matters)
2. int(t, s) = int(s, t)^(-1) (Theorem 2.3.2)
3. int(r, s) * int(s, t) = int(r, t) (Condition A: path composition)
4. int(s, s) = e (the identity interval)
5. The direction is encoded in the argument order

# Construction / Recognition

## To Construct:
1. Identify the starting element s and target element t
2. Compute the directed measurement from s to t in the relevant GIS

## To Recognize:
1. Check if the interval specifies both magnitude and direction (from...to...)
2. Verify that reversing the direction inverts the interval

# Context & Application

Directed intervals capture the asymmetry between "going from C to G" and "going from G to C." This is fundamental to voice-leading analysis, transformation networks, and the GIS framework generally. All examples in Chapter 2 use directed intervals: scale steps up, semitones up, clockwise hours, frequency quotients, temporal units later.

# Examples

**Example 1** (p. 47): In chromatic pitch space: int(C4, G4) = 7 semitones (ascending), int(G4, C4) = -7 semitones (descending).

**Example 2** (p. 47): In pitch-class space mod 12: int(E, F) = 1, int(F, E) = 11. The direction determines whether we go 1 step clockwise or 11 steps clockwise.

**Example 3** (p. 47): In diatonic pitch space: int(C4, E4) = 2, int(E4, G4) = 2, and int(C4, G4) = 4. Composition: 2 + 2 = 4.

# Relationships

## Builds Upon
- **Group** — intervals live in a group (IVLS)

## Enables
- **Generalized Interval System** — the GIS formalizes directed intervals
- **GIS Condition A** — path composition of directed intervals

## Related
- **GIS Theorem 2.3.2** — proves int(t, s) = int(s, t)^(-1)

# Common Errors

- **Error**: Treating int(s, t) as undirected (losing the sign or direction).
  **Correction**: int(s, t) and int(t, s) are generally different; direction must be tracked.

# Common Confusions

- **Confusion**: In modular spaces, thinking "down 1" and "up 11" are different intervals.
  **Clarification**: In mod 12 pitch-class space, int(F, E) = 11, not -1. Both describe the same clockwise distance.

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, opening discussion and Theorem 2.3.2, pp. 47-52.

# Verification Notes

- Definition source: synthesized from opening discussion of Chapter 2
- Confidence rationale: explicit throughout Chapter 2, though no single formal definition
- Re-extracted from v2 card; preserved: chromatic and pitch-class examples, Figure 0.1 reference
