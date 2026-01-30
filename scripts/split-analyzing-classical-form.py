#!/usr/bin/env python3
"""
Split Analyzing Classical Form: An Approach for the Classroom
by William E. Caplin into individual chapter files.
Uses chapter mapping derived from book.json and book.md analysis.
"""

import re
from pathlib import Path

# Paths
SOURCE_FILE = Path("sources-md/analyzing-classical-form/book.md")
OUTPUT_DIR = Path("sources-md/analyzing-classical-form")

# Section definitions (derived from analysis of book.json and book.md)
# The book has:
# - Chapter 1 (before Part I)
# - Part I: Conventional Theme Types (Chapters 2-8)
# - Part II: Sonata Form (Chapters 9-16)
# - Part III: Other Full-Movement Forms (Chapters 17-20)
# - Back matter: Notes, Glossary, Indexes

SECTIONS = [
    {
        'type': 'preface',
        'title': 'Preface',
        'start_line': 344,
        'pdf_page': 15,
    },
    {
        'type': 'intro',
        'title': 'About the Companion Website',
        'start_line': 405,
        'pdf_page': 19,
    },
    # Chapter 1 comes before Part I
    {
        'type': 'chapter',
        'num': 1,
        'part': None,
        'title': 'A Review of Harmony',
        'start_line': 415,
        'pdf_page': 23,
    },
    # Part I: Conventional Theme Types
    {
        'type': 'chapter',
        'num': 2,
        'part': 'I',
        'title': 'The Sentence',
        'start_line': 1019,  # Includes Part I title page
        'pdf_page': 55,
    },
    {
        'type': 'chapter',
        'num': 3,
        'part': 'I',
        'title': 'The Period',
        'start_line': 1778,
        'pdf_page': 95,
    },
    {
        'type': 'chapter',
        'num': 4,
        'part': 'I',
        'title': 'Hybrid Themes',
        'start_line': 2236,
        'pdf_page': 121,
    },
    {
        'type': 'chapter',
        'num': 5,
        'part': 'I',
        'title': 'Phrase Deviations, Cadential Deviations, and Framing Functions',
        'start_line': 2659,
        'pdf_page': 145,
    },
    {
        'type': 'chapter',
        'num': 6,
        'part': 'I',
        'title': 'Compound Themes',
        'start_line': 3309,
        'pdf_page': 188,
    },
    {
        'type': 'chapter',
        'num': 7,
        'part': 'I',
        'title': 'The Small Ternary (Rounded Binary)',
        'start_line': 3726,
        'pdf_page': 217,
    },
    {
        'type': 'chapter',
        'num': 8,
        'part': 'I',
        'title': 'The Small Binary',
        'start_line': 4394,
        'pdf_page': 260,
    },
    # Part II: Sonata Form
    {
        'type': 'chapter',
        'num': 9,
        'part': 'II',
        'title': 'Sonata Form: An Overview',
        'start_line': 4711,  # Includes Part II title page
        'pdf_page': 283,
    },
    {
        'type': 'chapter',
        'num': 10,
        'part': 'II',
        'title': 'Exposition (I): Main Theme',
        'start_line': 5122,
        'pdf_page': 308,
    },
    {
        'type': 'chapter',
        'num': 11,
        'part': 'II',
        'title': 'Exposition (II): Transition',
        'start_line': 5410,
        'pdf_page': 330,
    },
    {
        'type': 'chapter',
        'num': 12,
        'part': 'II',
        'title': 'Exposition (III): Subordinate Theme',
        'start_line': 6028,
        'pdf_page': 375,
    },
    {
        'type': 'chapter',
        'num': 13,
        'part': 'II',
        'title': 'Development',
        'start_line': 6915,
        'pdf_page': 442,
    },
    {
        'type': 'chapter',
        'num': 14,
        'part': 'II',
        'title': 'Recapitulation',
        'start_line': 7650,
        'pdf_page': 497,
    },
    {
        'type': 'chapter',
        'num': 15,
        'part': 'II',
        'title': 'Coda',
        'start_line': 8262,
        'pdf_page': 541,
    },
    {
        'type': 'chapter',
        'num': 16,
        'part': 'II',
        'title': 'Slow Introduction',
        'start_line': 8624,
        'pdf_page': 573,
    },
    # Part III: Other Full-Movement Forms
    {
        'type': 'chapter',
        'num': 17,
        'part': 'III',
        'title': 'Slow-Movement Forms',
        'start_line': 8781,  # Includes Part III title page
        'pdf_page': 587,
    },
    {
        'type': 'chapter',
        'num': 18,
        'part': 'III',
        'title': 'Minuet/Trio Form',
        'start_line': 9289,
        'pdf_page': 629,
    },
    {
        'type': 'chapter',
        'num': 19,
        'part': 'III',
        'title': 'Rondo Forms',
        'start_line': 9760,
        'pdf_page': 664,
    },
    {
        'type': 'chapter',
        'num': 20,
        'part': 'III',
        'title': 'Concerto Form',
        'start_line': 10221,
        'pdf_page': 694,
    },
    # Back matter
    {
        'type': 'backmatter',
        'id': 'notes',
        'title': 'Notes',
        'start_line': 10644,
        'pdf_page': 721,
    },
    {
        'type': 'backmatter',
        'id': 'glossary',
        'title': 'Glossary of Terms',
        'start_line': 10751,
        'pdf_page': 725,
    },
    {
        'type': 'backmatter',
        'id': 'index-compositions',
        'title': 'Index of Musical Compositions',
        'start_line': 11005,
        'pdf_page': 739,
    },
    {
        'type': 'backmatter',
        'id': 'index-general',
        'title': 'General Index',
        'start_line': 11258,
        'pdf_page': 744,
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
    print("Splitting Analyzing Classical Form: An Approach for the Classroom")
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

    print(f"Extracted frontmatter to {frontmatter_file.name}")
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
        if section['type'] in ['preface', 'intro']:
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
            f.write(f"title: \"{title}\"\n")
            if section['type'] == 'chapter':
                f.write(f"chapter_number: {section['num']}\n")
                if section['part']:
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
            part_str = f" (Part {section['part']})" if section['part'] else ""
            print(f"  Ch {section['num']:2d}{part_str} -> {filename}")
        elif section['type'] in ['preface', 'intro']:
            print(f"  {title} -> {filename}")
        else:
            print(f"  {title} -> {filename}")
        print(f"      PDF page {pdf_page:3d} | Lines {start:5d}-{end:5d} ({line_count:4d} lines)")

    print()
    print("=" * 80)
    print("Summary")
    print("=" * 80)
    chapter_count = len([s for s in SECTIONS if s['type'] == 'chapter'])
    print(f"Frontmatter extracted: 00-frontmatter.md")
    print(f"Preface and About extracted")
    print(f"Chapters extracted: {chapter_count}")
    print(f"  - Chapter 1 (before Part I)")
    print(f"  - Part I: Chapters 2-8 (Conventional Theme Types)")
    print(f"  - Part II: Chapters 9-16 (Sonata Form)")
    print(f"  - Part III: Chapters 17-20 (Other Full-Movement Forms)")
    print(f"Back matter extracted: notes, glossary, indexes")
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
    split_into_chapters()
