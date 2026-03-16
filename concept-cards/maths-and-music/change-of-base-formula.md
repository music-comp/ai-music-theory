---
# === CORE IDENTIFICATION ===
concept: Change of Base Formula
slug: change-of-base-formula

# === CLASSIFICATION ===
category: logarithms-and-measurement
subcategory: conversion
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
section: "Different Bases"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "property L4"
  - change of base

# === TYPED RELATIONSHIPS ===
prerequisites:
  - logarithmic-functions-as-inverses
  - logarithm-properties
extends:
  - logarithm-properties
related:
  - natural-logarithm
  - multiplicative-to-additive-conversion
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can I compute a logarithm of one base using a logarithm of a different base?"
  - "How are logarithms of different bases related?"
---

# Quick Definition

The change of base formula log_b(x) = log_a(x) / log_a(b) allows conversion between logarithms of different bases, making any logarithm computable from any other.

# Core Definition

For positive reals a, b (both != 1) and x > 0 (Wright, pp. 69-70):

**(L4)** log_b(x) = log_a(x) / log_a(b)

This shows that log_b and log_a are proportional as functions, with constant of proportionality 1/log_a(b). Proof: Let u = log_a(x), v = log_b(x), w = log_a(b). Then a^u = x, b^v = x, a^w = b. From the last two: x = (a^w)^v = a^(wv), so wv = u, giving v = u/w, which is (L4).

# Prerequisites

- **Logarithmic functions as inverses** — The formula relates two logarithmic functions
- **Logarithm properties** — The proof uses the relationship between logarithms and exponents

# Key Properties

1. log_b(x) and log_a(x) are proportional, differing only by the constant factor 1/log_a(b)
2. The constant of proportionality depends on both bases but not on x
3. Geometrically, the graph of log_b(x) is a vertical stretch/compression of log_a(x)

# Construction / Recognition

## To Convert Between Bases
1. Choose an available base a (typically e or 10 from a calculator)
2. Compute log_a(x) and log_a(b) separately
3. Divide: log_b(x) = log_a(x) / log_a(b)

# Context & Application

The change of base formula is practically essential because calculators typically provide only ln (base e) or log_10. To compute log_2(r) for musical interval conversion, one uses log_2(r) = ln(r) / ln(2). This makes all ratio-to-additive conversions accessible with a standard calculator.

# Examples

**Example 1** (p. 69): The functions log_6(x) and log_3(x) differ by a vertical stretch factor of log_3(6) ~ 1.631.

**Example 2** (p. 70): Using the natural logarithm to compute any base: log_b(x) = ln(x) / ln(b). Setting a = e gives formula (L5).

**Example 3** (p. 72): 1200 * log_2(3/2) = 1200 * (ln(3/2) / ln(2)) ~ 701.955 cents.

# Relationships

## Builds Upon
- **Logarithm properties** — L4 extends the logarithm toolkit with cross-base conversion

## Enables
- **Natural logarithm** — L5 is the special case a = e of L4
- **Multiplicative-to-additive conversion** — L4 makes all conversions computable with any available logarithm

## Related
- **Natural logarithm** — The most common computational base for applying L4

# Common Errors

- **Error**: Computing log_a(b) / log_a(x) instead of log_a(x) / log_a(b)
  **Correction**: The formula divides log_a(x) by log_a(b), not the other way around

- **Error**: Forgetting that the constant 1/log_a(b) depends on both bases
  **Correction**: The proportionality constant changes when either base a or base b changes

# Common Confusions

- **Confusion**: Thinking different bases produce fundamentally different information
  **Clarification**: All logarithms are proportional; changing the base only rescales the output by a constant factor

# Source Reference

Chapter 5: "Logarithms and Musical Intervals," pp. 69-70. See the proof of L4 and the geometric interpretation as vertical stretching.

# Verification Notes

- Definition source: Direct from Wright, pp. 69-70, labeled as property L4
- Confidence rationale: High — explicitly stated, proved, and applied
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: geometric interpretation, calculator application context
