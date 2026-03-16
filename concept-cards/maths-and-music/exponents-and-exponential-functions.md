---
# === CORE IDENTIFICATION ===
concept: Exponents and Exponential Functions
slug: exponents-and-exponential-functions

# === CLASSIFICATION ===
category: logarithms-and-measurement
subcategory: exponentials
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
section: "Exponents"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - exponential function
  - exponential map

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - logarithmic-functions-as-inverses
  - multiplicative-to-additive-conversion
  - interval-as-frequency-ratio
contrasts_with:
  - logarithmic-functions-as-inverses

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the exponential function and how is it defined?"
  - "How do exponential functions convert additive interval measurements to frequency ratios?"
  - "Why is the exponential function one-to-one?"
---

# Quick Definition

The exponential function f(x) = b^x maps real numbers to positive reals, providing the mathematical foundation for converting additive interval measurements (semitones, cents) to multiplicative frequency ratios.

# Core Definition

For a positive real number b != 1 (the *base*), the exponential function f: R -> R+ is defined by f(x) = b^x. For positive integers n, b^n is the n-fold product b * b * ... * b; b^(-n) = 1/b^n; b^(1/n) = the nth root of b. The rule of exponents b^(st) = (b^s)^t, together with the calculus concept of limit, extends the definition to all real x, making f continuous. For b > 1, the function is strictly increasing and one-to-one, with domain R and range R+ (Wright, p. 66).

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. Domain is R (all real numbers); range is R+ (positive reals)
2. For b > 1, f(x) = b^x is strictly increasing and one-to-one
3. b^(s+t) = b^s * b^t (converts addition to multiplication)
4. b^(st) = (b^s)^t (exponent rule)
5. b^0 = 1 (identity)
6. b^(-x) = 1/b^x (reciprocal)
7. Being one-to-one and onto R+, the function has an inverse (the logarithm)

# Construction / Recognition

## To Evaluate b^x for Rational x
1. For positive integer n: compute b^n as the n-fold product b * b * ... * b
2. For b^(-n): compute 1/b^n
3. For b^(1/n): compute the nth root of b
4. For general rational x = p/q: compute (b^p)^(1/q) or equivalently the qth root of b^p
5. For irrational x: the value is defined via limits (continuity)

# Context & Application

The exponential function is central to interval theory because formulas like r = 2^(x/12) (semitones to ratio) and r = 2^(x/1200) (cents to ratio) are exponential functions. It establishes a group isomorphism from (R, +) to (R+, *), which is the mathematical basis for converting between additive and multiplicative interval measurements.

# Examples

**Example 1** (p. 66): Computing b^(-2/3):
b^((-2)(1/3)) = (b^(-2))^(1/3) = (1/b^2)^(1/3) = cube_root(1/b^2)

**Example 2** (p. 67): The graph of f(x) = b^x passes through (0, 1) for any base b, since b^0 = 1. For b > 1 the graph curves upward.

**Example 3** (implied, pp. 70-71): 2^(7/12) ~ 1.498 gives the equal-tempered fifth ratio; plotting keyboard note frequencies vs. pitch number gives an exponential curve.

# Relationships

## Builds Upon
This is foundational; no prior concepts required.

## Enables
- **Logarithmic functions as inverses** — The logarithm is defined as the inverse of the exponential
- **Multiplicative-to-additive conversion** — Exponential converts additive measurements to frequency ratios

## Related
- **Interval as frequency ratio** — Exponentials express the ratio interpretation of intervals

## Contrasts With
- **Logarithmic functions as inverses** — The logarithm reverses the exponential; one converts additive to multiplicative, the other multiplicative to additive

# Common Errors

- **Error**: Confusing b^(s+t) with b^s + b^t
  **Correction**: b^(s+t) = b^s * b^t (exponents convert addition to multiplication, not to addition)

- **Error**: Attempting to evaluate b^x for b <= 0 or b = 1
  **Correction**: The base must be positive and not equal to 1 for the exponential function to be well-defined and one-to-one

# Common Confusions

- **Confusion**: Believing b^x is only defined for integer or rational exponents
  **Clarification**: The calculus concept of limit extends b^x to all real numbers x, making f continuous

- **Confusion**: Thinking the exponential function maps onto all of R
  **Clarification**: The range is R+ (strictly positive reals); b^x > 0 for all x

# Source Reference

Chapter 5: "Logarithms and Musical Intervals," pp. 66-67. See the graph of f(x) = b^x and the discussion of the rule of exponents.

# Verification Notes

- Definition source: Direct from Wright, pp. 66-67
- Confidence rationale: High — explicit definition with formal properties stated
- Uncertainties: None
- Cross-reference status: Verified against planned extractions
- Re-extraction notes: Re-extracted from v2 card; preserved: group isomorphism remark, musical context about equal intervals producing non-equal frequency differences
