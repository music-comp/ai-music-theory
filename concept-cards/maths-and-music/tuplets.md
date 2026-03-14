---
concept: Tuplets
category: theory
source: "Mathematics and Music"
chapter: "Horizontal Structure"
chapter_number: 2
pdf_page: 30
authors: "David Wright"
---

# Quick Definition
A method for dividing a note duration into k equal parts where k is not a power of 2, notated by grouping notes under the integer k, enabling rhythmic divisions like triplets and quintuplets.

# Formal Definition
To divide the $\frac{1}{2^n}$-th note into $k$ equal notes (where $k$ is not a power of 2), find the unique positive integer $r$ such that $2^r < k < 2^{r+1}$, then notate the tuplet as a group of $k$ $\frac{1}{2^{n+r}}$-th notes overset or underset by the integer $k$. The resulting tuplet is called the $\frac{1}{2^{n+r}}$-th note $k$-tuplet.

# Mathematical Context
The construction relies on finding $r$ such that $2^r < k < 2^{r+1}$, i.e., $r = \lfloor \log_2 k \rfloor$ (but $k$ is not itself a power of 2). Each note in the $k$-tuplet has duration $\frac{1}{k}$ of the original $\frac{1}{2^n}$-th note, which equals $\frac{1}{k \cdot 2^n}$ of a whole note. The notation uses $\frac{1}{2^{n+r}}$-th notes because $2^{n+r}$ is the nearest power-of-2 subdivision that gives notes slightly longer than the desired duration.

# Musical Context
Tuplets are the most basic form of polyrhythm -- the imposition of simultaneous differing rhythmic patterns. The most common tuplet is the triplet (dividing a duration into 3 equal parts). Tuplets enable rhythmic flexibility beyond the binary subdivision system. The concept has an interesting similarity to harmonics: dividing a duration into $n$ equal parts parallels a vibration $n$ times faster than a fundamental.

# Examples
- **Triplet**: divide quarter note ($\frac{1}{2^2}$-th note) into 3 parts. Since $2^1 < 3 < 2^2$, $r = 1$. Write 3 eighth notes ($\frac{1}{2^3}$-th notes) with "3" above.
- **Quintuplet**: divide quarter note into 5 parts. Since $2^2 < 5 < 2^3$, $r = 2$. Write 5 sixteenth notes ($\frac{1}{2^4}$-th notes) with "5" above.
- An eighth note triplet divides one quarter note's duration equally among three eighth notes
- Exercise 4(c): dividing a whole note into 11 equal notes requires $r = 3$ (since $2^3 < 11 < 2^4$), giving 11 eighth notes with "11" above

# Related Concepts
- Note Durational Values
- Rhythm
- Meter and Time Signatures
- Horizontal Structure

# Common Confusions
- The notation uses $\frac{1}{2^{n+r}}$-th notes that are individually longer than the actual tuplet notes -- the "3" or "5" label signals that $k$ of these notes fill the space of a smaller number
- A triplet of eighth notes does NOT have each note equal to a normal eighth note; each triplet eighth is $\frac{2}{3}$ of a normal eighth
- Tuplets where $k$ IS a power of 2 are not needed since the standard durational system already handles binary subdivisions

# Source Reference
Chapter 2, "Tuplets" section, pp. 34-35 (PDF)
