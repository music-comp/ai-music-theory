---
concept: "Reference Point (ref)"
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
A reference point (ref) is a fixed element in a GIS space S chosen to serve as the origin for the LABEL function, establishing a coordinate system for the space.

# Formal Definition
Given a GIS (S, IVLS, int), a referential member ref is any chosen element of S used to define the LABEL function via LABEL(s) = int(ref, s). The choice of ref determines how elements of S are labeled by intervals, but does not affect the intrinsic intervallic relationships between elements.

# Mathematical Formulation
**Role in LABEL function:**
LABEL(s) = int(ref, s)

**Properties:**
- LABEL(ref) = int(ref, ref) = e (the identity element of IVLS)
- Changing ref changes all labels but preserves intervals
- For any two reference choices ref1 and ref2:
  LABEL_ref2(s) = int(ref1, ref2) * LABEL_ref1(s)

# Musical Context/Application
The reference point corresponds to choosing an "origin" in musical space. Common choices include:
- C as reference for pitch classes (yielding integer notation 0-11)
- "Time-point zero" in temporal spaces
- A reference spectrum in timbral spaces

The choice raises aesthetic and methodological questions:
- Why privilege one pitch class over another a priori?
- Should the reference be contextually determined by the music?
- Does the music itself project a referential element?

# Examples
**Fixed-do vs. movable-do analogy:** In fixed-do solfege, C is always "do" regardless of key, like using C as a fixed reference point. In movable-do, the tonic of the current key is "do," like choosing ref contextually.

**Tuning reference:** String players tune their instruments to A; this might suggest using an A-labeling system as "methodologically most accurate" for string music.

**Multiple valid references:** In a piece where E is clearly the tonic, one might argue for:
- C as ref (conventional, yielding E = 4)
- E as ref (contextual, yielding E = 0)
- A as ref (tuning-based)

# Related Concepts
- LABEL Function
- Generalized Interval System (GIS)
- Interval Function (int)
- Transposition Operations (Ti)
- Interval-Preserving Operations (Pi)

# Common Confusions
1. The reference point is not intrinsic to the GIS structure; it is an auxiliary choice made for computational convenience.

2. Students may conflate the reference point with a musical "tonic" or "center." These are distinct: ref is a formal construct, while tonics are perceptual/analytical.

3. Different reference points yield different labels but the same GIS. The underlying intervallic structure is independent of ref.

4. Computations can be "muddled by the algebraic influence of irrelevant intervals" arising from irrelevant relations of ref to the objects under study.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Section 3.1, pp. 62-63
