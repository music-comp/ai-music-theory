#!/usr/bin/env python3
"""
Split Post-Tonal Theory book into individual chapter files.
Uses chapter mapping derived from book.json and book.md analysis.
"""

import re
from pathlib import Path

# Paths
SOURCE_FILE = Path("sources-md/post-tonal-theory/book.md")
OUTPUT_DIR = Path("sources-md/post-tonal-theory")

# Chapter definitions (derived from analysis of book.json and book.md)
CHAPTERS = [
    {
        'num': 1,
        'title': 'Basic Concepts of Pitch and Interval',
        'start_line': 390,
        'pdf_page': 17
    },
    {
        'num': 2,
        'title': 'Pitch-Class Sets',
        'start_line': 1142,
        'pdf_page': 59
    },
    {
        'num': 3,
        'title': 'Some Additional Properties and Relationships',
        'start_line': 1978,
        'pdf_page': 111
    },
    {
        'num': 4,
        'title': 'Motive, Voice Leading, and Harmony',
        'start_line': 2886,
        'pdf_page': 175
    },
    {
        'num': 5,
        'title': 'Centricity and Referential Pitch Collections',
        'start_line': 3649,
        'pdf_page': 244
    },
    {
        'num': 6,
        'title': 'Basic Concepts of Twelve-Tone Music',
        'start_line': 4373,
        'pdf_page': 310
    }
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
    print("Splitting Post-Tonal Theory into chapters")
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
        # Add metadata header
        f.write("---\n")
        f.write("title: Frontmatter\n")
        f.write("pdf_page: 0\n")
        f.write("book_md_line: 1\n")
        f.write("---\n\n")
        f.write(frontmatter_content)

    print(f"✓ Extracted frontmatter to {frontmatter_file.name}")
    print(f"  Lines: 1-{frontmatter_end} ({frontmatter_end} lines)")
    print()

    # Extract each chapter
    print("Extracting chapters:")
    print("-" * 80)

    created_files = []

    for i, chapter in enumerate(CHAPTERS):
        chapter_num = chapter['num']
        start = chapter['start_line']
        pdf_page = chapter['pdf_page']
        title = chapter['title']

        # End is either the next chapter or end of file
        if i + 1 < len(CHAPTERS):
            end = CHAPTERS[i + 1]['start_line']
        else:
            end = total_lines

        # Extract chapter content
        chapter_content = ''.join(lines[start:end])

        # Create filename with slugified title
        title_slug = slugify(title)
        filename = f"{chapter_num:02d}-{title_slug}.md"
        filepath = OUTPUT_DIR / filename

        # Write chapter file with metadata header
        with open(filepath, 'w', encoding='utf-8') as f:
            # Add YAML metadata header
            f.write("---\n")
            f.write(f"title: {title}\n")
            f.write(f"chapter_number: {chapter_num}\n")
            f.write(f"pdf_page: {pdf_page}\n")
            f.write(f"book_md_line: {start}\n")
            f.write("---\n\n")

            # Add chapter heading if not already present
            # (Chapters 1 and 2 don't have proper headings)
            if chapter_num in [1, 2]:
                f.write(f"# Chapter {chapter_num}: {title}\n\n")

            f.write(chapter_content)

        line_count = end - start
        created_files.append({
            'filename': filename,
            'chapter_num': chapter_num,
            'title': title,
            'pdf_page': pdf_page,
            'line_count': line_count
        })

        print(f"  Ch {chapter_num} → {filename}")
        print(f"      PDF page {pdf_page:3d} | Lines {start:4d}-{end:4d} ({line_count:4d} lines)")

    print()
    print("=" * 80)
    print("Summary")
    print("=" * 80)
    print(f"✓ Frontmatter extracted: 00-frontmatter.md")
    print(f"✓ Chapters extracted: {len(created_files)}")
    print()

    print("Files created:")
    print("-" * 80)
    print(f"{'Filename':<50} {'PDF Pg':<8} {'Lines':<8}")
    print("-" * 80)
    print(f"{'00-frontmatter.md':<50} {'0':<8} {frontmatter_end:<8}")
    for f in created_files:
        print(f"{f['filename']:<50} {f['pdf_page']:<8} {f['line_count']:<8}")

    print()
    print("=" * 80)
    print(f"✓ Split complete! All files created in {OUTPUT_DIR}/")
    print("=" * 80)

if __name__ == "__main__":
    split_into_chapters()
