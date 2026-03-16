---
# === CORE IDENTIFICATION ===
concept: Tuplets
slug: tuplets

# === CLASSIFICATION ===
category: rhythm-and-form
subcategory: duration
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Horizontal Structure"
chapter_number: 2
pdf_page: 30
section: "Tuplets"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "tuplet"
  - "triplet"
  - "quintuplet"
  - "k-tuplet"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - note-durational-values
extends:
  - note-durational-values
related:
  - rhythm
  - meter-and-time-signatures
  - swing-rhythm
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a tuplet and how is it constructed?"
  - "How do you divide a note into a non-power-of-2 number of equal parts?"
  - "What is the mathematical formula for determining tuplet notation?"
---

# Quick Definition

A method for dividing a note duration into $k$ equal parts where $k$ is not a power of 2, notated by grouping notes under the integer $k$, enabling rhythmic divisions like triplets and quintuplets.

# Core Definition

To divide the $\frac{1}{2^n}$-th note into $k$ equal notes (where $k$ is not a power of 2), find the unique positive integer $r$ such that $2^r < k < 2^{r+1}$, and notate the tuplet as a group of $k$ $\frac{1}{2^{n+r}}$-th notes overset or underset by the integer $k$. The resulting tuplet is called the $\frac{1}{2^{n+r}}$-th note $k$-tuplet. This is the most basic form of *polyrhythm* (Wright, pp. 34-35).

# Prerequisites

- **Note Durational Values** — Tuplets extend the power-of-2 duration system to non-power-of-2 divisions

# Key Properties

1. The construction finds $r$ such that $2^r < k < 2^{r+1}$ (i.e., $r = \lfloor \log_2 k \rfloor$ for non-power-of-2 $k$)
2. Each note in the $k$-tuplet has duration $\frac{1}{k \cdot 2^n}$ of a whole note
3. The notation uses $\frac{1}{2^{n+r}}$-th notes because $2^{n+r}$ is the nearest power-of-2 subdivision
4. Tuplets are the most basic form of polyrhythm
5. Dividing a duration into $n$ equal parts parallels the $n$th harmonic (vibration $n$ times faster)

# Construction / Recognition

## To construct a $k$-tuplet of the $\frac{1}{2^n}$-th note:

1. Find $r$ such that $2^r < k < 2^{r+1}$
2. Write $k$ notes of type $\frac{1}{2^{n+r}}$-th note
3. Group them together with the number $k$ above or below
4. The group fills the same duration as one $\frac{1}{2^n}$-th note

# Context & Application

Tuplets enable rhythmic flexibility beyond the binary subdivision system. The most common tuplet is the triplet (dividing into 3). The concept has an interesting similarity to harmonics: dividing a duration into $n$ equal parts parallels a vibration $n$ times faster than a fundamental pitch.

# Examples

- **Triplet**: divide quarter note ($n=2$) into 3. Since $2^1 < 3 < 2^2$, $r = 1$. Write 3 eighth notes with "3" above (p. 34)
- **Quintuplet**: divide quarter note into 5. Since $2^2 < 5 < 2^3$, $r = 2$. Write 5 sixteenth notes with "5" above (p. 34)
- Exercise 4(c): divide whole note into 11. Since $2^3 < 11 < 2^4$, $r = 3$. Write 11 eighth notes with "11" above
- An eighth note triplet: each triplet eighth = $\frac{2}{3}$ of a normal eighth note

# Relationships

## Builds Upon
- **Note Durational Values** — Tuplets extend the binary duration system

## Enables
- **Swing Rhythm** — Swing involves a pervasive triplet figure
- Complex polyrhythmic patterns

## Related
- **Rhythm** — Tuplets are rhythmic devices
- **Meter and Time Signatures** — Compound time signatures relate to triplet subdivisions

# Common Errors

- **Error**: Thinking each triplet eighth note equals a normal eighth note
  **Correction**: Each triplet eighth is $\frac{2}{3}$ of a normal eighth — the "3" label signals compression

# Common Confusions

- **Confusion**: Thinking tuplets are needed when $k$ is a power of 2
  **Clarification**: The standard durational system already handles powers of 2; tuplets are for non-power-of-2 divisions
- **Confusion**: Assuming the notation shows the actual duration of each note
  **Clarification**: The notation uses $\frac{1}{2^{n+r}}$-th notes that are individually *longer* than the actual tuplet notes; the $k$ label modifies their interpretation

# Source Reference

Chapter 2: "Horizontal Structure", "Tuplets" section, pp. 34-35 (PDF).

# Verification Notes

- Definition source: Direct from source, pp. 34-35
- Confidence rationale: High — explicit formula with construction procedure and worked examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: triplet/quintuplet examples, harmonics analogy, polyrhythm connection
