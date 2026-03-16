---
# === CORE IDENTIFICATION ===
concept: Group of Modular Intervals
slug: group-of-modular-intervals

# === CLASSIFICATION ===
category: algebra-in-music
subcategory: groups
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "The Group of Modular Intervals"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - modular interval group

# === TYPED RELATIONSHIPS ===
prerequisites:
  - group-of-intervals
  - modular-equivalence-on-the-real-numbers
extends:
  - group-of-intervals
related:
  - modular-chromatic-intervals
  - octave-equivalence-formalized
  - wrapping-real-line-around-circle
contrasts_with:
  - group-of-intervals
  - modular-chromatic-intervals

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the group of modular intervals?"
  - "How does interval composition work modulo octave?"
---

# Quick Definition

The group of interval equivalence classes modulo octave, identified with (R/~, +), where composition is defined by choosing representatives, adding, and taking the equivalence class of the sum.

# Core Definition

The group of modular intervals is (R/~, +), where x-bar + y-bar = (x+y)-bar (well-defined by an argument analogous to the well-definedness proof for Z_m). Elements are equivalence classes of intervals modulo octave. The identity is 0-bar (unison class), and the inverse of x-bar is (-x)-bar (Wright, p. 89).

# Prerequisites

- **Group of intervals** — The full interval group (R, +) from which this is derived
- **Modular equivalence on the real numbers** — The equivalence relation producing R/~

# Key Properties

1. Elements are equivalence classes of intervals modulo octave
2. Addition is well-defined on classes
3. Identity: 0-bar (unison/octave class)
4. Inverse of x-bar is (-x)-bar
5. The group is continuous (parameterized by the circle)
6. Z_12 (or Z_n) is a discrete subgroup

# Construction / Recognition

## To Compose Modular Intervals
1. Choose representatives x, y for the two interval classes
2. Compute x + y
3. Take the equivalence class (x+y)-bar
4. The result is independent of representatives chosen

# Context & Application

In this group, a third + a ninth = a tritone (since a ninth is octave-equivalent to a second). A fourth + a fifth = unison (not an octave, since octave = unison in this group). This captures the musical intuition that interval relationships persist across different octave registers.

# Examples

**Example 1** (p. 89): Third + ninth = tritone: 4-bar + 14-bar = 18-bar = 6-bar (in semitone measure, mod 12).

**Example 2** (p. 89): Fourth + fifth = unison: 5-bar + 7-bar = 12-bar = 0-bar.

**Example 3** (p. 89): This group includes non-chromatic intervals: the just major third (ratio 5/4, ~386.3 cents) has its own class distinct from the tempered major third (400 cents).

# Relationships

## Builds Upon
- **Group of intervals** — (R/~, +) is the quotient of (R, +) by octave equivalence
- **Modular equivalence on the real numbers** — The equivalence relation defining the quotient

## Enables
- **Modular chromatic intervals** — Z_12 is the discrete subgroup of chromatic classes

## Related
- **Wrapping real line around circle** — The wrapping homomorphism w: R -> R/~
- **Octave equivalence formalized** — The motivating musical concept

## Contrasts With
- **Group of intervals** — (R, +) distinguishes intervals in different octaves; (R/~, +) does not
- **Modular chromatic intervals** — Z_12 is discrete; (R/~, +) is continuous

# Common Errors

- **Error**: Confusing the group of modular intervals with Z_12
  **Correction**: Z_12 contains only chromatic interval classes; (R/~, +) contains all interval classes including microtonal

# Common Confusions

- **Confusion**: Thinking this group contains "octave" as a distinct element
  **Clarification**: The octave class equals the unison class (0-bar) in this group

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," p. 89 (The Group of Modular Intervals section).

# Verification Notes

- Definition source: Direct from Wright, p. 89
- Confidence rationale: High — explicit definition
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: continuous vs. discrete distinction, just third example, third+ninth composition
