---
concept: Wrapping Real Line Around Circle
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
---

# Quick Definition
The geometric visualization of modular equivalence as a function that wraps the infinite number line around a circle of circumference $m$, preserving distance as arc length.

# Formal Definition
The wrapping function $w: \mathbb{R} \to \mathbb{R}/{\sim}$ is defined by $w(x) = \bar{x}$ (the equivalence class of $x$ modulo $m$). It maps each real number to its position on a circle of circumference $m$, with $x = 0$ placed at the top. The function preserves the additive structure: $w(x + y) = w(x) + w(y)$, making it a surjective group homomorphism from $(\mathbb{R}, +)$ to $(\mathbb{R}/{\sim}, +)$.

# Mathematical Context
The Generalized Division Algorithm guarantees that for each point $p$ on the circle, there is exactly one representative $r \in [0, m)$ mapping to $p$. The set $\mathbb{R}/{\sim}$ is parameterized by the circle just as $\mathbb{R}$ is parameterized by the line. The restriction to $\mathbb{Z}$ maps integers to $m$ equally spaced points on the circle, giving the "m-hour clock" representation of $\mathbb{Z}_m$.

# Musical Context
This wrapping models octave equivalence geometrically. When $m = 12$ (semitones), wrapping the real line of all possible semitone counts around a circle of circumference 12 identifies all octave-equivalent intervals. The 12 clock positions represent the 12 note classes. The continuous wrapping (for $\mathbb{R}$) captures all intervals, while the discrete wrapping (for $\mathbb{Z}$) captures only chromatic intervals.

# Examples
- For $m = 8$: the numbers $0, 8, 16, -8$ all wrap to the top of the circle; $3$ and $11$ wrap to the same point
- For $m = 12$: the integers $0, 1, 2, \ldots, 11$ map to 12 equally spaced positions (the chromatic note classes)
- The origin $x = 0$ is conventionally placed at the top of the circle
- The number 6.5 with $m = 8$ wraps to the same position as $-1.5$

# Related Concepts
- Modular Equivalence on the Real Numbers
- Modular Equivalence on the Integers
- Homomorphism
- Generalized Division Algorithm
- Modular Clock
- Octave Equivalence Formalized

# Common Confusions
- The wrapping function is a homomorphism but not an isomorphism; many distinct real numbers map to the same point on the circle
- The circle has circumference $m$, not radius $m$ or diameter $m$
- Wrapping preserves distance locally (as arc length) but identifies points that differ by multiples of $m$

# Source Reference
Chapter 7, "Modular Equivalence on the Real Numbers" section, p. 82 (PDF)
