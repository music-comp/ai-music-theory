---
concept: Cents
category: theory
source: "Mathematics and Music"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
authors: "David Wright"
---

# Quick Definition
A cent is 1/100 of a semitone, or equivalently 1/1200 of an octave. It provides a fine-grained additive unit for measuring musical intervals, particularly useful for microtuning.

# Formal Definition
The cent is defined by dividing each semitone into 100 equal intervals, so that 1200 cents equal one octave. The ratio corresponding to one cent is:

c = 2^(1/1200) ~ 1.0005778

The interval of x cents has ratio:

r = 2^(x/1200)    (Formula 4.3)

Cents, like semitones and octaves, is an additive measurement of intervals.

# Mathematical Context
Cents are the special case of n-chromatic units with n = 1200. The conversion formula r = 2^(x/1200) is a specific instance of the general formula r = 2^(x/n). The inverse conversion (ratio to cents) is x = 1200 * log_2(r), derived in Chapter 5. Cents provide sufficient resolution for practical purposes: 1 cent is imperceptible to most listeners, and even 10 cents is difficult to perceive.

# Musical Context
Cents are the standard unit for describing deviations from equal temperament, comparing tuning systems, and specifying microtuning adjustments. They allow precise quantification of small differences that cannot be expressed in whole semitones. For example, the just perfect fifth (ratio 3/2) is approximately 702 cents, compared to the equal-tempered fifth at exactly 700 cents -- a difference of about 2 cents.

# Examples
- 1 cent: ratio 2^(1/1200) ~ 1.0005778 (imperceptible)
- 17 cents: ratio 2^(17/1200) ~ 1.009868
- 100 cents = 1 semitone
- 1200 cents = 1 octave
- The just fifth (3/2) is approximately 702 cents vs. the tempered fifth at 700 cents

# Related Concepts
- Semitone Ratio
- N-Chromatic Units
- Microtuning
- Converting Ratios to Cents
- Multiplicative and Additive Measurements

# Common Confusions
- Cents are not "hundredths of a semitone" in the frequency sense; they are equal *ratio* subdivisions
- 1 cent is effectively inaudible; cents are a measurement tool, not a perceptual unit
- Cents are additive (100 + 100 = 200 cents = 2 semitones), not multiplicative

# Source Reference
Chapter 4: "Ratios and Musical Intervals," pp. 61-62.
