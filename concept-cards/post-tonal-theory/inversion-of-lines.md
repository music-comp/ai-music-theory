---
concept: Inversion of Lines vs. Sets
slug: inversion-of-lines
category: operations
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.4.2 Line (or series) of pitches"
extraction_confidence: high
aliases:
  - line inversion
  - melodic inversion
prerequisites:
  - inversion
extends:
  - inversion
related:
  - transposition-of-lines
  - inversional-equivalence
contrasts_with: []
answers_questions:
  - "What is preserved when I invert a line of pitches vs. a set of pitch classes?"
  - "How does inversion affect contour?"
---

# Quick Definition
Inversion operates differently on lines and sets: inverting a line of pitches preserves order and reverses contour; inverting a line of pitch classes replaces each interval with its mod 12 complement; inverting a set preserves only interval-class content.

# Core Definition
Like transposition, inversion applies at multiple levels:

1. **Line of pitches** (Section 2.4.2): Order is preserved and "contour is reversed -- each ascending pitch interval is replaced by a descending one, and vice versa." Pairs of corresponding notes sum to the index number.

2. **Line of pitch classes** (Section 2.4.3): Order is preserved. Each ordered pitch-class interval is replaced by its *complement mod 12* (its reversal in direction): -1 (=11) becomes 1; -4 (=8) becomes 4; etc. Pairs of corresponding pitch classes sum to n.

3. **Set of pitch classes** (Section 2.4.4): The normal forms of inversionally related sets appear as mirror images. First element of one maps to last element of the other. Interval-class content is preserved.

# Prerequisites
- **Inversion (In)** -- the operation being applied at each level

# Key Properties
1. **Pitch line**: contour reversed; order preserved; each pair sums to n
2. **PC line**: each opci replaced by its complement mod 12; order preserved; each pair sums to n
3. **PC set**: mirror-image interval succession in normal form; interval-class content preserved
4. In all cases, corresponding elements sum to the index number n

# Construction / Recognition
**For sets**: To invert a set at In, subtract each element from n, then put in normal form. The result will be a mirror image of the original normal form.

**For lines**: To invert a line at In, subtract each element from n, preserving the order.

# Context & Application
Understanding these distinctions clarifies what "inversion" means at each level of abstraction. In practice, most analytical work involves set inversion (level 3), but recognizing inversional contour relationships in melodies (levels 1-2) provides additional analytical insight.

# Examples
**Example 2-18** (p. 71, Bartok, *Mikrokosmos*, no. 141, "Subject and Reflection"): Two melodies related by I8 as *lines of pitches*. Each ascending interval in one becomes descending in the other. Every pair of corresponding notes sums to 8.

**Example 2-19/2-20** (pp. 71--72, Schoenberg, *String Quartet No. 4*): Two melodies as *lines of pitch classes* related by I9. Each ordered pitch-class interval is replaced by its complement: 11 becomes 1, 8 becomes 4, etc.

**Example 2-21** (p. 73, Schoenberg, op. 11, no. 1): Three *sets* related by inversion. Normal forms appear as mirror images: [G, G#, B] (intervals 1-3) and [Db, E, F] (intervals 3-1).

# Relationships
## Builds Upon
- **Inversion (In)** -- applied at each level
## Enables
- **Multi-level analysis** -- recognizing inversion at the appropriate level of abstraction
## Related
- **Transposition of lines vs. sets** -- parallel distinctions for transposition
- **Inversional equivalence** -- the set-level consequence

# Common Errors
- **Error**: Expecting inverted sets to have reversed contour. **Correction**: Sets have no contour. Only lines have contour, and only pitch-line inversion reverses it.

# Common Confusions
- **Confusion**: Interval complement vs. interval inversion. **Clarification**: In pitch-class inversion of a line, each ordered pc interval is replaced by its complement mod 12. The interval 8 becomes 4 (not -8), because 8 + 4 = 12 = 0 mod 12.

# Source Reference
Chapter 2: Pitch-Class Sets, Sections 2.4.2--2.4.4, pages 71--74.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: three levels explicitly distinguished in the text
- Re-extraction notes: new card; parallels transposition-of-lines card
