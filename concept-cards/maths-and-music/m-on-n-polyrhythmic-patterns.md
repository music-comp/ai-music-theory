---
concept: M on N Polyrhythmic Patterns
category: application
source: "Mathematics and Music"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
authors: "David Wright"
---

# Quick Definition
A compositional technique where a pattern of $m$ notes or beats is superimposed against a pattern of $n$, exploiting the mathematical fact that when $\gcd(m, n) = 1$, the combined pattern takes exactly $mn$ units to complete its cycle.

# Formal Definition
An m-on-n pattern occurs when a cycle of $m$ elements (pitches, rhythmic figures) is repeated against a cycle of $n$ elements (beats, rhythmic groups). When $\gcd(m, n) = 1$, the element $[m]$ is a generator of $\mathbb{Z}_n$ (and $[n]$ is a generator of $\mathbb{Z}_m$), ensuring that the double pattern completes after exactly $m \times n$ units and not before. Each cycle of $m$ starts on a different element of $\mathbb{Z}_n$, exhausting all $n$ possibilities.

# Mathematical Context
The pattern works because $\gcd(m, n) = 1$ implies $[m]$ generates $\mathbb{Z}_n$. Labeling beat positions $1, 2, \ldots, mn$ and marking every $m$th position, the sequence of positions modulo $n$ is $[m], [2m], [3m], \ldots, [nm] = [0]$, which runs through all of $\mathbb{Z}_n$ before repeating. There is perfect symmetry: viewing the pattern from either cycle's perspective yields the same analysis with $m$ and $n$ swapped.

# Musical Context
Composers use m-on-n patterns to create musical tension and a sense of fulfillment when the double cycle completes. The technique is a form of polyrhythm, fundamentally different from tuplets. It gives the listener the choice of counting beats in groups of $m$ or $n$, creating rhythmic ambiguity and interest.

# Examples
- **"In the Mood"** (3 on 4): three pitches ($C_4$, $E_4^\flat$, $A_4^\flat$) cycled through a four-eighth-note rhythmic pattern. Both cycles complete together at 12 eighth notes ($3 \times 4$), not before. The multiples of $[3]$ in $\mathbb{Z}_4$ are $[3], [2], [1], [0]$, exhausting all elements.
- **"Rhapsody in Blue"** (3 on 5): three pitches ($D_4^\sharp$, $D_4$, $C_4^\sharp$) against a five-note rhythmic figure. Double pattern completes after $15$ notes ($3 \times 5$), spanning three measures.
- **"Ain't No Sunshine"** (3 on 16): a rhythmic figure of 3 sixteenth notes repeated in 4/4 time (16 sixteenth notes per measure). The pattern runs for $48$ sixteenth notes ($3 \times 16$) = 3 measures before both cycles restart.

# Related Concepts
- Greatest Common Divisor
- Relatively Prime Integers
- Cyclic Group and Generator
- GCD Condition for Generators

# Common Confusions
- The m-on-n technique requires $\gcd(m, n) = 1$ to work fully; if $\gcd(m, n) > 1$, the pattern completes in fewer than $mn$ units
- M-on-n patterns are distinct from tuplets (which subdivide beats unequally); polyrhythm here involves two independent cycles
- The technique can involve pitch cycling against rhythm (as in "In the Mood") or rhythmic figure cycling against meter (as in "Ain't No Sunshine")

# Source Reference
Chapter 8, "Patterns of m on n in Music" section, p. 100 (PDF)
