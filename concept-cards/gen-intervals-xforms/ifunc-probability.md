---
concept: IFUNC as Probability Distribution
slug: ifunc-probability

category: generalized-set-theory
subcategory: interval-functions
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "5.1.8"

extraction_confidence: high

aliases:
  - "Theorem 5.1.8"
  - IFUNC probability interpretation

prerequisites:
  - ifunc
  - set-in-gis
extends:
  - ifunc
related:
  - emb-probability
  - convolution-interpretation
contrasts_with: []

answers_questions:
  - "How can IFUNC be interpreted as a probability distribution?"
  - "How does IFUNC model statistical intervallic textures?"
---

# Quick Definition
IFUNC can be interpreted as a probability distribution: IFUNC(X, Y)(i)/(MN) measures the probability that a randomly selected pair (s from X, t from Y) will span interval i.

# Core Definition
Theorem 5.1.8: "Let X and Y have respective cardinalities M and N. Select a member s of X at random and a member t of Y at random. Then the number IFUNC(X, Y)(i)/(MN) measures the probability that int(s, t) will be found to equal i" (Lewin, p. 133). The proof: there are MN possible pairs, and IFUNC(X, Y)(i) of them have int(s, t) = i.

# Prerequisites
- **IFUNC** — The function being given probabilistic interpretation
- **Set in a GIS** — X and Y must be finite sets with cardinalities M and N

# Key Properties
1. P(int(s, t) = i) = IFUNC(X, Y)(i) / (M * N)
2. Sum of all probabilities = 1 (since sum of IFUNC values = MN)
3. IFUNC(X, Y)(i) = 0 means interval i cannot occur between X and Y
4. Maximal IFUNC values indicate intervals that appear "often"
5. Minimal nonzero IFUNC values indicate "scarce" intervals

# Construction / Recognition
## To Compute:
1. Calculate IFUNC(X, Y)(i) for the interval of interest
2. Divide by card(X) * card(Y)
3. The result is the probability of randomly drawing that interval

## To Recognize:
1. When IFUNC values are described as "frequent" or "scarce," the probabilistic backdrop is implicit

# Context & Application
The probabilistic interpretation allows IFUNC to model statistical intervallic textures, particularly in improvisatory or aleatoric contexts. When two instruments improvise on different pitch collections, IFUNC predicts the statistical distribution of intervals between them. Even in composed music, the probabilistic backdrop tells us which intervals are statistically "rare" or "common" between two sets, providing a null hypothesis against which compositional choices can be evaluated. Lewin uses this in the Schoenberg Violin Fantasy analysis.

# Examples
**Example 1** (pp. 131-132, Figure 5.8): In Schoenberg's Violin Fantasy op. 47, violin plays Y = {Bb, A, C#, B, F, G} while piano plays X = {Eb, E, C, D, Ab, Gb}. IFUNC(X, Y) shows "many" odd intervals and "few" even intervals. IFUNC(X, Y)(0) = 0 (no common tones). The "scarce" intervals 4 and 8 (each appearing only twice) bind boundary tones of the phrase, an analytically significant finding against the statistical backdrop.

**Example 2** (p. 133): A clarinet improvising on pitch set X and a flute improvising on Y would produce a statistical field of intervals modeled by IFUNC(X, Y) according to Theorem 5.1.8.

# Relationships
## Builds Upon
- **IFUNC** — Gives IFUNC a probabilistic meaning

## Enables
- **EMB Probability** — Analogous probabilistic interpretation for EMB

## Related
- **Convolution Interpretation** — Another mathematical lens on IFUNC

# Common Errors
- **Error**: Interpreting the probability as predicting what will happen in a composition
  **Correction**: In composed music, intervals are not randomly distributed; the probability provides a backdrop for comparison, not a prediction

# Common Confusions
- **Confusion**: Thinking "scarce" intervals are musically unimportant
  **Clarification**: Lewin shows that statistically scarce intervals (like 4 and 8 in the Schoenberg example) often have special structural significance precisely because of their rarity

# Source Reference
Chapter 5: Generalized Set Theory (1), Theorem 5.1.8 and Figure 5.8 analysis, pp. 131-133.

# Verification Notes
- Definition source: Direct from Theorem 5.1.8
- Confidence rationale: Explicit theorem with proof and extended analytical application
- Re-extraction notes: Re-extracted from v2 card; preserved: Schoenberg Violin Fantasy example, clarinet/flute improvisation example, emphasis on "scarce" intervals. Added v3.1 structure.
