---
concept: Logarithmic Functions as Inverses
slug: logarithmic-functions-as-inverses

category: logarithms-and-measurement
subcategory: logarithms
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
section: "Logarithmic Functions"

extraction_confidence: high

aliases:
  - logarithm
  - log function
  - inverse exponential

prerequisites:
  - exponents-and-exponential-functions
extends: []
related:
  - logarithm-properties
  - change-of-base-formula
  - natural-logarithm
contrasts_with:
  - exponents-and-exponential-functions

answers_questions:
  - "What is a logarithm and why is it useful for measuring intervals?"
  - "How is the logarithm defined as the inverse of the exponential function?"
  - "What does the statement log_b(x) = y mean?"
---

# Quick Definition

The logarithm log_b(x) is defined as the inverse of the exponential function b^x. The statement log_b(x) = y means exactly the same as b^y = x.

# Core Definition

The function g(x) = log_b(x) is the inverse function of f(x) = b^x, meaning:
- f(g(x)) = x, i.e., b^(log_b(x)) = x
- g(f(x)) = x, i.e., log_b(b^x) = x

The domain of log_b is R+ (positive reals) and its range is R (all reals). For b > 1, log_b is strictly increasing and one-to-one. Its graph is obtained by reflecting the graph of b^x across the line y = x. The base b is always positive and not equal to 1, usually taken greater than 1 (Wright, pp. 67-68).

# Prerequisites

- **Exponents and exponential functions** — The logarithm is defined as the inverse of the exponential function; understanding one-to-one functions and their inverses is required

# Key Properties

1. Domain is R+ (positive reals); range is R (all reals)
2. log_b(x) = y means b^y = x
3. b^(log_b(x)) = x for all x > 0
4. log_b(b^x) = x for all real x
5. For b > 1, log_b is strictly increasing and one-to-one
6. log_b(1) = 0 for any valid base (since b^0 = 1)
7. Graph is the reflection of b^x across y = x

# Construction / Recognition

## To Evaluate log_b(x)
1. Determine if x can be expressed as a power of b
2. If x = b^n for some known n, then log_b(x) = n
3. Otherwise, use the change of base formula: log_b(x) = ln(x)/ln(b)

## Examples of Direct Evaluation
- log_3(9) = 2, since 3^2 = 9
- log_b(sqrt(b)) = 1/2, since b^(1/2) = sqrt(b)
- log_b(1) = 0, since b^0 = 1

# Context & Application

The logarithm answers the question: "Given a frequency ratio r, how many semitones (or cents, or octaves) does it represent?" This is the inverse of the question answered by the exponential function. Without logarithms, one can convert semitones to ratios (r = 2^(x/12)) but cannot convert ratios back to semitones. The logarithm completes the conversion toolkit.

# Examples

**Example 1** (p. 67): log_3(9) = 2, because 3^2 = 9.

**Example 2** (p. 67): log_b(sqrt(b)) = 1/2, because b^(1/2) = sqrt(b).

**Example 3** (implied): log_2(8) = 3, because 2^3 = 8 — the ratio 8 spans 3 octaves.

# Relationships

## Builds Upon
- **Exponents and exponential functions** — The logarithm is defined as the inverse of the exponential function

## Enables
- **Logarithm properties** — The properties L1-L3 describe how logarithms behave
- **Change of base formula** — Allows conversion between logarithms of different bases
- **Logarithmic pitch scale** — Plotting pitch by log of frequency gives equal spacing for equal intervals
- **Multiplicative-to-additive conversion** — The logarithm is the tool that converts ratios to cents/semitones

## Related
- **Natural logarithm** — The specific case where the base is e

## Contrasts With
- **Exponents and exponential functions** — The exponential converts additive to multiplicative; the logarithm converts multiplicative to additive

# Common Errors

- **Error**: Attempting to compute log_b(0) or log_b of a negative number
  **Correction**: log_b(x) is only defined for x > 0; the domain is R+

- **Error**: Confusing log_b(x) = y with b^x = y
  **Correction**: log_b(x) = y means b^y = x; the argument x is the power, and y is the exponent

# Common Confusions

- **Confusion**: Thinking the logarithm is a number rather than a function
  **Clarification**: log_b by itself is meaningless without an argument; it is a function from R+ to R

- **Confusion**: Believing log_b(x) can produce only integer values
  **Clarification**: log_b(x) produces real-valued outputs; e.g., log_2(3) is irrational

# Source Reference

Chapter 5: "Logarithms and Musical Intervals," pp. 67-68. See the definition of the inverse function and the graph of g(x) = log_b(x).

# Verification Notes

- Definition source: Direct from Wright, pp. 67-68
- Confidence rationale: High — explicit definition with inverse function characterization
- Uncertainties: None
- Cross-reference status: Verified against planned extractions
- Re-extraction notes: Re-extracted from v2 card; preserved: musical context about completing the conversion toolkit, direct evaluation examples
