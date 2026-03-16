---
concept: Pitch-Class Set
slug: pitch-class-set

category: analysis
subcategory: set-theory
tier: advanced

source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "Pitch-Class Sets, Normal Order, and Transformations"
chapter_number: 8
pdf_page: null
section: "VIII.3"

extraction_confidence: high

aliases:
  - "pc set"
  - "pcset"

prerequisites:
  - pitch-class
  - integer-notation
extends: []
related:
  - normal-order
  - set-class
  - prime-form
contrasts_with:
  - twelve-tone-row

answers_questions:
  - "What is a pitch-class set?"
  - "How does a pc set differ from a set class?"
  - "How are pitch-class sets transformed by transposition and inversion?"
---

# Quick Definition
A pitch-class set (pc set) is any group of pitch classes treated as a unit for analysis, written in square brackets in normal order (e.g., [3, 8, 9]). Unlike a twelve-tone row, a pc set is unordered -- it defines a collection of pitch classes without specifying sequence.

# Core Definition
A pitch-class set is a collection of pitch classes grouped together by an analyst. Any group of pitch classes can form a pc set. Sets are typically written in normal order (most compact ascending arrangement) within square brackets. Two fundamental operations transform pc sets: transposition (Tn, adding n to each integer mod 12) preserves the ordered intervallic content, while inversion (In, subtracting each integer from n, or inverting then transposing by n) reverses interval directions while preserving interval sizes. Transposition is notated Tn where n is the index number; inversion is notated In. To identify a transposition between two sets, subtract one from the other -- if all differences are equal, they are Tn-related.

# Prerequisites
- Pitch class and integer notation
- Mod-12 arithmetic

# Key Properties
1. Any group of pitch classes can be a pc set
2. Sets are unordered collections (unlike rows, which are ordered)
3. Written in square brackets in normal order: [x, y, z]
4. Transposition (Tn): add n to each integer mod 12
5. Inversion (In): subtract each integer from n (n-x=y), or invert then transpose
6. Two In methods: invert-then-transpose, or subtraction method (both yield same result)
7. The index number n for an inversion can be found by adding corresponding integers of two inversionally related sets

# Context & Application
Pc sets are the basic analytical units in post-tonal set theory. Analysts segment music into pc sets based on contiguity, shared rhythm, texture, articulation, register, or other musical cues. The power of set theory lies in revealing relationships between sets through transposition and inversion, showing how a composer creates unity from seemingly disparate materials. Sets can be compared by placing them in normal order, then determining their prime form to identify the set class.

# Examples
**Example 1** (Debussy, La cathedrale engloutie): The opening motive {D, E, B} = [2, 4, 11] is transposed T4 to {F-sharp, G-sharp, D-sharp} = [3, 6, 8] at m. 18, representing the cathedral's ascent.

**Example 2**: T4 of [11, 2, 4]: 11+4=3, 2+4=6, 4+4=8, giving [3, 6, 8].

**Example 3** (Chen Yi, Duo Ye): [2, 4, 7] is inverted by I8 to become [1, 4, 6]. Subtraction method: 8-2=6, 8-4=4, 8-7=1, yielding [1, 4, 6].

# Relationships
## Builds Upon
- **pitch-class** -- Pc sets are collections of pitch classes
- **integer-notation** -- Sets use integer representation
## Related
- **normal-order** -- The standard way to write a pc set
- **set-class** -- The group of all Tn/In-related pc sets
- **prime-form** -- The label for a set class
## Contrasts With
- **twelve-tone-row** -- A row is ordered; a set is unordered

# Common Confusions
- **Confusion**: A pc set must have some special property to be valid
  **Clarification**: Any group of pitch classes can be a set; the analyst decides what to group
- **Confusion**: Pc set and set class are the same thing
  **Clarification**: A pc set is one specific collection; a set class is the group of all Tn/In-related pc sets
- **Confusion**: Inversion In always produces the same set class as the original
  **Clarification**: Yes, by definition Tn/In-related sets belong to the same set class

# Source Reference
Open Music Theory, Part VIII, Chapter 3: "Pitch-Class Sets, Normal Order, and Transformations."

# Verification Notes
- Definition source: Directly from 08-03 source chapter
- Confidence rationale: High -- clearly defined with detailed operations
- Preserved from v2: Debussy and Chen Yi examples, transposition/inversion procedures
- Cross-reference status: Verified against set theory quick reference sheet
