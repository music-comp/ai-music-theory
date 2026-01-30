---
concept: Unfolding Interval Vector
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
The unfolding interval vector tracks how the interval content of a set develops note-by-note as elements are added through time, revealing the temporal process by which intervallic patterns emerge for a listener.

# Formal Definition
Given a temporally ordered sequence of elements entering at successive time-points, the unfolding interval vector is a progressive record of the intervals that become available as each new element is added. Each new element contributes new intervals to all previously heard elements.

# Mathematical Formulation
**Construction process:**
1. First element (s1): no intervals yet
2. Second element (s2): interval vector contains int(s1, s2)
3. Third element (s3): vector adds int(s1, s3) and int(s2, s3)
4. And so on...

**For n elements:**
After element n arrives, the interval vector contains all int(si, sj) where i < j <= n (only forward-in-time intervals).

**In direct-product GIS:**
Each interval is a compound (pitch-class interval, temporal interval), tracking both dimensions simultaneously.

# Musical Context/Application
The unfolding interval vector models the listener's developing perception of intervallic structure. It addresses the question: At what point during the music do we first hear enough to perceive a pattern?

Key application: determining when mensural structure becomes perceptible (the "ictus" in Lewin's theory).

# Examples
**Figure 3.3 (Webern analysis):**

After (Eb, 0): no intervals
After (B, 3): {(8, 3)}
After (Bb, 4): {(8, 3), (7, 4), (11, 1)}
After (D, 5): {(8, 3), (7, 4), (11, 1), (11, 5), (3, 2), (4, 1)}
After (C#, 10): adds {(10, 10), (2, 7), (3, 6), (11, 5)}
After (C, 11): adds {(9, 11), (1, 8), (2, 7), (10, 6), (11, 1)}

**Analytical observations:**
- At time-point 5: first recurrence emerges (pitch-class interval 11)
- Time-point 5 is proposed as first "ictus"
- At time-point 10: (11, 5) recurs--first recurring GIS3-interval
- At time-point 11: (11, 1) and (2, 7) recur

**Perceptual significance:**
- Predominance of an interval signals its structural importance
- Recurrence of compound intervals creates associations between dimensions

# Related Concepts
- Direct-Product GIS
- Interval Vector
- Webern Piano Variations Analysis
- Ictus
- Mensural Structure

# Common Confusions
1. **Direction of intervals:** Only "forward" intervals (earlier to later) are counted, not symmetric pairs.

2. **Accumulation vs. snapshot:** The unfolding vector grows; it records all intervals heard so far, not just new ones.

3. **Compound intervals:** In GIS3, each interval is a pair. Recurrence means the same pair (not just same pitch-class interval or same temporal interval).

4. **The ictus concept:** The moment when interval patterns first become perceptible is analytically significant, not just the total interval content.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Figure 3.3 discussion, pp. 72-77
