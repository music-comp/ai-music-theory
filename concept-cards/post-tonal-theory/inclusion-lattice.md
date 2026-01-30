---
concept: Inclusion Lattice
category: analysis
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 138
unit: null
authors: Joseph N. Straus
---

# Quick Definition
A hierarchical diagram showing all subset classes of a given set class and their relationships; it reveals the complete abstract subset structure from the largest set down to its smallest meaningful subsets.

# Formal Definition
An inclusion lattice is a visual representation listing all the subset classes of a given set class, organized hierarchically by cardinality. It shows:
- The original set class at the top
- All pentachord classes that are abstract subsets (for hexachords)
- All tetrachord classes that are abstract subsets of those pentachords
- All trichord classes that are abstract subsets of those tetrachords
- And so on down to dyads

Lines connect set classes to show direct subset relationships. The lattice reveals the complete structural "genealogy" of a set class.

# Mathematical Formulation/Recognition
To construct an inclusion lattice for set class X:
1. Start with X at the top
2. Find all (n-1)-note abstract subsets of X
3. For each of those, find all (n-2)-note abstract subsets
4. Continue down to trichords (or dyads if desired)
5. Draw connecting lines to show which subsets are contained in which supersets

The lattice structure may branch and converge: a trichord class may be a subset of multiple tetrachord classes, which in turn may be subsets of the same pentachord class.

# Musical Context/Application
Inclusion lattices are useful for:
- Understanding the complete subset potential of a collection
- Comparing what a composer emphasized versus what was possible
- Planning compositions using hierarchically related set classes
- Analyzing how a piece exploits or avoids certain subset relationships

The lattice shows abstract compositional potential; actual music emphasizes only selected paths through the lattice.

# Examples
From Example 3-30: Inclusion lattice for sc(014589) - the hexatonic collection:

```
                    (014589)
                        |
                    (01458)
                   /   |   \
              (0148) (0347) (0158) (0348)
              /    \    |     |      |
          (048)   (014)(037)(015)(016)(048)
```

All six 5-note subsets of (014589) are members of sc(01458).
The tetrachordal subsets include (0148), (0347), (0158), and (0348).
The trichordal subsets include (048) - augmented triad, (014), (037) - major/minor triad, (015), and (016).

From Example 3-31 and 3-32: Schoenberg uses (014589) differently in two pieces:
- In op. 19, no. 2: Projects (0148) and (048) as registral subsets
- In Ode to Napoleon: Projects major and minor triads (037)
- Both are valid paths through the same inclusion lattice

# Related Concepts
- Abstract subset
- Literal subset
- Set class
- Subset and superset relation
- Superset

# Common Confusions
- Thinking the lattice shows which subsets actually appear in a piece (it shows what's possible, not what's used)
- Assuming all paths through the lattice are equally likely or important musically
- Forgetting that the same trichord class may appear under multiple tetrachord classes
- Not recognizing that highly symmetrical sets have more redundant (convergent) lattices

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.8.2, pages 138-139
