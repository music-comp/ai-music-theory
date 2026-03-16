---
concept: Time-Spanning Network
slug: time-spanning-network

category: transformation-theory
subcategory: graph-network-extensions
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.7.6"

extraction_confidence: medium

aliases: []

prerequisites:
  - transformation-network-definition
  - time-span
extends:
  - transformation-network-definition
related:
  - carriage-return-function
  - precedence-ordered-system
contrasts_with: []

answers_questions:
  - "How can a transformation network model when musical events occur?"
  - "What is a time-spanning network?"
---

# Quick Definition
A time-spanning network is a transformation network augmented with a TIMESPAN function mapping each node to a time span, allowing the network to model both transformational relationships and the temporal locations of musical events.

# Core Definition
A time-spanning network is a transformation network together with a function TIMESPAN that maps each node into a certain time span. The musical event corresponding to the contents of that node occurs over that time span. TIMESPAN(N) can model the exact time span over which CONTENTS(N) occurs, or a range of time during which CONTENTS(N) might occur (Lewin, Section 9.7.6, p. 248).

An alternative approach embeds time spans directly in CONTENTS: instead of a node N with CONTENTS(N) = Gb and TIMESPAN(N) = (3.5, 0.5), one has CONTENTS(N) = (Gb, (3.5, 0.5)). This requires a more complex family of transformations incorporating both object-transformations and time-span-transformations.

# Prerequisites
- **Transformation network** — the base structure being augmented
- **Time span** — the temporal objects assigned to nodes

# Key Properties
1. Adds temporal information without changing the transformational content
2. Two approaches: separate TIMESPAN function, or complex CONTENTS with pairs
3. Addresses discrepancies between precedence ordering and musical chronology
4. Makes explicit when formal input/output nodes are actually heard

# Construction / Recognition
## To Construct:
1. Build a transformation network
2. Add a TIMESPAN function mapping each node to a time span (onset, duration)
## To Recognize:
1. Identify a transformation network with temporal information attached to nodes

# Context & Application
Time-spanning networks address the gap between formal graph structure and musical time. When a precedence-ordered network's input nodes are not heard first, the TIMESPAN function makes this discrepancy explicit and analyzable.

# Examples
**Example 1** (Figure 9.15, p. 248): Beethoven Appassionata. Each node of Figure 9.14(b) receives a time span. The Gb node has TIMESPAN = (3.5, 0.5), showing that it occurs after Db despite being a formal input.

# Relationships
## Builds Upon
- **Transformation network** — the base structure
## Related
- **Carriage return function** — time-spanning networks make carriage returns explicit
- **Precedence-ordered system** — temporal information reveals chronology-precedence mismatches

# Common Errors
- **Error**: Conflating TIMESPAN with CONTENTS
  **Correction**: In the basic approach, TIMESPAN is a separate function; in the alternative, it is embedded in CONTENTS but requires more complex transformations

# Common Confusions
- **Confusion**: Thinking TIMESPAN changes the transformational structure
  **Clarification**: TIMESPAN adds information to the network; the graph and its TRANSIT labels remain unchanged

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.7.6, pp. 247-249. See Figure 9.15.

# Verification Notes
- Definition source: synthesized from Section 9.7.6 discussion (not a numbered definition)
- Confidence rationale: medium -- concept described in context but not formally defined with a number
- Re-extracted from v2 card; preserved: two approaches, Appassionata example
