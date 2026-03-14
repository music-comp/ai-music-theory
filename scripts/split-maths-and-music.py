#!/usr/bin/env python3
"""
Split Mathematics and Music by David Wright into individual chapter files.
Uses chapter mapping derived from book.json TOC and book.md heading analysis.
"""

import re
from pathlib import Path

# Paths
SOURCE_FILE = Path("sources-md/maths-and-music/book.md")
OUTPUT_DIR = Path("sources-md/maths-and-music")

# Section definitions derived from book.json TOC and book.md heading analysis
# page_id values from book.json, start_line values from grep of book.md headings
SECTIONS = [
    {
        'type': 'intro',
        'num': 0,
        'title': 'Introduction',
        'start_line': 32,
        'pdf_page': 6,
    },
    {
        'type': 'chapter',
        'num': 1,
        'title': 'Basic Mathematical and Musical Concepts',
        'start_line': 105,
        'pdf_page': 14,
    },
    {
        'type': 'chapter',
        'num': 2,
        'title': 'Horizontal Structure',
        'start_line': 377,
        'pdf_page': 30,
    },
    {
        'type': 'chapter',
        'num': 3,
        'title': 'Harmony and Related Numerology',
        'start_line': 617,
        'pdf_page': 44,
    },
    {
        'type': 'chapter',
        'num': 4,
        'title': 'Ratios and Musical Intervals',
        'start_line': 880,
        'pdf_page': 58,
    },
    {
        'type': 'chapter',
        'num': 5,
        'title': 'Logarithms and Musical Intervals',
        'start_line': 1050,
        'pdf_page': 66,
    },
    {
        'type': 'chapter',
        'num': 6,
        'title': 'Chromatic Scales',
        'start_line': 1254,
        'pdf_page': 74,
    },
    {
        'type': 'chapter',
        'num': 7,
        'title': 'Octave Identification and Modular Arithmetic',
        'start_line': 1372,
        'pdf_page': 82,
    },
    {
        'type': 'chapter',
        'num': 8,
        'title': 'Algebraic Properties of the Integers',
        'start_line': 1732,
        'pdf_page': 100,
    },
    {
        'type': 'chapter',
        'num': 9,
        'title': 'The Integers as Intervals',
        'start_line': 1931,
        'pdf_page': 110,
    },
    {
        'type': 'chapter',
        'num': 10,
        'title': 'Timbre and Periodic Functions',
        'start_line': 2069,
        'pdf_page': 118,
    },
    {
        'type': 'chapter',
        'num': 11,
        'title': 'The Rational Numbers As Musical Intervals',
        'start_line': 2477,
        'pdf_page': 138,
    },
    {
        'type': 'chapter',
        'num': 12,
        'title': 'Tuning The Scale To Obtain Rational Intervals',
        'start_line': 2735,
        'pdf_page': 152,
    },
    {
        'type': 'backmatter',
        'id': 'bibliography',
        'title': 'Bibliography',
        'start_line': 2884,
        'pdf_page': 162,
    },
    {
        'type': 'backmatter',
        'id': 'index',
        'title': 'Index',
        'start_line': 2895,
        'pdf_page': 163,
    },
]


def slugify(text):
    """Convert text to filename-safe slug."""
    text = text.lower()
    text = re.sub(r'[^\w\s-]', '', text)
    text = re.sub(r'[-\s]+', '-', text)
    if len(text) > 50:
        text = text[:50]
    return text.strip('-')


def split_into_chapters():
    """Main function to split the book into chapters."""
    print("=" * 80)
    print("Splitting Mathematics and Music")
    print("by David Wright")
    print("=" * 80)
    print()

    print(f"Reading {SOURCE_FILE}...")
    with open(SOURCE_FILE, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    total_lines = len(lines)
    print(f"Total lines: {total_lines}")
    print()

    # Extract frontmatter (everything before Introduction)
    frontmatter_end = SECTIONS[0]['start_line']
    frontmatter_content = ''.join(lines[:frontmatter_end])

    frontmatter_file = OUTPUT_DIR / "00-frontmatter.md"
    with open(frontmatter_file, 'w', encoding='utf-8') as f:
        f.write("---\n")
        f.write("title: Frontmatter\n")
        f.write("pdf_page: 0\n")
        f.write("book_md_line: 1\n")
        f.write("---\n\n")
        f.write(frontmatter_content)

    print(f"  Extracted frontmatter to {frontmatter_file.name}")
    print(f"      Lines: 1-{frontmatter_end} ({frontmatter_end} lines)")
    print()

    created_files = []

    print("Extracting sections:")
    print("-" * 80)

    for i, section in enumerate(SECTIONS):
        start = section['start_line']
        pdf_page = section['pdf_page']
        title = section['title']

        # End is either the next section or end of file
        if i + 1 < len(SECTIONS):
            end = SECTIONS[i + 1]['start_line']
        else:
            end = total_lines

        section_content = ''.join(lines[start:end])
        title_slug = slugify(title)

        if section['type'] == 'intro':
            filename = "00-introduction.md"
        elif section['type'] == 'chapter':
            ch_num = section['num']
            filename = f"{ch_num:02d}-{title_slug}.md"
        else:  # backmatter
            filename = f"{section['id']}.md"

        filepath = OUTPUT_DIR / filename

        with open(filepath, 'w', encoding='utf-8') as f:
            f.write("---\n")
            f.write(f"title: \"{title}\"\n")
            if section['type'] == 'chapter':
                f.write(f"chapter_number: {section['num']}\n")
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

        label = section['type'].capitalize()
        if section['type'] == 'chapter':
            label = f"Ch {section['num']:2d}"
        print(f"  {label:>10s} -> {filename}")
        print(f"             PDF page {pdf_page:3d} | Lines {start:5d}-{end:5d} ({line_count:5d} lines)")

    print()
    print("=" * 80)
    print("Summary")
    print("=" * 80)

    chapter_count = len([s for s in SECTIONS if s['type'] == 'chapter'])
    backmatter_count = len([s for s in SECTIONS if s['type'] == 'backmatter'])

    print(f"  Frontmatter: 1 file")
    print(f"  Introduction: 1 file")
    print(f"  Chapters: {chapter_count} files")
    print(f"  Backmatter: {backmatter_count} files")
    print(f"  Total files: {1 + len(created_files)}")
    print()

    print("Files created:")
    print("-" * 80)
    print(f"{'Filename':<55} {'PDF Pg':<8} {'Lines':<8}")
    print("-" * 80)
    print(f"{'00-frontmatter.md':<55} {'0':<8} {frontmatter_end:<8}")
    for cf in created_files:
        print(f"{cf['filename']:<55} {cf['pdf_page']:<8} {cf['line_count']:<8}")

    print()
    print("=" * 80)
    print(f"Split complete! All files created in {OUTPUT_DIR}/")
    print("=" * 80)


if __name__ == "__main__":
    split_into_chapters()
