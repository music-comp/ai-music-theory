---
source: Open Music Theory
part: "IX. Twelve-Tone Music"
part_number: 9
chapter: "Naming Conventions for Rows"
chapter_number: 2
url: https://viva.pressbooks.pub/openmusictheory/
license: CC-BY-SA
---

# Naming Conventions for Rows

Key Takeaways

  * This chapter goes through the different ways of representing twelve-tone material: 
    * pitches by pitch name or [pb_glossary id="2535"]pitch class[/pb_glossary];
    * rows and transformations with P0 starts on C (fixed zero) or P0 starts wherever we chose (moveable zero)
    * matrices, setting out these different row conventions.
  * When reading other writing on twelve-tone music, be prepared for any of these conventions to be used. But in your own work, simply choose one you feel comfortable with and use it consistently.



In analyzing twelve-tone music, there are different conventions for labeling rows, transformations, and even pitches and intervals. This chapter compares the main approaches that you're most likely to encounter in analytical writings. The focus is on rows and matrices, but before we get to that, let's deal first with the pitches themselves. 

# Pitch

As we've seen [earlier in the book](https://viva.pressbooks.pub/openmusictheory/chapter/pitch-and-pitch-class/), it is useful in some analytical contexts to use pitch-class notation ([integers](https://viva.pressbooks.pub/openmusictheory/chapter/pitch-and-pitch-class/) from 0 for C to 11 for B) as an alternative to spelling out those pitches (e.g., C♯ vs D♭). This convention is mostly associated with non-tonal music (including most twelve-tone music), where it can be handy for performing the kinds of mathematical operations we've seen (in both [pitch-class set analysis](https://viva.pressbooks.pub/openmusictheory/chapter/pc-sets-normal-order-and-transformations/) and [twelve-tone music](https://viva.pressbooks.pub/openmusictheory/chapter/basics-of-twelve-tone-theory#operations)) and for sidestepping questions of pitch spelling. There's often still a logic to the pitch spellings used in a twelve-tone piece, but that logic is often of a different and perhaps less generalizable kind. For instance, using specific pitch spellings in a row-form representation usually doesn't reflect a hierarchy or tonality in the same way that the pitches of a scale do in tonal music. 

# Rows

For [pb_glossary id="8426"]rows[/pb_glossary], the main difference in notation and labeling centers on a single choice about which pitch to organize our rows around: 

  1. the **same pitch** in all contexts (conventionally, that pitch is C)
  2. a pitch that's **important to the musical context** in question

For instance, in the [Basics of Twelve-Tone Theory chapter](https://viva.pressbooks.pub/openmusictheory/chapter/basics-of-twelve-tone-theory/), we set out the row of Elisabeth Lutyens’s  _Motet_ starting on C, which gave us the twelve-tone row 0–11–3–7–8–4–2–6–5–1–9–10. Alternatively, we could set out the P0 starting on D, as the first voice to enter (alto) starts on D4 and proceeds to sing the first [pb_glossary id="8434"]hexachord[/pb_glossary] of this prime-form row on that pitch level.[footnote]The actual row distribution is a bit more complicated. See Parsons 1999 for an analysis and discussion.[/footnote] That would give us a P0 of 2–1–5–9–10–6–4–8–7–3–11–0. 

## Option 1: P0 starts on C (fixed zero)

In this convention, whatever you decide the prime form to be, the transposition of that form starting on C is P0. This is probably the most common convention today, and sometimes called "zero-centered" or "fixed-zero" (by analogy to tonal solfège systems). As we have set P0 to begin on C, I0 also begins on C, and R0 and RI0 will _end_ on C. This separation of P0 and I0 from R0 and RI0 makes sense because we prefer P0 and R0 to be exact retrogrades of one other (and likewise I0 and RI0). We could theoretically have an even more consistently "zero-centered" system in which all of P0, I0, R0 and RI0 begin on C, but that's not a convention that people have widely adopted. In summary: 

  * P0 starts with C
  * I0 starts with C (same pitch class as P0)
  * R0 starts with the last note of P0 (by definition, not C)
  * RI0 starts with the last note of I0 (by definition, not C)



## Option 2: P0 starts wherever we chose (moveable zero)

In the alternative method, the P0 form is assigned to either the first form of the row or the one that is most meaningful—regardless of what pitch class begins the row. Depending on the context, this may be evident from the piece, deduced from the analysis, or allocated semi-arbitrarily. Transpositions and other operations are then worked out in the same way, in relation to that P0 form. This convention is sometimes called "original-centered" or "movable-zero" (to continue the solfège analogy). In summary: 

  * P0 takes a transposition (and thus starts with a pitch) chosen by the analyst
  * I0 still starts with the same note as P0
  * R0 still starts with the last note of P0
  * RI0 still starts with the last note of I0



## Same? Different? Better? Worse?

As the two summaries suggest, these naming conventions are actually not so different. It bears repeating that for all naming systems, transposition and the other operations all work in the same way, so it's mostly just a matter of where you start: which row form you use as the referential form to relate others to. And as is so often the case when multiple parallel naming conventions emerge, there are both benefits and downsides to each approach. If you’re analyzing music that makes you want to assign P0 in a musically sensitive way, then the moveable-zero convention may suit your purposes. But if you go down that route, then you'll probably feel compelled to come up with a "good" reason for the pitch level of P0 in all your analyses, and that may not always be appropriate. The fixed-zero system has the benefit of clarity and consistency. That's probably why it's become more common in recent scholarship, but that doesn’t necessarily make it "better." Indeed, in many cases, it won't even be clear which orientation should be P and which I (or R for that matter). Unfortunately, there isn’t yet a widely recognized system for making such determinations. 

# Matrices

Before we wrap this up, there’s one final confusion to add to the pile: how to set out these conventions on the row matrix. Here are three types. 

## Type 1

First, here’s a reminder of the matrix we saw for the Lutyens example [in the last chapter](https://viva.pressbooks.pub/openmusictheory/chapter/basics-of-twelve-tone-theory/) (**P 0** starts on C and is in the top row). This is probably the most common and standard form.  | **I 0** | **I 11** | **I 3** | **I 7** | **I 8** | **I 4** | **I 2** | **I 6** | **I 5** | **I 1** | **I 9** | **I 10** |   
---|---|---|---|---|---|---|---|---|---|---|---|---|---  
**P 0** | 0 | 11 | 3 | 7 | 8 | 4 | 2 | 6 | 5 | 1 | 9 | 10 | **R 10**  
**P 1** | 1 | 0 | 4 | 8 | 9 | 5 | 3 | 7 | 6 | 2 | 10 | 11 | **R 11**  
**P 9** | 9 | 8 | 0 | 4 | 5 | 1 | 11 | 3 | 2 | 10 | 6 | 7 | **R 7**  
**P 5** | 5 | 4 | 8 | 0 | 1 | 9 | 7 | 11 | 10 | 6 | 2 | 3 | **R 3**  
**P 4** | 4 | 3 | 7 | 11 | 0 | 8 | 6 | 10 | 9 | 5 | 1 | 2 | **R 2**  
**P 8** | 8 | 7 | 11 | 3 | 4 | 0 | 10 | 2 | 1 | 9 | 5 | 6 | **R 6**  
**P 10** | 10 | 9 | 1 | 5 | 6 | 2 | 0 | 4 | 3 | 11 | 7 | 8 | **R 8**  
**P 6** | 6 | 5 | 9 | 1 | 2 | 10 | 8 | 0 | 11 | 7 | 3 | 4 | **R 4**  
**P 7** | 7 | 6 | 10 | 2 | 3 | 11 | 9 | 1 | 0 | 8 | 4 | 5 | **R 5**  
**P 11** | 11 | 10 | 2 | 6 | 7 | 3 | 1 | 5 | 4 | 0 | 8 | 9 | **R 9**  
**P 3** | 3 | 2 | 6 | 10 | 11 | 7 | 5 | 9 | 8 | 4 | 0 | 1 | **R 1**  
**P 2** | 2 | 1 | 5 | 9 | 10 | 6 | 4 | 8 | 7 | 3 | 11 | 0 | **RI 0**  
| **RI 2** | **RI 1** | **RI 5** | **RI 9** | **RI 10** | **RI 6** | **RI 4** | **RI 8** | **RI 7** | **RI 3** | **RI 11** | **RI 0** |   
  
## Type 2

Now here’s the same matrix, with P0 still on the top row, but with that P0 starting on D. Note how the lists of row forms stay the same (**P 0**, **P 1**, **P 9**…), but the pitches have moved around.  | **I 0** | **I 11** | **I 3** | **I 7** | **I 8** | **I 4** | **I 2** | **I 6** | **I 5** | **I 1** | **I 9** | **I 10** |   
---|---|---|---|---|---|---|---|---|---|---|---|---|---  
**P 0** | 2 | 1 | 5 | 9 | 10 | 6 | 4 | 8 | 7 | 3 | 11 | 0 | **R 10**  
**P 1** | 3 | 2 | 6 | 10 | 11 | 7 | 5 | 9 | 8 | 4 | 0 | 1 | **R 11**  
**P 9** | 11 | 10 | 2 | 6 | 7 | 3 | 1 | 5 | 4 | 0 | 8 | 9 | **R 7**  
**P 5** | 7 | 6 | 10 | 2 | 3 | 11 | 9 | 1 | 0 | 8 | 4 | 5 | **R 3**  
**P 4** | 6 | 5 | 9 | 1 | 2 | 10 | 8 | 0 | 11 | 7 | 3 | 4 | **R 2**  
**P 8** | 10 | 9 | 1 | 5 | 6 | 2 | 0 | 4 | 3 | 11 | 7 | 8 | **R 6**  
**P 10** | 0 | 11 | 3 | 7 | 8 | 4 | 2 | 6 | 5 | 1 | 9 | 10 | **R 8**  
**P 6** | 8 | 7 | 11 | 3 | 4 | 0 | 10 | 2 | 1 | 9 | 5 | 6 | **R 4**  
**P 7** | 9 | 8 | 0 | 4 | 5 | 1 | 11 | 3 | 2 | 10 | 6 | 7 | **R 5**  
**P 11** | 1 | 0 | 4 | 8 | 9 | 5 | 3 | 7 | 6 | 2 | 10 | 11 | **R 9**  
**P 3** | 5 | 4 | 8 | 0 | 1 | 9 | 7 | 11 | 10 | 6 | 2 | 3 | **R 1**  
**P 2** | 4 | 3 | 7 | 11 | 0 | 8 | 6 | 10 | 9 | 5 | 1 | 2 | **RI 0**  
| **RI 2** | **RI 1** | **RI 5** | **RI 9** | **RI 10** | **RI 6** | **RI 4** | **RI 8** | **RI 7** | **RI 3** | **RI 11** | **RI 0** |   
  
## Type 3

Perhaps most confusing of all is a kind of hybrid version where we still have the D version on the top row, but now we label it **P 2**. So: 

  * We organize the row class around a chosen pitch/transposition (here D).
  * We still label the row forms around the alternative option (**P 0** starts on C).

Note how this time, comparing it with the version above, the pitches have stayed the same, but the lists of row forms have changed (**P x**, **P y**…).  | **I 2** | **I 1** | **I 5** | **I 9** | **I 10** | **I 6** | **I 4** | **I 8** | **I 7** | **I 3** | **I 11** | **I 0** |   
---|---|---|---|---|---|---|---|---|---|---|---|---|---  
**P 2** | 2 | 1 | 5 | 9 | 10 | 6 | 4 | 8 | 7 | 3 | 11 | 0 | **R 2**  
**P 3** | 3 | 2 | 6 | 10 | 11 | 7 | 5 | 9 | 8 | 4 | 0 | 1 | **R 3**  
**P 11** | 11 | 10 | 2 | 6 | 7 | 3 | 1 | 5 | 4 | 0 | 8 | 9 | **R 11**  
**P 7** | 7 | 6 | 10 | 2 | 3 | 11 | 9 | 1 | 0 | 8 | 4 | 5 | **R 7**  
**P 6** | 6 | 5 | 9 | 1 | 2 | 10 | 8 | 0 | 11 | 7 | 3 | 4 | **R 6**  
**P 10** | 10 | 9 | 1 | 5 | 6 | 2 | 0 | 4 | 3 | 11 | 7 | 8 | **R 10**  
**P 0** | 0 | 11 | 3 | 7 | 8 | 4 | 2 | 6 | 5 | 1 | 9 | 10 | **R 0**  
**P 8** | 8 | 7 | 11 | 3 | 4 | 0 | 10 | 2 | 1 | 9 | 5 | 6 | **R 8**  
**P 9** | 9 | 8 | 0 | 4 | 5 | 1 | 11 | 3 | 2 | 10 | 6 | 7 | **R 9**  
**P 1** | 1 | 0 | 4 | 8 | 9 | 5 | 3 | 7 | 6 | 2 | 10 | 11 | **R 1**  
**P 5** | 5 | 4 | 8 | 0 | 1 | 9 | 7 | 11 | 10 | 6 | 2 | 3 | **R 5**  
**P 4** | 4 | 3 | 7 | 11 | 0 | 8 | 6 | 10 | 9 | 5 | 1 | 2 | **R 4**  
| **RI 2** | **RI 1** | **RI 5** | **RI 9** | **RI 10** | **RI 6** | **RI 4** | **RI 8** | **RI 7** | **RI 3** | **RI 11** | **R I0** |   
  
## Summary

In summary, the first row can read: 

  * **P 0**, starting on 0
  * **P 0**, starting on n (here 2)
  * **P n**, starting on n

All of these naming and matrix-generating conventions are out there. It’s best simply to be aware of these options and check that you have the right convention in mind when you come across one (especially where the matrices neglect to explicitly label the row names). 

Further Reading

  * Parsons, Laurel, 1999. “Music and Text in Elisabeth Lutyens’s Wittgenstein Motet.”  _Canadian University Music Review_ 20 (1): 71–100.



Assignments

  1. Chose any row from the [Twelve-Tone Anthology](https://viva.pressbooks.pub/openmusictheory/chapter/anthology-twelve-tone/) that interests you and write out the row matrix with all 48 row forms (i.e., with numbers on the grid as shown above) in each of the three ways shown above. (Then choose your favorite method and never do this again!)


