---
concept: Inclusion Lattice
slug: inclusion-lattice
category: analysis
subcategory: inclusion
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 138
section: "3.8.2 Inclusion lattice"
extraction_confidence: high
aliases:
  - "subset lattice"
prerequisites:
  - subset-superset-relation
  - set-class
extends:
  - subset-superset-relation
related:
  - abstract-subset
  - projecting-subsets
contrasts_with: []
answers_questions:
  - "How do I visualize the complete subset structure of a set class?"
  - "What is an inclusion lattice?"
---

# Quick Definition
A hierarchical diagram listing all abstract subset classes of a given set class and their relationships, revealing the complete structural potential from the largest set down to its smallest meaningful subsets.

# Core Definition
An inclusion lattice is a visual representation listing all subset classes of a given set class, organized hierarchically by cardinality. Lines connect set classes to show direct subset relationships. The lattice reveals the complete "genealogy" of a set class: its pentachord subsets, their tetrachord subsets, those trichord subsets, and so on. The lattice may branch and converge: a trichord class may appear under multiple tetrachord classes (Straus, pp. 138-139).

# Prerequisites
- **Subset and superset relation** -- the relationship being displayed
- **Set class** -- the objects in the lattice

# Key Properties
1. Organized hierarchically by decreasing cardinality
2. Lines show direct (one-step) subset relationships
3. The lattice shows abstract (set-class-level) inclusion
4. A single subset class may appear under multiple supersets
5. The lattice represents potential, not actual musical usage

# Construction / Recognition
To construct an inclusion lattice for set class X:
1. Place X at the top
2. Find all (n-1)-note abstract subset classes
3. For each, find all (n-2)-note abstract subset classes
4. Continue down to trichords (or dyads)
5. Draw connecting lines for direct containment

# Context & Application
The lattice shows compositional potential: the composer chooses which paths through the lattice to emphasize. Comparing the lattice with actual musical usage reveals deliberate compositional choices -- which subsets are projected and which are suppressed.

# Examples
**Example 1** (p. 138, Ex. 3-30): Inclusion lattice for sc(014589) -- hexatonic collection:
- All six 5-note subsets are members of sc(01458)
- The 4-note subsets include (0148), (0347), (0158), and (0348)
- The trichord subsets include (048) augmented triad, (014), (037) major/minor triad, (015), (016)

**Example 2** (p. 138, Ex. 3-31): Schoenberg, Little Piano Pieces, op. 19, no. 2 -- a (014589) chord arranged to project (0148) and (048) as registral subsets. The composer has chosen to feature specific paths through the lattice.

**Example 3** (p. 139, Ex. 3-32): Schoenberg, Ode to Napoleon -- the same (014589) collection [D, Eb, F#, G, Bb, B] arranged to project major and minor triads (037) -- G minor, B minor, Eb major. Different path through the same lattice.

# Relationships
## Builds Upon
- **Subset and superset relation** -- the lattice systematizes this

## Enables
- **Projecting subsets** -- the lattice shows what can be projected

## Related
- **Abstract subset** -- the type of containment shown in the lattice

# Common Errors
- Thinking the lattice shows which subsets actually appear in a piece (it shows potential, not usage)
- Assuming all paths through the lattice are equally likely or important

# Common Confusions
- Highly symmetrical sets have more convergent (redundant) lattices
- The same trichord class may be reachable through multiple tetrachord classes

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.8.2, pp. 138-139

# Verification Notes
Upgraded from old v2 card. Preserved hexatonic collection lattice, both Schoenberg examples contrasting different paths through the same lattice. Added v3 template fields.
