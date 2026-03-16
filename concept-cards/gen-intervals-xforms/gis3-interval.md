---
concept: GIS3-Interval
slug: gis3-interval

category: generalized-interval-systems
subcategory: direct-product-gis
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 68
section: "3.3"

extraction_confidence: high

aliases:
  - "compound interval"
  - "direct-product interval"

prerequisites:
  - direct-product-gis
extends: []
related:
  - unfolding-interval-vector
  - mensural-structure
  - ictus
contrasts_with: []

answers_questions:
  - "How can pitch and temporal intervals be tracked simultaneously?"
  - "What is a compound interval in a direct-product GIS?"
  - "How does recurrence of GIS3-intervals reveal compositional structure?"
---

# Quick Definition
A GIS3-interval is a compound interval in a direct-product GIS combining pitch-class intervals and temporal intervals. Recurrent GIS3-intervals reveal compositional associations between pitch and rhythmic structure.

# Core Definition
In GIS3 = GIS1 x GIS2 (pitch-class x time-point), elements are (pitch-class, time-point) pairs and intervals are (pitch-class interval, temporal interval) pairs (Example 3.3.1). A GIS3-interval (n, t) signifies pitch-class interval n and temporal interval t beats between two events. When a specific GIS3-interval recurs across multiple pairs of elements, it creates a compositional association binding pitch structure to temporal structure.

# Prerequisites
- **Direct-product GIS** — GIS3 is constructed as the direct product of a pitch-class GIS and a time-point GIS

# Key Properties
1. GIS3-intervals are ordered pairs (pitch-class interval, temporal interval)
2. The group operation is componentwise: (n1, t1) * (n2, t2) = (n1 + n2, t1 + t2)
3. Recurrence of a GIS3-interval is more specific than recurrence of either component alone
4. GIS3-intervals capture cross-domain associations invisible to single-domain analysis

# Construction / Recognition
## To Construct:
1. Choose a pitch-class GIS (e.g., Z/12Z) and a time-point GIS (e.g., integers under addition)
2. Form the direct product: S3 = S1 x S2, IVLS3 = IVLS1 x IVLS2
3. Compute int3((p1, t1), (p2, t2)) = (p2 - p1 mod 12, t2 - t1)
## To Recognize:
1. Look for pairs of musical events sharing the same compound (pitch, time) interval
2. Identify recurrent GIS3-intervals across the texture

# Context & Application
GIS3-intervals capture how pitch and temporal structure work together in composition. When a specific GIS3-interval recurs, certain pitch intervals become bound to certain temporal intervals, revealing thematic relationships that span dimensions. Lewin applies this in his analysis of Webern's Piano Variations op. 27 (Figures 3.1-3.6).

# Examples
**Example 1** (Figures 3.1-3.3, pp. 68-77): Webern Piano Variations, recurrent GIS3-intervals:
- (11, 1): B-Bb, C#-C, A-G# -- pitch-class 11 bound to the beat
- (11, 5): Eb-D, D-C# -- pitch-class 11 associated with 5-beat spans
- (3, 2): B-D, E-G -- pitch-class 3 at 2-beat intervals
- (2, 7): connects accompaniment figures

**Example 2** (Figure 3.4, p. 74): The GIS3-interval (11, 5) spanning from the opening Eb to D at the first ictus acquires structural significance as a thematic-mensural association, suggesting a "5/4 meter" hearing.

# Relationships
## Builds Upon
- **Direct-product GIS** — GIS3 is the formal construction combining two GIS structures
## Enables
- **Unfolding interval vector** — Tracks GIS3-intervals as they accumulate in time
- **Mensural structure** — Recurrent temporal components of GIS3-intervals define perceived meter
- **Ictus** — The moment when GIS3-interval patterns first become perceptible
## Related
- **Webern Piano Variations analysis** — The primary analytical application

# Common Errors
- **Error**: Treating a GIS3-interval as a product of its components (e.g., (11, 5) as 55)
  **Correction**: A GIS3-interval is an ordered pair; the components are independent dimensions

# Common Confusions
- **Confusion**: Recurrence of pitch-class interval 11 and recurrence of temporal interval 5 separately imply recurrence of (11, 5)
  **Clarification**: A GIS3-interval recurrence requires the same pair; both components must co-occur in the same element pairs
- **Confusion**: The "3" in GIS3 indicates a mathematical property
  **Clarification**: It is simply Lewin's label for the third GIS in his exposition, combining GIS1 (pitch-class) and GIS2 (time-point)

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Example 3.3.1 and Figures 3.1-3.6, pages 68-77.

# Verification Notes
- Definition source: Direct from Example 3.3.1
- Confidence rationale: High -- clearly defined and extensively illustrated
- Re-extraction notes: Re-extracted from v2 card; preserved: Webern analysis details, recurrence examples, mensural interpretation
