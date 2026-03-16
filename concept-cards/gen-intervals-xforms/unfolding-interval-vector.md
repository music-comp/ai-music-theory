---
concept: Unfolding Interval Vector
slug: unfolding-interval-vector

category: generalized-interval-systems
subcategory: rhythmic-analysis
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 72
section: "3.3"

extraction_confidence: high

aliases:
  - "progressive interval vector"

prerequisites:
  - gis3-interval
  - direct-product-gis
extends: []
related:
  - ictus
  - mensural-structure
contrasts_with: []

answers_questions:
  - "How does the interval content of a set develop note-by-note over time?"
  - "When do intervallic patterns first become perceptible to a listener?"
  - "How does the unfolding interval vector reveal the emergence of structure?"
---

# Quick Definition
The unfolding interval vector tracks how the interval content of a set develops note-by-note as elements are added through time, revealing the temporal process by which intervallic patterns emerge for a listener.

# Core Definition
Given a temporally ordered sequence of elements entering at successive time-points, the unfolding interval vector is a progressive record of all intervals that become available as each new element is added. Each new element contributes new intervals to all previously heard elements. In a direct-product GIS, each interval is a compound pair (pitch-class interval, temporal interval), tracking both dimensions simultaneously.

# Prerequisites
- **GIS3-interval** — The compound intervals tracked by the vector
- **Direct-product GIS** — The framework in which compound intervals are computed

# Key Properties
1. Only "forward" intervals (earlier to later) are counted
2. The vector grows cumulatively: it records all intervals heard so far
3. After element n arrives, the vector contains all int(s_i, s_j) where i < j <= n
4. In compound GIS, recurrence means the same pair, not just same component
5. The moment of first predominance signals structural emergence (the ictus)

# Construction / Recognition
## To Construct:
1. Order elements chronologically: s_1, s_2, s_3, ...
2. After s_1: no intervals
3. After s_2: add int(s_1, s_2)
4. After s_3: add int(s_1, s_3) and int(s_2, s_3)
5. After s_n: add int(s_k, s_n) for all k < n
## To Recognize:
1. A cumulative inventory of intervals growing with each new musical event
2. A tool for identifying when patterns first emerge

# Context & Application
The unfolding interval vector models the listener's developing perception of intervallic structure. It addresses the question: at what point during the music do we first hear enough to perceive a pattern? Its key application is determining when mensural structure becomes perceptible (the "ictus").

# Examples
**Example 1** (Figure 3.3, pp. 72-75): Webern Piano Variations unfolding:
- After (Eb, 0): no intervals
- After (B, 3): {(8, 3)}
- After (Bb, 4): {(8, 3), (7, 4), (11, 1)}
- After (D, 5): {(8, 3), (7, 4), (11, 1), (11, 5), (3, 2), (4, 1)}
- At time-point 5: first recurrence of pitch-class interval 11; proposed as first ictus

**Example 2** (Figure 3.5, p. 75): After (C#, 10): the GIS3-interval (11, 5) recurs for the first time -- the first recurring compound interval. After (C, 11): (11, 1) and (2, 7) recur.

# Relationships
## Builds Upon
- **GIS3-interval** — The compound intervals that populate the vector
- **Direct-product GIS** — The framework for computing compound intervals
## Enables
- **Ictus** — The moment when the vector first shows predominance
- **Mensural structure** — Revealed by the recurrence patterns in the vector
## Related
- **Interval vector** — The static (non-temporal) analog

# Common Errors
- **Error**: Including backward intervals (later to earlier) in the vector
  **Correction**: Only forward intervals (int(s_i, s_j) with i < j) are counted

# Common Confusions
- **Confusion**: The unfolding vector is a snapshot at one moment
  **Clarification**: It is a cumulative record that grows; it captures the developing perception over time
- **Confusion**: Recurrence of one component (e.g., pitch-class interval 11) implies recurrence of the compound interval
  **Clarification**: Compound recurrence requires the same pair (e.g., (11, 5)); component recurrence alone is less specific

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Figure 3.3 and discussion, pages 72-77.

# Verification Notes
- Definition source: Synthesized from Lewin's analytical demonstration
- Confidence rationale: High -- procedure is clearly demonstrated even if not given a formal definition
- Re-extraction notes: Re-extracted from v2 card; preserved: Webern unfolding data, compound recurrence distinction, ictus connection
