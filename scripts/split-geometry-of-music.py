#!/usr/bin/env python3
"""
Split A Geometry of Music: Harmony and Counterpoint in the Extended Common Practice
by Dmitri Tymoczko into individual chapter files.
Uses chapter mapping derived from book.json and book.md analysis.
"""

import re
from pathlib import Path

# Paths
SOURCE_FILE = Path("sources-md/geometry-of-music/book.md")
OUTPUT_DIR = Path("sources-md/geometry-of-music")

# Section definitions (derived from analysis of book.json and book.md)
# The book has:
# - Front matter: Acknowledgments, Contents, About Companion Website, Introduction
# - Part I: Theory (Chapters 1-5)
# - Part II: History and Analysis (Chapters 6-10)
# - Conclusion
# - Appendices A-F
# - References, Index

SECTIONS = [
    {
        'type': 'acknowledgments',
        'title': 'Acknowledgments',
        'start_line': 75,
        'pdf_page': 7,
    },
    {
        'type': 'contents',
        'title': 'Contents',
        'start_line': 91,
        'pdf_page': 9,
    },
    {
        'type': 'about',
        'title': 'About the Companion Website',
        'start_line': 270,
        'pdf_page': 15,
    },
    {
        'type': 'intro',
        'title': 'Introduction',
        'start_line': 278,
        'pdf_page': 17,
    },
    # Part I: Theory
    {
        'type': 'chapter',
        'num': 1,
        'part': 'I',
        'title': 'Five Components of Tonality',
        'start_line': 292,  # Includes Part I header
        'pdf_page': 21,
    },
    {
        'type': 'chapter',
        'num': 2,
        'part': 'I',
        'title': 'Harmony and Voice Leading',
        'start_line': 640,
        'pdf_page': 46,
    },
    {
        'type': 'chapter',
        'num': 3,
        'part': 'I',
        'title': 'A Geometry of Chords',
        'start_line': 1195,
        'pdf_page': 83,
    },
    {
        'type': 'chapter',
        'num': 4,
        'part': 'I',
        'title': 'Scales',
        'start_line': 1790,
        'pdf_page': 134,
    },
    {
        'type': 'chapter',
        'num': 5,
        'part': 'I',
        'title': 'Macroharmony and Centricity',
        'start_line': 2300,
        'pdf_page': 172,
    },
    # Part II: History and Analysis
    {
        'type': 'chapter',
        'num': 6,
        'part': 'II',
        'title': 'The Extended Common Practice',
        'start_line': 2860,  # Includes Part II header
        'pdf_page': 213,
    },
    {
        'type': 'chapter',
        'num': 7,
        'part': 'II',
        'title': 'Functional Harmony',
        'start_line': 3262,
        'pdf_page': 244,
    },
    {
        'type': 'chapter',
        'num': 8,
        'part': 'II',
        'title': 'Chromaticism',
        'start_line': 3914,
        'pdf_page': 286,
    },
    {
        'type': 'chapter',
        'num': 9,
        'part': 'II',
        'title': 'Scales in Twentieth-Century Music',
        'start_line': 4440,
        'pdf_page': 325,
    },
    {
        'type': 'chapter',
        'num': 10,
        'part': 'II',
        'title': 'Jazz',
        'start_line': 4962,
        'pdf_page': 370,
    },
    # Conclusion
    {
        'type': 'conclusion',
        'title': 'Conclusion',
        'start_line': 5456,
        'pdf_page': 409,
    },
    # Appendices
    {
        'type': 'appendix',
        'id': 'appendix-a',
        'title': 'Appendix A: Measuring Voice-Leading Size',
        'start_line': 5488,
        'pdf_page': 415,
    },
    {
        'type': 'appendix',
        'id': 'appendix-b',
        'title': 'Appendix B: Chord Geometry: A More Technical Look',
        'start_line': 5548,
        'pdf_page': 419,
    },
    {
        'type': 'appendix',
        'id': 'appendix-c',
        'title': 'Appendix C: Discrete Voice-Leading Lattices',
        'start_line': 5729,
        'pdf_page': 430,
    },
    {
        'type': 'appendix',
        'id': 'appendix-d',
        'title': 'Appendix D: The Interscalar Interval Matrix',
        'start_line': 5788,
        'pdf_page': 436,
    },
    {
        'type': 'appendix',
        'id': 'appendix-e',
        'title': "Appendix E: Scale, Macroharmony, and Lerdahl's Basic Space",
        'start_line': 5894,
        'pdf_page': 442,
    },
    {
        'type': 'appendix',
        'id': 'appendix-f',
        'title': 'Appendix F: Some Study Questions, Problems, and Activities',
        'start_line': 5940,
        'pdf_page': 446,
    },
    # Back matter
    {
        'type': 'backmatter',
        'id': 'references',
        'title': 'References',
        'start_line': 6069,
        'pdf_page': 453,
    },
    {
        'type': 'backmatter',
        'id': 'index',
        'title': 'Index',
        'start_line': 6375,
        'pdf_page': 463,
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
    print("Splitting A Geometry of Music: Harmony and Counterpoint")
    print("in the Extended Common Practice")
    print("by Dmitri Tymoczko")
    print("=" * 80)
    print()

    print(f"Reading {SOURCE_FILE}...")
    with open(SOURCE_FILE, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    total_lines = len(lines)
    print(f"Total lines: {total_lines}")
    print()

    # Extract frontmatter (everything before Acknowledgments)
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
        if section['type'] in ['acknowledgments', 'contents', 'about', 'intro']:
            filename = f"00-{title_slug}.md"
        elif section['type'] == 'chapter':
            chapter_num = section['num']
            filename = f"{chapter_num:02d}-{title_slug}.md"
        elif section['type'] == 'conclusion':
            filename = "11-conclusion.md"
        elif section['type'] == 'appendix':
            filename = f"{section['id']}.md"
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
        elif section['type'] in ['acknowledgments', 'contents', 'about', 'intro']:
            print(f"  {title} -> {filename}")
        elif section['type'] == 'conclusion':
            print(f"  {title} -> {filename}")
        elif section['type'] == 'appendix':
            print(f"  {title} -> {filename}")
        else:
            print(f"  {title} -> {filename}")
        print(f"      PDF page {pdf_page:3d} | Lines {start:4d}-{end:4d} ({line_count:4d} lines)")

    print()
    print("=" * 80)
    print("Summary")
    print("=" * 80)
    chapter_count = len([s for s in SECTIONS if s['type'] == 'chapter'])
    appendix_count = len([s for s in SECTIONS if s['type'] == 'appendix'])
    print(f"Frontmatter extracted: 00-frontmatter.md")
    print(f"Front matter sections: Acknowledgments, Contents, About Companion Website, Introduction")
    print(f"Chapters extracted: {chapter_count}")
    print(f"  - Part I: Chapters 1-5 (Theory)")
    print(f"  - Part II: Chapters 6-10 (History and Analysis)")
    print(f"Conclusion extracted")
    print(f"Appendices extracted: {appendix_count} (A-F)")
    print(f"Back matter extracted: References, Index")
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
