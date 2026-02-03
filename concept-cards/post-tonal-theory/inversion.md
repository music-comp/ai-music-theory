---
concept: "Inversion (In)"
category: theory
source: Introduction to Post-Tonal Theory (Fifth Edition)
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
unit: null
authors: Joseph N. Straus
---

# Quick Definition
Inversion (In) is an operation that maps each pitch class x to pitch class y such that x + y = n (the index number), effectively reflecting pitch classes around an axis.

# Formal Definition
Inversion is an operation represented as In, where n is the index number (also called the sum). To invert a pitch class x at In, subtract x from n: In(x) = n - x (mod 12). The index number represents the sum of any pair of pitch classes that map onto each other under that inversion. Unlike transposition, inversion is its own inverse: performing the same In twice returns to the original.

# Mathematical Formulation/Recognition
**For a single pitch class x:**
In(x) = (n - x) mod 12

**Finding the index number:**
If In maps x onto y, then n = x + y (mod 12)

**For a set {a, b, c}:**
In{a, b, c} = {(n-a), (n-b), (n-c)} mod 12

**Key property:**
In(In(x)) = x (inversion is self-inverse)

**Relationship to transposition:**
- Inversion followed by inversion at different indices = transposition
- In followed by Im = T(m-n)

# Musical Context/Application
Inversion preserves interval-class content while reversing the direction of intervals. Lines related by inversion have opposite contours: what goes up in one goes down by the same amount in the other. Sets related by inversion can be written as mirror images of each other in normal form.

# Examples
**Example 2-18** (Bartok, Mikrokosmos, no. 141, "Subject and Reflection"): Two melodies related by I8. Each pair of corresponding notes sums to 8: Bb + Bb = 10 + 10 = 8 (mod 12), Ab + C = 8 + 0 = 8, etc.

**Example 2-21** (Schoenberg, Three Piano Pieces, op. 11, no. 1): Three sets related by I0 and I6. The sets [G, G#, B], [Db, E, F], and [G, Bb, B] show inversional relationships where normal forms appear as mirror images.

# Related Concepts
- Index number
- Transposition (Tn)
- Set class
- Inversional axis
- Inversion (Ixy notation)

# Common Confusions
Inversion (In) should not be confused with inverse transposition. In is an index number (a sum), whereas inverse transpositions are complementary Tn operations. Also, In is its own inverse: to undo In, perform In again (unlike transposition, where Tn requires T(12-n) to undo).

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.4, pp. 53-58
