#!/usr/bin/env python3
"""
Split Fundamentals of Musical Composition by Arnold Schoenberg
into individual chapter files.
Uses chapter mapping derived from book.json and book.md analysis.
"""

import re
from pathlib import Path

# Paths
SOURCE_FILE = Path("sources-md/fundamentals-music-comp/book.md")
OUTPUT_DIR = Path("sources-md/fundamentals-music-comp")

# Section definitions (derived from analysis of book.json and book.md)
# The book has:
# - Front matter: Introduction, Editor's Preface, Explanatory Note, Glossary
# - Part I: Construction of Themes (Chapters I-XII)
# - Part II: Small Forms (Chapters XIII-XVII)
# - Part III: Large Forms (Chapters XVIII-XX)
# - Back matter: Appendix, Index

SECTIONS = [
    {
        'type': 'intro',
        'title': 'Introduction',
        'start_line': 146,
        'pdf_page': 6,
    },
    {
        'type': 'preface',
        'title': "Editor's Preface",
        'start_line': 156,
        'pdf_page': 7,
    },
    {
        'type': 'note',
        'title': 'Explanatory Note',
        'start_line': 178,
        'pdf_page': 8,
    },
    {
        'type': 'glossary_front',
        'title': 'Glossary',
        'start_line': 198,
        'pdf_page': 9,
    },
    # Part I: Construction of Themes
    {
        'type': 'chapter',
        'num': 1,
        'part': 'I',
        'title': 'The Concept of Form',
        'start_line': 216,  # Includes Part I header
        'pdf_page': 10,
    },
    {
        'type': 'chapter',
        'num': 2,
        'part': 'I',
        'title': 'The Phrase',
        'start_line': 246,
        'pdf_page': 11,
    },
    {
        'type': 'chapter',
        'num': 3,
        'part': 'I',
        'title': 'The Motive',
        'start_line': 298,
        'pdf_page': 14,
    },
    {
        'type': 'chapter',
        'num': 4,
        'part': 'I',
        'title': 'Connecting Motive-Forms',
        'start_line': 409,
        'pdf_page': 18,
    },
    {
        'type': 'chapter',
        'num': 5,
        'part': 'I',
        'title': 'Construction of Simple Themes (1): Beginning the Sentence',
        'start_line': 461,
        'pdf_page': 20,
    },
    {
        'type': 'chapter',
        'num': 6,
        'part': 'I',
        'title': 'Construction of Simple Themes (2): Antecedent of the Period',
        'start_line': 547,
        'pdf_page': 22,
    },
    {
        'type': 'chapter',
        'num': 7,
        'part': 'I',
        'title': 'Construction of Simple Themes (3): Consequent of the Period',
        'start_line': 627,
        'pdf_page': 24,
    },
    {
        'type': 'chapter',
        'num': 8,
        'part': 'I',
        'title': 'Construction of Simple Themes (4): Completion of the Sentence',
        'start_line': 743,
        'pdf_page': 39,
    },
    {
        'type': 'chapter',
        'num': 9,
        'part': 'I',
        'title': 'The Accompaniment',
        'start_line': 898,
        'pdf_page': 51,
    },
    {
        'type': 'chapter',
        'num': 10,
        'part': 'I',
        'title': 'Character and Mood',
        'start_line': 1036,
        'pdf_page': 56,
    },
    {
        'type': 'chapter',
        'num': 11,
        'part': 'I',
        'title': 'Melody and Theme',
        'start_line': 1080,
        'pdf_page': 59,
    },
    {
        'type': 'chapter',
        'num': 12,
        'part': 'I',
        'title': 'Advice for Self-Criticism',
        'start_line': 1305,
        'pdf_page': 68,
    },
    # Part II: Small Forms
    {
        'type': 'chapter',
        'num': 13,
        'part': 'II',
        'title': 'The Small Ternary Form',
        'start_line': 1365,  # Includes Part II header
        'pdf_page': 70,
    },
    {
        'type': 'chapter',
        'num': 14,
        'part': 'II',
        'title': 'Uneven, Irregular and Asymmetrical Construction',
        'start_line': 1543,
        'pdf_page': 76,
    },
    {
        'type': 'chapter',
        'num': 15,
        'part': 'II',
        'title': 'The Minuet',
        'start_line': 1575,
        'pdf_page': 78,
    },
    {
        'type': 'chapter',
        'num': 16,
        'part': 'II',
        'title': 'The Scherzo',
        'start_line': 1666,
        'pdf_page': 85,
    },
    {
        'type': 'chapter',
        'num': 17,
        'part': 'II',
        'title': 'Theme and Variations',
        'start_line': 1848,
        'pdf_page': 95,
    },
    # Part III: Large Forms
    {
        'type': 'chapter',
        'num': 18,
        'part': 'III',
        'title': 'The Parts of Larger Forms (Subsidiary Formulations)',
        'start_line': 2025,  # Includes Part III header
        'pdf_page': 100,
    },
    {
        'type': 'chapter',
        'num': 19,
        'part': 'III',
        'title': 'The Rondo Forms',
        'start_line': 2237,
        'pdf_page': 106,
    },
    {
        'type': 'chapter',
        'num': 20,
        'part': 'III',
        'title': 'The Sonata-Allegro (First Movement Form)',
        'start_line': 2416,
        'pdf_page': 110,
    },
    # Back matter
    {
        'type': 'backmatter',
        'id': 'appendix',
        'title': 'Appendix',
        'start_line': 2697,
        'pdf_page': 118,
    },
    {
        'type': 'backmatter',
        'id': 'index',
        'title': 'Index',
        'start_line': 2737,
        'pdf_page': 119,
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

def roman_to_int(roman):
    """Convert Roman numeral to integer for chapter numbering."""
    roman_values = {
        'I': 1, 'II': 2, 'III': 3, 'IV': 4, 'V': 5,
        'VI': 6, 'VII': 7, 'VIII': 8, 'IX': 9, 'X': 10,
        'XI': 11, 'XII': 12, 'XIII': 13, 'XIV': 14, 'XV': 15,
        'XVI': 16, 'XVII': 17, 'XVIII': 18, 'XIX': 19, 'XX': 20
    }
    return roman_values.get(roman, 0)

def split_into_chapters():
    """Main function to split the book into chapters."""
    print("=" * 80)
    print("Splitting Fundamentals of Musical Composition")
    print("by Arnold Schoenberg")
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
        if section['type'] in ['intro', 'preface', 'note', 'glossary_front']:
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
            print(f"  Ch {section['num']:2d} (Part {section['part']}) -> {filename}")
        elif section['type'] in ['intro', 'preface', 'note', 'glossary_front']:
            print(f"  {title} -> {filename}")
        else:
            print(f"  {title} -> {filename}")
        print(f"      PDF page {pdf_page:3d} | Lines {start:4d}-{end:4d} ({line_count:4d} lines)")

    print()
    print("=" * 80)
    print("Summary")
    print("=" * 80)
    chapter_count = len([s for s in SECTIONS if s['type'] == 'chapter'])
    print(f"Frontmatter extracted: 00-frontmatter.md")
    print(f"Front matter sections: Introduction, Editor's Preface, Explanatory Note, Glossary")
    print(f"Chapters extracted: {chapter_count}")
    print(f"  - Part I: Chapters 1-12 (Construction of Themes)")
    print(f"  - Part II: Chapters 13-17 (Small Forms)")
    print(f"  - Part III: Chapters 18-20 (Large Forms)")
    print(f"Back matter extracted: Appendix, Index")
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
