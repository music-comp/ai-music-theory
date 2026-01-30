---
concept: Set Class
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
A set class, denoted /X/, is the canonical equivalence class containing set X - that is, the collection of all sets that can be derived from X by operations in the canonical group.

# Formal Definition
Definition 5.2.2: We shall write /X/ to denote the canonical equivalence-class containing the set X. /X/ will be called, for short, the "set class of X."

Locutions (5.2.3): "X' is a form of X" means that X' is canonically equivalent to X. /X/ may be referred to as "the forms of X."

# Mathematical Formulation
/X/ = {A(X) : A in CANON}

Properties:
- If X' is in /X/, then /X'/ = /X/ (same equivalence class)
- |/X/| <= |CANON| (number of forms bounded by size of canonical group)
- If X has symmetry (some non-identity A fixes X), then |/X/| < |CANON|

The set class depends on both X and CANON:
- Different canonical groups produce different set classes
- Notation should properly be /X/_CANON to show this dependence

# Musical Context/Application
Set classes represent abstract intervallic configurations independent of transposition and/or inversion. In Forte's theory, set classes are identified by names like 3-11 (major/minor triad when CANON includes inversions). The set class captures the intrinsic intervallic properties shared by all its member sets.

# Examples
In pitch-class space with X = {C, E, G}:

If CANON = transpositions only:
- /X/ = {{C,E,G}, {C#,F,G#}, {D,F#,A}, ...} (12 major triads)
- The set class is "the major triads"

If CANON = transpositions and inversions:
- /X/ = {all major triads, all minor triads} (24 sets)
- The set class is Forte's 3-11, "the harmonic triads"

Cardinality example:
- {C, E, G#} has only 4 forms under transposition (augmented triad is symmetric)
- /augmented triad/ has only 4 members, not 12

# Related Concepts
- Canonical Group
- Canonical Equivalence
- Forms of a Set
- EMB (Embedding Function)
- Forte Set Classes

# Common Confusions
The term "set class" grates on mathematical logicians since it conflates two technical terms. Lewin acknowledges this but uses it because it has become standard in atonal theory. His earlier term "chord type" is more descriptive but loses intuitive meaning when applied to non-pitch sets (rhythmic sets, timbral sets, etc.).

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Definition 5.2.2 and Locutions 5.2.3
