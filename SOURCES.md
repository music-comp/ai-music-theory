# SOURCES.md — Annotated Source Inventory

## Overview

This document catalogs all sources for the music theory skill, with access paths,
priorities, processing status, and notes on what each contributes.

---

## Core Texts (In Processing Order)

### 1. Open Music Theory (FOUNDATION)
- **Citation**: Gotham, M. et al. (2022). *Open Music Theory*. 
- **Path**: `~/Dropbox/Apps/General Books/Music/[2022] Gotham - Open Music Theory.pdf`
- **Also Available**: XML version (original format)
- **Pages**: 1297
- **License**: CC-BY-SA (can freely use and adapt)
- **Status**: [ ] Not started
- **Priority**: #1 — PROCESS FIRST

**Why First?**
- Establishes baseline terminology Claude needs
- Pedagogical orientation (explains concepts clearly)
- CC-licensed = low stakes for experimentation
- Large but comprehensive = thorough grounding

**Key Content**:
- Fundamentals (pitch, intervals, scales, chords)
- Diatonic harmony and voice leading
- Form and analysis
- Post-tonal theory introduction
- Species counterpoint

**Processing Notes**:
- Large file — process in sections
- XML version may be cleaner for extraction
- Focus on chapters covering fundamentals first
- Skip exercises/worksheets initially

---

### 2. Lewin - Generalized Musical Intervals and Transformations
- **Citation**: Lewin, D. (2007). *GMIT* (Revised Edition). Oxford University Press.
- **Path**: `~/Dropbox/Apps/Oxford University Press/[2007] Lewin - Generalized Musical Intervals and Transformations - Revised Edition.pdf`
- **Pages**: 290
- **Status**: [ ] Not started
- **Priority**: #2 — FOUNDATIONAL THEORY

**Why Important?**
- Defines GIS (Generalized Interval System)
- Establishes "transformational attitude"
- Foundation for all subsequent mathematical music theory

**Key Content**:
- GIS definition and examples
- Interval-preserving operations  
- Simply transitive group actions
- Transformation graphs and networks

**Processing Notes**:
- **LEWIN PROTOCOL APPLIES** (see PIPELINE.md §2)
- Mathematical density requires careful validation
- OCR quality verified as good (marker handles it)
- Chapter-by-chapter validation against physical copy

---

### 3. Tymoczko - A Geometry of Music
- **Citation**: Tymoczko, D. (2011). *A Geometry of Music*. Oxford University Press.
- **Path**: `~/Dropbox/Apps/Oxford University Press/[2011] Tymoczko - A Geometry of Music.pdf`
- **Pages**: 469
- **Status**: [ ] Not started
- **Priority**: #3

**Why Important?**
- Voice-leading as geometric paths
- Chord spaces as orbifolds (T^n/S_n)
- "Extended common practice" framework
- Bridges math and composition

**Key Content**:
- Voice-leading geometry
- Chord space construction
- Efficient voice leading
- Scalar collections
- Five components of tonality

**Processing Notes**:
- Dense mathematical content
- Many diagrams (may need description)
- Complements Lewin but different approach

---

### 4. Tymoczko - Tonality: An Owner's Manual (CROWN JEWEL)
- **Citation**: Tymoczko, D. (2023). *Tonality: An Owner's Manual*. Oxford University Press.
- **Path**: `~/Dropbox/Apps/Oxford University Press/[2023] Tymoczko - Tonality - An Owners Manual.epub`
- **Format**: EPUB (use pandoc)
- **Pages**: ~612 (printed equivalent)
- **Status**: [ ] Not started
- **Priority**: #1 for advanced content (but requires #3 first)

**Why Important?**
- Most mature statement of Tymoczko's theory
- Refines and extends *Geometry of Music*
- More compositional applications
- Hints at categorical/homotopical extensions

**Processing Notes**:
- EPUB format — pandoc conversion
- Verify mathematical notation survives conversion
- May need post-processing for formulas

---

### 5. Cohn - Audacious Euphony
- **Citation**: Cohn, R. (2012). *Audacious Euphony*. Oxford University Press.
- **Path**: `~/Dropbox/Apps/Oxford University Press/[2012] Cohn - Audacious Euphony.pdf`
- **Pages**: 256
- **Status**: [ ] Not started
- **Priority**: #5

**Why Important?**
- Neo-Riemannian theory definitive treatment
- P, L, R operations
- Hexatonic systems
- Bridges 19th-century chromaticism and modern theory

**Key Content**:
- PLR group and its structure
- Hexatonic poles
- Tonnetz and its geometry
- Analyses of Wagner, Brahms, etc.

**Processing Notes**:
- More accessible than Lewin
- Good musical examples

---

## Reference & Pedagogical Texts (Integrated)

These texts supplement the core lineage. Each brings a unique perspective that
helps create a more balanced and complete view. The cognitive load of harmonizing
different approaches is worth the payoff.

---

### 6. Gollin - Oxford Handbook of Neo-Riemannian Music Theories
- **Citation**: Gollin, E. & Rehding, A. (2012). Oxford University Press.
- **Path**: `~/Dropbox/Apps/Oxford University Press/[2012] Gollin - The Oxford Handbook of Neo-Riemannian Music Theories.pdf`
- **Pages**: 624
- **Status**: [ ] Not started
- **Priority**: Reference (consult for specific Neo-Riemannian topics)

**Purpose**: Comprehensive reference for Neo-Riemannian approaches. Multiple 
authors provide varied perspectives. Use to supplement Cohn, not replace.

**Unique Contribution**: Historical context, alternative formalizations, 
applications to specific repertoires that Cohn doesn't cover.

---

### 7. Straus - Introduction to Post-Tonal Theory
- **Citation**: Straus, J. (2016). *Introduction to Post-Tonal Theory* (4th ed.). Norton.
- **Path**: `~/Dropbox/Apps/General Books/Music/[2016] Straus - Introduction to Post-Tonal Theory - 4th Edition.pdf`
- **Pages**: 413
- **Status**: [ ] Not started
- **Priority**: **HIGH for atonal/serial content**

**Purpose**: The standard textbook for pitch-class set theory. Clear, systematic
pedagogy that complements Lewin's more advanced treatment.

**Unique Contribution**:
- Definitive set class terminology and notation
- Comprehensive twelve-tone technique coverage
- Atonal analysis methods with worked examples
- Bridges OMT's introduction to Lewin's abstractions

**Harmonization Notes**: 
- Use Straus for *terminology and basic operations*
- Use Lewin for *transformational perspective on same material*
- Straus provides the "what", Lewin provides the "why it's a GIS"

---

### 8. Wright - Mathematics and Music
- **Citation**: Wright, D. (2009). *Mathematics and Music*. AMS.
- **Path**: `~/Dropbox/Apps/General Books/Music/[2009] Wright - Mathematics and Music.pdf`
- **Pages**: 176
- **Status**: [ ] Not started
- **Priority**: **HIGH for mathematical foundations**

**Purpose**: Accessible bridge between pure mathematics and music theory.
Written by a mathematician for a general audience.

**Unique Contribution**:
- Explicit group theory fundamentals *in musical context*
- Tuning systems and temperament (historical and mathematical)
- Acoustics foundations (overtone series, consonance)
- Change ringing as combinatorics (!)
- Rhythm and meter mathematically formalized

**Harmonization Notes**:
- Complements the abstract-algebra-topology skill
- Provides musical motivation for mathematical structures
- Good source for "why does Z₁₂ matter musically?"
- Tuning content not well covered elsewhere in our sources

---

### 9. Hutchinson - Music Theory for the 21st-Century Classroom
- **Citation**: Hutchinson, R. (2023). 
- **Path**: `~/Dropbox/Apps/General Books/Music/[2023] Hutchinson - Music Theory for the 21st-Century Classroom.epub`
- **Format**: EPUB (use pandoc)
- **Pages**: ~484
- **Status**: [ ] Not started
- **Priority**: **MEDIUM - modern pedagogical perspective**

**Purpose**: Contemporary music theory pedagogy. Represents current teaching
practice and terminology in academic settings.

**Unique Contribution**:
- Most recent pedagogical consensus (2023)
- May include updated terminology/approaches
- Likely addresses contemporary music more than older texts
- Good for "how is this taught now?" perspective

**Harmonization Notes**:
- Cross-reference with OMT for terminology alignment
- Note any terminology drift from older sources
- Useful for pedagogical framing in guides

---

### 10. Carter - Music Theory: From Absolute Beginner to Expert
- **Citation**: Carter (2016). 
- **Path**: `~/Dropbox/Apps/General Books/Music/[2016] Carter - Music Theory - From Absolute Beginner to Expert.pdf`
- **Pages**: 22 (audio book companion)
- **Status**: [ ] Not started
- **Priority**: LOW (but useful for accessibility baseline)

**Purpose**: Extreme beginner perspective. Useful for understanding what
complete novices need explained.

**Unique Contribution**:
- Shows the "explain like I'm five" level
- Good for testing if our explanations are too jargon-heavy
- Accessibility check: can we explain X at this level if needed?

**Harmonization Notes**:
- Not a source of content, but a calibration tool
- If Carter explains something, our "simple" explanation should be at least
  that accessible when requested

---

## Integration Strategy

When building unified concept cards, draw from multiple sources:

| Concept Type | Primary | Secondary | Tertiary |
|--------------|---------|-----------|----------|
| Fundamentals | OMT | Hutchinson | Wright |
| Set Theory | Straus | OMT (intro) | Lewin (transformational view) |
| Voice Leading | Tymoczko | OMT | — |
| Neo-Riemannian | Cohn | Gollin | Tymoczko |
| Mathematical | Wright | Lewin | Papadopoulos |
| Historical | Wright | Gollin | — |

**Harmonization Principle**: When sources differ, document in DEBATES.md.
Don't paper over disagreements—they're often where the interesting insights
live.

---

## Papers

### 10. Papadopoulos - Mathematics and Group Theory in Music
- **Citation**: Papadopoulos, A. (2014).
- **Path**: `~/Dropbox/Apps/Papers/Music Theory/[2014] Papadopoulos - Mathematics and group theory in music/`
- **Formats**: LaTeX source (.tex) AND PDF
- **Pages**: 33
- **Status**: [x] Accessible, content verified
- **Priority**: High value, low effort

**Why Important?**
- Explicit group theory → Messiaen connection
- Modes of limited transposition formalized
- Non-retrogradable rhythms as symmetry
- Bridges our Messiaen companion work

**Processing Notes**:
- LaTeX source available = clean extraction
- Short = easy to process completely

---

### 11. Fiore - Music and Mathematics
- **Citation**: Fiore, T. (2009).
- **Path**: `~/Dropbox/Apps/Papers/Music Theory/[2009] Fiore - Music and Mathematics.pdf`
- **Pages**: 36
- **Status**: [ ] Not started
- **Priority**: Reference

---

### 12. Acef-Sanchez - Group Theory with Musical Applications
- **Citation**: Acef-Sanchez et al. (2012).
- **Path**: `~/Dropbox/Apps/Papers/Music Theory/[2012] Acef-Sanchez - An Introduction to Group Theory with applications to Mathematical Music Theory.pdf`
- **Pages**: 142
- **Status**: [ ] Not started
- **Priority**: Reference (supplements abstract algebra skill)

---

## Special Source: Messiaen

### 13. Messiaen - Technique of My Musical Language
- **Citation**: Messiaen, O. (1944). *Technique de mon langage musical*.
- **Path**: `~/Dropbox/Apps/General Books/Music/[1944] Messiaen - The Technique of My Musical Language.pdf`
- **Pages**: ~100
- **Status**: [ ] Not started
- **Priority**: BONUS — connects to Messiaen Companion project

**Relationship to Project**:
- Source text for the Messiaen Companion we built
- Modes of limited transposition defined here
- Non-retrogradable rhythms explained
- Papadopoulos paper formalizes these concepts

---

## Validation Materials

### Claude Discussion Transcripts
Located in user's archives. ~144KB total across two sessions.

**The Mathematics of Scale Structure** (~32KB)
- Parts I-VII covering scale theory, group theory, "why twelve"

**Generator-Induced Harmony** (~112KB)
- Parts I-IX covering quartal/quintal harmony, compositional applications

**Purpose**: Validation test cases. If the skill can support recreating these
journeys, it's working.

---

## Processing Order Summary

| Phase | Source | Est. Effort | Unlocks |
|-------|--------|-------------|---------|
| 1 | Open Music Theory | High (large) | Foundational concepts, terminology |
| 2 | Lewin GMIT | Medium | GIS, transformations |
| 3 | Tymoczko Geometry | Medium | Voice-leading geometry |
| 4 | Tymoczko Tonality | Medium | Advanced synthesis |
| 5 | Cohn Audacious | Low | Neo-Riemannian |
| 6 | Papadopoulos paper | Low | Messiaen formalization |
| — | Others | As needed | Reference material |

---

## Explicitly Excluded

### Mazzola - The Topos of Music
- **Why Excluded**: Per Tymoczko's documented critique
  - Weak arguments
  - Mathematical contradictions  
  - Failed predictions
  - Overly complex without musical payoff
- **Policy**: May reference individual concepts where well-founded, but do not
  build on this framework

---

*Last updated: [date]*
