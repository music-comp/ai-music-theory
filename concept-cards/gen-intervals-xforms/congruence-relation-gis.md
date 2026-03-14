---
concept: Congruence Relation in GIS
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
A congruence relation on a group is an equivalence relation compatible with the group operation: if i is congruent to i' and j is congruent to j', then ij is congruent to i'j'. Congruences on IVLS induce quotient GIS structures.

# Formal Definition
A relation CONG on a group IVLS is a congruence if:
1. CONG is an equivalence relation (reflexive, symmetric, transitive)
2. For all i, i', j, j' in IVLS: if i CONG i' and j CONG j', then ij CONG i'j'

Condition (2) ensures the quotient group IVLS/CONG is well-defined.

# Mathematical Formulation
**Quotient group construction:**
IVLS/CONG = {congruence classes of IVLS}
[i] * [j] = [ij] (well-defined by condition 2)

**Example (integers mod 12):**
IVLS = Z (integers)
CONG: i ~ i' iff 12 divides (i' - i)
IVLS/CONG = Z/12Z (integers mod 12)

**Induced equivalence on S:**
s EQUIV s' iff int(s, s') CONG e (identity)

# Musical Context/Application
Congruence relations formalize "modularization":
- Pitch space modularized by octave equivalence
- Time-point space modularized by measure length
- Just-intonation space modularized by octave

The congruence determines which intervals are "equivalent to zero" and thus which elements of S become identified.

# Examples
**Chromatic to pitch-class (Section 3.2):**
- IVLS1 = Z (semitones)
- CONG: i ~ i' iff i' = i + 12k for some integer k
- IVLS2 = Z/12Z (pitch-class intervals)
- EQUIV: pitches are equivalent if they differ by octaves

**Just intonation to pitch classes:**
- IVLS1 = {2^a * 3^b * 5^c} under multiplication
- CONG: i ~ i' iff i' = 2^n * i for some integer n
- IVLS2 = pairs (b, c) (dominants and mediants)
- EQUIV: pitches are equivalent if they differ by octaves

**Time-points to beat-classes:**
- IVLS1 = Z (beats)
- CONG: i ~ i' iff i' = i + Nk for some integer k
- IVLS2 = Z/NZ (beat-classes in N-beat measure)

# Related Concepts
- Quotient GIS
- Equivalence Relation
- Induced Equivalence
- Group Homomorphism
- Octave Equivalence

# Common Confusions
1. **Congruence on IVLS, not S:** The congruence is defined on the interval group. The equivalence on S is induced by the congruence.

2. **Compatibility condition:** The product of congruent elements must be congruent. This ensures quotient operations are well-defined.

3. **Not every equivalence works:** An arbitrary equivalence on S may not yield a quotient GIS. The equivalence must be induced by a congruence on IVLS.

4. **Kernel interpretation:** The congruence class of e (identity) is a normal subgroup. The congruence classes are cosets of this kernel.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Sections 3.2.1-3.2.4, pp. 64-68
