---
concept: Pitch-Class Intervals (Paths)
slug: pitch-class-interval

category: geometric-theory
subcategory: measurement
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.2"

extraction_confidence: high

aliases:
  - "paths in pitch-class space"
  - "directed pitch-class interval"

prerequisites:
  - pitch-class-space
extends:
  - pitch-class-space
related:
  - distance-in-music
  - voice-leading-in-pitch-class-space
  - transposition
contrasts_with: []

answers_questions:
  - "What is a pitch-class interval?"
  - "What is pitch-class space?"
---

# Quick Definition
In Tymoczko's framework, pitch-class intervals are modeled as directed paths on the pitch-class circle, combining an initial pitch class, a direction (ascending/descending), and a distance.

# Core Definition
Tymoczko models intervals in pitch-class space as paths on the pitch-class circle rather than as undirected distances. A path specifies a starting pitch class, a direction (clockwise/ascending or counterclockwise/descending), and a distance. The notation C +4-> E represents a four-semitone ascending (clockwise) path from C to E, while C -8-> E represents an eight-semitone descending (counterclockwise) path. Paths can even wrap around the circle one or more times. This is an innovation over traditional pitch-class intervals, which specify only starting and ending points without direction, and cannot distinguish the closely related progressions that differ only in the direction of motion.

# Prerequisites
- **pitch-class-space** — Paths are defined on the pitch-class circle

# Key Properties
1. Combines starting point, direction, and distance
2. Distinguished from traditional pitch-class intervals (which lack direction)
3. Can wrap around the circle one or more times
4. Capture the difference between ascending and descending motion to the same pitch class
5. Correspond to particular ways of moving from one pitch class to another
6. Central to the definition of voice leadings in pitch-class space

# Construction / Recognition
## To Construct/Create:
1. Choose a starting pitch class
2. Choose a direction (ascending/clockwise or descending/counterclockwise)
3. Choose a distance (in semitones, possibly greater than 12)
## To Identify/Recognize:
1. Check whether an interval specifies not just two pitch classes but a direction of motion
2. Ascending C to E by 4 semitones is a different path from descending C to E by 8 semitones

# Context & Application
Paths in pitch-class space are essential for defining voice leadings between pitch-class sets. The numbers above voice-leading arrows (e.g., +5, 0, +1, -2, -1) are paths describing how each voice moves. The convention is that when a voice moves by shortest path (within -6 to +6 semitones), the path numbers can be omitted, with tritones ascending by convention.

# Examples
**Example 1** (p. 50, Fig 2.2.2): Three passages moving C to E by four ascending semitones are similar (C +4-> E); passages moving C to E by eight descending semitones (C -8-> E) are different but also mutually similar.

# Relationships
## Builds Upon
- **pitch-class-space** — Paths exist on the pitch-class circle
## Enables
- **voice-leading-in-pitch-class-space** — Defined as collections of paths
## Related
- **distance-in-music** — Paths generalize the concept of distance to include direction
- **transposition** — Transposition by x semitones is a path of length x
## Contrasts With
- Traditional (undirected) pitch-class intervals that specify only magnitude

# Common Errors
- **Error**: Assuming all paths from C to E are equivalent
  **Correction**: C +4-> E (4 ascending) and C -8-> E (8 descending) are different paths representing different musical motions

# Common Confusions
- **Confusion**: Confusing paths with distances
  **Clarification**: Distance is the shortest unsigned length between two pitch classes; a path has direction and can be longer than the shortest distance

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.2, pages 49-51.

# Verification Notes
- Definition source: Direct from Section 2.2, which introduces paths as an innovation
- Confidence rationale: High — explicitly defined and contrasted with traditional intervals
- Cross-reference status: Verified; essential to the voice-leading framework
