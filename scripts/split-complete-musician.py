#!/usr/bin/env python3
"""
Split The Complete Musician: An Integrated Approach to Tonal Theory,
Analysis, and Listening by Steven G. Laitz into individual part files.
Uses section mapping derived from book.json and book.md analysis.

Note: This book has 37 chapters across 9 parts. Due to the complexity
and inconsistent heading styles, we split by Parts rather than chapters.
"""

import re
from pathlib import Path

# Paths (note: directory has typo "muscician")
SOURCE_FILE = Path("sources-md/complete-muscician/book.md")
OUTPUT_DIR = Path("sources-md/complete-muscician")

# Section definitions (derived from analysis of book.json and book.md)
# The book has:
# - Front matter: TOC, Preface
# - Part 1: The Foundation of Tonal Music (Chapters 1-6)
# - Part 2: Merging Melody and Harmony (Chapters 7-11)
# - Part 3: A New Harmonic Function... (Chapters 12-16)
# - Part 4: New Chords Complete the Diatonic Spectrum (Chapters 17-19)
# - Part 5: Creating Larger Forms (Chapters 20-23)
# - Part 6: Chromaticism (Chapters 24-26)
# - Part 7: Expressive Chromaticism (Chapters 27-30)
# - Part 8: Large Forms: Ternary, Rondo, Sonata (Chapters 31-33)
# - Part 9: Introduction to Nineteenth-Century Harmony (Chapters 34-37)
# - Back matter: Indexes

SECTIONS = [
    {
        'type': 'preface',
        'title': 'Preface',
        'start_line': 640,
        'pdf_page': 19,
    },
    {
        'type': 'part',
        'num': 1,
        'title': 'Part 1: The Foundation of Tonal Music',
        'chapters': '1-6',
        'start_line': 810,
        'pdf_page': 29,
    },
    {
        'type': 'part',
        'num': 2,
        'title': 'Part 2: Merging Melody and Harmony',
        'chapters': '7-11',
        'start_line': 4307,
        'pdf_page': 187,
    },
    {
        'type': 'part',
        'num': 3,
        'title': 'Part 3: A New Harmonic Function and Additional Melodic and Harmonic Embellishments',
        'chapters': '12-16',
        'start_line': 6562,
        'pdf_page': 307,
    },
    {
        'type': 'part',
        'num': 4,
        'title': 'Part 4: New Chords Complete the Diatonic Spectrum',
        'chapters': '17-19',
        'start_line': 8918,
        'pdf_page': 431,
    },
    {
        'type': 'part',
        'num': 5,
        'title': 'Part 5: Creating Larger Forms',
        'chapters': '20-23',
        'start_line': 10228,
        'pdf_page': 489,
    },
    {
        'type': 'part',
        'num': 6,
        'title': 'Part 6: Chromaticism',
        'chapters': '24-26',
        'start_line': 11711,
        'pdf_page': 565,
    },
    {
        'type': 'part',
        'num': 7,
        'title': 'Part 7: Expressive Chromaticism',
        'chapters': '27-30',
        'start_line': 13022,
        'pdf_page': 643,
    },
    {
        'type': 'part',
        'num': 8,
        'title': 'Part 8: Large Forms: Ternary, Rondo, Sonata',
        'chapters': '31-33',
        'start_line': 14496,
        'pdf_page': 729,
    },
    {
        'type': 'part',
        'num': 9,
        'title': 'Part 9: Introduction to Nineteenth-Century Harmony',
        'chapters': '34-37',
        'start_line': 15388,
        'pdf_page': 817,
    },
    # Back matter
    {
        'type': 'backmatter',
        'id': 'index-terms',
        'title': 'Index of Terms and Concepts',
        'start_line': 16821,
        'pdf_page': 905,
    },
    {
        'type': 'backmatter',
        'id': 'index-examples',
        'title': 'Index of Musical Examples and Exercises',
        'start_line': 17378,
        'pdf_page': 912,
    },
]

def slugify(text):
    """Convert text to filename-safe slug."""
    # Convert to lowercase
    text = text.lower()
    # Remove special characters
    text = re.sub(r'[^\w\s-]', '', text)
    # Replace spaces and multiple hyphens with single hyphen
    text = re.sub(r'[-\s]+', '-', text)
    # Limit length
    if len(text) > 50:
        text = text[:50]
    return text.strip('-')

def split_into_parts():
    """Main function to split the book into parts."""
    print("=" * 80)
    print("Splitting The Complete Musician: An Integrated Approach to")
    print("Tonal Theory, Analysis, and Listening")
    print("by Steven G. Laitz (Second Edition)")
    print("=" * 80)
    print()

    print(f"Reading {SOURCE_FILE}...")
    with open(SOURCE_FILE, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    total_lines = len(lines)
    print(f"Total lines: {total_lines}")
    print()

    # Extract frontmatter (everything before Preface)
    frontmatter_end = SECTIONS[0]['start_line']
    frontmatter_content = ''.join(lines[:frontmatter_end])

    # Create frontmatter with metadata
    frontmatter_file = OUTPUT_DIR / "00-frontmatter.md"
    with open(frontmatter_file, 'w', encoding='utf-8') as f:
        f.write("---\n")
        f.write("title: Frontmatter\n")
        f.write("pdf_page: 0\n")
        f.write("book_md_line: 1\n")
        f.write("---\n\n")
        f.write(frontmatter_content)

    print(f"Extracted frontmatter to {frontmatter_file.name}")
    print(f"  Lines: 1-{frontmatter_end} ({frontmatter_end} lines)")
    print()

    # Extract each section
    print("Extracting parts and back matter:")
    print("-" * 80)

    created_files = []

    for i, section in enumerate(SECTIONS):
        start = section['start_line']
        pdf_page = section['pdf_page']
        title = section['title']

        # End is either the next section or end of file
        if i + 1 < len(SECTIONS):
            end = SECTIONS[i + 1]['start_line']
        else:
            end = total_lines

        # Extract section content
        section_content = ''.join(lines[start:end])

        # Create filename based on section type
        if section['type'] == 'preface':
            filename = "00-preface.md"
        elif section['type'] == 'part':
            part_num = section['num']
            # Extract just the descriptive part after "Part X: "
            desc = title.split(': ', 1)[1] if ': ' in title else title
            desc_slug = slugify(desc)
            filename = f"{part_num:02d}-{desc_slug}.md"
        else:  # backmatter
            filename = f"{section['id']}.md"

        filepath = OUTPUT_DIR / filename

        # Write section file with metadata header
        with open(filepath, 'w', encoding='utf-8') as f:
            # Add YAML metadata header
            f.write("---\n")
            f.write(f"title: \"{title}\"\n")
            if section['type'] == 'part':
                f.write(f"part_number: {section['num']}\n")
                f.write(f"chapters: \"{section['chapters']}\"\n")
            f.write(f"pdf_page: {pdf_page}\n")
            f.write(f"book_md_line: {start}\n")
            f.write("---\n\n")
            f.write(section_content)

        line_count = end - start
        created_files.append({
            'filename': filename,
            'type': section['type'],
            'title': title,
            'pdf_page': pdf_page,
            'line_count': line_count
        })

        if section['type'] == 'part':
            print(f"  Part {section['num']} (Ch {section['chapters']}) -> {filename}")
        elif section['type'] == 'preface':
            print(f"  {title} -> {filename}")
        else:
            print(f"  {title} -> {filename}")
        print(f"      PDF page {pdf_page:3d} | Lines {start:5d}-{end:5d} ({line_count:5d} lines)")

    print()
    print("=" * 80)
    print("Summary")
    print("=" * 80)
    part_count = len([s for s in SECTIONS if s['type'] == 'part'])
    print(f"Frontmatter extracted: 00-frontmatter.md")
    print(f"Preface extracted: 00-preface.md")
    print(f"Parts extracted: {part_count}")
    print(f"  - Part 1: Chapters 1-6 (The Foundation of Tonal Music)")
    print(f"  - Part 2: Chapters 7-11 (Merging Melody and Harmony)")
    print(f"  - Part 3: Chapters 12-16 (A New Harmonic Function...)")
    print(f"  - Part 4: Chapters 17-19 (New Chords Complete the Diatonic Spectrum)")
    print(f"  - Part 5: Chapters 20-23 (Creating Larger Forms)")
    print(f"  - Part 6: Chapters 24-26 (Chromaticism)")
    print(f"  - Part 7: Chapters 27-30 (Expressive Chromaticism)")
    print(f"  - Part 8: Chapters 31-33 (Large Forms)")
    print(f"  - Part 9: Chapters 34-37 (Nineteenth-Century Harmony)")
    print(f"Back matter extracted: Index of Terms, Index of Examples")
    print()

    print("Files created:")
    print("-" * 80)
    print(f"{'Filename':<55} {'PDF Pg':<8} {'Lines':<8}")
    print("-" * 80)
    print(f"{'00-frontmatter.md':<55} {'0':<8} {frontmatter_end:<8}")
    for f in created_files:
        print(f"{f['filename']:<55} {f['pdf_page']:<8} {f['line_count']:<8}")

    print()
    print("=" * 80)
    print(f"Split complete! All files created in {OUTPUT_DIR}/")
    print("=" * 80)

if __name__ == "__main__":
    split_into_parts()
