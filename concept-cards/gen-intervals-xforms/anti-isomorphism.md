---
concept: Anti-Isomorphism
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
An anti-isomorphism is a bijective map between groups that reverses the order of multiplication: f(ab) = f(b)f(a). The transposition operations form a group anti-isomorphic to IVLS, explaining why transposition composition "reverses" interval order.

# Formal Definition
A function f: G -> H between groups is an anti-isomorphism if:
1. f is bijective (1-to-1 and onto)
2. f(ab) = f(b)f(a) for all a, b in G

Note: condition (2) says f reverses the multiplication order.

Contrast with isomorphism: f(ab) = f(a)f(b) (preserves order).

# Mathematical Formulation
**Theorem 3.4.2:** The map f: IVLS -> TNSPS defined by f(i) = T_i is an anti-isomorphism.

**Key formula:**
T_i T_j = T_{ji}    (not T_{ij})

**Proof idea:**
int(s, T_i(T_j(s))) = int(s, T_j(s)) * int(T_j(s), T_i(T_j(s)))
                    = j * i
                    = ji

So T_j(s) then T_i equals T_{ji}(s).

**Consequence:**
- f(ij) = T_{ij}
- f(i)f(j) = T_i T_j = T_{ji}
- Thus f(ij) =/= f(i)f(j) unless ij = ji

# Musical Context/Application
The anti-isomorphism explains why we must be careful with transposition composition:
- "Transpose by i then transpose by j" is NOT "transpose by ij"
- It IS "transpose by ji"

In commutative groups (like pitch-class intervals), ij = ji, so this subtlety disappears. In non-commutative groups, it matters.

# Examples
**Commutative case (pitch classes):**
T_5 T_3 = T_{3+5} = T_8 = T_{5+3}
Order doesn't matter since 3 + 5 = 5 + 3 mod 12.

**Non-commutative case (time spans):**
T_{(1,2)} T_{(3,4)} = T_{(3,4)(1,2)}
                    = T_{(3 + 4*1, 4*2)}
                    = T_{(7, 8)}

T_{(3,4)} T_{(1,2)} = T_{(1,2)(3,4)}
                    = T_{(1 + 2*3, 2*4)}
                    = T_{(7, 8)}

(These happen to be equal, but try other pairs.)

**Contrast P operations:**
P_i P_j = P_{ij}    (isomorphism, preserves order)
T_i T_j = T_{ji}    (anti-isomorphism, reverses order)

# Related Concepts
- Group Isomorphism
- Transposition Operation (Ti)
- Group of Transpositions (TNSPS)
- Interval-Preserving Operations (isomorphism)
- Non-Commutative Groups

# Common Confusions
1. **Anti- doesn't mean "opposite":** Anti-isomorphism is still a bijection preserving algebraic structure. It just reverses multiplication order.

2. **In commutative groups:** Anti-isomorphism = isomorphism (since ab = ba, reversing order doesn't change anything).

3. **Reading order:** "T_i then T_j" means apply T_j first, then T_i. The result is T_{ji}, with j appearing first in the subscript product.

4. **This is why we care:** The anti-isomorphism between IVLS and TNSPS is NOT a defect. It's the correct algebraic relationship.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.4.2, pp. 77-79
