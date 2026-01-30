#!/usr/bin/env python3
"""
Split Audacious Euphony: Chromaticism and the Triad's Second Nature
by Richard Cohn into individual chapter files.
Uses chapter mapping derived from book.json and book.md analysis.
"""

import re
from pathlib import Path

# Paths
SOURCE_FILE = Path("sources-md/audacious-euphony/book.md")
OUTPUT_DIR = Path("sources-md/audacious-euphony")

# Chapter definitions (derived from analysis of book.json and book.md)
SECTIONS = [
    {
        'type': 'intro',
        'num': 0,
        'title': 'Introduction',
        'start_line': 162,
        'pdf_page': 9,
    },
    {
        'type': 'chapter',
        'num': 1,
        'title': 'Mapping the Triadic Universe',
        'start_line': 254,
        'pdf_page': 19,
    },
    {
        'type': 'chapter',
        'num': 2,
        'title': 'Hexatonic Cycles',
        'start_line': 410,
        'pdf_page': 35,
    },
    {
        'type': 'chapter',
        'num': 3,
        'title': 'Reciprocity',
        'start_line': 706,
        'pdf_page': 61,
    },
    {
        'type': 'chapter',
        'num': 4,
        'title': 'Weitzmann Regions',
        'start_line': 898,
        'pdf_page': 77,
    },
    {
        'type': 'chapter',
        'num': 5,
        'title': 'A Unified Model of Triadic Voice-Leading Space',
        'start_line': 1160,
        'pdf_page': 101,
    },
    {
        'type': 'chapter',
        'num': 6,
        'title': 'Navigating the Triadic Universe: Three Compositional Scripts',
        'start_line': 1493,
        'pdf_page': 129,
    },
    {
        'type': 'chapter',
        'num': 7,
        'title': 'Dissonance',
        'start_line': 1792,
        'pdf_page': 156,
    },
    {
        'type': 'chapter',
        'num': 8,
        'title': 'Syntactic Interaction and the Convertible Tonnetz',
        'start_line': 2203,
        'pdf_page': 186,
    },
    {
        'type': 'chapter',
        'num': 9,
        'title': 'Double Syntax and the Soft Revolution',
        'start_line': 2454,
        'pdf_page': 212,
    },
    {
        'type': 'backmatter',
        'id': 'glossary',
        'title': 'Glossary',
        'start_line': 2622,
        'pdf_page': 229,
    },
    {
        'type': 'backmatter',
        'id': 'bibliography',
        'title': 'Bibliography',
        'start_line': 2671,
        'pdf_page': 233,
    },
    {
        'type': 'backmatter',
        'id': 'index',
        'title': 'Index',
        'start_line': 2993,
        'pdf_page': 247,
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
    print("Splitting Audacious Euphony: Chromaticism and the Triad's Second Nature")
    print("by Richard Cohn")
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
        if section['type'] == 'intro':
            filename = "00-introduction.md"
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
        elif section['type'] == 'intro':
            print(f"  Intro  → {filename}")
        else:
            print(f"  {title} → {filename}")
        print(f"      PDF page {pdf_page:3d} | Lines {start:4d}-{end:4d} ({line_count:4d} lines)")

    print()
    print("=" * 80)
    print("Summary")
    print("=" * 80)
    chapter_count = len([s for s in SECTIONS if s['type'] == 'chapter'])
    print(f"✓ Frontmatter extracted: 00-frontmatter.md")
    print(f"✓ Introduction extracted: 00-introduction.md")
    print(f"✓ Chapters extracted: {chapter_count}")
    print(f"✓ Back matter extracted: glossary.md, bibliography.md, index.md")
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
