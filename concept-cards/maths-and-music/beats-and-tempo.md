---
# === CORE IDENTIFICATION ===
concept: Beats and Tempo
slug: beats-and-tempo

# === CLASSIFICATION ===
category: rhythm-and-form
subcategory: duration
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Horizontal Structure"
chapter_number: 2
pdf_page: 30
section: "Duration of Notes"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "beat"
  - "tempo"
  - "beats per minute"
  - "BPM"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - horizontal-structure
extends: []
related:
  - note-durational-values
  - meter-and-time-signatures
  - rhythm
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a beat in music?"
  - "What is tempo and how is it measured?"
  - "What is the mathematical relationship between tempo and note duration?"
---

# Quick Definition

Beats are the temporal units by which music is notated and counted, and tempo is the frequency of these beats, usually measured in beats per minute.

# Core Definition

Time durations in music are measured in *beats*, which are the temporal units by which music is notated. Frequently one beat represents the time interval by which one would "count off" the passing of time during performance. *Tempo* is the frequency of this count-off, measured in beats per minute (BPM). Music is not always performed at constant tempo; compositions may have internal tempo changes or passages performed ad lib or with rubato, where strict tempo gives way to artistic liberty (Wright, p. 30).

# Prerequisites

- **Horizontal Structure** — Beats and tempo are fundamental to temporal organization

# Key Properties

1. The beat establishes a unit of measurement for the time axis
2. Tempo defines the scaling between musical time (beats) and physical time
3. At tempo $T$ BPM, one beat lasts $60/T$ seconds
4. The time signature determines which note value receives one beat
5. Tempo is not always constant — rubato allows artistic flexibility

# Construction / Recognition

## To convert between beats and physical time:

1. Identify the tempo in BPM
2. One beat = $60/T$ seconds
3. Duration in seconds = duration in beats $\times (60/T)$

# Context & Application

The beat is the pulse that listeners tap their feet to and performers count internally. Common tempos range from about 40 BPM (Grave) to 200+ BPM (Prestissimo). Metronome markings specify exact tempos. The mapping from notated duration to physical duration is linear: $d_{\text{physical}} = d_{\text{beats}} \cdot (60/T)$.

# Examples

- At tempo 120 BPM, one beat lasts $60/120 = 0.5$ seconds (p. 30)
- At tempo 60 BPM, one beat lasts exactly 1 second
- A quarter note at 120 BPM in $\frac{4}{4}$ time lasts 0.5 seconds
- Rubato: a performer might slow down approaching a cadence, then resume tempo (p. 30)

# Relationships

## Builds Upon
- **Horizontal Structure** — Beats are the basic temporal unit

## Enables
- **Note Durational Values** — Notes are measured in beats
- **Meter and Time Signatures** — Meter organizes beats into measures
- **Rhythm** — Rhythm fills beats with specific patterns

# Common Errors

- **Error**: Assuming the quarter note always gets one beat
  **Correction**: Which note gets one beat depends on the time signature's bottom number

# Common Confusions

- **Confusion**: Conflating beat (unit) with tempo (rate)
  **Clarification**: The beat is the unit of time; tempo is the speed at which beats occur
- **Confusion**: Thinking rubato means "no tempo"
  **Clarification**: Rubato means flexible, temporary deviations from a prevailing tempo

# Source Reference

Chapter 2: "Horizontal Structure", "Duration of Notes" section, p. 30 (PDF).

# Verification Notes

- Definition source: Direct from source, p. 30
- Confidence rationale: High — explicit definition with physical-time conversion
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: $60/T$ conversion formula, rubato definition, BPM range examples
