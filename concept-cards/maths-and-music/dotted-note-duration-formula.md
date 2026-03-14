---
concept: Dotted Note Duration Formula
category: theory
source: "Mathematics and Music"
chapter: "Horizontal Structure"
chapter_number: 2
pdf_page: 30
authors: "David Wright"
---

# Quick Definition
A dot after a note extends its duration by half, a second dot adds a quarter, and in general m dots multiply the original duration d by $[2 - (1/2)^m]$, approaching but never reaching 2d.

# Formal Definition
A note of duration $d$ followed by $m$ dots has duration:
$$d_m = d\left[2 - \left(\frac{1}{2}\right)^m\right]$$
This is derived from the geometric series:
$$d_m = d\left(1 + \frac{1}{2} + \frac{1}{2^2} + \cdots + \frac{1}{2^m}\right) = d\sum_{i=0}^{m}\left(\frac{1}{2}\right)^i = d\left[\frac{1 - \left(\frac{1}{2}\right)^{m+1}}{1 - \frac{1}{2}}\right]$$

Equivalently: $d_m = d\left[1 + \frac{2^m - 1}{2^m}\right]$.

# Mathematical Context
The formula uses the finite geometric series sum $\sum_{i=0}^{m} r^i = \frac{1 - r^{m+1}}{1 - r}$ for $r \neq 1$, applied with $r = \frac{1}{2}$. As $m \to \infty$, $d_m \to 2d$, since $\lim_{m \to \infty}\left[2 - \left(\frac{1}{2}\right)^m\right] = 2$. The infinite sum $\sum_{i=0}^{\infty}\left(\frac{1}{2}\right)^i = 2$ captures this limit. The value $d_m$ is always strictly less than $2d$ for any finite $m$.

# Musical Context
One dot multiplies duration by $3/2$ (the most common case). Two dots multiply by $7/4$. Three dots multiply by $15/8$. In practice, more than two dots are rarely used. The dotted note is fundamental to rhythmic notation: a dotted quarter note in $\frac{4}{4}$ time has duration $\frac{3}{2}$ beats, a dotted half note has 3 beats. Dots apply to rests as well.

# Examples
- Dotted sixteenth note (4 beats per whole note): $d = \frac{1}{4}$, $m = 1$: $d_1 = \frac{1}{4} \cdot \frac{3}{2} = \frac{3}{8}$ beats
- Double-dotted sixteenth: $d_2 = \frac{1}{4} \cdot \frac{7}{4} = \frac{7}{16}$ beats
- Triply dotted sixteenth (2 beats per whole note): $d = \frac{1}{8}$, $m = 3$: $d_3 = \frac{1}{8}\left[2 - \frac{1}{8}\right] = \frac{1}{8} \cdot \frac{15}{8} = \frac{15}{64}$ beats
- A dotted half note in $\frac{4}{4}$: $d = 2$ beats, $d_1 = 2 \cdot \frac{3}{2} = 3$ beats

# Related Concepts
- Note Durational Values
- Geometric Series and Summation
- Meter and Time Signatures
- Ties and Slurs

# Common Confusions
- Each dot adds half of the previous addition, not half of the original duration: the second dot adds $d/4$ (not $d/2$), the third adds $d/8$
- The total duration of an $m$-dotted note always falls strictly below $2d$, never reaching it
- Dots apply to rests as well as to notes

# Source Reference
Chapter 2, "Dots" section, pp. 32-34 (PDF); equation (2.1)
