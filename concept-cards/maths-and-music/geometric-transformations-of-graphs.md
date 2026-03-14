---
concept: Geometric Transformations of Graphs
category: theory
source: "Mathematics and Music"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
authors: "David Wright"
---

# Quick Definition
Four operations -- vertical shift, horizontal shift, vertical stretch, and horizontal stretch -- that systematically move or deform the graph of a function, forming the mathematical basis for melodic transformations in music.

# Formal Definition
Let $c \in \mathbb{R}$ and $y = f(x)$ be a function. The four geometric transformations are:
1. **Vertical shift**: $y = f(x) + c$ shifts the graph upward by $c$
2. **Horizontal shift**: $y = f(x - c)$ shifts the graph to the right by $c$
3. **Vertical stretch**: $y = cf(x)$ stretches the graph vertically by factor $c$
4. **Horizontal stretch**: $y = f(x/c)$ stretches the graph horizontally by factor $c$ (where $c \neq 0$)

# Mathematical Context
When $c < 0$ in shifts, the direction reverses (upward becomes downward, rightward becomes leftward). When $0 < c < 1$ in stretches, the transformation is a compression. When $c < 0$ in stretches, a reflection occurs: about the $x$-axis for vertical stretch, about the $y$-axis for horizontal stretch. These transformations compose and can be applied sequentially.

# Musical Context
These transformations have direct musical analogues:
- **Horizontal shift** corresponds to translation (repetition in time)
- **Vertical shift** corresponds to transposition (shifting pitch up or down)
- **Horizontal stretch** corresponds to augmentation or diminution (changing tempo/duration)
- **Vertical stretch** relates to changes in amplitude (dynamics)
- **Horizontal reflection** ($c < 0$ in horizontal stretch) corresponds to retrogression

# Examples
- The graph of $y = x^2 + 1$ is $y = x^2$ shifted up by 1
- The graph of $y = (x - 3)^2$ is $y = x^2$ shifted right by 3
- The graph of $y = 2x^2$ is $y = x^2$ stretched vertically by factor 2
- Musical transposition up a fourth corresponds to a vertical shift of the pitch-vs-time graph

# Related Concepts
- Functions and Graphs
- Translation
- Transposition
- Retrogression

# Common Confusions
- Horizontal shift by $c$ uses $f(x - c)$, not $f(x + c)$ -- the sign is counterintuitive
- Horizontal stretch by $c$ uses $f(x/c)$, not $f(cx)$ -- again counterintuitive
- "Stretch" with $0 < c < 1$ is actually a compression

# Source Reference
Chapter 1, "Transformations of Graphs" section, pp. 16-17 (PDF)
