---
concept: Tn-Type
slug: tn-type
category: set-theory
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.6 Set Class"
extraction_confidence: high
aliases:
  - transpositional type
  - Tn-class
prerequisites:
  - transposition
  - pitch-class-set
extends:
  - transpositional-equivalence
related:
  - set-class
  - normal-form
contrasts_with:
  - set-class
answers_questions:
  - "What is a Tn-type?"
  - "What distinguishes Tn-type from Tn/TnI-type set classes?"
  - "How many sets are in a Tn-type?"
---

# Quick Definition
A Tn-type is a class of sets that are all related to each other by transposition only, forming one half of a typical set class.

# Core Definition
A Tn-type is a collection of pitch-class sets where every member is related to every other member by transposition (Tn) -- but not necessarily by inversion. A typical Tn-type contains twelve sets, one at each transposition level. A set class normally comprises two Tn-types: one for the set and its transpositions, and one for the inverted set and its transpositions. For inversionally symmetrical sets, the two Tn-types collapse into one and the Tn-type equals the set class.

# Prerequisites
- **Transposition (Tn)** -- the defining operation
- **Pitch-class set** -- the elements of a Tn-type

# Key Properties
1. Normally contains 12 sets (one at each transposition level)
2. All members related by Tn (not necessarily by In)
3. Members have identical interval successions in normal form
4. A set class typically = Tn-type 1 + Tn-type 2 (related by inversion)
5. For symmetrical sets: Tn-type = set class

# Construction / Recognition
**To identify a Tn-type:**
1. Take a set in normal form
2. Transpose it to all 12 levels (T0 through T11)
3. These 12 sets form one Tn-type

**Familiar examples:**
- The 12 minor triads form one Tn-type
- The 12 major triads form another Tn-type
- Together, all 24 major and minor triads form the set class sc(037)

# Context & Application
Distinguishing Tn-type from set class can be analytically valuable. Some passages may use only transposition (staying within a Tn-type), while others use both transposition and inversion (traversing the full set class). Identifying this distinction reveals compositional strategies.

# Examples
**Example 2-29** (p. 78): Two columns of sets. Each column represents a Tn-type (twelve sets related by transposition). The two columns together form one set class.

**Example 2-30** (p. 79, Crumb, "Gargoyles"): Right-hand trichords all belong to the same Tn-type (related by transposition). Left-hand trichords share a different Tn-type. The two Tn-types together form sc(016).

# Relationships
## Builds Upon
- **Transpositional equivalence** -- all members of a Tn-type are transpositionally equivalent
## Enables
- **Set class** -- a set class normally comprises two Tn-types
## Related
- **Normal form** -- members of the same Tn-type have identical interval successions in normal form
## Contrasts With
- **Set class (T/I type)** -- a set class includes both transposition and inversion; a Tn-type includes transposition only

# Common Errors
- **Error**: Assuming same prime form means same Tn-type. **Correction**: Sets with the same prime form share a set class but may be in different Tn-types (related by inversion, not transposition).

# Common Confusions
- **Confusion**: Is a Tn-type always half of a set class? **Clarification**: For inversionally symmetrical sets, the Tn-type equals the entire set class. For all other sets, the set class contains two distinct Tn-types.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.6, pages 78--79.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: explicitly defined in context of set-class discussion
- Re-extraction notes: preserved old card's major/minor triad example; upgraded to v3 template
