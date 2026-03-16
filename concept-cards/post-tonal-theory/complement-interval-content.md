---
concept: Complement Interval Content
slug: complement-interval-content
category: set-theory
subcategory: complement
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 131
section: "3.7.1 Interval content"
extraction_confidence: high
aliases:
  - "intervallic similarity of complements"
  - "complement vector relationship"
prerequisites:
  - complement-relation
  - interval-class-vector
extends:
  - complement-relation
related:
  - hexachordal-complement-theorem
contrasts_with: []
answers_questions:
  - "Why do complementary sets sound similar?"
  - "What is the intervallic relationship between a set and its complement?"
---

# Quick Definition
The proportional similarity in interval-class vectors between complementary sets: the difference in occurrences of each interval class equals the difference in set sizes (except for ic6, where it is half the size difference).

# Core Definition
A set and its complement always have a strikingly similar distribution of intervals. For complementary sets of sizes n and (12 - n), the difference in the count of each interval class equals (12 - 2n) -- the difference between the two set sizes. The exception is ic6, where the difference is half that value, (12 - 2n)/2. The larger set is like an expanded version of the smaller complement. This similarity persists even when the sets are transposed or inverted away from literal complementation (Straus, pp. 131-132).

# Prerequisites
- **Complement relation** -- the relationship producing this property
- **Interval-class vector** -- the measure of interval content

# Key Properties
1. Size difference k = (12 - n) - n = 12 - 2n
2. For ic1 through ic5: complement has k more occurrences
3. For ic6: complement has k/2 more occurrences
4. For hexachords (n = 6): k = 0, so vectors are identical
5. The relationship holds for both literal and abstract complements

# Construction / Recognition
Formula: If set of size n has vector [a, b, c, d, e, f], its complement has:
- Vector = [a+k, b+k, c+k, d+k, e+k, f+k/2] where k = 12 - 2n

Quick check: subtract the smaller vector from the larger. All differences should be equal (except ic6 = half that value).

# Context & Application
This intervallic similarity explains why complementary sets sound related despite sharing no pitch classes (in the literal case). It is the acoustic foundation of the complement relation and explains why aggregate-based music (including twelve-tone music) achieves coherence when divided into complementary subsets.

# Examples
**Example 1** (p. 131, Ex. 3-23):
- sc4-18 (0147): vector [102111]
- sc8-18 (01235689): vector [546553]
- Difference: 4 more of each ic (except ic6: 2 more)
- k = 12 - 2(4) = 4

**Example 2** (p. 134, Ex. 3-26): Schoenberg, Little Piano Pieces, op. 19, no. 2:
- sc4-19 (0148): vector [101310]
- sc8-19 (01245689): vector [545752]
- Both are rich in ic4 -- no four- or eight-note set contains more ic4s
- The complement relation explains why the final chord sounds similar to the larger collection

# Relationships
## Builds Upon
- **Complement relation** -- the relationship this formalizes

## Enables
- **Hexachordal complement theorem** -- the k=0 special case

# Common Errors
- Expecting complementary sets to have contrasting interval content (they are actually similar)
- Applying the regular difference to ic6 (must halve it)

# Common Confusions
- The similarity holds even for abstract complements (sets that share some pitch classes)
- For hexachords, the relationship is exact equality, not just similarity

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.7.1, pp. 131-132

# Verification Notes
New card extracted from source. Content was previously distributed across complement-relation and hexachordal-complement-theorem cards; now given its own focused card.
