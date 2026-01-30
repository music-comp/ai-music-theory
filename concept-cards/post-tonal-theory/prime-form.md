---
concept: Prime Form
category: theory
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
unit: null
authors: Joseph N. Straus
---

# Quick Definition
Prime form is the standardized representation of a set class, beginning with 0 and arranged most compactly to the left.

# Formal Definition
Prime form is the way of identifying a set class as a string of pitch-class integers beginning on 0. It is the representation that is most packed to the left (smallest intervals first). All members of a set class share the same prime form. Prime forms are written in parentheses without commas, using T and E for 10 and 11. For example, (014), (0134), (027), (016).

# Mathematical Formulation/Recognition
**Algorithm for finding prime form:**

1. Put the set in normal form
2. Extract the interval succession
3. Compare intervals reading left-to-right and right-to-left
4. Choose whichever is more packed to the left (smaller intervals first)
5. Build the prime form starting from 0 using that interval succession

**Notation conventions:**
- Parentheses: (014)
- No commas
- T = 10, E = 11
- Always starts with 0

**Quick method using clockface:**
- Find the largest gap between pitch classes
- Try reading clockwise from the gap's end (assign 0)
- Try reading counterclockwise from the gap's beginning (assign 0)
- Choose whichever has smaller numbers toward the left

# Musical Context/Application
Prime form is the standard identifier for a set class. When we say a collection "is a (014)," we mean it belongs to the set class whose prime form is (014). The List of Set Classes catalogs all prime forms, enabling quick identification of any set's set-class membership.

# Examples
**Example 2-33**: Various sets and their set classes:
- {D#, E, G}, {D, D#, F#}, {Ab, A, C}, {B, C, Eb} are all members of sc(014)
- {G, Ab, Bb, B}, {Bb, B, Db, D} are members of sc(0134)
- {C#, E, F#}, {A, B, D}, {C, Eb, F} are members of sc(025)

**Example 2-34**: Step-by-step procedure for finding prime form:
- [C#, F, F#, G]: intervals 4-1-1, which reads 1-1-4 from right; prime form (0126)
- [Bb, D, F, F#]: intervals 4-3-1; prime form (0148)
- [F, F#, A]: intervals 1-3; prime form (014)

# Related Concepts
- Set class
- Normal form
- List of Set Classes
- Forte names
- Interval-class vector

# Common Confusions
Prime form and normal form are different. Normal form is specific to a particular set; prime form identifies the set class. A set in normal form [G, G#, B] has prime form (014), as does [Db, E, F]. The prime form abstracts away from the specific pitch-class content to identify the type of set.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.7, pp. 66-68
