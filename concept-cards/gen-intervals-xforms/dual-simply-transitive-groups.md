---
concept: Dual Simply Transitive Groups
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups"
chapter_number: B
pdf_page: 282
unit: null
authors: David Lewin
---

# Quick Definition
Dual simply transitive groups are pairs of transformation groups (like STRANS1 and STRANS2) where each group consists exactly of the operations that commute with every member of the other group, creating complementary GIS structures on the same set.

# Formal Definition
Given a set S and a simply transitive group STRANS on S, the dual group STRANS' consists of all transformations on S that commute with every member of STRANS. The dual group STRANS' is itself simply transitive on S, and every transformation that commutes with all members of STRANS' is already a member of STRANS. This creates a symmetric, dual relationship between the two groups.

# Mathematical Formulation
Dual group construction:
- Given: STRANS simply transitive on S
- Define: STRANS' = {f : S -> S | fg = gf for all g in STRANS}
- Then: STRANS' is simply transitive on S
- And: STRANS = {f : S -> S | fg = gf for all g in STRANS'}

GIS duality:
- GIS1: STRANS = transpositions, STRANS' = interval-preserving
- GIS2: STRANS' = transpositions, STRANS = interval-preserving

Special case:
- If STRANS is commutative (abelian), then STRANS' = STRANS

# Musical Context/Application
Dual simply transitive groups demonstrate that a single musical space can support multiple, equally valid GIS structures. The choice of which group to treat as "transpositions" determines the resulting interval structure. Neither GIS is more "correct" than the other; they represent different but complementary perspectives on the same musical materials.

# Examples
**From the text:**
"More generally, suppose now that S is any family of objects and that STRANS is any simply transitive group of operations on S. Consider the family STRANS' of transformations f on S such that f commutes with every member of the given group STRANS."

**Dual relationship proven:**
"It can be proved that STRANS' is itself a simply transitive group of operations on S, and that every transformation A which commutes with every member of STRANS' is (already) a member of the given group STRANS."

**Role in GIS structures:**
"When S is considered as a GIS whose formal transpositions are the members of STRANS, then the members of STRANS' will be the interval-preserving operations. Dually, when S is considered as a GIS whose formal transpositions are the members of STRANS', then the members of STRANS will be the interval-preserving operations."

**Commutative special case:**
"If STRANS is commutative, then STRANS' will be precisely STRANS itself."

This explains why the familiar chromatic GIS (with commutative transposition group Z_12) has transpositions and interval-preserving operations from the same group.

**Octatonic example:**
STRANS1 and STRANS2 are duals on the octatonic collection:
- STRANS2 = all operations commuting with everything in STRANS1
- STRANS1 = all operations commuting with everything in STRANS2

# Related Concepts
- Simply Transitive Groups
- STRANS1 Group
- STRANS2 Group
- GIS1 and GIS2
- Interval-Preserving Operations
- Commutative vs. Non-Commutative Groups

# Common Confusions
In commutative (abelian) groups, the dual group equals the original group, so the duality is trivial. The interesting cases arise with non-commutative groups, where the dual group is genuinely different. The octatonic case is interesting precisely because STRANS1 and STRANS2 are different groups (both non-commutative).

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups
