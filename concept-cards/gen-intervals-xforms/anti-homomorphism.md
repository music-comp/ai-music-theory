---
concept: Anti-homomorphism
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
An anti-homomorphism is a function between semigroups that reverses the order of products: f(x1x2) = f(x2)f(x1).

# Formal Definition
An anti-homomorphism of one semigroup into another is a function f satisfying f(x1)f(x2) = f(x2x1). This reverses the order of composition compared to a homomorphism. Every anti-homomorphism of (X, BIN) is a homomorphism of (X, ANTIBIN) where ANTIBIN(x1, x2) = BIN(x2, x1).

# Mathematical Formulation
- f: (X, BIN) -> (X', BIN') is an anti-homomorphism if f(x1)f(x2) = f(x2x1)
- Equivalently: f(x1x2) = f(x2)f(x1)
- Define ANTIBIN(x1, x2) = BIN(x2, x1)
- Then every anti-homomorphism of (X, BIN) is a homomorphism of (X, ANTIBIN)
- Anti-isomorphism: an anti-homomorphism that is 1-to-1 and onto

# Musical Context/Application
Anti-homomorphisms arise when dealing with both transposition and interval-preserving operations. The map from intervals i to transposition operations Ti is an anti-isomorphism: TiTj = Ti+j but the map sends i to Ti, and the product in the interval group is ij = i + j while TiTj corresponds to j + i in the reversed sense. Understanding when maps are homomorphisms vs. anti-homomorphisms is essential for keeping track of operation order.

# Examples
From Section 1.11.4: Consider a group with elements i, j, k, ... and two families of operations: "transpositions" Ti and "interval-preserving operations" Pi.
- P-operations combine: PiPj = Pij (homomorphism)
- T-operations combine: TiTj = Tji (anti-homomorphism)
- The map i -> Pi is an isomorphism
- The map i -> Ti is an anti-isomorphism

This distinction arises from left vs. right orthography: using right orthography for T-operations would make that map an isomorphism instead.

# Related Concepts
- Homomorphism
- Isomorphism
- Left Orthography
- Right Orthography
- Transposition Operations
- Interval-Preserving Operations

# Common Confusions
- Anti-homomorphisms reverse order: f(xy) = f(y)f(x), NOT f(xy) = f(x)f(y)
- An anti-homomorphism to a commutative group is also a homomorphism
- Switching orthography conventions can turn anti-homomorphisms into homomorphisms
- The distinction matters crucially for non-commutative groups

# Source Reference
Chapter 1: Mathematical Preliminaries, Section 1.11.4
