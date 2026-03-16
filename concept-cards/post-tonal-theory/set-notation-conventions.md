---
concept: Set Notation Conventions
slug: set-notation-conventions
category: set-theory
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: null
extraction_confidence: high
aliases:
  - bracket conventions
  - set notation
prerequisites:
  - pitch-class-set
extends:
  - pitch-class-set
related:
  - normal-form
  - prime-form
  - interval-class-vector
contrasts_with: []
answers_questions:
  - "What do the different bracket types mean in set notation?"
  - "How are pitch-class sets, normal forms, and prime forms notated?"
---

# Quick Definition
Set notation conventions use different bracket types to distinguish between unordered sets, ordered sets (normal form), prime forms, and interval-class vectors.

# Core Definition
Post-tonal theory employs consistent notation conventions to indicate the status of pitch-class collections. Each bracket type signals a different level of abstraction: curly braces for raw unordered collections, square brackets for normal form, parentheses for prime form, and angle brackets for interval-class vectors.

# Prerequisites
- **Pitch-class set** -- the collections being notated

# Key Properties
1. **Curly braces {}**: Unordered pitch-class sets -- {C, E, G} or {0, 4, 7}
2. **Square brackets []**: Normal form (ordered) -- [C, E, G] or [0, 4, 7]
3. **Parentheses ()**: Prime form -- (037), always starting on 0
4. **Angle brackets <>**: Interval-class vectors -- <001110>
5. Commas between elements except in prime form
6. T and E stand for 10 and 11 in prime form's compact format

# Construction / Recognition
When notating:
- Use {} when listing pitch classes with no particular ordering
- Use [] when presenting normal form
- Use () when identifying a set class by its prime form
- Use <> when giving the interval-class vector
- In prime form, omit commas: (014), (0126), (01369)
- Use sc(prime form) to explicitly name the set class: sc(014)

# Context & Application
Consistent notation prevents confusion about whether a collection's ordering is significant. When analyzing, curly braces indicate raw pitch-class content; square brackets show the set has been put into normal form for comparison; parentheses identify the abstract set class.

# Examples
The same musical idea notated at different levels:
- Raw collection: {G, G#, B} or {7, 8, 11}
- Normal form: [G, G#, B] or [7, 8, 11]
- Set class: (014) or sc(014)
- Interval-class vector: <101100>

**Example 2-33** (p. 83): Sets identified with their set classes -- e.g., the sets from Webern's op. 24 are all members of sc(014).

# Relationships
## Builds Upon
- **Pitch-class set** -- the fundamental object being notated
## Enables
- **Normal form** -- square bracket notation
- **Prime form** -- parenthesis notation
- **Set class** -- sc() notation
## Related
- **Interval-class vector** -- angle bracket notation

# Common Errors
- **Error**: Using parentheses for normal form or square brackets for prime form. **Correction**: Parentheses are reserved for prime form (always starting on 0); square brackets for normal form (specific pitch classes).

# Common Confusions
- **Confusion**: Some sources use different conventions. **Clarification**: Always verify the conventions used in any source you consult. This book uses [] for normal form and () for prime form consistently.

# Source Reference
Chapter 2: Pitch-Class Sets, Sections 2.1--2.8.

# Verification Notes
- Definition source: synthesized from conventions used throughout chapter
- Confidence rationale: conventions consistently applied throughout the textbook
- Re-extraction notes: preserved old card's notation table; upgraded to v3 template
