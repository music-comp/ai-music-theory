#!/usr/bin/env python3
"""
Split Twentieth-Century Harmony: Creative Aspects and Practice
by Vincent Persichetti into individual chapter files.
Uses chapter mapping derived from book.json and book.md analysis.
"""

import re
from pathlib import Path

# Paths
SOURCE_FILE = Path("sources-md/20th-century-harmony/book.md")
OUTPUT_DIR = Path("sources-md/20th-century-harmony")

# Chapter definitions (derived from analysis of book.json and book.md)
SECTIONS = [
    {
        'type': 'foreword',
        'title': 'Foreword',
        'start_line': 113,
        'pdf_page': 8,
    },
    {
        'type': 'chapter',
        'num': 1,
        'title': 'Intervals',
        'start_line': 139,
        'pdf_page': 12,
    },
    {
        'type': 'chapter',
        'num': 2,
        'title': 'Scale Materials',
        'start_line': 451,
        'pdf_page': 30,
    },
    {
        'type': 'chapter',
        'num': 3,
        'title': 'Chords by Thirds',
        'start_line': 1037,
        'pdf_page': 65,
    },
    {
        'type': 'chapter',
        'num': 4,
        'title': 'Chords by Fourths',
        'start_line': 1492,
        'pdf_page': 92,
    },
    {
        'type': 'chapter',
        'num': 5,
        'title': 'Added-Note Chords',
        'start_line': 1755,
        'pdf_page': 108,
    },
    {
        'type': 'chapter',
        'num': 6,
        'title': 'Chords by Seconds',
        'start_line': 1913,
        'pdf_page': 120,
    },
    {
        'type': 'chapter',
        'num': 7,
        'title': 'Polychords',
        'start_line': 2141,
        'pdf_page': 134,
    },
    {
        'type': 'chapter',
        'num': 8,
        'title': 'Compound and Mirror Harmony',
        'start_line': 2528,
        'pdf_page': 162,
    },
    {
        'type': 'chapter',
        'num': 9,
        'title': 'Harmonic Direction',
        'start_line': 2794,
        'pdf_page': 181,
    },
    {
        'type': 'chapter',
        'num': 10,
        'title': 'Timing and Dynamics',
        'start_line': 3300,
        'pdf_page': 211,
    },
    {
        'type': 'chapter',
        'num': 11,
        'title': 'Embellishment and Transformation',
        'start_line': 3639,
        'pdf_page': 229,
    },
    {
        'type': 'chapter',
        'num': 12,
        'title': 'Key Centers',
        'start_line': 3951,
        'pdf_page': 247,
    },
    {
        'type': 'chapter',
        'num': 13,
        'title': 'Harmonic Synthesis',
        'start_line': 4331,
        'pdf_page': 270,
    },
    {
        'type': 'backmatter',
        'id': 'index-composers',
        'title': 'Index of Composers',
        'start_line': 4448,
        'pdf_page': 280,
    },
    {
        'type': 'backmatter',
        'id': 'index-subject',
        'title': 'Subject Index',
        'start_line': 4555,
        'pdf_page': 284,
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

def split_into_chapters():
    """Main function to split the book into chapters."""
    print("=" * 80)
    print("Splitting Twentieth-Century Harmony: Creative Aspects and Practice")
    print("by Vincent Persichetti")
    print("=" * 80)
    print()

    print(f"Reading {SOURCE_FILE}...")
    with open(SOURCE_FILE, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    total_lines = len(lines)
    print(f"Total lines: {total_lines}")
    print()

    # Extract frontmatter (everything before Foreword)
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

    print(f"✓ Extracted frontmatter to {frontmatter_file.name}")
    print(f"  Lines: 1-{frontmatter_end} ({frontmatter_end} lines)")
    print()

    # Extract each section
    print("Extracting chapters and back matter:")
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
        title_slug = slugify(title)
        if section['type'] == 'foreword':
            filename = "00-foreword.md"
        elif section['type'] == 'chapter':
            chapter_num = section['num']
            filename = f"{chapter_num:02d}-{title_slug}.md"
        else:  # backmatter
            filename = f"{section['id']}.md"

        filepath = OUTPUT_DIR / filename

        # Write section file with metadata header
        with open(filepath, 'w', encoding='utf-8') as f:
            # Add YAML metadata header
            f.write("---\n")
            f.write(f"title: {title}\n")
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

        if section['type'] == 'chapter':
            print(f"  Ch {section['num']:2d} → {filename}")
        elif section['type'] == 'foreword':
            print(f"  Foreword → {filename}")
        else:
            print(f"  {title} → {filename}")
        print(f"      PDF page {pdf_page:3d} | Lines {start:4d}-{end:4d} ({line_count:4d} lines)")

    print()
    print("=" * 80)
    print("Summary")
    print("=" * 80)
    chapter_count = len([s for s in SECTIONS if s['type'] == 'chapter'])
    print(f"✓ Frontmatter extracted: 00-frontmatter.md")
    print(f"✓ Foreword extracted: 00-foreword.md")
    print(f"✓ Chapters extracted: {chapter_count}")
    print(f"✓ Indexes extracted: index-composers.md, index-subject.md")
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
    print(f"✓ Split complete! All files created in {OUTPUT_DIR}/")
    print("=" * 80)

if __name__ == "__main__":
    split_into_chapters()
