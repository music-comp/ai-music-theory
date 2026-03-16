---
concept: Transpositional Combination
slug: transpositional-combination
category: analysis
subcategory: transpositional combination
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 140
section: "3.9 Transpositional Combination (TC)"
extraction_confidence: high
aliases:
  - "TC"
  - "TC property"
prerequisites:
  - transposition
  - inversional-symmetry
  - subset-superset-relation
extends:
  - inversional-symmetry
related:
  - transpositional-symmetry
contrasts_with: []
answers_questions:
  - "What is transpositional combination?"
  - "Which tetrachords have the TC property?"
---

# Quick Definition
The process of combining a set with one or more transpositions of itself to create a larger set; notated as n*m (interval n transposed by m), with TC being a more exclusive property than inversional symmetry.

# Core Definition
Transpositional combination (TC) is the combination of a set with one or more transpositions of itself to create a larger set. The larger set is said to have the TC property if it can be divided into two or more subsets related by transposition. The notation n*m indicates interval n combined with its transposition by m semitones. While inversional symmetry relates to combining sets with inversions, TC relates to combining with transpositions. All sets with the TC property are also inversionally symmetrical, but not all inversionally symmetrical sets have the TC property -- TC is the more exclusive property (Straus, pp. 140-142).

# Prerequisites
- **Transposition** -- the operation combining the subsets
- **Inversional symmetry** -- TC implies inversional symmetry
- **Subset and superset relation** -- TC creates superset from transposed subsets

# Key Properties
1. TC notation: n*m (interval n transposed by m)
2. The operation is commutative: n*m produces the same set class as m*n
3. All TC sets are inversionally symmetrical (but not vice versa)
4. TC is more exclusive than inversional symmetry
5. TC structures can be built recursively: (n*m)*k
6. Sets like (0127) and (0248) are inversionally symmetrical but lack TC

# Construction / Recognition
All 13 tetrachords with the TC property (from Ex. 3-35):

| Tetrachord | TC |
|------------|-----|
| (0123) | 1*2 |
| (0134) | 1*3 |
| (0145) | 1*4 |
| (0156) | 1*5 |
| (0167) | 1*6, 5*6 |
| (0235) | 2*3 |
| (0246) | 2*4 |
| (0257) | 2*5 |
| (0268) | 2*6, 4*6 |
| (0347) | 3*4 |
| (0358) | 3*5 |
| (0369) | 3*6 |
| (0158) | 4*5 |

# Context & Application
TC allows analysis of how larger structures are built from smaller cells. The TC property creates audible relationships: listeners can perceive the transpositionally related subsets as variants of each other. It is particularly useful for analyzing music that constructs larger sonorities from motivic cells.

# Examples
**Example 1** (p. 141, Ex. 3-34): Stravinsky, Symphony of Psalms, first movement:
- Bass: F-Ab (ic3), then E-G (ic3, a semitone lower) = 3*1
- Also readable as 1*3: two semitones (E-F and G-Ab) at T3
- Result: sc(0134) = [E, F, G, Ab]
- Another (0134): [Bb, B, C#, D]
- Combined at T6: full structure = (3*1)*6

**Example 2**: (0167) can be analyzed as either 1*6 or 5*6, illustrating that some tetrachords have multiple TC decompositions.

# Relationships
## Builds Upon
- **Inversional symmetry** -- TC implies this property

## Related
- **Transpositional symmetry** -- a different kind of transpositional property

# Common Errors
- Confusing TC (combining with transpositions) with inversional symmetry (combining with inversions)
- Thinking TC applies to all inversionally symmetrical sets (it is more restrictive)

# Common Confusions
- Some tetrachords have multiple TC analyses (e.g., (0167) = 1*6 or 5*6)
- The combined sets must not share pitch classes for the resulting set to have the full expected cardinality

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.9, pp. 140-142

# Verification Notes
Upgraded from old v2 card. Preserved complete TC tetrachord table, Stravinsky Symphony of Psalms example with recursive (3*1)*6 analysis, and TC-implies-In-symmetry relationship. Added v3 template fields.
