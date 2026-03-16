---
concept: Recognizing Transpositional Relations
slug: recognizing-transpositional-relations
category: operations
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.3.6 Recognizing sets related by transposition"
extraction_confidence: high
aliases:
  - transpositional relation test
prerequisites:
  - transposition
  - normal-form
  - transposition-number
extends:
  - transposition
related:
  - transpositional-equivalence
  - inversional-equivalence
contrasts_with:
  - inversional-equivalence
answers_questions:
  - "How do I tell if two sets are related by transposition?"
  - "How do I find the transposition interval between two sets?"
---

# Quick Definition
Two sets are related by transposition if, in normal form, they have the same interval succession. The transposition number n is found by subtracting corresponding elements.

# Core Definition
"If two sets are related by transposition at interval n, for each note in the first set there will be a corresponding note in the second set that lies n semitones away" (Straus, Ch. 2). The test involves putting both sets in normal form: "Transpositionally related pitch-class sets in normal form have the same succession of intervals reading from left to right." The transposition number is found by subtracting: n = y - x (mod 12) for any pair of corresponding elements.

# Prerequisites
- **Transposition (Tn)** -- the relationship being detected
- **Normal form** -- required for the comparison
- **Transposition number** -- the n to be found

# Key Properties
1. Same interval succession in normal form = transpositionally related
2. First-to-first, second-to-second correspondence in normal form
3. All corresponding pairs yield the same difference n
4. Formula: n = y - x (mod 12)

# Construction / Recognition
**Test procedure:**
1. Put both sets in normal form
2. Extract the interval succession for each
3. If identical, the sets are transpositionally related
4. To find n: subtract any element of the first set from the corresponding element of the second
5. Verify: all corresponding pairs should give the same n

# Context & Application
This test is one of the most common analytical operations: given two sets suspected of being related, determine if and how they are connected. Transpositional relations produce identical interval successions; inversional relations produce mirror-image successions. Distinguishing between these is fundamental to set-class analysis.

# Examples
**Example 2-10** (p. 66): [1, 3, 4, 7] and [5, 7, 8, 11] both have interval succession 2-1-3, confirming transpositional equivalence. n = 5-1 = 7-3 = 8-4 = 11-7 = 4. Set 2 = T4(Set 1).

**Example 2-11** (p. 67, Stravinsky, *Agon*): Two circled sets put in normal form: [7, 8, 10, 11] and [10, 11, 1, 2]. Both have interval succession 1-2-1. n = 10-7 = 11-8 = 1-10 = 2-11 = 3 (mod 12). Set 2 = T3(Set 1).

# Relationships
## Builds Upon
- **Transposition (Tn)** -- the relationship being tested
- **Normal form** -- the diagnostic tool
- **Transposition number** -- the quantity being calculated
## Enables
- **Transpositional equivalence** -- confirmed by this test
## Related
- **Inversional equivalence** -- the alternative outcome (mirror-image succession)
## Contrasts With
- **Inversional equivalence** -- identical succession = transposition; mirror-image = inversion

# Common Errors
- **Error**: Comparing sets not in normal form. **Correction**: Always put both sets in normal form before comparing interval successions.
- **Error**: Subtracting in the wrong order. **Correction**: n = y - x, where y is in the second set and x is the corresponding element in the first. Be consistent about which set is "from" and which is "to."

# Common Confusions
- **Confusion**: Two sets with the same prime form must be transpositionally related. **Clarification**: They may be related by inversion instead. Same prime form = same set class, but same Tn-type requires identical (not mirror-image) interval successions in normal form.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.3.6, pages 66--67.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: explicit test procedure with worked examples
- Re-extraction notes: new card; extracted from section 2.3.6
