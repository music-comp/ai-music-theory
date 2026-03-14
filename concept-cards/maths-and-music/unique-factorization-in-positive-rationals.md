---
concept: Unique Factorization in Q+
category: theory
source: "Mathematics and Music"
chapter: "The Rational Numbers As Musical Intervals"
chapter_number: 11
pdf_page: 138
authors: "David Wright"
---

# Quick Definition
Every positive rational number can be uniquely expressed as a product of distinct primes raised to nonzero integer powers (positive or negative), extending the fundamental theorem of arithmetic from integers to rationals.

# Formal Definition
Let x be in Q+. Then x can be factored as x = p1^a1 * p2^a2 * ... * pr^ar where r >= 0, p1, p2, ..., pr are distinct primes, and a1, a2, ..., ar are nonzero integers. This factorization is unique up to reordering of factors. If a1, ..., ai are positive and a(i+1), ..., ar are negative, setting bj = -a(i+j) and qj = p(i+j), we can write x = (p1^a1 * ... * pi^ai) / (q1^b1 * ... * qs^bs) with all primes distinct and all exponents positive.

# Mathematical Context
This theorem extends the Fundamental Theorem of Arithmetic (unique factorization in Z+) to Q+. The key difference is that exponents may be any nonzero integer, not just positive integers, since denominators introduce negative exponents. The theorem follows from unique factorization in Z+ by expressing the rational as a fraction and canceling common prime factors.

# Musical Context
This factorization is the primary tool for analyzing rational intervals. The primes appearing in the factorization determine the "character" of the interval: intervals involving only primes 2 and 3 are Pythagorean; those involving 2, 3, and 5 are in the 5-limit; those involving 7 are septimal. The uniqueness of factorization underpins the proof that equally-tempered intervals (except multi-octaves) are irrational.

# Examples
- 3/2 = 2^(-1) * 3^1 (just fifth: primes 2 and 3)
- 5/4 = 2^(-2) * 5^1 (just major third: primes 2 and 5)
- 81/80 = 2^(-4) * 3^4 * 5^(-1) (comma of Didymus: primes 2, 3, and 5)
- 1,222,452/11,180,400 reduces to (7^2 * 3^3)/(11^2 * 5^2 * 2^2) by factoring numerator and denominator

# Related Concepts
- Rational Interval
- p-Limit Tuning
- Irrationality of Equally-Tempered Intervals
- Comma of Didymus
- Comma of Pythagoras

# Common Confusions
- Unlike Z+ factorization, exponents here can be negative (representing factors in the denominator)
- The factorization applies to the ratio after full cancellation -- one must first reduce the fraction before reading off the prime structure
- A rational number x is an integer if and only if all exponents in its prime factorization are nonnegative

# Source Reference
Chapter 11: "The Rational Numbers As Musical Intervals," pp. 138-139.
