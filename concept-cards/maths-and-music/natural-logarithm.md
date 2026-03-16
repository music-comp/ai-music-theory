---
# === CORE IDENTIFICATION ===
concept: Natural Logarithm
slug: natural-logarithm

# === CLASSIFICATION ===
category: logarithms-and-measurement
subcategory: logarithms
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
section: "Calculating Using the Natural Logarithm"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - ln
  - log_e

# === TYPED RELATIONSHIPS ===
prerequisites:
  - logarithmic-functions-as-inverses
  - change-of-base-formula
extends:
  - change-of-base-formula
related:
  - multiplicative-to-additive-conversion
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the natural logarithm?"
  - "How can the natural logarithm be used to compute musical interval conversions?"
---

# Quick Definition

The natural logarithm (ln x = log_e x) uses the transcendental number e ~ 2.71828 as its base. It is the most commonly available logarithm on calculators and is used with the change of base formula to compute any other logarithm.

# Core Definition

The natural logarithm is log_e(x), commonly denoted ln(x), where e ~ 2.71828 is the base of the natural exponential function. Using the change of base formula (L4) with a = e gives (Wright, p. 70):

**(L5)** log_b(x) = ln(x) / ln(b)

This allows any logarithm to be computed using ln. Similarly, log_10 (also commonly available on calculators) can be used via the same formula with a = 10.

# Prerequisites

- **Logarithmic functions as inverses** — The natural logarithm is a specific logarithmic function
- **Change of base formula** — L5 is the special case of L4 with a = e

# Key Properties

1. ln(x) = log_e(x), where e ~ 2.71828
2. e is a transcendental number significant in calculus (d/dx e^x = e^x)
3. Any logarithm can be computed via ln: log_b(x) = ln(x) / ln(b)
4. ln(2) ~ 0.6931 is a frequently needed constant for musical conversions

# Construction / Recognition

## To Convert a Ratio r to Cents Using ln
1. Compute ln(r) using a calculator
2. Compute ln(2) ~ 0.6931
3. Evaluate x = 1200 * (ln(r) / ln(2))

# Context & Application

While e has no special musical significance, the natural logarithm is the standard computational bridge. For musical applications, the key formula is log_2(r) = ln(r) / ln(2), which converts any ratio to octaves and can then be scaled to semitones (multiply by 12) or cents (multiply by 1200). The musically meaningful base is 2 (for octaves) or 2^(1/12) (for semitones).

# Examples

**Example 1** (p. 70): Setting a = e in L4 gives the formula log_b(x) = ln(x) / ln(b).

**Example 2** (p. 72, worked example): Computing the cents value of the ratio 3/2:
x = 1200 * (ln(3/2) / ln(2)) = 1200 * ((ln 3 - ln 2) / ln 2) ~ 701.955 cents.

**Example 3** (p. 70): log_10 can also be used: log_b(x) = log_10(x) / log_10(b).

# Relationships

## Builds Upon
- **Change of base formula** — L5 is the special case of L4 using base e

## Enables
- **Multiplicative-to-additive conversion** — ln provides the practical computational tool for all interval conversions

## Related
- **Multiplicative-to-additive conversion** — ln is the computational workhorse for converting ratios to cents/semitones

# Common Errors

- **Error**: Using ln(r) directly as the interval measurement without dividing by ln(b)
  **Correction**: The formula requires log_b(x) = ln(x) / ln(b); omitting the denominator gives the wrong base

# Common Confusions

- **Confusion**: Believing e ~ 2.71828 has intrinsic musical meaning
  **Clarification**: e has no special musical significance; it is simply a computational convenience. The musically natural base is 2 (for octaves)

- **Confusion**: Thinking ln is the "best" logarithm for music
  **Clarification**: ln and log_10 both work equally well for musical conversions; the change of base formula handles the conversion

# Source Reference

Chapter 5: "Logarithms and Musical Intervals," p. 70. See the derivation of formula L5.

# Verification Notes

- Definition source: Direct from Wright, p. 70
- Confidence rationale: High — explicitly defined with formula L5
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: computational examples, clarification about e having no musical significance
