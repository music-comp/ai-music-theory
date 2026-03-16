---
concept: Interval Function int
slug: interval-function-int

category: generalized-interval-systems
subcategory: interval-mechanics
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "Definition 2.3.1, Theorem 2.3.2"

extraction_confidence: high

aliases:
  - "int function"
  - "interval function"

prerequisites:
  - musical-space-s
  - interval-group-ivls
  - function
extends: []
related:
  - generalized-interval-system
  - interval-composition
  - simply-transitive-action
  - transposition
contrasts_with: []

answers_questions:
  - "What is the formal definition of the interval function int in a GIS?"
  - "What conditions must the function int satisfy?"
  - "Why is int(s, t) directed rather than symmetric?"
---

# Quick Definition
The function int(s, t) assigns to each ordered pair of elements in a musical space S the unique directed interval from s to t in the interval group IVLS, subject to two conditions that ensure intervals compose correctly and that the space is complete.

# Core Definition
"int is a function mapping S x S into IVLS, all subject to the two conditions (A) and (B) following. (A): For all r, s, and t in S, int(r, s)int(s, t) = int(r, t). (B): For every s in S and every i in IVLS, there is a unique t in S which lies the interval i from s, that is a unique t which satisfies the equation int(s, t) = i" (Lewin, Definition 2.3.1, p. 47).

From these conditions, Theorem 2.3.2 derives: "int(s, s) = e and int(t, s) = int(s, t)^(-1) for every s and t in S."

# Prerequisites
- **Musical space S** — The domain from which ordered pairs are drawn
- **Interval group IVLS** — The codomain, a mathematical group
- **Function** — int is formally a function from S x S into IVLS

# Key Properties
1. int: S x S -> IVLS maps ordered pairs of space elements to intervals
2. Condition (A): int(r, s) * int(s, t) = int(r, t) — intervals compose along paths
3. Condition (B): for every s and i, there is a unique t with int(s, t) = i — space is complete
4. int(s, s) = e (identity) for all s — derived from Conditions (A) and (B)
5. int(t, s) = int(s, t)^(-1) for all s, t — reversing direction inverts the interval

# Construction / Recognition
## To Construct:
1. Identify elements s, t in the musical space S
2. Compute the directed measurement from s to t according to the conventions of the GIS
3. The result is a member of IVLS
## To Recognize:
1. A function taking ordered pairs from a musical space and returning group elements
2. Satisfying both Condition (A) and Condition (B)

# Context & Application
The interval function formalizes our intuition of "directed distance" between musical elements. Lewin introduces it provisionally as "our intuition of a directed measurement or motion behaving like an 'interval from s to t.'" The direction matters: int(s, t) and int(t, s) are group inverses, not equal, capturing the asymmetry between ascending and descending.

# Examples
**Example 1** (p. 47): Chromatic pitch space — int(C4, G4) = 7 (semitones up), int(G4, C4) = -7. Verification: 7 + (-7) = 0 = e.

**Example 2** (p. 47): Pitch-class space — int(E, F) = 1, int(F, E) = 11. Verification: 1 + 11 = 12 = 0 mod 12.

**Example 3** (p. 47, Theorem 2.3.2 proof): "int(s, s)int(s, s) = int(s, s), via Condition (A). Multiply both sides of that equation by int(s, s)^(-1); we obtain int(s, s) = e as asserted."

# Relationships
## Builds Upon
- **Musical space S** — The set from which elements are drawn
- **Interval group IVLS** — The group into which int maps
## Enables
- **Generalized Interval System** — int is the third component of the GIS triple (S, IVLS, int)
- **Transposition** — T_i(s) is defined as the unique t with int(s, t) = i
## Related
- **Simply transitive action** — Condition (B) establishes simple transitivity of IVLS on S

# Common Errors
- **Error**: Treating int(s, t) as undirected (symmetric)
  **Correction**: int is directed: int(s, t) and int(t, s) are group inverses, not equal

# Common Confusions
- **Confusion**: Thinking Conditions (A) and (B) alone are insufficient to derive int(s, s) = e
  **Clarification**: Theorem 2.3.2 proves int(s, s) = e and int(t, s) = int(s, t)^(-1) follow from (A) and (B); these do not need to be separately stipulated

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1, Theorem 2.3.2, pages 47-48.

# Verification Notes
- Definition source: Direct quotation from Definition 2.3.1 and Theorem 2.3.2
- Confidence rationale: Core formal definition, extensively discussed
- Re-extraction notes: Re-extracted from v2 card; preserved: multiple space examples, theorem proof, directionality emphasis
