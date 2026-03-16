---
concept: Direct-Product GIS
slug: direct-product-gis

category: generalized-interval-systems
subcategory: formal-features
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
section: "3.3 Direct-Product GIS"

extraction_confidence: high

aliases:
  - "GIS1 x GIS2"
  - "product GIS"
  - "compound GIS"

prerequisites:
  - generalized-interval-system
  - direct-product
  - interval-group-ivls
extends:
  - generalized-interval-system
related:
  - time-span-gis
  - commutative-time-span-gis
contrasts_with:
  - quotient-gis

answers_questions:
  - "How do I construct a direct-product GIS?"
  - "What is a direct-product GIS?"
---

# Quick Definition
A direct-product GIS combines two GIS structures into a compound GIS whose space consists of ordered pairs from the original spaces and whose intervals are pairs of intervals from the original interval groups, enabling unified treatment of multiple musical dimensions.

# Core Definition
Given GIS_1 = (S_1, IVLS_1, int_1) and GIS_2 = (S_2, IVLS_2, int_2), the direct product GIS_3 = GIS_1 x GIS_2 is: S_3 = S_1 x S_2 (Cartesian product); IVLS_3 = IVLS_1 x IVLS_2 (direct-product group with componentwise operation (i_1, i_2)(j_1, j_2) = (i_1 j_1, i_2 j_2)); int_3((s_1, s_2), (t_1, t_2)) = (int_1(s_1, t_1), int_2(s_2, t_2)). This satisfies Conditions (A) and (B) of Definition 2.3.1 (Lewin, Definition 3.3.3, p. 77).

# Prerequisites
- **Generalized Interval System** — Both factor GIS structures must be understood
- **Direct Product** — The Cartesian product of sets and direct-product group construction
- **Interval Group (IVLS)** — The product group IVLS_1 x IVLS_2 must be understood as a group

# Key Properties
1. S_3 = S_1 x S_2: elements are ordered pairs (s_1, s_2)
2. IVLS_3 = IVLS_1 x IVLS_2: intervals are ordered pairs (i_1, i_2)
3. Group operation is componentwise: (i_1, i_2)(j_1, j_2) = (i_1 j_1, i_2 j_2)
4. int_3 acts componentwise: int_3((s_1, s_2), (t_1, t_2)) = (int_1(s_1, t_1), int_2(s_2, t_2))
5. If both factor groups are commutative, the product group is commutative
6. If either factor group is non-commutative, the product group is non-commutative

# Construction / Recognition
## To Construct:
1. Start with GIS_1 and GIS_2
2. Form S_3 = S_1 x S_2 (all ordered pairs)
3. Form IVLS_3 = IVLS_1 x IVLS_2 with componentwise operation
4. Define int_3 componentwise from int_1 and int_2
5. GIS_3 = (S_3, IVLS_3, int_3) is automatically a GIS

## To Recognize:
1. The space consists of ordered pairs from two simpler spaces
2. Intervals are ordered pairs, each component from a different interval group
3. The int function decomposes into independent components

# Context & Application
Direct-product GIS structures model conjoint musical dimensions. They allow pitch, time, duration, timbre, and other aspects to be treated as components of a unified intervallic system, enabling analysis of how these dimensions interact. This is the second principal method (along with quotient) for constructing new GIS structures from existing ones.

# Examples
**Example 1** (pp. 69-77): Pitch-class and time-point GIS (Example 3.3.1):
- GIS_1: 12 pitch classes, IVLS_1 = Z/12Z
- GIS_2: time-points, IVLS_2 = Z
- GIS_3: pairs (pitch-class, time-point)
- Sample: int_3((C#, 35), (F, 46)) = (4, 11)
- Applied to Webern Piano Variations op. 27, third movement

**Example 2** (pp. 76-77): Time-point and duration GIS (Example 3.3.2):
- GIS_1: time-points, IVLS_1 = Z (additive)
- GIS_2: durations, IVLS_2 = positive rationals (multiplicative)
- GIS_3: pairs (s, x) where s = attack time, x = duration
- int_3((s, x), (t, y)) = (t - s, y/x)
- This is a commutative time-span GIS

# Relationships
## Builds Upon
- **Generalized Interval System** — the product GIS is itself a GIS, combining two others
- **Direct Product** — uses the Cartesian product of sets and direct-product group

## Enables
- **Commutative Time-Span GIS** — Example 3.3.2 constructs a commutative time-span GIS as a direct product
- **Timbral GIS** — timbral GIS structures use direct products of multiple (R+, *) factors

## Related
- **Time-Span GIS** — the non-commutative time-span GIS is motivated by the direct-product construction but differs from it

## Contrasts With
- **Quotient GIS** — direct product combines two spaces; quotient reduces one space

# Common Errors
- **Error**: Computing intervals by taking the product or sum of the two components
  **Correction**: The direct-product interval is a pair of intervals; each component is computed independently according to its own group operation

- **Error**: Mixing up additive and multiplicative components
  **Correction**: In Example 3.3.2, the first component is additive (t - s) and the second is multiplicative (y/x); each uses its own group structure

# Common Confusions
- **Confusion**: Thinking the two dimensions of a direct-product GIS interact
  **Clarification**: In the direct-product construction, the two dimensions are formally independent. Interaction is revealed analytically (e.g., recurrent GIS_3-intervals in Webern), not structurally.

- **Confusion**: Confusing direct-product GIS with the non-commutative time-span GIS
  **Clarification**: The commutative time-span GIS of Example 3.3.2 is a direct product; the non-commutative time-span GIS of 4.1.3 is not, despite involving similar pairs (a, x).

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Examples 3.3.1-3.3.2 and Definition 3.3.3, pp. 69-77.

# Verification Notes
- Definition source: direct from Definition 3.3.3
- Confidence rationale: high — explicit definition with worked examples
- Re-extraction notes: Re-extracted from v2 card; preserved: Webern analysis example, distinction from quotient GIS, note about independent dimensions
