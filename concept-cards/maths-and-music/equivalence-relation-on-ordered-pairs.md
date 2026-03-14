---
concept: Equivalence Relation on Ordered Pairs
category: theory
source: "Mathematics and Music"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
authors: "David Wright"
---

# Quick Definition
The ratio of two positive real numbers is formally defined as an equivalence class of ordered pairs, where two pairs (a, b) and (a', b') are equivalent when a/b = a'/b'.

# Formal Definition
On the set (R+)^2 of ordered pairs of positive reals, define a relation declaring (a, b) ~ (a', b') if a/b = a'/b', equivalently if a'b = ab'. This is an equivalence relation. The equivalence class of (a, b) is denoted (a : b) or a : b, called the ratio of a and b. The set of all equivalence classes is denoted (R+ : R+). The function phi: (R+ : R+) -> R+ defined by phi((a:b)) = a/b is well-defined, one-to-one, and onto, establishing a bijection between ratio classes and positive reals.

# Mathematical Context
This construction parallels the standard construction of rational numbers from pairs of integers. The equivalence relation ensures that (2:3) = (4:6) = (1/2 : 3/4), just as the fraction 2/3 = 4/6. The bijection phi allows us to identify each ratio class with a single positive real number, which simplifies computation while maintaining the formal foundation. This abstraction is essential because musical intervals operate on pairs of frequencies, not single numbers.

# Musical Context
Since pitches are identified with positive real numbers (frequencies), this equivalence relation applies directly to pairs of pitches (f_2, f_1). The equivalence class f_2 : f_1 captures the interval between them. Different pairs of notes can produce the same interval: for example, (440, 220) and (880, 440) both belong to the class 2:1 (the octave).

# Examples
- (2:3) = (4:6) = (1/2 : 3/4) -- all represent the same ratio
- (440 : 220) = (880 : 440) = (2:1) -- the octave
- The bijection phi maps (3:2) to 3/2 = 1.5, the ratio associated with a nearly perfect fifth

# Related Concepts
- Interval as Frequency Ratio
- Multiplicative Composition of Intervals
- Unison and Opposite Intervals

# Common Confusions
- A ratio is an equivalence class, not a single pair -- many pairs represent the same ratio
- The notation a:b represents a class, while a/b represents the corresponding real number
- This is a ratio of real numbers, not integers -- irrational ratios like sqrt(2):1 are perfectly valid

# Source Reference
Chapter 4: "Ratios and Musical Intervals," pp. 58-59.
