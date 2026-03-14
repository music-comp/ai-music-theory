---
concept: Sieve of Eratosthenes
category: technique
source: "Mathematics and Music"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
authors: "David Wright"
unit: null
---

# Quick Definition
An ancient systematic procedure for finding all prime numbers up to a given limit by iteratively crossing out multiples of each prime, attributed to the Greek mathematician Eratosthenes.

# Formal Definition
To find all primes $\leq n$: list the integers $1, 2, 3, \ldots, n$. Cross out $1$. For each remaining integer $m > 1$ (starting with $m = 2$), cross out all multiples of $m$ that are greater than $m$. After processing all $m$ up to $\sqrt{n}$, the remaining numbers are precisely the primes $\leq n$.

# Mathematical Context
The algorithm works because any composite number $n$ must have a prime factor $\leq \sqrt{n}$. After crossing out 1, multiples of 2, multiples of 3, and continuing, only primes remain. The process need only check divisors up to $\sqrt{n}$ because if $n = ab$ with $a \leq b$, then $a \leq \sqrt{n}$. The sieve demonstrates that primes become sparser as numbers grow, though there are infinitely many.

# Musical Context
The sieve identifies which integers are prime, which directly relates to prime intervals in music. It also helps determine the Euler phi function $\phi(n)$ (by identifying which integers are relatively prime to $n$ through their prime factorizations), which counts the generating intervals in the n-chromatic scale.

# Examples
- Sieving up to 30: cross out 1; cross out multiples of 2 (4, 6, 8, ...); cross out multiples of 3 (9, 15, 21, 27); cross out multiples of 5 (25). Primes remaining: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29
- Only need to check up to $\sqrt{30} \approx 5.48$, so checking 2, 3, and 5 suffices
- After the sieve: the 10 primes $\leq 30$ are identified

# Related Concepts
- Prime Numbers
- Unique Prime Factorization
- Euler Phi Function

# Common Confusions
- The sieve crosses out 1 first; 1 is not considered prime
- You only need to sieve up to $\sqrt{n}$, not up to $n$ itself; this is a significant efficiency gain
- The sieve finds ALL primes up to $n$, not just some of them; it is a complete algorithm

# Source Reference
Chapter 8, "Sieve of Eratosthenes" section, p. 100 (PDF)
