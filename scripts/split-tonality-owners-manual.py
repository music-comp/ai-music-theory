#!/usr/bin/env python3
"""
Split Tonality: An Owner's Manual by Dmitri Tymoczko into individual chapter files.
Uses chapter mapping derived from analysis of the pandoc EPUB-to-markdown conversion.

This book has numbered chapters (1-10), each preceded by a "Prelude" section,
plus a conclusion, four appendices, and backmatter.
"""

import re
from pathlib import Path

# Paths
SOURCE_FILE = Path("sources-md/tonality-owners-manual/book.md")
OUTPUT_DIR = Path("sources-md/tonality-owners-manual")

# Section definitions derived from analysis of heading anchors in book.md
# Page numbers extracted from EPUB anchors like []{#08_chapter1.xhtml_page_1}
SECTIONS = [
    {
        'type': 'preface',
        'num': 0,
        'title': 'Preface and Acknowledgments',
        'start_line': 526,
        'pdf_page': 11,  # page xi
    },
    {
        'type': 'chapter',
        'num': 1,
        'title': 'Implicit Musical Knowledge',
        'start_line': 641,
        'pdf_page': 1,
    },
    {
        'type': 'prelude',
        'num': 2,
        'title': 'Prelude: Transposition Along a Collection',
        'start_line': 2590,
        'pdf_page': 37,
    },
    {
        'type': 'chapter',
        'num': 2,
        'title': 'Rock Logic',
        'start_line': 3065,
        'pdf_page': 47,
    },
    {
        'type': 'prelude',
        'num': 3,
        'title': 'Prelude: The Tinctoris Transform',
        'start_line': 4956,
        'pdf_page': 87,
    },
    {
        'type': 'chapter',
        'num': 3,
        'title': 'Line and Configuration',
        'start_line': 5381,
        'pdf_page': 96,
    },
    {
        'type': 'prelude',
        'num': 4,
        'title': 'Prelude: Sequence and Function',
        'start_line': 7779,
        'pdf_page': 151,
    },
    {
        'type': 'chapter',
        'num': 4,
        'title': 'Repetition',
        'start_line': 7963,
        'pdf_page': 155,
    },
    {
        'type': 'prelude',
        'num': 5,
        'title': 'Prelude: Three Varieties of Analytical Reduction',
        'start_line': 10043,
        'pdf_page': 203,
    },
    {
        'type': 'chapter',
        'num': 5,
        'title': 'Nonharmonic Tones',
        'start_line': 10383,
        'pdf_page': 210,
    },
    {
        'type': 'prelude',
        'num': 6,
        'title': 'Prelude: Functional and Scale-Degree Analysis',
        'start_line': 12491,
        'pdf_page': 253,
    },
    {
        'type': 'chapter',
        'num': 6,
        'title': 'The Origins of Functional Tonality',
        'start_line': 12744,
        'pdf_page': 257,
    },
    {
        'type': 'prelude',
        'num': 7,
        'title': 'Prelude: Could the Martians Understand Our Music?',
        'start_line': 14888,
        'pdf_page': 303,
    },
    {
        'type': 'chapter',
        'num': 7,
        'title': 'Functional Progressions',
        'start_line': 15279,
        'pdf_page': 311,
    },
    {
        'type': 'prelude',
        'num': 8,
        'title': 'Prelude: Chromatic or Diatonic?',
        'start_line': 17478,
        'pdf_page': 357,
    },
    {
        'type': 'chapter',
        'num': 8,
        'title': 'Modulation',
        'start_line': 17634,
        'pdf_page': 361,
    },
    {
        'type': 'prelude',
        'num': 9,
        'title': 'Prelude: Hearing and Hearing-As',
        'start_line': 19292,
        'pdf_page': 402,
    },
    {
        'type': 'chapter',
        'num': 9,
        'title': 'Heterogeneous Hierarchy',
        'start_line': 19550,
        'pdf_page': 407,
    },
    {
        'type': 'prelude',
        'num': 10,
        'title': 'Prelude: Why Beethoven?',
        'start_line': 21701,
        'pdf_page': 451,
    },
    {
        'type': 'chapter',
        'num': 10,
        'title': 'Beethoven Theorist',
        'start_line': 21969,
        'pdf_page': 457,
    },
    {
        'type': 'conclusion',
        'num': 0,
        'title': 'Conclusion',
        'start_line': 24900,
        'pdf_page': 527,
    },
    {
        'type': 'appendix',
        'num': 1,
        'title': 'Appendix 1: Fundamentals',
        'start_line': 25162,
        'pdf_page': 533,
    },
    {
        'type': 'appendix',
        'num': 2,
        'title': 'Appendix 2: Deriving the Spiral Diagrams',
        'start_line': 25683,
        'pdf_page': 545,
    },
    {
        'type': 'appendix',
        'num': 3,
        'title': 'Appendix 3: Sequence and Transformation',
        'start_line': 26095,
        'pdf_page': 555,
    },
    {
        'type': 'appendix',
        'num': 4,
        'title': 'Appendix 4: Corpus Analysis, Statistics, and Grammar',
        'start_line': 26677,
        'pdf_page': 569,
    },
    {
        'type': 'backmatter',
        'id': 'terms-symbols-abbreviations',
        'title': 'Terms, Symbols, and Abbreviations',
        'start_line': 27005,
        'pdf_page': 575,
    },
    {
        'type': 'backmatter',
        'id': 'bibliography',
        'title': 'Bibliography',
        'start_line': 27331,
        'pdf_page': 583,
    },
    {
        'type': 'backmatter',
        'id': 'index',
        'title': 'Index',
        'start_line': 28850,
        'pdf_page': 599,
    },
]


def slugify(text):
    """Convert text to filename-safe slug."""
    text = text.lower()
    # Remove special characters
    text = re.sub(r'[^\w\s-]', '', text)
    # Replace spaces and multiple hyphens with single hyphen
    text = re.sub(r'[-\s]+', '-', text)
    if len(text) > 50:
        text = text[:50]
    return text.strip('-')


def split_into_chapters():
    """Main function to split the book into chapters."""
    print("=" * 80)
    print("Splitting Tonality: An Owner's Manual")
    print("by Dmitri Tymoczko")
    print("=" * 80)
    print()

    print(f"Reading {SOURCE_FILE}...")
    with open(SOURCE_FILE, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    total_lines = len(lines)
    print(f"Total lines: {total_lines}")
    print()

    # Extract frontmatter (everything before preface)
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

    # Track file numbering: we need a sequential file number
    # Preludes and chapters are interleaved, so use sequential numbering
    file_num = 0
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

        if section['type'] == 'preface':
            file_num += 1
            filename = f"{file_num:02d}-preface.md"
        elif section['type'] == 'prelude':
            file_num += 1
            filename = f"{file_num:02d}-{title_slug}.md"
        elif section['type'] == 'chapter':
            file_num += 1
            ch_num = section['num']
            filename = f"{file_num:02d}-ch{ch_num:02d}-{title_slug}.md"
        elif section['type'] == 'conclusion':
            file_num += 1
            filename = f"{file_num:02d}-conclusion.md"
        elif section['type'] == 'appendix':
            file_num += 1
            # Strip "appendix N " prefix from slug to avoid redundancy
            clean_title = re.sub(r'^appendix\s+\d+\s*', '', title, flags=re.IGNORECASE).strip(': ')
            clean_slug = slugify(clean_title)
            filename = f"{file_num:02d}-appendix-{section['num']}-{clean_slug}.md"
        else:  # backmatter
            filename = f"{section['id']}.md"

        filepath = OUTPUT_DIR / filename

        with open(filepath, 'w', encoding='utf-8') as f:
            f.write("---\n")
            f.write(f"title: \"{title}\"\n")
            if section['type'] == 'chapter':
                f.write(f"chapter_number: {section['num']}\n")
            elif section['type'] == 'appendix':
                f.write(f"appendix_number: {section['num']}\n")
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
        elif section['type'] == 'appendix':
            label = f"App {section['num']}"
        print(f"  {label:>10s} -> {filename}")
        print(f"             PDF page {pdf_page:3d} | Lines {start:5d}-{end:5d} ({line_count:5d} lines)")

    print()
    print("=" * 80)
    print("Summary")
    print("=" * 80)

    chapter_count = len([s for s in SECTIONS if s['type'] == 'chapter'])
    prelude_count = len([s for s in SECTIONS if s['type'] == 'prelude'])
    appendix_count = len([s for s in SECTIONS if s['type'] == 'appendix'])
    backmatter_count = len([s for s in SECTIONS if s['type'] == 'backmatter'])

    print(f"  Frontmatter: 1 file")
    print(f"  Preface: 1 file")
    print(f"  Chapters: {chapter_count} files")
    print(f"  Preludes: {prelude_count} files")
    print(f"  Conclusion: 1 file")
    print(f"  Appendices: {appendix_count} files")
    print(f"  Backmatter: {backmatter_count} files")
    print(f"  Total files: {1 + len(created_files)}")
    print()

    print("Files created:")
    print("-" * 80)
    print(f"{'Filename':<60} {'PDF Pg':<8} {'Lines':<8}")
    print("-" * 80)
    print(f"{'00-frontmatter.md':<60} {'0':<8} {frontmatter_end:<8}")
    for cf in created_files:
        print(f"{cf['filename']:<60} {cf['pdf_page']:<8} {cf['line_count']:<8}")

    print()
    print("=" * 80)
    print(f"Split complete! All files created in {OUTPUT_DIR}/")
    print("=" * 80)


if __name__ == "__main__":
    split_into_chapters()
