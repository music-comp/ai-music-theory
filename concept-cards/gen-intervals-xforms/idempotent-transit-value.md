---
concept: Idempotent TRANSIT Value
slug: idempotent-transit-value

category: transformation-theory
subcategory: graph-network-definitions
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.2.2"

extraction_confidence: high

aliases: []

prerequisites:
  - transformation-graph-definition
  - transit-function
  - semigroup
extends: []
related:
  - identity-element
  - operation-graph
contrasts_with: []

answers_questions:
  - "Why must TRANSIT(N, N) be idempotent?"
  - "What happens to reflexive TRANSIT values when SGP is a group?"
---

# Quick Definition
In any transformation graph, TRANSIT(N, N) must be an idempotent element of SGP (satisfying ee = e), a consequence of the consistency criterion 9.2.1(D). When SGP is a group, this forces TRANSIT(N, N) to be the identity.

# Core Definition
In a transformation graph (NODES, ARROW, SGP, TRANSIT), TRANSIT(N, N) must be idempotent for every node N. This is proved by considering two arrow chains from N to N of different lengths: the chain of length 1 (just N to N) and the chain of length 2 (N to N to N). By criterion 9.2.1(D), TRANSIT(N, N) = TRANSIT(N, N) * TRANSIT(N, N), so the element is idempotent. The only idempotent in a group is the identity element, so in operation graphs, TRANSIT(N, N) = identity for all N (Lewin, Section 9.2.2, p. 227).

# Prerequisites
- **Transformation graph** — the structure in which TRANSIT(N, N) is defined
- **TRANSIT function** — the function whose reflexive values are constrained
- **Semigroup** — idempotency is a semigroup-theoretic property

# Key Properties
1. Follows from criterion 9.2.1(D) by comparing arrow chains of lengths 1 and 2 from N to N
2. In a group, the only idempotent is the identity: if zz = z then z = e
3. In a semigroup, multiple idempotents may exist; different nodes could have different idempotent TRANSIT(N, N) values
4. Any arrow chain returning to its starting node must have total TRANSIT product equal to TRANSIT(N, N)

# Construction / Recognition
## To Construct:
1. This is a derived property, not directly constructed
2. When building a graph with group SGP, set TRANSIT(N, N) = identity
## To Recognize:
1. Check that TRANSIT(N, N) satisfies ee = e in SGP for all nodes N

# Context & Application
In most musical applications, SGP is a group (transpositions, inversions, Klang transformations), so TRANSIT(N, N) = identity is the norm. This means "staying at the same node" corresponds to applying the identity transformation. The semigroup case (with non-identity idempotents) is theoretically possible but rare in practice.

# Examples
**Example 1** (Section 9.2.2, p. 227): Proof: consider chains N0 = N1 = N (length 1, giving x1 = TRANSIT(N,N)) and M0 = M1 = M2 = N (length 2, giving y1 = y2 = TRANSIT(N,N)). By criterion (D), x1 = y2 * y1, so TRANSIT(N,N) = TRANSIT(N,N) * TRANSIT(N,N).

**Example 2**: In a Klang transformation operation graph, SGP is a group, so TRANSIT(N, N) = IDENT for all nodes.

# Relationships
## Builds Upon
- **Transformation graph** — this is a derived property of transformation graphs
- **Semigroup** — idempotency is defined within semigroup theory
## Enables
- **Operation graph** — in operation graphs, the identity requirement follows immediately
## Related
- **Identity element** — the identity is the unique idempotent in a group

# Common Errors
- **Error**: Freely choosing TRANSIT(N, N) without checking idempotency
  **Correction**: TRANSIT(N, N) is constrained by criterion (D); it must be idempotent

# Common Confusions
- **Confusion**: Assuming TRANSIT(N, N) is always the identity
  **Clarification**: It is the identity only when SGP is a group; in a general semigroup it may be any idempotent

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.2.2 (Optional), p. 227.

# Verification Notes
- Definition source: direct from Section 9.2.2 proof
- Confidence rationale: explicit proof provided in source
- Re-extracted from v2 card; preserved: proof structure, group vs semigroup distinction
