---
# === CORE IDENTIFICATION ===
concept: Homomorphism
slug: homomorphism

# === CLASSIFICATION ===
category: algebra-in-music
subcategory: morphisms
tier: advanced

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Homomorphism"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - group homomorphism
  - structure-preserving map

# === TYPED RELATIONSHIPS ===
prerequisites:
  - group
extends: []
related:
  - isomorphism
  - group-of-intervals
  - wrapping-real-line-around-circle
contrasts_with:
  - isomorphism

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a homomorphism?"
  - "How do homomorphisms relate to musical interval conversions?"
---

# Quick Definition

A function between two groups that preserves the group operation, mapping the composition of elements in one group to the composition of their images in the other.

# Core Definition

Given two groups (G, *) and (G', o), a function phi: G -> G' is a group homomorphism if for all x, y in G, phi(x * y) = phi(x) o phi(y). A homomorphism necessarily maps the identity of G to the identity of G': phi(e) = e' (Wright, pp. 89-90).

# Prerequisites

- **Group** — Homomorphisms are defined between groups

# Key Properties

1. phi(x * y) = phi(x) o phi(y) for all x, y in G
2. phi(e) = e' (identity maps to identity)
3. phi(x^(-1)) = phi(x)^(-1) (inverses map to inverses)
4. A homomorphism can be surjective (onto) without being injective (one-to-one), or vice versa
5. A bijective homomorphism is an isomorphism

# Construction / Recognition

## To Verify a Homomorphism
1. Confirm the function maps between two groups
2. Check that phi(x * y) = phi(x) o phi(y) for all x, y
3. The identity and inverse properties follow automatically

# Context & Application

The key homomorphisms in music theory are: (1) the exponential f(r) = b^r from (R, +) to (R+, *), converting additive to multiplicative measurement; (2) the logarithm g(x) = log_b(x) from (R+, *) to (R, +), converting multiplicative to additive; and (3) the wrapping function w: R -> R/~ modeling octave equivalence.

# Examples

**Example 1** (p. 90): phi: {1, -1} -> Z_2 defined by phi(1) = [0], phi(-1) = [1] is a homomorphism and isomorphism.

**Example 2** (p. 90): The wrapping function w: R -> R/~ defined by w(x) = x-bar is a homomorphism (onto but not one-to-one).

**Example 3** (p. 90): f(r) = b^r from (R, +) to (R+, *): f(r + s) = b^(r+s) = b^r * b^s = f(r) * f(s). This is a homomorphism (and isomorphism).

**Example 4** (p. 90): g(x) = log_b(x) from (R+, *) to (R, +): g(xy) = log_b(xy) = log_b(x) + log_b(y) = g(x) + g(y). This is the inverse isomorphism.

# Relationships

## Builds Upon
- **Group** — Homomorphisms map between groups

## Enables
- **Isomorphism** — A bijective homomorphism

## Related
- **Group of intervals** — The exponential and logarithm are homomorphisms between interval representations
- **Wrapping real line around circle** — The wrapping function is a surjective homomorphism

## Contrasts With
- **Isomorphism** — An isomorphism is a bijective homomorphism; not every homomorphism is bijective

# Common Errors

- **Error**: Assuming every homomorphism is bijective
  **Correction**: The wrapping function w is a homomorphism that is onto but not one-to-one; it is not an isomorphism

# Common Confusions

- **Confusion**: Thinking the same function is always a homomorphism regardless of which groups are involved
  **Clarification**: Homomorphism is a property relative to the group operations; the same function between different group structures may or may not be a homomorphism

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," pp. 89-90 (Homomorphism section). See examples (1)-(3).

# Verification Notes

- Definition source: Direct from Wright, pp. 89-90
- Confidence rationale: High — formal definition with multiple examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: all four examples, identity-preserving property
