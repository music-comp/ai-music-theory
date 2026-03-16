---
concept: Symmetry and Set Class Size
slug: symmetry-and-set-class-size
category: set-theory
subcategory: symmetry
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 128
section: "3.5 Symmetry and Set Class"
extraction_confidence: high
aliases:
  - "set class size"
  - "number of sets in a class"
prerequisites:
  - degrees-of-transpositional-symmetry
  - degrees-of-inversional-symmetry
  - set-class
extends:
  - degrees-of-transpositional-symmetry
  - degrees-of-inversional-symmetry
related:
  - transpositional-symmetry
  - inversional-symmetry
contrasts_with: []
answers_questions:
  - "How many distinct sets are in a set class?"
  - "How does symmetry affect set class size?"
---

# Quick Definition
The inverse relationship between a set class's degree of symmetry and the number of distinct sets it contains: dividing 24 by the total number of self-mapping operations gives the number of sets in the class.

# Core Definition
The size of a set class (the number of distinct pitch-class sets it contains) is inversely proportional to its degree of symmetry. For any set class, the number of sets = 24 / (number of self-mapping operations). Most set classes have degree (1, 0) and contain 24 distinct sets. The more symmetrical the set, the fewer distinct members the set class contains (Straus, pp. 128-129).

# Prerequisites
- **Degrees of transpositional symmetry** -- contributes to total self-mapping count
- **Degrees of inversional symmetry** -- contributes to total self-mapping count
- **Set class** -- the collection being measured

# Key Properties
1. Most set classes: degree (1, 0), 24 / 1 = 24 sets
2. With In-symmetry: degree (1, 1), 24 / 2 = 12 sets
3. Formula: total self-mappings = Tn-degree + In-degree (when In-degree > 0) or just Tn-degree
4. Examples: (0167) degree (2, 2) -> 24 / 4 = 6 sets; (0369) degree (4, 4) -> 24 / 8 = 3 sets

# Construction / Recognition
For a set class with degree (n, m):
- Total self-mapping operations = n + m (when m > 0)
- Total self-mapping operations = n (when m = 0)
- Number of distinct sets = 24 / total

# Context & Application
Highly symmetrical sets have fewer distinct transpositions, which is a distinctive characteristic making these sonorities immediately recognizable. The diminished seventh chord (0369) has only 3 distinct forms; the whole-tone scale (02468T) has only 2. Complement-related set classes always have the same degree of symmetry and thus the same number of sets.

# Examples
**Example 1** (p. 128, Ex. 3-19): sc(0167), degree (2, 2):
- Each set maps onto itself via 4 operations: T0, T6, I1, I7
- 24 / 4 = 6 distinct sets

The six members:
| Set | Operations |
|-----|-----------|
| [0,1,6,7] | T0, T6, I1, I7 |
| [1,2,7,8] | T1, T7, I2, I8 |
| [2,3,8,9] | T2, T8, I3, I9 |
| [3,4,9,10] | T3, T9, I4, I10 |
| [4,5,10,11] | T4, T10, I5, I11 |
| [5,6,11,0] | T5, T11, I6, I0 |

**Example 2** (p. 129): Diminished seventh chord (0369), degree (4, 4): 24 / 8 = 3 distinct forms.

# Relationships
## Builds Upon
- **Degrees of transpositional symmetry** and **Degrees of inversional symmetry** -- combined to compute class size

## Related
- **Complement relation** -- complement-related set classes share the same degree of symmetry

# Common Errors
- Forgetting that most set classes have 24 members
- Miscounting self-mapping operations (must count both Tn and In operations)

# Common Confusions
- "Smaller" set classes (fewer members) are not less important -- they are often the most compositionally distinctive
- Inversional symmetry at one level doubles the symmetry operations even though it adds only one In level

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.5, pp. 128-129

# Verification Notes
Upgraded from old v2 card. Preserved complete (0167) table with all six members and their operations, diminished seventh example, and formula. Added v3 template fields.
