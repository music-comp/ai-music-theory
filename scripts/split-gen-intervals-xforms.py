#!/usr/bin/env python3
"""
Split Generalized Musical Intervals and Transformations (David Lewin)
into individual chapter files.
Uses chapter mapping derived from book.json and book.md analysis.
"""

import re
from pathlib import Path

# Paths
SOURCE_FILE = Path("sources-md/gen-intervals-xforms/book.md")
OUTPUT_DIR = Path("sources-md/gen-intervals-xforms")

# Chapter definitions (derived from analysis of book.json and book.md)
# Note: This book has complex structure - not all chapters have explicit # headings
# PDF pages are from book.json (0-indexed); book pages are from the printed book
CHAPTERS = [
    {
        'num': 1,
        'title': 'Mathematical Preliminaries',
        'start_line': 416,
        'pdf_page': 32,  # Approx: book page 1 + ~31 page offset
    },
    {
        'num': 2,
        'title': 'Generalized Interval Systems (1): Preliminary Examples and Definition',
        'start_line': 656,
        'pdf_page': 47,
    },
    {
        'num': 3,
        'title': 'Generalized Interval Systems (2): Formal Features',
        'start_line': 819,
        'pdf_page': 62,
    },
    {
        'num': 4,
        'title': 'Generalized Interval Systems (3): A Non-Commutative GIS; Some Timbral GIS Models',
        'start_line': 1415,
        'pdf_page': 91,
    },
    {
        'num': 5,
        'title': 'Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions',
        'start_line': 1809,
        'pdf_page': 119,  # Approx: book page 88 + ~31
    },
    {
        'num': 6,
        'title': 'Generalized Set Theory (2): The Injection Function',
        'start_line': 2316,
        'pdf_page': 154,  # Approx: book page 123 + ~31
    },
    {
        'num': 7,
        'title': 'Transformation Graphs and Networks (1): Intervals and Transpositions',
        'start_line': 2777,
        'pdf_page': 188,  # Approx: book page 157 + ~31
    },
    {
        'num': 8,
        'title': 'Transformation Graphs and Networks (2): Non-Intervallic Transformations',
        'start_line': 2978,
        'pdf_page': 206,  # Approx: book page 175 + ~31
    },
    {
        'num': 9,
        'title': 'Transformation Graphs and Networks (3): Formalities',
        'start_line': 3177,
        'pdf_page': 224,  # Approx: book page 193 + ~31
    },
    {
        'num': 10,
        'title': 'Transformation Graphs and Networks (4): Some Further Analyses',
        'start_line': 3557,
        'pdf_page': 251,  # Approx: book page 220 + ~31
    },
]

APPENDICES = [
    {
        'id': 'A',
        'title': 'Melodic and Harmonic GIS Structures; Some Notes on the History of Tonal Theory',
        'start_line': 3843,
        'pdf_page': 276,
    },
    {
        'id': 'B',
        'title': 'Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups',
        'start_line': 3895,
        'pdf_page': 282,  # Approximate
    },
]

# Index starts around line 3921
INDEX_START = 3921

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
    print("Splitting Generalized Musical Intervals and Transformations")
    print("by David Lewin")
    print("=" * 80)
    print()

    print(f"Reading {SOURCE_FILE}...")
    with open(SOURCE_FILE, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    total_lines = len(lines)
    print(f"Total lines: {total_lines}")
    print()

    # Extract frontmatter (everything before first chapter)
    frontmatter_end = CHAPTERS[0]['start_line']
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

    # Build complete list of sections (chapters + appendices + index)
    all_sections = []

    for ch in CHAPTERS:
        all_sections.append({
            'type': 'chapter',
            'num': ch['num'],
            'title': ch['title'],
            'start_line': ch['start_line'],
            'pdf_page': ch['pdf_page'],
        })

    for app in APPENDICES:
        all_sections.append({
            'type': 'appendix',
            'id': app['id'],
            'title': app['title'],
            'start_line': app['start_line'],
            'pdf_page': app['pdf_page'],
        })

    all_sections.append({
        'type': 'index',
        'title': 'Index',
        'start_line': INDEX_START,
        'pdf_page': 286,  # Approximate
    })

    # Extract each section
    print("Extracting chapters and appendices:")
    print("-" * 80)

    created_files = []

    for i, section in enumerate(all_sections):
        start = section['start_line']
        pdf_page = section['pdf_page']
        title = section['title']

        # End is either the next section or end of file
        if i + 1 < len(all_sections):
            end = all_sections[i + 1]['start_line']
        else:
            end = total_lines

        # Extract section content
        section_content = ''.join(lines[start:end])

        # Create filename based on section type
        if section['type'] == 'chapter':
            chapter_num = section['num']
            title_slug = slugify(title)
            filename = f"{chapter_num:02d}-{title_slug}.md"
        elif section['type'] == 'appendix':
            app_id = section['id']
            title_slug = slugify(title)
            filename = f"appendix-{app_id.lower()}-{title_slug}.md"
        else:  # index
            filename = "index.md"

        filepath = OUTPUT_DIR / filename

        # Write section file with metadata header
        with open(filepath, 'w', encoding='utf-8') as f:
            # Add YAML metadata header
            f.write("---\n")
            f.write(f"title: {title}\n")
            if section['type'] == 'chapter':
                f.write(f"chapter_number: {section['num']}\n")
            elif section['type'] == 'appendix':
                f.write(f"appendix_id: {section['id']}\n")
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
        else:
            print(f"  {section['type'].capitalize()} → {filename}")
        print(f"      PDF page {pdf_page:3d} | Lines {start:4d}-{end:4d} ({line_count:4d} lines)")

    print()
    print("=" * 80)
    print("Summary")
    print("=" * 80)
    print(f"✓ Frontmatter extracted: 00-frontmatter.md")
    print(f"✓ Chapters extracted: {len(CHAPTERS)}")
    print(f"✓ Appendices extracted: {len(APPENDICES)}")
    print(f"✓ Index extracted: 1")
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
