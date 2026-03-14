---
concept: Irrationality of Equally-Tempered Intervals
category: theory
source: "Mathematics and Music"
chapter: "The Rational Numbers As Musical Intervals"
chapter_number: 11
pdf_page: 138
authors: "David Wright"
---

# Quick Definition
All intervals in any equally-tempered scale are irrational (have irrational frequency ratios) except for iterations of the octave. This is a fundamental theorem explaining why equal temperament and just intonation are inherently incompatible.

# Formal Definition
Theorem: Let I be an interval between two notes in an n-tone equally-tempered chromatic scale. If I is not an iteration of octaves (i.e., its ratio is not a power of 2), then I is an irrational interval -- its corresponding ratio lies outside Q+.

# Mathematical Context
Proof sketch: Suppose interval I has rational ratio x in Q+. Since I lies in Z_n (the modular chromatic group), it has finite order, so x^n = 2^k for some positive integer n and integer k. By unique factorization, x = p1^a1 * p2^a2 * ... * pr^ar, giving x^n = p1^(n*a1) * ... * pr^(n*ar) = 2^k. The uniqueness of prime factorization forces 2 to be the only prime among {p1, ..., pr}. Therefore r = 1 and p1 = 2 (unless k = 0, giving unison), so x = 2^(a1), meaning I is an iteration of octaves. This contradicts the assumption that I is not a multi-octave, proving the theorem.

# Musical Context
This theorem tells us that no equally-tempered interval (other than the octave itself) is a just interval. The tempered fifth, major third, minor third, and all other intervals in the 12-tone (or any n-tone) scale are irrational numbers. However, many just intervals are closely approximated by tempered ones -- the fifth at 700 cents vs. 701.96 cents for 3/2, the major third at 400 cents vs. 386.31 cents for 5/4. This likely explains why the 12-chromatic scale gained acceptance: it provides tolerably close approximations of the most important just intervals.

# Examples
- The tempered fifth 2^(7/12) is irrational (close to but not equal to the rational 3/2)
- The tempered major third 2^(4/12) = 2^(1/3) is irrational (not equal to 5/4)
- The tempered semitone 2^(1/12) is irrational
- Only multi-octaves (2^k for integer k) are rational among equally-tempered intervals
- This holds for any n-tone equal temperament, not just 12-tone

# Related Concepts
- Unique Factorization in Q+
- Rational Interval
- Just Interval
- Equal Temperament versus Just Intonation

# Common Confusions
- The theorem does not say equal temperament is "wrong" -- it says equal temperament and just intonation are mathematically distinct systems that can only approximate each other
- The theorem applies to all equally-tempered scales (any n), not just 12-tone
- Augmented triads and diminished seventh chords, which divide the octave equally, can only exist with irrational intervals -- they may be considered a product of equal temperament

# Source Reference
Chapter 11: "The Rational Numbers As Musical Intervals," pp. 145-146.
