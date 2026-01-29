#!/usr/bin/env python3
"""
Split the 21st Century Classroom book into individual chapter files.
"""

import re
from pathlib import Path

# Paths
SOURCE_FILE = Path("sources-md/21st-century-classroom/book.md")
OUTPUT_DIR = Path("sources-md/21st-century-classroom/chapters")

def find_chapter_boundaries(lines):
    """Find all chapter start positions marked by 'Chapter X' headings."""
    chapters = []

    for i, line in enumerate(lines):
        # Match patterns like:
        # # <span...>Chapter 1
        # # **Chapter 9**
        match = re.search(r'Chapter (\d+)', line)
        if match and line.startswith('#'):
            chapter_num = int(match.group(1))

            # Look ahead for the chapter title (usually next # heading)
            title = f"Chapter {chapter_num}"
            for j in range(i + 1, min(len(lines), i + 5)):
                next_line = lines[j].strip()
                # Skip empty lines
                if not next_line:
                    continue
                # Check if it's a title heading
                if next_line.startswith('#'):
                    # Extract title, removing markdown and spans
                    clean_title = re.sub(r'#\s*', '', next_line)
                    clean_title = re.sub(r'\*\*', '', clean_title)
                    clean_title = re.sub(r'<[^>]+>', '', clean_title)
                    clean_title = clean_title.strip()
                    if clean_title and clean_title != f"Chapter {chapter_num}":
                        title = clean_title
                    break

            chapters.append({
                'number': chapter_num,
                'start_line': i,
                'title': title
            })

    return chapters

def slugify(text):
    """Convert text to filename-safe slug."""
    text = text.lower()
    text = re.sub(r'[^\w\s-]', '', text)
    text = re.sub(r'[-\s]+', '-', text)
    return text.strip('-')

def split_into_chapters():
    """Main function to split the book into chapters."""
    print(f"Reading {SOURCE_FILE}...")
    with open(SOURCE_FILE, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    total_lines = len(lines)
    print(f"Total lines: {total_lines}")

    # Find chapter boundaries
    chapters = find_chapter_boundaries(lines)
    print(f"\nFound {len(chapters)} chapters:")

    for chapter in chapters:
        print(f"  Ch {chapter['number']:2d}. Line {chapter['start_line']:5d} - {chapter['title']}")

    # Create output directory
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    # Extract frontmatter (everything before first chapter)
    if chapters:
        frontmatter_end = chapters[0]['start_line']
        frontmatter_content = ''.join(lines[:frontmatter_end])

        frontmatter_file = OUTPUT_DIR / "00-frontmatter.md"
        with open(frontmatter_file, 'w', encoding='utf-8') as f:
            f.write(frontmatter_content)
        print(f"\nExtracted frontmatter to {frontmatter_file}")

    # Extract each chapter
    for i, chapter in enumerate(chapters):
        start = chapter['start_line']

        # End is either the next chapter or end of file
        if i + 1 < len(chapters):
            end = chapters[i + 1]['start_line']
        else:
            end = total_lines

        # Extract chapter content
        chapter_content = ''.join(lines[start:end])

        # Create filename
        title_slug = slugify(chapter['title'])
        if len(title_slug) > 50:
            title_slug = title_slug[:50]

        filename = f"{chapter['number']:02d}-{title_slug}.md"
        filepath = OUTPUT_DIR / filename

        # Write chapter file
        with open(filepath, 'w', encoding='utf-8') as f:
            # Add metadata header
            f.write(f"# Chapter {chapter['number']}: {chapter['title']}\n\n")
            f.write(f"<!-- Original line: {start} -->\n\n")
            f.write(chapter_content)

        print(f"  Chapter {chapter['number']:2d} -> {filename} ({end - start} lines)")

    print(f"\n✓ Split complete! {len(chapters)} chapters + frontmatter extracted to {OUTPUT_DIR}/")
    print(f"\nNote: Image references already point to ./images/")
    print(f"      Images remain in: sources-md/21st-century-classroom/images/")

if __name__ == "__main__":
    split_into_chapters()
