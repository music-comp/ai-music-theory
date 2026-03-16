---
concept: Subset and Superset Relation
slug: subset-superset-relation
category: set-theory
subcategory: inclusion
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 137
section: "3.8 Inclusion Relation (Subsets and Supersets)"
extraction_confidence: high
aliases:
  - "inclusion relation"
  - "subset/superset"
prerequisites:
  - pitch-class-set
  - set-class
extends:
  - pitch-class-set
related:
  - literal-subset
  - abstract-subset
  - inclusion-lattice
  - subsets-of-same-type
contrasts_with:
  - complement-relation
answers_questions:
  - "How do subsets and supersets work in pitch-class set theory?"
  - "How many subsets does a set contain?"
---

# Quick Definition
The relationship where one set is contained within another; a set of n pitch classes contains 2^n total subsets, and the subset/superset relation can be literal (actual containment) or abstract (set-class-level containment).

# Core Definition
If Set X is included in Set Y, then X is a subset of Y and Y is a superset of X. A set containing n pitch classes contains 2^n subsets in total, including the null set, singletons, and the set itself. The musically interesting subsets number 2^n - (n + 2). Subset relations can be literal (specific pitch-class containment) or abstract (set-class containment). The subsets are an abstract musical potential; the composer chooses which to emphasize (Straus, pp. 137-141).

# Prerequisites
- **Pitch-class set** -- the objects in the relationship
- **Set class** -- needed for abstract subset relations

# Key Properties
1. Total subsets = 2^n for a set of size n
2. A 4-note set: 1 null + 4 singletons + 6 dyads + 4 trichords + 1 improper = 16 subsets
3. Musically interesting subsets = 2^n - (n + 2)
4. Some subsets may be members of the same set class (reducing variety)
5. The bigger the set, the more numerous the subsets

# Construction / Recognition
Subset enumeration for a 4-note set [G#, A, B, C]:
- 1 four-note subset (improper): [G#, A, B, C]
- 4 three-note subsets: [G#, A, B], [G#, A, C], [G#, B, C], [A, B, C]
- 6 two-note subsets (dyads): [G#, A], [G#, B], [G#, C], [A, B], [A, C], [B, C]
- 4 one-note subsets (singletons)
- 1 null set
- Total: 2^4 = 16

# Context & Application
Smaller collections frequently combine into larger ones, and larger collections divide into smaller ones. The subsets of a set are compositional potential: composers choose which to emphasize through register, articulation, rhythm, and other parameters, and which to suppress.

# Examples
**Example 1** (p. 137, Ex. 3-28): A four-note set [G#, A, B, C] contains 16 subsets as enumerated above.

**Example 2** (p. 137, Ex. 3-29): sc(0268) -- all four trichordal subsets are members of the same set class (026). This extreme redundancy is characteristic of highly symmetrical sets.

**Example 3** (p. 139, Ex. 3-31/3-32): Schoenberg uses the same (014589) chord differently in two pieces. In op. 19, no. 2: projects (0148) and (048) registrally. In Ode to Napoleon: projects major and minor triads. Same inclusion lattice, different compositional choices.

# Relationships
## Builds Upon
- **Pitch-class set** -- the objects in the relationship

## Enables
- **Inclusion lattice** -- systematic display of all subset classes
- **Literal subset** and **Abstract subset** -- the two modes of the relationship

## Contrasts With
- **Complement relation** -- about excluded pcs, not included ones

# Common Errors
- Thinking all subsets are equally important musically (only those articulated compositionally matter)
- Confusing literal subsets (specific pcs) with abstract subsets (set-class relations)

# Common Confusions
- The improper subset (the set itself) is technically a subset
- 2^n counts all subsets including trivial ones; musically interesting subsets are far fewer

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.8, pp. 137-141

# Verification Notes
Upgraded from old v2 card. Preserved subset enumeration, (0268) example, and Schoenberg comparison. Added 2^n formula and v3 template fields.
