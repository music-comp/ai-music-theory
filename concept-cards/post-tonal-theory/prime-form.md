---
concept: Prime Form
slug: prime-form
category: set-theory
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.7 Prime Form"
extraction_confidence: high
aliases:
  - canonical form
prerequisites:
  - normal-form
  - set-class
extends:
  - normal-form
related:
  - prime-form-algorithm
  - list-of-set-classes
  - forte-names
contrasts_with:
  - normal-form
answers_questions:
  - "What is prime form?"
  - "How does prime form identify a set class?"
  - "How do I determine the prime form of a set?"
---

# Quick Definition
Prime form is the standardized representation of a set class, beginning with 0 and most packed to the left.

# Core Definition
Set classes are identified by their *prime form*, a representation that begins with 0 and is most packed to the left (smallest intervals first). Prime forms are written in parentheses with no commas separating the elements, using T and E for 10 and 11. For example, sc(014), sc(0134), sc(016). "Using *sc* as an abbreviation for set class, we would say that each of those sets is a member of sc(014) or, more familiarly, that each of those sets 'is a (014)'" (Straus, Ch. 2).

# Prerequisites
- **Normal form** -- prime form is derived from normal form
- **Set class** -- prime form is the identifier for a set class

# Key Properties
1. Always begins with 0
2. Most packed to the left (smallest intervals first)
3. Written in parentheses without commas: (014), (0126)
4. T = 10, E = 11 in compact format
5. All members of a set class share the same prime form
6. Each prime form appears exactly once in the List of Set Classes

# Construction / Recognition
See **prime-form-algorithm** for the full procedure. In brief:
1. Put set in normal form
2. Extract the interval succession
3. Compare reading left-to-right vs. right-to-left
4. Choose whichever is more packed to the left
5. Build the prime form starting from 0 using that interval succession

**Clockface shortcut**: Find the widest gap. Try assigning 0 to each end of the gap and reading in each direction. Choose the version with smaller numbers toward the left.

# Context & Application
Prime form is the standard identifier for a set class, enabling cross-referencing across pieces, composers, and analytical literature. When we say a collection "is a (014)," we mean it belongs to the set class whose prime form is (014). The List of Set Classes catalogs all prime forms.

# Examples
**Example 2-33** (p. 83): Sets and their set classes:
- {D#, E, G}, {D, D#, F#}, {Ab, A, C}, {B, C, Eb} -- all sc(014)
- {G, Ab, Bb, B}, {Bb, B, Db, D} -- sc(0134)
- {C#, E, F#}, {A, B, D}, {C, Eb, F} -- sc(025)
- {D, Eb, Ab}, {F, Gb, B} -- sc(016)
- {C, E, F}, {D, Eb, G} -- sc(015)
- {G#, A, Bb, B, D} etc. -- sc(01236)

**Example 2-34** (p. 83): Three worked examples:
- [C#, F, F#, G]: intervals 4-1-1; reversed: 1-1-4 (more packed left); prime form **(0126)**
- [Bb, D, F, F#]: intervals 4-3-1; reversed: 1-3-4; prime form **(0148)**
- [F, F#, A]: intervals 1-3 (already packed left); prime form **(014)**

# Relationships
## Builds Upon
- **Normal form** -- the starting point for computing prime form
- **Set class** -- prime form is the canonical label for a set class
## Enables
- **List of Set Classes** -- organized by prime form
- **Cross-piece comparison** -- same prime form = same set class regardless of key or context
## Related
- **Prime form algorithm** -- the procedure for computing prime form
- **Forte names** -- alternative labeling (3-11 vs. (037))
## Contrasts With
- **Normal form** -- normal form preserves specific pitch classes; prime form always starts on 0 and identifies the abstract type

# Common Errors
- **Error**: Forgetting to compare both directions. **Correction**: Always compare the interval succession both left-to-right and right-to-left, choosing the more left-packed version.
- **Error**: Including commas in prime form. **Correction**: Convention is parentheses with no commas: (014), not (0,1,4).

# Common Confusions
- **Confusion**: Normal form vs. prime form. **Clarification**: Normal form [G, G#, B] is specific to a particular set; prime form (014) identifies the entire set class. A set in normal form [Db, E, F] also has prime form (014).

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.7, pages 82--85.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: explicitly defined with worked examples and comprehensive set-class table
- Re-extraction notes: preserved old card's worked examples from Example 2-34; added Example 2-33 table; upgraded to v3 template
