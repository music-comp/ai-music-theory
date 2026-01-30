---
concept: Directed Interval
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
A directed interval is an interval that specifies not just the distance between two elements but also the direction - the interval FROM s TO t, denoted int(s, t).

# Formal Definition
In the GIS framework, int(s, t) denotes the directed measurement, distance, or motion from s to t. The order of arguments matters: int(s, t) and int(t, s) are generally different (in fact, inverses). This directedness is represented visually by an arrow from s to t in transformation networks.

# Mathematical Formulation
- int(s, t) is the interval FROM s TO t (order matters)
- int(t, s) = int(s, t)^(-1) (Theorem 2.3.2)
- The direction is encoded in the function argument order
- In pitch space: int(C4, G4) = 7, int(G4, C4) = -7
- In pitch-class space: int(C, G) = 7, int(G, C) = 5 (not -7, since mod 12)

# Musical Context/Application
Directed intervals capture the asymmetry between "going from C to G" versus "going from G to C." This is fundamental to voice-leading analysis, where the direction of motion matters. The arrow notation in transformation networks (Figure 0.1 and throughout) visualizes this directedness. Composing directed intervals corresponds to following a path through the musical space.

# Examples
From the Introduction and Chapter 2:
- Figure 0.1: An arrow marked i extends from point s to point t, representing int(s, t)
- int(C4, G4) = 7 semitones (ascending)
- int(G4, C4) = -7 semitones (descending)
- In mod 12: int(G, C) = 5 (going "up" 5 = going "down" 7)

Composition: If we go from r to s (interval int(r,s)) and then from s to t (interval int(s,t)), the total is int(r,t) = int(r,s) * int(s,t).

Reversal: int(t, s) = int(s, t)^(-1) shows that reversing direction inverts the interval.

# Related Concepts
- Interval Function int
- Generalized Interval System
- GIS Condition A
- Theorem 2.3.2
- Transformation Network

# Common Confusions
- int(s, t) is not the same as int(t, s) unless the interval is its own inverse
- "Undirected interval" would only give |int(s, t)|, losing information
- In mod 12, "down 7" becomes "up 5" - both give int(G, C) = 5
- The notation int(s, t) explicitly encodes s as source and t as target

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, opening discussion, Theorem 2.3.2
