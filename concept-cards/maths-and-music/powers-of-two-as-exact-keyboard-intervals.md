---
concept: Powers of Two as Exact Keyboard Intervals
category: theory
source: "Mathematics and Music"
chapter: "The Integers as Intervals"
chapter_number: 9
pdf_page: 110
authors: "David Wright"
unit: null
---

# Quick Definition
The only positive integers that can be played exactly on an equally tempered keyboard are powers of 2 (1, 2, 4, 8, 16, ...). All other integer ratios are necessarily approximated with some error.

# Formal Definition
**Theorem:** The only keyboard intervals which have integer ratios are the powers of 2.

**Proof:** Suppose n is a positive integer that is a keyboard interval. Then n is obtained by composing k semitones for some integer k >= 0. Since the semitone has ratio 2^(1/12), we have n = (2^(1/12))^k = 2^(k/12). Raising to the 12th power: n^12 = 2^k. By the Unique Factorization Theorem, n can have only 2 in its prime factorization. Therefore n is a power of 2.

# Mathematical Context
This theorem applies not just to 12-tone equal temperament but to any m-chromatic scale that equally divides the octave. In an m-chromatic scale, keyboard intervals have ratio 2^(k/m), and the same argument shows only powers of 2 can be integers: n = 2^(k/m) implies n^m = 2^k, so n must be a power of 2.

# Musical Context
This result is profound: it means the equally tempered keyboard can never perfectly render any interval other than multiple octaves. The fifth, fourth, major third, and all other consonant intervals based on primes other than 2 are inherently approximations. This is the fundamental trade-off of equal temperament -- gaining the ability to play in all keys at the cost of pure intervals.

# Examples
- 2^0 = 1: unison (0 semitones), exact
- 2^1 = 2: one octave (12 semitones), exact
- 2^2 = 4: two octaves (24 semitones), exact
- 2^3 = 8: three octaves (36 semitones), exact
- 3: NOT a power of 2, requires approximation (~2 cents error)
- 5: NOT a power of 2, requires approximation (~14 cents error)

# Related Concepts
- Keyboard Approximation of Integer Ratios
- Error Calculation in Cents
- Integral Interval

# Common Confusions
Students sometimes think certain intervals other than octaves are "exact" on the keyboard. They are not. Even the perfect fifth (ratio 3:2) is approximately 2 cents off. The theorem proves this is unavoidable for any equal division of the octave.

# Source Reference
Chapter 9: "The Integers as Intervals," p. 115 (PDF page 110). The theorem and proof appear under "Non-Chromatic Nature of Intervals Other Than Multiple Octaves."
