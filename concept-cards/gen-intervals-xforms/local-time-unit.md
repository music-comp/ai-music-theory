---
concept: Local Time Unit
slug: local-time-unit

category: timbral-temporal-systems
subcategory: rhythmic-structures
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS; Some Timbral GIS Models"
chapter_number: 4
pdf_page: 91
section: "4.1"

extraction_confidence: medium

aliases:
  - "locally referential time-unit"
  - "local beat"
  - "local tempo"

prerequisites:
  - time-span-gis
  - referential-time-unit-problem
extends: []
related:
  - carter-string-quartet-analysis
  - stockhausen-klavierstuck-xi
contrasts_with: []

answers_questions:
  - "What is a local time unit?"
  - "How do multiple local time units coexist in a texture?"
---

# Quick Definition
A local time unit is a duration that serves as a referential measure for a particular instrument, voice, or temporal region within a composition, without necessarily governing the entire texture. The time-span GIS handles multiple local units automatically.

# Core Definition
In music with multiple tempo layers, each layer has its own locally referential time unit. Any time span's duration x may serve as the local unit for measuring other spans in the same context. The interval int((a, x), (b, y)) = ((b-a)/x, y/x) uses x as the measuring unit, so the time-span GIS automatically treats each first span as locally referential. Lewin emphasizes that asking "which is THE beat?" is the wrong question — one should ask "what are the local units and how do they relate?" (Lewin, pp. 98-107).

# Prerequisites
- **Time-Span GIS** — The GIS that naturally accommodates local units
- **Referential Time-Unit Problem** — Motivates the concept of local rather than global units

# Key Properties
1. Each instrument/layer may have its own local time unit
2. The time-span GIS uses x (the first span's duration) as the measuring rod
3. Players should feel their local beat internally, not subordinate to a possibly nonexistent global beat
4. "Foot-tapping" a global beat distorts lyric lines with autonomous local tempi

# Examples
**Example 1** (pp. 99-100): Carter, mm. 22-32: viola at MM180, cello at MM48, first violin at MM36, second violin at MM96 — each with its own local time unit.

**Example 2** (p. 101): "These lyric lines are not syncopated... Rather, each line has its own autonomous local time-unit, with respect to which it should project an essentially 'first-species' character."

# Relationships
## Builds Upon
- **Time-Span GIS** — naturally accommodates local units
- **Referential Time-Unit Problem** — motivates the concept

## Related
- **Carter String Quartet Analysis** — the primary example
- **Stockhausen Klavierstuck XI** — another example with local units per group

# Common Confusions
- **Confusion**: Thinking local time units create "syncopation" against a global beat
  **Clarification**: Without a global beat, there is no syncopation — each line has its own first-species character relative to its local unit

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, pp. 98-107.

# Verification Notes
- Definition source: synthesized from extended discussion
- Confidence rationale: medium — concept discussed at length but not formally defined
- Re-extraction notes: Re-extracted from v2 card; preserved: Carter example, first-species character observation, performance implications
