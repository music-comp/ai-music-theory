---
concept: Interval Vector
slug: interval-vector
category: analysis
subcategory: set-theory
tier: advanced
source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "Set Class and Prime Form"
chapter_number: 8
pdf_page: null
section: "VIII.4"
extraction_confidence: high
aliases:
  - "ic vector"
prerequisites:
  - interval-class
  - set-class
extends: []
related:
  - z-relation
  - forte-number
contrasts_with: []
answers_questions:
  - "What is an interval vector?"
  - "How do you calculate an interval vector?"
  - "Why is the interval vector important for set-class identification?"
---

# Quick Definition
An interval vector is a six-digit summary of a set class's interval-class content, listing the number of occurrences of each interval class (ic 1 through ic 6) in angle brackets, e.g., <1,0,1,1,1,0> for set class (014).

# Core Definition
The interval vector counts all possible intervals between pairs of pitch classes in a set, categorized by interval class. It has six positions corresponding to ic 1 through ic 6. All members of the same set class share the same interval vector, making it a defining characteristic. The vector reveals the set's "sonic fingerprint" -- its distribution of semitones, whole tones, thirds, fourths, and tritones. Z-related set classes (exceptionally) share the same interval vector despite being different set classes.

# Prerequisites
- Interval class and set class

# Key Properties
1. Six digits in angle brackets: <ic1, ic2, ic3, ic4, ic5, ic6>
2. All members of a set class share the same interval vector
3. Total count depends on cardinality: for n pcs, n(n-1)/2 intervals
4. Identifies the "sonic color" of a set class
5. Z-related pairs share the same vector despite different prime forms

# Context & Application
The interval vector is listed in set class tables alongside prime form and Forte number. It explains why members of the same set class sound similar.

# Examples
**Example 1**: Major triad (037): intervals are M3(ic4), m3(ic3), P5(ic5). Vector: <0,0,1,1,1,0>.
**Example 2**: (014): intervals are m2(ic1), M3(ic4), m3(ic3). Vector: <1,0,1,1,0,0>.
**Example 3**: Chromatic trichord (012): intervals are m2(ic1), M2(ic2), m2(ic1 again -- wait: ic1+ic1+ic2). Vector: <2,1,0,0,0,0>.

# Relationships
## Builds Upon
- **interval-class** -- The vector counts interval classes
- **set-class** -- The vector characterizes a set class
## Related
- **z-relation** -- Z-related pairs share the same vector

# Common Confusions
- **Confusion**: The interval vector specifies the intervals in order
  **Clarification**: It counts how many of each ic, not the order in which they appear

# Source Reference
Open Music Theory, Part VIII, Chapter 4: "Set Class and Prime Form," set class table reference.

# Verification Notes
- Definition source: Referenced in 08-04
- Confidence rationale: High
- Preserved from v2: Definition, examples
- Cross-reference status: Verified
