---
concept: Interval Function int
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
The function int(s, t) assigns to each ordered pair of elements in a musical space the directed interval from s to t in the interval group IVLS.

# Formal Definition
In a GIS (S, IVLS, int), int is a function mapping S x S into IVLS, subject to:
(A) For all r, s, t in S: int(r, s) * int(s, t) = int(r, t)
(B) For every s in S and i in IVLS, there exists a unique t in S with int(s, t) = i

From these conditions, two theorems follow:
- int(s, s) = e (identity) for all s
- int(t, s) = int(s, t)^(-1) for all s, t

# Mathematical Formulation
- int: S x S -> IVLS
- int(s, t) is the directed interval FROM s TO t
- Condition (A): intervals compose along paths
- Condition (B): given start point and interval, endpoint is unique
- int(s, s) = e (Theorem 2.3.2)
- int(t, s) = int(s, t)^(-1) (Theorem 2.3.2)

# Musical Context/Application
The interval function formalizes our intuition of "directed distance" between musical elements. When we ask "what interval is it from C to G?", we compute int(C, G). The direction matters: int(C, G) is not the same as int(G, C) unless the group is trivial. This captures the asymmetry between ascending and descending intervals.

# Examples
From Chapter 2:
- Chromatic pitch space: int(C4, G4) = 7 (semitones up), int(G4, C4) = -7
- Pitch-class space: int(C, G) = 7, int(G, C) = 5 (since 7 + 5 = 12 = 0 mod 12)
- Just intonation: int(C4, G4) = 3/2 (frequency ratio)
- Time-point space: int(beat 1, beat 5) = 4 (time units later)

Condition (A) example: int(C4, E4) = 4, int(E4, G4) = 3, int(C4, G4) = 7, and 4 + 3 = 7.

Theorem proof: int(s, s)int(s, s) = int(s, s) by Condition (A). Multiplying by int(s, s)^(-1) gives int(s, s) = e.

# Related Concepts
- Generalized Interval System
- Musical Space S
- Interval Group IVLS
- Directed Interval
- Transposition

# Common Confusions
- int(s, t) is directed: FROM s TO t, not just "between s and t"
- int(s, t) and int(t, s) are inverses, not equal (unless the interval is its own inverse)
- The notation "int(s, t)" uses parentheses - this is a function of two arguments
- Condition (B) ensures each interval from s leads to exactly one t

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1, Theorem 2.3.2
