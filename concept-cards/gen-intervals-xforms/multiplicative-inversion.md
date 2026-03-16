---
concept: Multiplicative Inversion
slug: multiplicative-inversion

category: analytical-applications
subcategory: rhythmic-analysis
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (4): Some Further Analyses"
chapter_number: 10
pdf_page: 251
section: "10.1"

extraction_confidence: high

aliases:
  - "I_p (durational)"

prerequisites:
  - durational-motive
extends: []
related:
  - multiplicative-transposition
  - rich-relations-in-rhythm
  - mozart-k550-development-analysis
contrasts_with: []

answers_questions:
  - "What is multiplicative inversion of a durational series?"
  - "How do multiplicative and additive inversion compare in the durational domain?"
---

# Quick Definition
Multiplicative inversion of a durational series divides a fixed product p by each duration value, producing an inversion where longer durations become shorter and vice versa. It can yield the same result as additive inversion about a sum s in certain cases.

# Core Definition
Given durational series S = (d1, d2, d3) and inversional product p, multiplicative inversion Ip(S) = (p/d1, p/d2, p/d3). Alternatively, additive inversion about sum s gives Is(S) = (s-d1, s-d2, s-d3). Lewin notes: "We can regard the inversion as multiplicative, about the numerical product 8: 8 divided by 2, 4, and 4 (series 3) yields 4, 2, and 2 (series 4b). Or we can regard the inversion as additive, about the numerical sum 6: 6 take away 2, 4, and 4 also yields 4, 2, and 2" (Section 10.1, p. 254).

# Prerequisites
- **Durational motive** — the operand of multiplicative inversion

# Key Properties
1. Ip(S) = (p/d1, p/d2, ..., p/dn) for multiplicative inversion
2. Is(S) = (s-d1, s-d2, ..., s-dn) for additive inversion
3. Multiplicative and additive inversions can produce identical results
4. Ip is an involution: Ip(Ip(S)) = S
5. Neither multiplicative nor additive framework is inherently "correct"

# Construction / Recognition
## To Construct:
1. Choose product p (or sum s)
2. Divide p by (or subtract from s) each duration
## To Recognize:
1. Check if one series inverts another multiplicatively or additively

# Context & Application
Multiplicative inversion transforms the rhythmic profile of a durational motive, mapping long durations to short and vice versa. The dual computation (multiplicative or additive) reflects a genuine structural ambiguity in the durational domain not present in pitch-class inversion (which is always additive mod 12).

# Examples
**Example 1** (Section 10.1, p. 254): Series 3 = 2+4+4. Multiplicative inversion about 8: 8/2=4, 8/4=2, 8/4=2 -> series 4b = 4+2+2. Additive inversion about 6: 6-2=4, 6-4=2, 6-4=2 -> same result.

# Relationships
## Builds Upon
- **Durational motive** — the operand
## Related
- **Multiplicative transposition** — the other durational transformation
- **RICH-relations in rhythm** — inversion is a component of rhythmic RICH
- **Mozart K.550 development analysis** — primary example

# Common Errors
- **Error**: Assuming only one type of durational inversion exists
  **Correction**: Both multiplicative and additive inversion are valid; choose based on analytical context

# Common Confusions
- **Confusion**: Thinking multiplicative inversion always differs from additive inversion
  **Clarification**: They can produce identical results when p and s are chosen appropriately

# Source Reference
Chapter 10: Transformation Graphs and Networks (4): Some Further Analyses, Section 10.1, p. 254. See Figures 10.2-10.3.

# Verification Notes
- Definition source: direct from Section 10.1 with Lewin's dual computation
- Confidence rationale: high -- explicitly worked out with both methods
- Re-extracted from v2 card; preserved: dual computation example, involution property
