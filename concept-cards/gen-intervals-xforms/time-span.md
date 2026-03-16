---
# === CORE IDENTIFICATION ===
concept: Time Span
slug: time-span

# === CLASSIFICATION ===
category: timbral-temporal-systems
subcategory: rhythmic-structures
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS; Some Timbral GIS Models"
chapter_number: 4
pdf_page: 91
section: "4.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "TMSPS element"
  - "(a, x)"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-interval-system
extends: []
related:
  - time-span-gis
  - commutative-time-span-gis
  - time-span-interval-group
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a time span in Lewin's theory?"
  - "How does the time-span GIS relate to simpler time-point and duration GIS structures?"
---

# Quick Definition
A time span is an ordered pair (a, x) modeling a musical event that "begins at time a" and "extends x units of time" thereafter, combining location and extension into a single mathematical object.

# Core Definition
Definition 4.1.1: A time span is an ordered pair (a, x) where a is any real number (attack time) and x is any positive real number (duration). The family of all time spans is denoted TMSPS (Lewin, Definition 4.1.1, p. 91).

# Prerequisites
- **Generalized Interval System** — Time spans become elements of GIS structures

# Key Properties
1. a can be any real number (including negative, for events before time zero)
2. x must be strictly positive (events have non-zero duration)
3. TMSPS = {(a, x) : a in R, x in R+}
4. Time spans combine location (when) with extension (how long)
5. The definition does not fix a time unit — both a and x are measured in some implicit unit

# Construction / Recognition
## To Construct:
1. Identify the attack time a (relative to some time-point zero)
2. Identify the duration x (in some time unit)
3. Form the pair (a, x)

## To Recognize:
1. An ordered pair where first component is real (any sign) and second is positive real
2. Represents a musical event with both temporal location and duration

# Context & Application
Time spans capture both when something happens and how long it lasts, unlike time-points (location only) or durations (extension only). They are the elements of Lewin's non-commutative GIS for rhythm, designed for analyzing music where multiple local tempi coexist. The concept was first developed in Lewin's "On Formal Intervals between Time-Spans" (Music Perception, 1984).

# Examples
**Example 1** (p. 91): Basic time spans:
- (0, 1): event starting at time 0, lasting 1 unit
- (3.5, 2): event starting at time 3.5, lasting 2 units

**Example 2** (Figure 4.4, p. 74): Two pairs of time spans at different tempi, where s_1 and t_1 are in a slow instrument and s_2 and t_2 are in a fast instrument.

# Relationships
## Builds Upon
- **Generalized Interval System** — time spans serve as elements of GIS structures

## Enables
- **Time-Span GIS** — the non-commutative GIS on TMSPS
- **Commutative Time-Span GIS** — the simpler commutative GIS on TMSPS
- **Time-Span Interval Group** — the group of intervals between time spans

# Common Errors
- **Error**: Setting x = 0 for a time-point
  **Correction**: Duration must be strictly positive. Use a separate time-point space for instantaneous events.

# Common Confusions
- **Confusion**: Thinking time spans are time intervals
  **Clarification**: A time span (a, x) is an element of S (the space). An interval between time spans is a different object in IVLS.

- **Confusion**: Assuming the time unit is fixed
  **Clarification**: The definition does not specify a unit. The entire theory (GIS 4.1.3) is developed to be independent of unit choice.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Definition 4.1.1, p. 91.

# Verification Notes
- Definition source: direct from Definition 4.1.1
- Confidence rationale: high — explicit definition
- Re-extraction notes: Re-extracted from v2 card; preserved: basic examples, confusion about time spans vs intervals, note about unit independence
