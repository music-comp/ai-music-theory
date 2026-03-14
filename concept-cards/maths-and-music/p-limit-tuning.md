---
concept: p-Limit Tuning
category: theory
source: "Mathematics and Music"
chapter: "Tuning The Scale To Obtain Rational Intervals"
chapter_number: 12
pdf_page: 152
authors: "David Wright"
---

# Quick Definition
p-limit tuning is a system of intonation that restricts all interval ratios to positive rationals whose prime factorizations involve only primes less than or equal to p.

# Formal Definition
Given a prime number p, the set of positive rational numbers x whose prime factorization x = p1^a1 * p2^a2 * ... * pr^ar has p1, ..., pr <= p forms a subgroup of (Q+, *). A scale or tuning system uses p-limit tuning if all interval ratios between its pitches lie in this subgroup. The key instances are: 3-limit (Pythagorean tuning, ratios of form 2^a * 3^b), 5-limit (just intonation, ratios involving primes 2, 3, 5), and 7-limit (including septimal intervals).

# Mathematical Context
The p-limit subgroups form a nested chain: {1} < 2-limit < 3-limit < 5-limit < 7-limit < 11-limit < ... Each subgroup is closed under multiplication and inversion (taking reciprocals), confirming it is indeed a subgroup of (Q+, *). The p-limit categorization provides a systematic way to classify rational intervals by their harmonic complexity -- lower limits correspond to simpler, more consonant intervals.

# Musical Context
The p-limit concept organizes the historical development of Western tuning: Pythagorean tuning (3-limit) uses only octaves and fifths; just intonation (5-limit) adds thirds; septimal tuning (7-limit) adds the "blue" seventh. Each expansion of the prime limit brings new intervals that are progressively less consonant but harmonically richer. The concept explains why different tuning systems have different strengths: 3-limit gives perfect fifths but poor thirds; 5-limit gives perfect thirds and fifths but cannot serve all keys equally.

# Examples
- 2-limit: only octaves (ratios 2^a), the trivial case
- 3-limit (Pythagorean): ratios 2^a * 3^b -- includes 3/2 (fifth), 9/8 (whole tone), 4/3 (fourth)
- 5-limit (just intonation): adds 5/4 (major third), 6/5 (minor third), 5/3 (major sixth), 10/9 (lesser whole tone)
- 7-limit: adds 7/4 (septimal seventh), 7/6 (septimal minor third), 8/7 (septimal whole tone)
- The comma of Didymus (81/80) is a 5-limit interval; the comma of Pythagoras (3^12/2^19) is a 3-limit interval

# Related Concepts
- Pythagorean Scale
- Just Intonation Scale
- Septimal Intervals
- Unique Factorization in Q+
- Rational Interval

# Common Confusions
- p-limit does not mean "intervals up to p" -- it means primes in the factorization are at most p. The ratio 81/64 is 3-limit despite involving numbers much larger than 3
- The mean-tone scale is not purely p-limit for any p, since it uses irrational intervals (e.g., 5^(1/4))
- Higher p-limit does not automatically mean "better tuning" -- it means more intervals are available, but each system has trade-offs

# Source Reference
Chapter 12: "Tuning The Scale To Obtain Rational Intervals," p. 152; also Chapter 11, Exercise 5, p. 150.
