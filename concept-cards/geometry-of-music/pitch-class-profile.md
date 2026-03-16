---
concept: Pitch-Class Profile
slug: pitch-class-profile

category: analysis
subcategory: centricity
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Macroharmony and Centricity"
chapter_number: 5
pdf_page: 188
section: "5.6"

extraction_confidence: high

aliases:
  - "Krumhansl-Shepard profile"
  - "pitch-class distribution"
  - "tonal hierarchy"

prerequisites:
  - rootedness
  - centricity-and-tonicity
extends: []
related:
  - generalized-theory-of-keys
  - macroharmony
contrasts_with: []

answers_questions:
  - "What are pitch-class profiles (Krumhansl-Shepard)?"
  - "How can we represent centricity formally?"
  - "How do modes differ from one another?"
---

# Quick Definition
A pitch-class profile is a bar graph assigning prominence values to each of the twelve pitch classes, representing the relative importance, stability, or salience of each note in a chord, mode, or key.

# Core Definition
Pitch-class profiles (originating with Krumhansl and Shepard 1979) are bar graphs with pitch classes on the x-axis and subjective prominence on the y-axis. They come in several forms. A binary (two-tiered) profile simply marks notes as in or out of a chord. A three-tiered profile distinguishes tonal center (value 2), non-centric scale notes (value 1), and non-scale notes (value 0) — sufficient to represent musical modes. More refined profiles use four or five tiers to capture subtler hierarchies, such as distinguishing tonic, dominant, mediant, remaining diatonic notes, and chromatic notes. In the limit, continuous profiles assign each pitch class a real-valued prominence, blurring the boundary between "inside" and "outside" the macroharmony. Tymoczko distinguishes pitch-class profiles (subjective judgments of importance) from pitch-class distributions (statistical note frequencies), noting they often correlate but are not identical.

# Prerequisites
- **rootedness** — Profiles represent rootedness at the chord level
- **centricity-and-tonicity** — Profiles represent tonicity at the key level

# Key Properties
1. Bar graph with 12 positions (one per pitch class) and prominence values on y-axis
2. Two-tiered (binary): notes are in or out of a chord
3. Three-tiered: distinguishes center, scale member, and non-member — sufficient for mode representation
4. Four/five-tiered: finer gradations (tonic > dominant > mediant > diatonic > chromatic)
5. Continuous: real-valued prominence with no sharp boundaries
6. Pitch-class distributions (statistical frequencies) often approximate subjective profiles

# Construction / Recognition
## To Construct/Create:
1. Assign prominence values to each pitch class based on musical context
2. Use higher values for more stable/prominent notes
3. For modes: assign 2 to tonic, 1 to other scale notes, 0 to non-scale notes
4. For compositional use: treat profiles as probability tables for note selection
## To Identify/Recognize:
1. Listen for which notes feel most stable/prominent
2. Count pitch-class frequencies as an approximate guide to the profile
3. Match against known profiles for familiar keys and modes

# Context & Application
Tymoczko uses pitch-class profiles both analytically and compositionally. Analytically, they can reveal different interpretations of the same key: the three soloists on "Freedom Jazz Dance" produce very different pitch-class distributions over the same Bb ostinato, suggesting different understandings of the key (Bb dorian, Bb mixolydian, and Bb octatonic). Compositionally, Tymoczko uses profiles as probability tables for computer-generated music and as guides for improvisation. He finds "an enormous amount of unexplored musical territory" in the space between traditional tonality and atonality, which profiles help visualize and navigate.

# Examples
**Example 1** (p. 189, Figure 5.6.6): Three-tiered profiles for C lydian, G ionian, C phrygian, G locrian — all sharing the same white-note macroharmony but with different tonal centers.

**Example 2** (p. 192, Figure 5.6.8): A five-tiered profile for C major: C (5), G (4), E (3), remaining diatonic notes (2), chromatic notes (1) — resembling Lerdahl's "basic space."

**Example 3** (p. 193-194, Figure 5.6.11): Pitch-class distributions for Miles Davis, Wayne Shorter, and Herbie Hancock soloing over the same Bb ostinato in "Freedom Jazz Dance" — Davis suggests Bb dorian, Shorter Bb mixolydian, Hancock something closer to Bb octatonic.

# Relationships
## Builds Upon
- **rootedness** — Profiles represent rootedness for chords
- **centricity-and-tonicity** — Profiles represent tonicity for keys
## Enables
- **generalized-theory-of-keys** — Profiles provide the framework for generalized keys
## Related
- **macroharmony** — The collection from which profiles are constructed
## Contrasts With
- None specifically

# Common Errors
- **Error**: Treating pitch-class profiles as identical to pitch-class distributions
  **Correction**: Profiles represent subjective prominence; distributions are statistical counts. They correlate but differ — G may appear more frequently than C in C major, but C is still the tonic.

# Common Confusions
- **Confusion**: Assuming a three-tiered profile is always sufficient
  **Clarification**: Many musical contexts require finer distinctions — e.g., E phrygian with emphasis on the 5th vs. 4th scale degree requires at least four tiers to represent the difference

# Source Reference
Chapter 5: Macroharmony and Centricity, Section 5.6, pages 188-196, Figures 5.6.3-5.6.14. Originates with Krumhansl and Shepard 1979.

# Verification Notes
- Definition source: Section 5.6, with extensive discussion and examples
- Confidence rationale: High — explicitly defined with multiple levels of refinement
- Cross-reference status: Krumhansl and Shepard 1979 cited; Lerdahl's "basic space" discussed in Appendix E
