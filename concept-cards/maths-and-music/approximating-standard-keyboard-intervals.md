---
concept: Approximating Standard Keyboard Intervals
category: application
source: "Mathematics and Music"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
authors: "David Wright"
---

# Quick Definition
The process of finding the closest interval in a non-standard n-chromatic scale to a given standard 12-chromatic keyboard interval.

# Formal Definition
To express $\ell$ semitones in n-chromatic units: $\ell \text{ semitones} = \frac{n}{12} \cdot \ell$ n-chromatic units. The best approximation is the nearest integer. More generally, to convert any frequency ratio $r$ to n-chromatic units: $x = n \log_2 r = n \frac{\ln r}{\ln 2}$.

# Mathematical Context
Since a semitone is $\frac{1}{12}$ of an octave and an n-chromatic unit is $\frac{1}{n}$ of an octave, the conversion factor is $\frac{n}{12}$. The n-chromatic unit in cents is $\frac{1200}{n}$, so $k$ n-chromatic units equals $\frac{1200k}{n}$ cents. The approximation error for a keyboard interval is the difference between the exact value $\frac{n\ell}{12}$ and its nearest integer, measured in n-chromatic units and convertible to cents.

# Musical Context
When experimenting with non-standard chromatic scales, musicians need to know how closely familiar intervals can be reproduced. Some non-standard scales approximate certain standard intervals well (e.g., the tritone is exact in any even-n scale) while others may be poorly approximated, affecting the recognizability of familiar chords and melodies.

# Examples
- In the 14-chromatic scale, a fourth (5 semitones) $= \frac{7}{6} \cdot 5 = \frac{35}{6} \approx 5.833$ units, best approximated by 6 units $= 6 \cdot \frac{1200}{14} \approx 514.29$ cents (14.29 cents sharp)
- The tritone is exactly $n/2$ units in any even-$n$ scale (being exactly half an octave)
- The ratio 0.75 in the 14-scale: $14 \cdot \frac{\ln 0.75}{\ln 2} \approx -5.81$ units (5.81 units downward)

# Related Concepts
- N-Chromatic Scale
- Non-Standard Chromatic Scales
- Twelve-Chromatic Scale

# Common Confusions
- The "best approximation" is the nearest integer number of n-chromatic units, but this may still differ substantially from the keyboard interval in cents
- Approximation quality varies by interval; a scale that well-approximates fifths may poorly approximate thirds

# Source Reference
Chapter 6, "Approximating Standard Keyboard Intervals" section, p. 74 (PDF)
