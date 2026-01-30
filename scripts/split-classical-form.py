#!/usr/bin/env python3
"""
Split Classical Form: A Theory of Formal Functions for the Instrumental Music
of Haydn, Mozart, and Beethoven by William E. Caplin into individual chapter files.
Uses chapter mapping derived from book.json and book.md analysis.
"""

import re
from pathlib import Path

# Paths
SOURCE_FILE = Path("sources-md/classical-form/book.md")
OUTPUT_DIR = Path("sources-md/classical-form")

# Section definitions (derived from analysis of book.json and book.md)
# The book has Parts I-IV containing 17 chapters total
SECTIONS = [
    {
        'type': 'preface',
        'title': 'Preface',
        'start_line': 48,
        'pdf_page': 6,
    },
    {
        'type': 'note',
        'title': 'Note on the Musical Examples',
        'start_line': 64,
        'pdf_page': 8,
    },
    {
        'type': 'intro',
        'title': 'Introduction',
        'start_line': 124,
        'pdf_page': 14,
    },
    # Part I: Preliminaries
    {
        'type': 'chapter',
        'num': 1,
        'part': 'I',
        'title': 'Some Basic Formal Functions: An Overview',
        'start_line': 168,
        'pdf_page': 20,
    },
    {
        'type': 'chapter',
        'num': 2,
        'part': 'I',
        'title': 'Fundamental Progressions of Harmony',
        'start_line': 412,
        'pdf_page': 34,
    },
    # Part II: Tight-Knit Themes
    {
        'type': 'chapter',
        'num': 3,
        'part': 'II',
        'title': 'Sentence',
        'start_line': 597,
        'pdf_page': 46,
    },
    {
        'type': 'chapter',
        'num': 4,
        'part': 'II',
        'title': 'Period',
        'start_line': 895,
        'pdf_page': 60,
    },
    {
        'type': 'chapter',
        'num': 5,
        'part': 'II',
        'title': 'Hybrid Themes and Compound Themes',
        'start_line': 1087,
        'pdf_page': 70,
    },
    {
        'type': 'chapter',
        'num': 6,
        'part': 'II',
        'title': 'Small Ternary',
        'start_line': 1307,
        'pdf_page': 82,
    },
    {
        'type': 'chapter',
        'num': 7,
        'part': 'II',
        'title': 'Small Binary',
        'start_line': 1557,
        'pdf_page': 98,
    },
    # Part III: Looser Formal Regions
    {
        'type': 'chapter',
        'num': 8,
        'part': 'III',
        'title': 'Subordinate Theme',
        'start_line': 1687,
        'pdf_page': 108,
    },
    {
        'type': 'chapter',
        'num': 9,
        'part': 'III',
        'title': 'Transition',
        'start_line': 2123,
        'pdf_page': 136,
    },
    {
        'type': 'chapter',
        'num': 10,
        'part': 'III',
        'title': 'Development',
        'start_line': 2335,
        'pdf_page': 150,
    },
    {
        'type': 'chapter',
        'num': 11,
        'part': 'III',
        'title': 'Recapitulation',
        'start_line': 2639,
        'pdf_page': 172,
    },
    {
        'type': 'chapter',
        'num': 12,
        'part': 'III',
        'title': 'Coda',
        'start_line': 2893,
        'pdf_page': 190,
    },
    # Part IV: Full-Movement Forms
    {
        'type': 'chapter',
        'num': 13,
        'part': 'IV',
        'title': 'Sonata Form',
        'start_line': 3071,
        'pdf_page': 206,
    },
    {
        'type': 'chapter',
        'num': 14,
        'part': 'IV',
        'title': 'Slow-Movement Forms',
        'start_line': 3311,
        'pdf_page': 220,
    },
    {
        'type': 'chapter',
        'num': 15,
        'part': 'IV',
        'title': 'Minuet/Trio Form',
        'start_line': 3504,
        'pdf_page': 230,
    },
    {
        'type': 'chapter',
        'num': 16,
        'part': 'IV',
        'title': 'Rondo Forms',
        'start_line': 3704,
        'pdf_page': 242,
    },
    {
        'type': 'chapter',
        'num': 17,
        'part': 'IV',
        'title': 'Concerto Form',
        'start_line': 3911,
        'pdf_page': 254,
    },
    # Back matter
    {
        'type': 'backmatter',
        'id': 'glossary',
        'title': 'Glossary of Terms',
        'start_line': 4084,
        'pdf_page': 264,
    },
    {
        'type': 'backmatter',
        'id': 'notes',
        'title': 'Notes',
        'start_line': 4299,
        'pdf_page': 270,
    },
    {
        'type': 'backmatter',
        'id': 'bibliography',
        'title': 'Bibliography',
        'start_line': 5160,
        'pdf_page': 300,
    },
    {
        'type': 'backmatter',
        'id': 'index-compositions',
        'title': 'Index of Classical Compositions',
        'start_line': 5321,
        'pdf_page': 304,
    },
    {
        'type': 'backmatter',
        'id': 'index-general',
        'title': 'General Index',
        'start_line': 5616,
        'pdf_page': 308,
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
    print("Splitting Classical Form: A Theory of Formal Functions")
    print("by William E. Caplin")
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
        if section['type'] in ['preface', 'note', 'intro']:
            filename = f"00-{title_slug}.md"
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
                f.write(f"part: {section['part']}\n")
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
            print(f"  Ch {section['num']:2d} (Part {section['part']}) → {filename}")
        elif section['type'] in ['preface', 'note', 'intro']:
            print(f"  {title} → {filename}")
        else:
            print(f"  {title} → {filename}")
        print(f"      PDF page {pdf_page:3d} | Lines {start:4d}-{end:4d} ({line_count:4d} lines)")

    print()
    print("=" * 80)
    print("Summary")
    print("=" * 80)
    chapter_count = len([s for s in SECTIONS if s['type'] == 'chapter'])
    print(f"✓ Frontmatter extracted: 00-frontmatter.md")
    print(f"✓ Preface, Note, Introduction extracted")
    print(f"✓ Chapters extracted: {chapter_count} (Parts I-IV)")
    print(f"✓ Back matter extracted: glossary, notes, bibliography, indexes")
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
