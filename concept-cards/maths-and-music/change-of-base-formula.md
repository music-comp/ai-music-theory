---
concept: Change of Base Formula
category: theory
source: "Mathematics and Music"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
authors: "David Wright"
---

# Quick Definition
The change of base formula log_b(x) = log_a(x) / log_a(b) allows conversion between logarithms of different bases, making any logarithm computable from any other.

# Formal Definition
For positive reals a, b (both != 1) and x > 0:

(L4) log_b(x) = log_a(x) / log_a(b)

This shows that log_b and log_a are proportional as functions, with constant of proportionality 1/log_a(b). Proof: Let u = log_a(x), v = log_b(x), w = log_a(b). Then a^u = x, b^v = x, a^w = b. From the last two: x = (a^w)^v = a^(wv), so wv = u, giving v = u/w, which is (L4).

# Mathematical Context
The change of base formula means that all logarithmic functions differ only by a constant multiplier. Geometrically, the graph of log_b(x) is a vertical stretch or compression of log_a(x) by the factor 1/log_a(b). For example, log_3(x) = log_6(x) * log_3(6), so log_3 is obtained by stretching log_6 vertically by the factor log_3(6) ~ 1.631.

# Musical Context
The change of base formula is practically essential because calculators typically provide only ln (natural logarithm, base e) or log_10. To compute log_2(r) -- needed for converting ratios to octaves, semitones, or cents -- one uses:

log_2(r) = ln(r) / ln(2)

This allows any ratio-to-additive conversion using a standard calculator.

# Examples
- log_2(r) = ln(r) / ln(2) -- the fundamental conversion formula for musical intervals
- 1200 * log_2(3/2) = 1200 * (ln(3/2) / ln(2)) ~ 701.955 cents
- log_6 and log_3 differ by a vertical stretch factor of log_3(6) ~ 1.631

# Related Concepts
- Logarithm Properties
- Natural Logarithm
- Converting Ratios to Cents
- Converting Ratios to Semitones

# Common Confusions
- The formula divides log_a(x) by log_a(b), not the other way around
- The constant of proportionality 1/log_a(b) depends on both bases but not on x
- This formula works for any pair of valid bases (positive, != 1)

# Source Reference
Chapter 5: "Logarithms and Musical Intervals," pp. 69-70.
