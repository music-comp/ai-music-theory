---
concept: Geometric Series and Summation
category: theory
source: "Mathematics and Music"
chapter: "Horizontal Structure"
chapter_number: 2
pdf_page: 30
authors: "David Wright"
---

# Quick Definition
The formula for summing a finite geometric series, $1 + r + r^2 + \cdots + r^m = \frac{1 - r^{m+1}}{1 - r}$, which underlies the mathematics of dotted note durations and converges to $\frac{1}{1-r}$ when $|r| < 1$.

# Formal Definition
For any integer $m \geq 0$ and any real number $r \neq 1$:
$$\sum_{i=0}^{m} r^i = 1 + r + r^2 + \cdots + r^m = \frac{1 - r^{m+1}}{1 - r}$$
The proof uses the telescoping product $(1 - r)(1 + r + r^2 + \cdots + r^m) = 1 - r^{m+1}$.

For $|r| < 1$, the infinite geometric series converges:
$$\sum_{i=0}^{\infty} r^i = \frac{1}{1 - r}$$

# Mathematical Context
The finite geometric series formula is proved by considering the product $(1 - r)(1 + r + r^2 + \cdots + r^m)$, which telescopes to $1 - r^{m+1}$ (Exercise 3, Chapter 2). When $r = \frac{1}{2}$, the partial sums are $1, \frac{3}{2}, \frac{7}{4}, \frac{15}{8}, \ldots$, approaching the limit 2. The infinite series $\sum_{i=0}^{\infty}\left(\frac{1}{2}\right)^i = 2$ involves the concept of limit from calculus.

# Musical Context
The geometric series with $r = \frac{1}{2}$ directly models dotted note durations. A note of duration $d$ with $m$ dots has duration $d \cdot \sum_{i=0}^{m}\left(\frac{1}{2}\right)^i$. The convergence to $2d$ means that no matter how many dots are added, the total duration never reaches twice the original. This elegant mathematical result explains why dotted notes behave as they do in the durational system.

# Examples
- $r = \frac{1}{2}, m = 1$: $1 + \frac{1}{2} = \frac{3}{2}$ (single dot multiplier)
- $r = \frac{1}{2}, m = 2$: $1 + \frac{1}{2} + \frac{1}{4} = \frac{7}{4}$ (double dot multiplier)
- $r = \frac{1}{2}, m = 3$: $1 + \frac{1}{2} + \frac{1}{4} + \frac{1}{8} = \frac{15}{8}$ (triple dot multiplier)
- $r = \frac{1}{2}, m \to \infty$: $\sum = 2$ (the limiting duration factor)
- General: $\sum_{i=0}^{4} 3^i = \frac{1 - 3^5}{1 - 3} = \frac{-242}{-2} = 121$

# Related Concepts
- Dotted Note Duration Formula
- Note Durational Values
- Sets and Number Systems

# Common Confusions
- The formula requires $r \neq 1$; when $r = 1$, the sum is simply $m + 1$
- The infinite series converges only when $|r| < 1$; for $|r| \geq 1$ it diverges
- The proof relies on the algebraic identity $(1-r)(1 + r + \cdots + r^m) = 1 - r^{m+1}$, not on induction (though induction also works)

# Source Reference
Chapter 2, "Dots" section, pp. 32-34 (PDF); equation (2.2); Exercise 3
