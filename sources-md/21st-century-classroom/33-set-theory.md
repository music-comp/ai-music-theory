# Chapter 33: Set Theory

<!-- Original line: 7735 -->

# <span id="page-476-2"></span><span id="page-476-0"></span>**Chapter 33**

# **Set Theory**

## <span id="page-476-1"></span>**33.1 Set Theory**

**Set theory** is the analytical technique we will use to analyze expressionist music. The primary composers associated with expressionism are Arnold Schoenberg (1874–1951), Anton Webern (1883–1945), and Alban Berg (1885–1935). In this text, we will associate **atonal music**—music that avoids traditional harmonies and scales—with expressionism. Instead of scales and chords, intervals are the building blocks of Expressionist music. Although composers began writing atonal music in 1908, there was no widely-accepted systematic analytical approach that could show relationships between different pieces until Allen Forte published his seminal The Structure of Atonal Music in 1973, in which Forte applied set theory mathematics to music. However, our approach to normal form and prime form will follow the slightly-modified approach set out by John Rahn in his Basic Atonal Theory (1980), which is the approach followed by Joseph Straus in his well-known and widely-used Introduction to Post-Tonal Theory. 1

## **33.1.1 Atonal Music**

Listen to the following example by Anton Webern.

![](./images/_page_476_Figure_6.jpeg)

![](./images/_page_476_Picture_7.jpeg)

**Figure 33.1.1** Webern, 5 movements for string quartet, No. 3. *Sehr bewegt*

Gone are the triadic structures we have studied throughout this text. In this music, intervals are paramount. Let us examine the intervals we find.

<sup>1</sup>Of the 208 sets that exist, only 6 are different between the Forte and Rahn methods for prime form. See [https://www.mta.ca/pc-set/pc-set\\_new/pages/pc-table/packed.html.](https://www.mta.ca/pc-set/pc-set_new/pages/pc-table/packed.html)

<span id="page-477-1"></span>![](./images/_page_477_Picture_2.jpeg)

Look at the intervallic structure of the first two chords,<sup>2</sup> not including the C# in the cello part. We see the interval of an augmented 5th below the interval of a minor 3rd in the first chord, and the interval of a minor sixth below the interval of a minor 3rd in the second chord. Notice that the names we use for intervals carry tonal implications. An augmented 5th would function differently than a minor 6th, but in atonal music, these intervals have the same sound, are separated by the same number of half steps, and have no tonal implications (they don't have to resolve any particular way). Therefore, analysts like Allen Forte used integers to represent pitches and intervals to remove the tonal implications of staff notation.

#### <span id="page-477-0"></span>33.1.2 Integer Notation for Pitches

One notable trait of set theory is that we will represent pitches with integers, as seen in the table below.

Note name: C C#/D
$$\flat$$
 D D#/E $\flat$  E F F#/G $\flat$  G G#/A $\flat$  A A#/B $\flat$  B Integer: 0 1 2 3 4 5 6 7 8 9 10 11

It may be helpful to remember that the C major triad (C, E, and G) consists of integers 0, 4, and 7.

Integer notation of pitches means we assume **enharmonic equivalence** of notes. For example, D, C\*, and E\* are all represented as pitch integer 2. We also assume **octave equivalence**, which itself presumes the notion of **pitch class**. When we say Beethoven's first symphony is in C, we refer not to any specific C ( $C_1$ ,  $C_2$ ,  $C_3$ , etc.), but to the concept of the pitch class C, which includes any and all Cs. Therefore, you would label the note C as pitch class 0, no matter the register in which it occurs.

#### 33.1.3 Integer Notation for Intervals

We will also measure intervals using integers, with each interval represented by the number of **semitones** (half steps) it contains. The following table contains the number of semitones in each interval.

<sup>&</sup>lt;sup>2</sup>Some authors call atonal chords "sonorities" to differentiate them from chords in the traditional triadic sense; we will continue to use "chord" in this text.

Table 33.1.2 Interval Integers

| Interval | Number of Semitones | Interval | Number of Semitones |
|----------|---------------------|----------|---------------------|
| m2       | 1                   | P5       | 7                   |
| M2       | 2                   | m6       | 8                   |
| m3       | 3                   | M6       | 9                   |
| M3       | 4                   | m7       | 10                  |
| P4       | 5                   | M7       | 11                  |
| TT       | 6                   | P8       | 12                  |

#### 33.1.4 Pitch-Class Sets

In atonal music we will analyze sets of pitch classes, hence the term "pitch-class set analysis." Let us return to the example by Webern, this time with integers for pitches and for intervals.

![](./images/_page_478_Picture_6.jpeg)

The first chord consists of  $E_{\flat}$ , B, and D, or pitch integers 3, 11, and 2. If we examine the intervallic distance, we find 8 semitones between pitch integers 3 and 11, and 3 semitones between 11 and 2. Note that we are working in a modulo 12 system, meaning we restart our numbering after 11 (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 1, 2, 3, etc.). We are used to modulo 12 thinking since we all deal with clocks. If a meeting ran from 11am to 2pm, it lasted 3 hours. Therefore, an interval from pitch integer 11 to pitch integer 2 spans 3 semitones. The second chord has the same intervallic construction.

Now, let's look at the two chords in the second half of the third measure.

![](./images/_page_478_Picture_9.jpeg)

We see right away that the second of these chords has the same construction as the two chords we examined the in earlier examples (a minor 6th below a minor 3rd). However, the first chord in this example ( $G_{\sharp}$ , C, A, or 8, 0, 9) appears to be different, with a diminished 4th from  $G_{\sharp}$  to C (an interval spanning 4 semitones, enharmonically equivalent to a major 3rd) below the interval of a major 6th from C to A (spanning 9 semitones). To see the relationship of this chord to the others, we need to learn about normal form and prime form.

## <span id="page-479-1"></span><span id="page-479-0"></span>33.2 Normal Form

**Normal form** represents the notes of a pitch-class set (as they occur in the music) in their most compact form.

To determine normal form, follow these steps:

1. Put the notes of a pitch-class set from a piece of music in ascending numeric order (like a scale). Eliminate any duplicate pitches.

![](./images/_page_479_Figure_6.jpeg)

2. Examine every possible ascending "scale" ordering of the notes until you find the most compact form—that is, the one that spans the smallest interval from lowest to highest note.

![](./images/_page_479_Figure_8.jpeg)

3. In the event that two orderings have the same interval span from lowest to highest note, choose the set that has the smaller interval between the first and penultimate notes.<sup>1</sup>

Measure first to penultimate note to break the tie:

![](./images/_page_479_Figure_11.jpeg)

4. In the event of an absolute tie, choose the set that begins on the smaller number. We will use a different set of pitches to demonstrate a complete tie. The normal form for the notes below is [2, 3, 8, 9].

![](./images/_page_479_Figure_13.jpeg)

In the event of an absolute tie, choose the set beginning on the smaller number.

<sup>&</sup>lt;sup>1</sup>The Forte method for determining prime form would have measured from first to second note to break the tie instead of first to penultimate note, which is method used Rahn's *Basic Atonal Theory*.

<span id="page-480-1"></span>5. Normal form is written in square brackets with commas. The normal form found in step 3 above was [11, 2, 3, 7].

## <span id="page-480-0"></span>33.3 Prime Form

Whereas normal form deals with the exact pitches as they occur in the music, **prime form** is usually a transposition and possibly an inversion of the normal form to its most essential form, much in the way an Eb major triad in second inversion belongs to the category "major triad," or a G<sup>7</sup>/F belongs to the more general idea "dominant seventh chord." Perhaps because of the primacy of C in music theory—many ideas are demonstrated in their relation to the C major scale—all prime forms are transposed to and start on C (pitch integer 0).

Following is the process for determining prime form.

1. Transpose the normal form—[11, 2, 3, 7] from the normal form example in the previous section—so it starts on C (pitch integer 0): 0, 3, 4, 8

![](./images/_page_480_Figure_7.jpeg)

Normal form transposed to start on 0

2. Invert the transposed normal form (what went up now goes down).

![](./images/_page_480_Figure_10.jpeg)

- 3. Write this inverted form in ascending form (4, 8, 9, 0), then do one of the following:
  - (a) If there were no ties when determining normal form, proceed to the step 4.
  - (b) If there were ties, put this inverted version through every ascending "scale" ordering to determine which is the most compact form from first note to penultimate note. In the example below, we see that the second ordering (the "tie loser" from normal form) is the most compact of the reorderings of the inverted normal form.

![](./images/_page_480_Picture_14.jpeg)

tie (break tie by measuring first to penultimate note)

4. Compare the normal form (transposed to 0) to the most compact inverted form (transposed to 0). The most compact form is the prime form. Prime form is written in parentheses with no commas: (0148).

![](./images/_page_481_Figure_2.jpeg)

Compare: Most compact version is Prime Form

(a) In the event the prime form reaches pitch integers 10 or 11, use T for 10 and E for 11; for example (013568T)

## 33.3.1 Application of Normal Form and Prime Form

Let's determine normal form and prime form of the first set from the Webern excerpt.

The first chord contained Eb, B, and D (3, 11, and 2).

![](./images/_page_481_Figure_8.jpeg)

The normal form is [11, 2, 3]. Below is the calculation to determine prime form.

![](./images/_page_481_Figure_10.jpeg)

Compare: most compact form is Prime Form

The prime form is (014).

Now let's determine the normal form and prime form for the third set we encountered:  $G \sharp$ , C, and A, or 8, 0, and 9.

<span id="page-482-0"></span>![](./images/_page_482_Figure_2.jpeg)

The normal form is [8, 9, 0].

In the example below, we transpose the normal form to zero, then invert it.

![](./images/_page_482_Figure_5.jpeg)

In the following example, we put the inverted normal form through the reorderings to find the most compact form, then compare it to the normal form.

![](./images/_page_482_Figure_7.jpeg)

We see that the third set has the same prime form—(014)—as the other sets in the opening measures of Webern's Op 5, No. 3. Prime form can allow us to see relationships that may not be apparent on the surface of the music.

#### 33.3.2 Segmentation

What about the C# in the cello part? Should it be included with the three notes from the chords? Will another similarity be revealed? **Segmentation** is the term for "segmenting" or determining which notes to group together and analyze in a passage. Usually, segmentation is based on the music—notes sounding together as a chord, or notes in a melodic line. However, analysts may look at every possible combination of notes to search for deeper layers of connection.

Below, we examine the first two chords with the C# included in each.

![](./images/_page_482_Picture_12.jpeg)

<span id="page-483-1"></span>![](./images/_page_483_Figure_2.jpeg)

Compare: most compact version is Prime Form

The prime form of the first set, when including the C $\sharp$  from the cello, is (0124).

Here is the second chord with the C# added to it.

![](./images/_page_483_Figure_6.jpeg)

Compare: most compact version is Prime Form

The prime form of the second set, when including the  $C\sharp$  from the cello, is (0236).

We do not see any relationship between these first two sets after including the C# with each three-note set. One doesn't know this until one examines this new segmentation.

#### <span id="page-483-0"></span>33.4 Interval Vector

An **interval vector** (also known as "Interval Class Content") is a list of every possible interval occurring in a pitch-class set. Calculating an interval vector is rather straightforward. First, after determing normal form, measure from the first note to all the other notes. Second, measure from the second note to all higher notes (not back or down to the first note). Continue measuring from each successive note to the notes following and you will have completed the interval vector.

Before demonstrating this, it is important to discuss the term "interval class." An **interval class** (abbreviated "ic") is the shortest distance between two notes measured in semitones. In the example below, C up to A is a major 6th. However, the shortest distance between C and A (measure downward) is a minor 3rd. Therefore, the largest interval class is 6 (the tritone), because any perfect 5th (for example, from C to G) has an interval class of 5 (a perfect 4th) because C down to G is a perfect 4th.

![](./images/_page_484_Figure_2.jpeg)

With this in mind, let's complete an interval vector of the a half-diminished 7th chord on G. First, arrange the notes in ascending order, then measure from the first note to the second, third, and fourth notes.

![](./images/_page_484_Figure_4.jpeg)

Second, measure from the second note to the third and fourth notes. We add one tally each for interval class (ic) 3 and 5.

![](./images/_page_484_Figure_6.jpeg)

Finally, measure from the third note to the fourth note, and the interval vector will be complete. We add one tally for ic4; the complete interval vector is 012011, which tells us a half-diminished chord has zero half steps, one major 2nd (shown in this voicing as a minor 7th), two minor 3rds, no major 3rds, one perfect 4th (shown in this voicing as perfect 5th), and one tritone.

![](./images/_page_484_Figure_8.jpeg)

An interval vector always contains 6 digits. When an interval class does not occur (the way the minor second did not occur in the G half-diminished seventh chord), place a zero in the column for that interval class.

## <span id="page-485-2"></span><span id="page-485-0"></span>**33.5 Forte Numbers**

When Allen Forte created a catalog of every possible 3-, 4-, 5-, 6-, 7-, 8-, and 9-note set in Appendix 1 of The Structure of Atonal Music, he labeled each prime form with two numbers separated by a hyphen. His labels (3–1, 3–2, etc.) are now known as "**Forte numbers**," and are seen in the tables in the [Lists of](#page-485-1) [Set Classes,](#page-485-1) which include prime forms and interval vectors as well.

## **33.5.1 Z-Relations**

"Z" in a set label (for example, 4–Z29) stands for "zygotic" ("twinned"), and is used when different prime forms have the same interval vector (for example, the interval vector for both 4–Z29 and 4–Z15 is 111111).

## <span id="page-485-1"></span>**33.6 Lists of Set Classes**

Below are lists of all set classes with prime form, Forte number, and interval vectors shown. Allen Forte published the original list of set classes in The Structure of Atonal Music in 1973. These lists use prime forms as calculated using the Rahn method. Prime forms of sets are ordered from most packed to the left to least packed to the left, as is found in the list of set classes in both John Rahn's Basic Atonal Theory and Joseph Straus' Introduction to Post-Tonal Theory. Sets are listed across from their **complements**. When taken together, complements can complete the 12-note chromatic scale when correctly transposed (and sometimes inverted).

**Table 33.6.1 List of Set Classes for 3- and 9-note sets (Trichords and Nonachords)**

| Prime | Forte  | Interval | Prime       | Forte  | Interval |
|-------|--------|----------|-------------|--------|----------|
| Form  | Number | Vector   | Form        | Number | Vector   |
| (012) | 3–1    | 210000   | (012345678) | 9–1    | 876663   |
| (013) | 3–2    | 111000   | (012345679) | 9–2    | 777663   |
| (014) | 3–3    | 101100   | (012345689) | 9–3    | 767763   |
| (015) | 3–4    | 100110   | (012345789) | 9–4    | 766773   |
| (016) | 3–5    | 100011   | (012346789) | 9–5    | 766674   |
| (024) | 3–6    | 020100   | (01234568T) | 9–6    | 686763   |
| (025) | 3–7    | 011010   | (01234578T) | 9–7    | 677673   |
| (026) | 3–8    | 010101   | (01234678T) | 9–8    | 676764   |
| (027) | 3–9    | 010020   | (01235678T) | 9–9    | 676683   |
| (036) | 3–10   | 002001   | (01234679T) | 9–10   | 668664   |
| (037) | 3–11   | 001110   | (01235679T) | 9–11   | 667773   |
| (048) | 3–12   | 000300   | (01245689T) | 9–12   | 666963   |

**Table 33.6.2 List of Set Classes for 4– and 8–note sets (Tetrachords and Octachords)**

| Prime  | Forte  | Interval | Prime        | Forte  | Interval |
|--------|--------|----------|--------------|--------|----------|
| Form   | Number | Vector   | Form         | Number | Vector   |
| (0123) | 4–1    | 321000   | (01234567)   | 8–1    | 765442   |
| (0124) | 4–2    | 221100   | (01234568)   | 8–2    | 665542   |
| (0125) | 4–4    | 211110   | (01234578)   | 8–4    | 655552   |
| (0126) | 4–5    | 210111   | (01234678)   | 8–5    | 654553   |
| (0127) | 4–6    | 210021   | (01235678)   | 8–6    | 654463   |
| (0134) | 4–3    | 212100   | (01234569)   | 8–3    | 656542   |
| (0135) | 4–11   | 121110   | (01234579)   | 8–11   | 565552   |
| (0136) | 4–13   | 112011   | (01234679)   | 8–13   | 556453   |
| (0137) | 4–Z29  | 111111   | (01235679)   | 8–Z29  | 555553   |
| (0145) | 4–7    | 201210   | (01234589)   | 8–7    | 645652   |
| (0146) | 4–Z15  | 111111   | (01234689)   | 8–Z15  | 555553   |
| (0147) | 4–18   | 102111   | (01235689)   | 8–18   | 546553   |
| (0148) | 4–19   | 101310   | (01245689)   | 8–19   | 545752   |
| (0156) | 4–8    | 200121   | (01234789)   | 8–8    | 644563   |
| (0157) | 4–16   | 110121   | (01235789)   | 8–16   | 554563   |
| (0158) | 4–20   | 101220   | (01245789)   | 8–20   | 545662   |
| (0167) | 4–9    | 200022   | (01236789)   | 8–9    | 644464   |
| (0235) | 4–10   | 122010   | (02345679)   | 8–10   | 566452   |
| (0236) | 4–12   | 112101   | (01345679)   | 8–12   | 556543   |
| (0237) | 4–14   | 111120   | (01245679)   | 8–14   | 555562   |
| (0246) | 4–21   | 030201   | (0123468T)   | 8–21   | 474643   |
| (0247) | 4–22   | 021120   | (0123568T)   | 8–22   | 465562   |
| (0248) | 4–24   | 020301   | (0124568T)   | 8–24   | 464743   |
| (0257) | 4–23   | 021030   | (0123578T)   | 8–23   | 465472   |
| (0258) | 4–27   | 012111   | (0124578T)   | 8–27   | 456553   |
| (0268) | 4–25   | 020202   | (0124678T)   | 8–25   | 464644   |
| (0347) | 4–17   | 102210   | (01345689)   | 8–17   | 546652   |
| (0358) | 4–26   | 012120   | (0134578T) 1 | 8–26   | 456562   |
| (0369) | 4–28   | 004002   | (0134679T)   | 8–28   | 448444   |

<sup>1</sup>Forte prime form for 8–26: (0124579T)

**Table 33.6.3 List of Set Classes for 5– and 7–note sets (Pentachords and Septachords)**

| Prime     | Forte  | Interval | Prime       | Forte  | Interval |
|-----------|--------|----------|-------------|--------|----------|
| Form      | Number | Vector   | Form        | Number | Vector   |
| (01234)   | 5–1    | 432100   | (0123456)   | 7–1    | 654321   |
| (01235)   | 5–2    | 332110   | (0123457)   | 7–2    | 554331   |
| (01236)   | 5–4    | 322111   | (0123467)   | 7–4    | 544332   |
| (01237)   | 5–5    | 321121   | (0123567)   | 7–5    | 543342   |
| (01245)   | 5–3    | 322210   | (0123458)   | 7–3    | 544431   |
| (01246)   | 5–9    | 231211   | (0123468)   | 7–9    | 453432   |
| (01247)   | 5–Z36  | 222121   | (0123568)   | 7–Z36  | 444342   |
| (01248)   | 5–13   | 2221311  | (0124568)   | 7–13   | 443532   |
| (01256)   | 5–6    | 311221   | (0123478)   | 7–6    | 533442   |
| (01257)   | 5–14   | 221131   | (0123578)   | 7–14   | 443352   |
| (01258)   | 5–Z38  | 212221   | (0124578)   | 7–Z38  | 434442   |
| (01267)   | 5–7    | 310132   | (0123678)   | 7–7    | 532353   |
| (01268)   | 5–15   | 220222   | (0124678)   | 7–15   | 442443   |
| (01346)   | 5–10   | 223111   | (0123469)   | 7–10   | 445332   |
| (01347)   | 5–16   | 213211   | (0123569)   | 7–16   | 435432   |
| (01348)   | 5–Z17  | 212320   | (0124569)   | 7–Z17  | 434541   |
| (01356)   | 5–Z12  | 222121   | (0123479)   | 7–Z12  | 444342   |
| (01357)   | 5–24   | 131221   | (0123579)   | 7–24   | 353442   |
| (01358)   | 5–27   | 122230   | (0124579)   | 7–27   | 344451   |
| (01367)   | 5–19   | 212122   | (0123679)   | 7–19   | 434343   |
| (01368)   | 5–29   | 122131   | (0124679)   | 7–29   | 344352   |
| (01369)   | 5–31   | 114112   | (0134679)   | 7–31   | 336333   |
| (01457)   | 5–Z18  | 212221   | (0145679) 2 | 7–Z18  | 434442   |
| (01458)   | 5–21   | 202420   | (0124589)   | 7–21   | 424641   |
| (01468)   | 5–30   | 121321   | (0124689)   | 7–30   | 343542   |
| (01469)   | 5–32   | 113221   | (0134689)   | 7–32   | 335442   |
| (01478)   | 5–22   | 202321   | (0125689)   | 7–22   | 424542   |
| (01568) 3 | 5–20   | 211231   | (0125679) 4 | 7–20   | 433452   |
| (02346)   | 5–8    | 232201   | (0234568)   | 7–8    | 454422   |
| (02347)   | 5–11   | 222220   | (0134568)   | 7–11   | 444441   |
| (02357)   | 5–23   | 132130   | (0234579)   | 7–23   | 354351   |
| (02358)   | 5–25   | 123121   | (0234679)   | 7–25   | 345342   |
| (02368)   | 5–28   | 122212   | (0135679)   | 7–28   | 344433   |
| (02458)   | 5–26   | 122311   | (0134579)   | 7–26   | 344532   |
| (02468)   | 5–33   | 040402   | (012468T)   | 7–33   | 262623   |
| (02469)   | 5–34   | 032221   | (013468T)   | 7–34   | 254442   |
| (02479)   | 5–35   | 032140   | (013568T)   | 7–35   | 254361   |
| (03458)   | 5–Z37  | 212320   | (0134578)   | 7–Z37  | 434541   |

In the table below, when no set is listed across from a six–note set, it is self–complementary (that is, it can combine with a transposed and possibly inverted set of itself to complete a 12-note chromatic scale.

<sup>2</sup>Forte prime form for 7–Z18: (0123589)

<sup>3</sup>Forte prime form for 5–20: (01378)

<sup>4</sup>Forte prime form for 7–20: (0124789)

**Table 33.6.4 List of Set Classes for 6-note sets (Hexachords)**

| Prime      | Forte  | Interval | Prime    | Forte  | Interval |
|------------|--------|----------|----------|--------|----------|
| Form       | Number | Vector   | Form     | Number | Vector   |
| (012345)   | 6–1    | 543210   |          |        |          |
| (012346)   | 6–2    | 4443211  |          |        |          |
| (012347)   | 6–Z36  | 433221   | (012356) | 6–Z3   | 433221   |
| (012348)   | 6–Z37  | 432321   | (012456) | 6–Z4   | 432321   |
| (012357)   | 6–9    | 342231   |          |        |          |
| (012358)   | 6–Z40  | 333231   | (012457) | 6–Z11  | 333231   |
| (012367)   | 6–5    | 422232   |          |        |          |
| (012368)   | 6–Z41  | 332232   | (012467) | 6–Z12  | 332232   |
| (012369)   | 6–Z42  | 324222   | (013467) | 6–Z13  | 324222   |
| (012378)   | 6–Z38  | 421242   | (012567) | 6–Z6   | 421242   |
| (012458)   | 6–15   | 323421   |          |        |          |
| (012468)   | 6–22   | 241422   |          |        |          |
| (012469)   | 6–Z46  | 233331   | (013468) | 6–Z24  | 233331   |
| (012478)   | 6–Z17  | 322332   | (012568) | 6–Z43  | 233331   |
| (012479)   | 6–Z47  | 233241   | (013568) | 6–Z25  | 233241   |
| (012569)   | 6–Z44  | 313431   | (013478) | 6–Z19  | 313431   |
| (012578)   | 6–18   | 322242   |          |        |          |
| (012579)   | 6–Z48  | 232341   | (013578) | 6–Z26  | 232341   |
| (012678)   | 6–7    | 420243   |          |        |          |
| (013457)   | 6–Z10  | 333321   | (023458) | 6–Z39  | 333321   |
| (013458)   | 6–14   | 323430   |          |        |          |
| (013469)   | 6–27   | 225222   |          |        |          |
| (013479)   | 6–Z49  | 224322   | (013569) | 6–Z28  | 224322   |
| (013579)   | 6–34   | 142422   |          |        |          |
| (013679)   | 6–30   | 224223   |          |        |          |
| (023679) 5 | 6–Z29  | 224232   | (014679) | 6–Z50  | 224232   |
| (014568)   | 6–16   | 322431   |          |        |          |
| (014579) 6 | 6–31   | 223431   |          |        |          |
| (014589)   | 6–20   | 303630   |          |        |          |
| (023457)   | 6–8    | 343230   |          |        |          |
| (023468)   | 6–21   | 242412   |          |        |          |
| (023469)   | 6–Z45  | 234222   | (023568) | 6–Z23  | 234222   |
| (023579)   | 6–33   | 143241   |          |        |          |
| (024579)   | 6–32   | 143250   |          |        |          |
| (02468T)   | 6–35   | 060603   |          |        |          |

## <span id="page-488-0"></span>**33.7 Transposition (T***n***)**

Transposition is an operation performed as T*n*, where *n* is the number of semitones *up* a set is transposed. For example, [1, 2, 4, 6] at T<sup>4</sup> is [5, 6, 8, 10].

<sup>5</sup>Forte prime form for 6–Z29: (013689)

<sup>6</sup>Forte prime form for 6–31: (013589)

![](./images/_page_489_Figure_2.jpeg)

When working in a modulo 12 system, remember that numbers larger than 12 have to be reduced to a number smaller than 12 by subtracting 12 from the larger number. For example, 6, 8, 10, 11 at  $T_9$  would result in 15, 17, 19, 20, which, after subtracting 12 from each number, results in 3, 5, 7, 8.

Table 33.7.1

| Pitch classes:          |   | 6  | 8  | 10 | 11 |
|-------------------------|---|----|----|----|----|
| at $T_9$ :              | + | 9  | 9  | 9  | 9  |
| Result:                 | • | 15 | 17 | 19 | 20 |
| Make numbers modulo 12: | _ | 12 | 12 | 12 | 12 |
| Result:                 |   | 3  | 5  | 7  | 8  |

## <span id="page-489-0"></span>33.8 Inversion $(T_nI)$

Inverting a set using  $T_nI$  is a compound operation. The first step is to invert each note below C using C as an axis. For example, E is a major 3rd above C, so E would invert to  $A\flat$ , a major third below C.

![](./images/_page_489_Figure_8.jpeg)

The second step of inversion is to apply the  $T_n$  interval. So, to calculate  $T_3I$  for the note E, one would first invert E to Ab (this is  $T_0I$ ), then transpose the Ab up 3 semitones to B. (Theorist Joseph Straus simplifies the nomenclature to  $I_n$  instead of  $T_nI$ , but the outcome remains the same.)

Let's try inverting a pitch-class set, applying  $T_7I$  to [2, 4, 5] (or D, E, and F). Inverting the notes to the opposite side of C using C as an axis yields pitch numbers 10, 8, and 7 (or B $\flat$ , A $\flat$ , and G), which in ascending order is 7, 8, and 10. Then transposing [7, 8, 10] at  $T_7$  raises each note 7 semitones, resulting in [2, 3, 5] (or D, E $\flat$ , and F).

![](./images/_page_489_Figure_11.jpeg)

#### 33.8.1 Identifying T<sub>n</sub>I for Inversionally-Related Sets

To determine n of  $T_nI$  for two inversionally-related sets, write the second set backward and add the notes of the two sets together. Each sum will equal n. Let's use our two sets from the previous example above: [2, 4, 5] and [2, 3, 5].

#### Table 33.8.1

First set in order: 
$$2$$
 4 5  
Second set backward:  $+$  5 3 2  
 $n$  of  $T_nI$ :  $7$  7

This confirms the sets are related at  $T_7I$ .

## <span id="page-490-0"></span>33.9 Practice Exercises

#### Exercise Group. Day One

<span id="page-490-1"></span>1. Put each set into normal form and prime form.

![](./images/_page_490_Picture_8.jpeg)

#### Exercise Group. Day Two

<span id="page-490-2"></span>2. For each of the six sets in the example below, determine the normal form, prime form, Forte number, and interval vector.

![](./images/_page_490_Picture_11.jpeg)

Exercise Group. Day Three

- <span id="page-491-0"></span>**3.** Transposition (T*n*) of Sets. Transpose the following sets as specified.
  - (a) Transpose [3, 6, 7] at T2: [ , , ]
  - (b) Transpose [2, 4, 8, 9] at T7: [ , , , ]
  - (c) Transpose [1, 2, 4, 7, 8] at T9: [ , , , , ]
- <span id="page-491-1"></span>**4.** Inversion (T*n*I) of Sets. Invert the following sets. Write your answers in normal form.
  - (a) Invert [7, 10, 11] at T0I: [ , , ]
  - (b) Invert [0, 2, 4] at T4I: [ , , ]
  - (c) Invert [4, 6, 10, 11] at T9I: [ , , , ]
- <span id="page-491-2"></span>**5.** Specify the interval of inversion from the first set to the second set.
  - (a) [2, 4, 7] inverts to [3, 6, 8] at what T*n*I?
  - (b) [1, 2, 4, 7] inverts to [4, 7, 9, 10] at what T*n*I?
  - (c) [6, 7, 10, 1, 2] inverts to [3, 4, 7, 10, 11] at what T*n*I?

Click [here to download the first homework assignment for this chapter.](http://musictheory.pugetsound.edu/hw/MT21C_HW_61.pdf)<sup>1</sup>

Click [here to download the second homework assignment for this chapter.](http://musictheory.pugetsound.edu/hw/MT21C_HW_62.pdf)<sup>2</sup> Click [here to download the third homework assignment for this chapter.](http://musictheory.pugetsound.edu/hw/MT21C_HW_63.pdf)<sup>3</sup>

PDF versions of the textbook, homework exercises, and practice exercises can be found at [musictheory.pugetsound.edu](http://musictheory.pugetsound.edu)<sup>4</sup>

<sup>1</sup>musictheory.pugetsound.edu/hw/MT21C\_HW\_61.pdf

<sup>2</sup>musictheory.pugetsound.edu/hw/MT21C\_HW\_62.pdf

<sup>3</sup>musictheory.pugetsound.edu/hw/MT21C\_HW\_63.pdf

<sup>4</sup>musictheory.pugetsound.edu

