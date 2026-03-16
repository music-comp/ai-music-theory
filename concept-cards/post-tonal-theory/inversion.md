---
concept: "Inversion (In)"
slug: inversion
category: operations
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.4 Inversion (In)"
extraction_confidence: high
aliases:
  - In
  - index-number inversion
prerequisites:
  - pitch-class-set
  - pitch-class
  - transposition
extends:
  - pitch-class
related:
  - index-number-sum
  - inversion-ixy
  - inversional-equivalence
  - mapping
contrasts_with:
  - transposition
answers_questions:
  - "How do I invert a pitch-class set?"
  - "What is the index number in In?"
  - "How does inversion differ from transposition?"
---

# Quick Definition
Inversion (In) maps each pitch class x to pitch class y such that x + y = n (the index number), effectively reflecting pitch classes around an axis.

# Core Definition
Inversion is an operation represented as In, where n is the *index number* (also called the *sum*). To invert a pitch class x at In, subtract x from n: In(x) = (n - x) mod 12. The index number n is the sum of any pair of pitch classes that map onto each other: x + y = n. Unlike transposition, inversion is its own inverse -- performing the same In twice returns to the original. Inversion preserves interval-class content while reversing interval direction.

# Prerequisites
- **Pitch-class set** -- the object being inverted
- **Pitch class** -- inversion operates on pitch classes via mod 12 arithmetic
- **Transposition (Tn)** -- understanding transposition helps contrast inversion

# Key Properties
1. In(x) = (n - x) mod 12
2. n = x + y for any mapped pair (index number is a *sum*)
3. In is its own inverse: In(In(x)) = x (all arrows are double-headed)
4. Lines related by inversion have opposite contours
5. Sets related by inversion have mirror-image interval successions in normal form
6. Sets related by inversion share the same interval-class content
7. First element of one normal form maps to last element of the other

# Construction / Recognition
**To invert a set at In**: Subtract each element from n (mod 12), then put result in normal form.
- Example: I5[1, 3, 4, 7] = [5-1, 5-3, 5-4, 5-7] = [4, 2, 1, 10] = [10, 1, 2, 4]

**To find n for two inversionally related sets**:
1. Put both in normal form
2. Check for mirror-image interval successions
3. Add corresponding elements (first + last, second + second-to-last): x + y = n
4. All such pairs should yield the same n

# Context & Application
Inversion is one of the two fundamental operations (with transposition) defining set-class equivalence. Lines related by inversion have opposite contour (ascending becomes descending). Sets related by inversion share interval-class content, giving them a similar sound despite different pitch-class content. The self-inverse property distinguishes inversion from transposition, which requires T(12-n) to undo.

# Examples
**Example 2-18** (p. 71, Bartok, *Mikrokosmos*, no. 141, "Subject and Reflection"): Two melodies related by I8. Each pair of corresponding notes sums to 8: Bb + Bb = 10 + 10 = 8 (mod 12), Ab + C = 8 + 0 = 8, F + Eb = 5 + 3 = 8.

**Example 2-21/2-22** (p. 73, Schoenberg, *Three Piano Pieces*, op. 11, no. 1): Three sets related by I0 and I6. Sets [G, G#, B] and [Db, E, F] are related at I0 (G+F=0, G#+E=0, B+Db=0). Sets [G, G#, B] and [G, Bb, B] are related at I6 (G+B=6, G#+Bb=6).

**Example 2-23** (p. 74): I5[1, 3, 4, 7] = [10, 1, 2, 4]. Subtract each pc from 5: 5-1=4, 5-3=2, 5-4=1, 5-7=10. Result in normal form: [10, 1, 2, 4].

# Relationships
## Builds Upon
- **Pitch-class set** -- the object being inverted
- **Transposition (Tn)** -- contrasting operation
## Enables
- **Inversional equivalence** -- sets related by In are inversionally equivalent
- **Set class** -- defined by equivalence under Tn and In
## Related
- **Index number (sum)** -- the n in In
- **Inversion (Ixy)** -- alternative notation emphasizing mapped pitch-class pairs
- **Mapping** -- inversion creates specific correspondences between elements
## Contrasts With
- **Transposition (Tn)** -- Tn adds a constant (difference); In subtracts from a constant (sum). Tn requires T(12-n) to undo; In is self-inverse.

# Common Errors
- **Error**: Adding n instead of subtracting from n. **Correction**: In(x) = n - x, not n + x. The n is the *sum* of the pair, so y = n - x.
- **Error**: Forgetting to put the result in normal form. **Correction**: After inverting, reorder elements in normal form.

# Common Confusions
- **Confusion**: In vs. TnI. **Clarification**: Previous editions and some sources describe inversion as TnI (invert around C, then transpose). The arithmetic is the same: for both TnI and In, n is the sum of two inversionally related notes.
- **Confusion**: Inversion (In) vs. inverse transposition. **Clarification**: In is a fundamentally different operation from T(12-n). In maps x to (n-x); T(12-n) maps x to (x + 12 - n).

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.4, pages 69--74.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: core operation defined explicitly with formula, multiple examples, and contrast with Tn
- Re-extraction notes: preserved old card's Bartok and Schoenberg examples; added Example 2-23 worked computation; added note on TnI vs In from source bibliography section; upgraded to v3 template
