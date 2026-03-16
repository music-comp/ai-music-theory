---
# === CORE IDENTIFICATION ===
concept: Error Calculation in Cents
slug: error-calculation-in-cents

# === CLASSIFICATION ===
category: pitch-and-intervals
subcategory: approximation
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Integers as Intervals"
chapter_number: 9
pdf_page: 110
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - integral-intervals
extends: []
related:
  - keyboard-approximation-of-integer-ratios
  - powers-of-two-as-exact-keyboard-intervals
  - in-the-cracks-intervals
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I calculate the approximation error for an integer ratio on the keyboard?"
  - "What does it mean for the keyboard to be 'sharp' or 'flat' relative to an integer ratio?"
---

# Quick Definition

A method for quantifying how closely an equally tempered keyboard interval approximates a pure integer ratio, by computing the difference in cents between the exact ratio and the nearest semitone.

# Core Definition

For a positive integer $n$, the exact interval in cents is $1200 \log_2 n$. The keyboard approximation is the nearest multiple of 100 cents ($k$ semitones where $k = \text{round}(1200 \log_2 n / 100)$). The error $E = 1200 \log_2 n - 100k$. Positive $E$ means the keyboard is sharp; negative means flat (Wright, Ch. 9, pp. 111-116).

# Prerequisites

- **Integral Intervals** -- The intervals whose errors are being calculated

# Key Properties

1. The cent scale is logarithmic, so errors are additive under composition
2. If $n$ has error $E(n)$ and $m$ has error $E(m)$, then $nm$ has error $E(n) + E(m)$
3. Powers of 2 have zero error
4. Errors under ~5 cents are generally imperceptible
5. Errors of ~14 cents (as with 5) are noticeable upon careful listening
6. Errors of ~31 cents or more (as with 7) are clearly audible

# Construction / Recognition

## To compute the error for integer n:
1. Compute $c = 1200 \log_2 n$ (exact cent value)
2. Find nearest integer $k = \text{round}(c / 100)$ (nearest semitone count)
3. Error $E = c - 100k$
4. If $E > 0$: keyboard is sharp; if $E < 0$: keyboard is flat

# Context & Application

The cent is a logarithmic unit dividing each semitone into 100 equal parts. Error calculation reveals which integer ratios are well-served by equal temperament and which are not. The historical debate over equal temperament centered on these specific errors.

# Examples

**Example 1** (p. 111): $1200 \log_2 3 \approx 1901.96$ cents; nearest = 1900 cents (19 semitones); error $\approx$ 2 cents flat.

**Example 2** (p. 112): $1200 \log_2 5 \approx 2786.31$ cents; nearest = 2800 cents; error $\approx$ 14 cents sharp.

**Example 3** (p. 113): $1200 \log_2 7 \approx 3368.83$ cents; nearest = 3400 cents; error $\approx$ 31 cents sharp.

**Example 4** (p. 114): $1200 \log_2 11 \approx 4151.32$ cents; nearest = 4200 cents; error $\approx$ 49 cents sharp.

# Relationships

## Enables
- **Keyboard Approximation of Integer Ratios** -- Error calculation is the core method

## Related
- **Powers of Two as Exact Keyboard Intervals** -- The zero-error cases
- **In-the-Cracks Intervals** -- The worst-error cases

# Common Errors

- **Error**: Forgetting that cents are logarithmic
  **Correction**: Doubling a frequency ratio does not double the cent value; it adds 1200 cents

- **Error**: Confusing "sharp" and "flat" in error terminology
  **Correction**: "Sharp" means the keyboard interval is higher than the pure ratio; "flat" means lower

# Common Confusions

- **Confusion**: Thinking "sharp" and "flat" refer to musical sharps and flats
  **Clarification**: Here they describe the direction of the approximation error relative to the pure ratio

# Source Reference

Chapter 9: "The Integers as Intervals," pp. 111-116.

# Verification Notes

- Definition source: Synthesized from the systematic calculations in the chapter
- Confidence rationale: Each calculation is explicitly performed in the source
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: all four computed examples, perceptibility thresholds
