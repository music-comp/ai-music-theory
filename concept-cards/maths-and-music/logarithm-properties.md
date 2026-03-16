---
concept: Logarithm Properties
slug: logarithm-properties

category: logarithms-and-measurement
subcategory: logarithms
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
section: "Properties of Logarithms"

extraction_confidence: high

aliases:
  - "logarithm laws L1-L3"
  - "properties L1 L2 L3"

prerequisites:
  - logarithmic-functions-as-inverses
  - exponents-and-exponential-functions
extends: []
related:
  - logarithmic-pitch-scale
  - multiplicative-to-additive-conversion
  - homomorphism
contrasts_with: []

answers_questions:
  - "How do logarithms convert multiplicative intervals to additive measurements?"
  - "What are the fundamental properties of logarithms?"
  - "Why do equal intervals appear as equal distances on a logarithmic scale?"
---

# Quick Definition

Logarithms transform multiplication into addition and division into subtraction. These properties (L1-L3) are the mathematical reason logarithms are essential for converting between multiplicative (ratio) and additive (semitone/cent) interval measurements.

# Core Definition

For any positive reals x, y and any real number p, with base b > 0, b != 1 (Wright, pp. 68-69):

- **(L1)** log_b(xy) = log_b(x) + log_b(y)
- **(L2)** log_b(x/y) = log_b(x) - log_b(y)
- **(L3)** log_b(x^p) = p * log_b(x)

Property (L1) derives from the law of exponents b^(s+t) = b^s * b^t: letting s = log_b(x) and t = log_b(y), we have b^(s+t) = b^s * b^t = x * y, so s + t = log_b(xy).

# Prerequisites

- **Logarithmic functions as inverses** — The properties are stated for the logarithm function
- **Exponents and exponential functions** — The proofs derive from the laws of exponents

# Key Properties

1. (L1) converts multiplication to addition
2. (L2) converts division to subtraction
3. (L3) converts exponentiation to scalar multiplication
4. Together, they characterize the logarithm as a group homomorphism from (R+, *) to (R, +)

# Construction / Recognition

## To Apply the Properties
1. To simplify log_b of a product: split into sum of logarithms (L1)
2. To simplify log_b of a quotient: split into difference of logarithms (L2)
3. To simplify log_b of a power: pull the exponent out as a multiplier (L3)
4. To combine logarithms: use L1-L3 in reverse

# Context & Application

Property (L2) is particularly significant for music: it ensures that if pitches x and y have the same interval ratio as x' and y', then log_b(x) - log_b(y) = log_b(x') - log_b(y'). This means equal intervals appear as equal distances on a logarithmic pitch axis, which is the mathematical basis for logarithmic pitch representation.

# Examples

**Example 1** (p. 68): Proof of L1 using exponent laws: let s = log_b(x), t = log_b(y); then b^(s+t) = b^s * b^t = x * y, so log_b(xy) = s + t = log_b(x) + log_b(y).

**Example 2** (p. 68): L2 applied to pitch: if x/y = x'/y' (same interval ratio), then log_b(x) - log_b(y) = log_b(x') - log_b(y') (same distance on logarithmic axis).

**Example 3** (p. 72, worked example): log_2(3/2) = log_2(3) - log_2(2) = log_2(3) - 1, using L2 to compute the cents value of a just fifth.

# Relationships

## Builds Upon
- **Logarithmic functions as inverses** — Properties describe the behavior of the logarithm function
- **Exponents and exponential functions** — Properties derive from the laws of exponents

## Enables
- **Logarithmic pitch scale** — L2 guarantees equal intervals = equal distances
- **Multiplicative-to-additive conversion** — Properties allow systematic conversion of ratios to additive measures

## Related
- **Homomorphism** — L1 is precisely the condition for the logarithm to be a group homomorphism from (R+, *) to (R, +)

# Common Errors

- **Error**: Writing log_b(x + y) = log_b(x) + log_b(y)
  **Correction**: The logarithm converts multiplication to addition, not addition to addition; log_b(x * y) = log_b(x) + log_b(y)

- **Error**: Writing log_b(x * y) = log_b(x) * log_b(y)
  **Correction**: The product of arguments becomes a sum of logarithms, not a product of logarithms

# Common Confusions

- **Confusion**: Believing L3 says (log_b(x))^p = p * log_b(x)
  **Clarification**: L3 says log_b(x^p) = p * log_b(x); the exponent must be inside the logarithm's argument

- **Confusion**: Thinking L1-L3 only work for integer exponents or arguments
  **Clarification**: Properties hold for all positive real arguments x, y and all real exponents p

# Source Reference

Chapter 5: "Logarithms and Musical Intervals," pp. 68-69. See the proof of L1 and the application to logarithmic pitch scales.

# Verification Notes

- Definition source: Direct from Wright, pp. 68-69, with labeled properties L1-L3
- Confidence rationale: High — explicitly stated and proved
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: homomorphism characterization, musical application of L2 for equal intervals
