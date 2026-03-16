---
concept: Syntonic Comma and Striche
slug: syntonic-comma-striche

category: pitch-space
subcategory: notation and pitch distinction
tier: intermediate

source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Edward Gollin"
chapter: "From Acoustical to Metaphorical: The Tonnetz from Oettingen to Riemann"
chapter_number: 9
pdf_page: null
section: null

extraction_confidence: high

aliases:
  - "Comma of Didymus"
  - "Striche (comma marks)"
  - "syntonic comma notation"

prerequisites:
  - tonnetz
extends: []
related:
  - nonconforming-tonnetz
  - oettingens-acoustical-matrix
  - enharmonic-equivalence
  - regional-space
contrasts_with: []

answers_questions:
  - "What are Striche and what do they indicate on the Tonnetz?"
  - "What is the syntonic comma and why does it matter for the Tonnetz?"
  - "How did the meaning of Striche change from Oettingen to late Riemann?"
---

# Quick Definition

The syntonic comma is a small pitch discrepancy (81:80, approximately 22 cents) between tones reached by different routes on the just-intoned Tonnetz; Striche are the notational marks (underlines/overlines) used by Oettingen and Riemann to distinguish these comma-different versions of like-named pitches.

# Core Definition

The **syntonic comma** (ratio 81:80, approximately 21.5 cents) is the difference between a tone reached by four perfect fifths and one reached by two octaves plus a major third. On the nonconforming Tonnetz, this means that like-named pitches at different positions have slightly different frequencies. **Striche** (German: "strokes" or "marks") are diacritical indicators devised by Oettingen: underlines indicate one syntonic comma lower, overlines one comma higher, with multiple marks for multiple comma differences. Gollin traces how these marks evolved from indicating real acoustic differences (Oettingen, early Riemann) to indicating path derivation in mental tone-space (late Riemann's "Ideen," 1914-15), where Riemann wrote: "Our imagination knows nothing of the intonational difference between d and d-bar, but rather equates both, imagining d as the lower fifth of a and yet at the same time also as the upper fifth of g" (Ch. 9).

# Prerequisites

- **Tonnetz**: Striche are notational devices used within the Tonnetz framework to mark comma differences

# Key Properties

1. **Syntonic comma ratio**: 81:80, approximately 21.5 cents
2. **Generation**: Arises as (3/2)^4 / (2^1 x 5/4) = 81/80
3. **Strich convention**: Underline = one comma lower; overline = one comma higher; multiple marks for multiple commas
4. **Regional bounding**: Syntonic images of the same pitch (e.g., two versions of D) bound diatonic regions on the nonconforming Tonnetz
5. **Distinct from enharmonic diesis**: The enharmonic diesis (128:125, approximately 41 cents) separates differently-spelled pitches (C# vs Db); the syntonic comma separates like-named pitches reached by different paths

# Construction / Recognition

To identify syntonic comma differences:
1. On the just-intoned Tonnetz, locate two instances of the same letter name (e.g., D)
2. Trace the path from a reference pitch to each instance
3. If the paths differ by one vertical step (major third axis), the pitches differ by one syntonic comma
4. Mark the lower version with an underline (Strich), the higher with an overline

Example: D as the upper fifth of G vs. D as the lower third-complement in F# — these differ by 81:80.

# Context & Application

Striche were essential for Oettingen's acoustical matrix and early Riemann, where they recorded real intonational differences a performer should respect. As Riemann shifted to a psychological foundation, Striche became markers of derivational pathways in mental space rather than tuning instructions. In the conforming (equal-tempered) Tonnetz, Striche are unnecessary because syntonic equivalence is assumed and all like-named pitches occupy a single node.

Cohn (Ch. 11) introduces the related concept of the **syntonic seam**: the boundary on the nonconforming Tonnetz where two syntonic versions of a pitch meet, marking the edge of a diatonic region. Crossing this seam indicates modulation.

# Examples

Gollin's example from Oettingen (Ch. 9): In the just-intoned Tonnetz, the C major diatonic region is bounded by two versions of D — one reached as the double fifth above C (d = fifth of fifth), the other as the third-complement below (d-bar). The parallelogram enclosing all seven scale degrees has these two D's at opposite corners.

Riemann's late position (from "Ideen," 1914-15): "Our imagination knows nothing of the intonational difference between d and d-bar, but rather equates both" — the Striche now indicate distinct mental derivations of the same heard pitch.

# Relationships

## Builds Upon
- tonnetz (Striche are a notation specific to the Tonnetz framework)

## Enables
- regional-space (syntonic images bound diatonic regions as parallelograms)
- nonconforming-tonnetz (Striche are essential for distinguishing positions on the infinite plane)

## Related
- oettingens-acoustical-matrix (Striche originated in Oettingen's frequency-calculating table)
- enharmonic-equivalence (the enharmonic diesis is a distinct but related pitch discrepancy)

## Contrasts With
(none specific)

# Common Errors

- **Error**: Treating Striche as accidentals (sharps, flats)
  **Correction**: Striche mark comma differences (approximately 22 cents), not chromatic alterations (100 cents); they indicate distinct tuning versions of the same letter name

# Common Confusions

- **Confusion**: Striche always indicate real tuning differences
  **Clarification**: For Oettingen and early Riemann, yes; but for late Riemann (after 1914), Striche indicate path derivation in mental space, not actual intonational differences

- **Confusion**: The syntonic comma and enharmonic diesis are the same thing
  **Clarification**: The syntonic comma (81:80, approximately 22 cents) separates like-named pitches reached by different paths; the enharmonic diesis (128:125, approximately 41 cents) separates differently-named pitches (e.g., C# vs Db)

# Source Reference

Gollin, Edward. "From Acoustical to Metaphorical: The Tonnetz from Oettingen to Riemann." In *The Oxford Handbook of Neo-Riemannian Music Theories*, edited by Edward Gollin and Alexander Rehding. Oxford University Press, 2011. Chapter 9.

Cohn, Richard. Chapter 11 (syntonic seam concept).

# Verification Notes

Re-extracted from v2 card; preserved: mathematical derivation, Strich convention detail, evolution through three stages, Riemann quotation from "Ideen," syntonic seam discussion. High confidence: explicitly defined and extensively discussed by Gollin in Ch. 9.
