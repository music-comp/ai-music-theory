---
concept: Interval-Type Specificity Spectrum
slug: interval-type-specificity-spectrum

category: intervals
subcategory: interval types
tier: foundational

source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Pitch and Interval"
chapter_number: 1
pdf_page: 29
section: "1.10 Interval Class"

extraction_confidence: high

aliases:
  - concrete to abstract interval hierarchy

prerequisites:
  - four-interval-types
extends:
  - four-interval-types
related:
  - ordered-pitch-interval
  - unordered-pitch-interval
  - ordered-pitch-class-interval
  - interval-class
contrasts_with: []

answers_questions:
  - "What information is lost at each step from opi to ic?"
  - "How do I choose which interval type to use?"
---

# Quick Definition
The four interval types form a specificity spectrum: each successive type discards information (direction, register, octave equivalence) to gain generality.

# Core Definition
Moving from ordered pitch interval to interval class, each step discards one dimension of information:
- **opi to upi**: discard direction (+ or -)
- **upi to opci**: discard register (apply mod 12), but restore direction
- **opi to opci**: discard register (apply mod 12)
- **opci to upci/ic**: discard direction, take smaller complement

The resulting spectrum ranges from a fully concrete interval description (opi +19: ascending compound fifth) to a fully abstract one (ic 5: any member of the "fifth/fourth family"). Straus emphasizes that the choice of level is an analytical decision, not a matter of correctness.

# Prerequisites
- **Four Interval Types** — the four types that form this spectrum

# Key Properties
1. Direction removed: opi to upi (sign removed)
2. Register removed: pi to pci (mod 12 applied)
3. Complementation applied: opci to ic (take smaller complement)
4. Information is irreversibly lost at each step: you cannot recover register from a pci
5. More abstract = more instances grouped together = more general claims

# Construction / Recognition
## To Construct:
1. Begin with opi (most specific)
2. Remove direction: get upi
3. Apply mod 12: get opci (from opi) or upci (from upi)
4. Take smaller complement of opci: get ic

## To Recognize:
1. Identify which dimensions of information an interval description includes
2. Place it on the spectrum accordingly

# Context & Application
Understanding this spectrum helps analysts make informed choices about what level of abstraction suits their analytical question. A contour analysis needs opi; a set-class analysis needs ic. Using the wrong level either obscures relevant information or retains irrelevant detail.

# Examples
**Example 1-17** (p. 29): The interval +19 semitones analyzed at each level:

| Type | Value | Information preserved |
|------|-------|----------------------|
| opi | +19 | size, direction, register |
| upi | 19 | size, register |
| opci | 7 (or -5) | size (mod 12), direction |
| upci/ic | 5 | abstract interval type only |

# Relationships
## Builds Upon
- **Four Interval Types** — the elements of the spectrum

## Enables
- **Informed analytical choices** — understanding what each type captures and what it discards

## Related
- **All four interval types** — this card explains the relationships among them

## Contrasts With
- (no contrast; this is a meta-analytical framework)

# Common Errors
- **Error**: Trying to recover lost information (e.g., determining the original register from an ic)
  **Correction**: Once information is discarded, it cannot be recovered. ic 5 could be pi 5, 7, 17, 19, etc.

# Common Confusions
- **Confusion**: Thinking more abstract = better
  **Clarification**: More abstract means more general. Sometimes specificity (opi) is needed; sometimes generality (ic) is needed. Neither is inherently better.

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.10, p. 29.

# Verification Notes
- Definition source: synthesized from Straus Section 1.10 discussion of Example 1-17
- Confidence rationale: high -- the spectrum is clearly described even though not given a formal name
- Re-extraction notes: New card; no prior version existed
