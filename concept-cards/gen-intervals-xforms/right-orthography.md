---
concept: Right Orthography
slug: right-orthography

category: mathematical-foundations
subcategory: notation-conventions
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.2.4"

extraction_confidence: high

aliases:
  - "right functional orthography"

prerequisites:
  - function
  - binary-composition
extends: []
related:
  - left-orthography
  - composition-of-functions
  - anti-homomorphism
contrasts_with:
  - left-orthography

answers_questions:
  - "What is right functional orthography and how does it differ from left orthography?"
  - "How does the order of composition change under right orthography?"
  - "Why does Lewin primarily use left orthography despite right orthography being more natural for his purposes?"
---

# Quick Definition
Right orthography is the notational convention of writing function names to the right of their arguments, as in (s)f, so that the composition ff' means "apply f first, then f'," matching the left-to-right reading order with the order of application.

# Core Definition
"Right functional orthography is preferred by some mathematicians for all contexts and by most mathematicians for some contexts. In right orthography, one writes 'sf' or '(s)f' for 'the operand s, transformed by the function f.' ... The composition function which we called 'f'f' in left orthography is called 'ff'' in right orthography, so as to be consistent: '(s)ff'' in right orthography is 's-transformed-by-f, all transformed by f''" (Lewin, Ch. 1, Section 1.2.4, p. 32).

# Prerequisites
- **Function** — Right orthography is an alternative notation for function application
- **Binary composition** — Composition order reverses between the two conventions

# Key Properties
1. (s)f denotes the value of function f applied to argument s
2. Composition ff' means "apply f first, then f'" — leftmost function applies first
3. The composition "ff'" in right orthography equals "f'f" in left orthography
4. Right orthography would be "abstractly more suitable" for Lewin's eventual purposes

# Construction / Recognition
## To Construct:
1. Write the argument s first, then the function name to its right: (s)f
2. For compositions, write functions in the order they are applied: ff' means f then f'
## To Recognize:
1. Function names appear to the right of arguments
2. In a composition, the leftmost function is applied first

# Context & Application
Lewin uses left orthography "almost exclusively" throughout the book, choosing it because of the "reader's presumed familiarity" despite right orthography being "abstractly more suitable for our eventual purposes." He uses right orthography only once, in Section 8.1.1, when discussing Klang transformations, "when its intuitive pertinence seems overwhelming."

# Examples
**Example 1** (p. 32): In right orthography, (C)T2I means "C transformed by T2, then transformed by I."
- (C)T2 = D
- (D)I = result of inverting D
- The composition T2I in right orthography equals IT2 in left orthography

**Example 2** (Section 8.1.1): When discussing Klang transformations, Lewin writes (C, +)T = (A, +), noting: "Right orthography will conform much better than left orthography to our intuitions in the contexts we shall be exploring just here."

# Relationships
## Builds Upon
- **Function** — Right orthography is an alternative way of writing function application
## Related
- **Composition of functions** — Composition order reverses between orthographies
- **Anti-homomorphism** — The map between left and right composition is an anti-homomorphism
## Contrasts With
- **Left orthography** — Standard convention where function name precedes argument: f(s)

# Common Errors
- **Error**: Mixing left and right orthography in the same calculation
  **Correction**: Consistently use one convention; "fg" in right orthography is "gf" in left

# Common Confusions
- **Confusion**: Thinking right orthography changes the mathematics
  **Clarification**: Both conventions describe the same functions and compositions; only the notational order differs. The composition ff' in right orthography and f'f in left orthography denote the same function.

# Source Reference
Chapter 1: Mathematical Preliminaries, Section 1.2.4, pages 32-33.

# Verification Notes
- Definition source: Direct quotation from Section 1.2.4
- Confidence rationale: Explicitly defined and discussed by Lewin
- Re-extraction notes: Re-extracted from v2 card; preserved: examples, contrast with left orthography, note about single use in Section 8.1.1
