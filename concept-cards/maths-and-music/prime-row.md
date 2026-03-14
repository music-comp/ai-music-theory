---
concept: Prime Row
category: technique
source: "Mathematics and Music"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
authors: "David Wright"
unit: null
---

# Quick Definition
The original, top row of a twelve-tone row chart, consisting of an ordered sequence of all 12 (or $n$) note classes that determines the entire chart.

# Formal Definition
The prime row (or original row) is an ordered sequence $a_1, a_2, \ldots, a_n$ of all elements of $\mathbb{Z}_n$, with $a_1 = [0]$. It occupies the first row of the row chart and, through the formula entry$(i,j) = a_j - a_i$, determines every other entry. The number of possible prime rows is $n!$ (since any permutation of the $n$ note classes is valid, though the first element is fixed as $[0]$ when using modular integer representation).

# Mathematical Context
When the prime row is expressed as elements of $\mathbb{Z}_n$, the first entry is $a_1 = [0]$ (representing the designated starting note class). The remaining entries $a_2, \ldots, a_n$ form a permutation of $[1], [2], \ldots, [n-1]$. The number of possible prime rows in the 12-chromatic scale is $12! = 479,001,600$ (when not fixing the starting note).

# Musical Context
The prime row is the creative choice that defines a twelve-tone composition. The composer selects an ordering of all 12 note classes, and this single decision generates the entire row chart through inversion and transposition. The prime row is typically used melodically (horizontally) in the composition.

# Examples
- E, G, F$\sharp$, A, G$\sharp$, C, F, D, D$\sharp$, C$\sharp$, B, B$\flat$, expressed as modular integers from E: [0], [3], [2], [5], [4], [8], [1], [10], [11], [9], [7], [6]
- The spelling may mix sharps and flats with no apparent pattern
- In a 7-tone example: the sequence C, G, D, B, A, E, F corresponds to [0], [4], [1], [6], [5], [2], [3] in $\mathbb{Z}_7$

# Related Concepts
- Row Chart
- Twelve-Tone Technique
- Inversion and Transposition of Rows
- Retrograde

# Common Confusions
- The term "prime" here refers to "first" or "original," not to prime numbers
- The prime row determines the entire row chart; there is no creative freedom in the remaining rows once the prime row is chosen
- When using modular arithmetic notation, the first entry must be $[0]$ for the inversion formula to work correctly

# Source Reference
Chapter 6, "Twelve-Tone Music" section, p. 74 (PDF)
