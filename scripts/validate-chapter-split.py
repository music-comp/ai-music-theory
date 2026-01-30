#!/usr/bin/env python3
"""
Validate chapter split against book.json table of contents.

This script compares the chapter files created by split-<source>.py
against the original book.json to ensure all chapters were extracted
correctly and PDF page mappings are accurate.

Usage:
    python3 scripts/validate-chapter-split.py <source-slug>

Example:
    python3 scripts/validate-chapter-split.py 21st-century-classroom
"""

import sys
import json
import re
from pathlib import Path
from typing import List, Dict, Optional

def parse_args():
    """Parse command line arguments."""
    if len(sys.argv) != 2:
        print(__doc__)
        sys.exit(1)
    return sys.argv[1]

def load_book_json(source_slug: str) -> Dict:
    """Load and parse book.json file."""
    json_path = Path(f"sources-md/{source_slug}/book.json")

    if not json_path.exists():
        print(f"❌ ERROR: {json_path} not found")
        sys.exit(1)

    with open(json_path, 'r', encoding='utf-8') as f:
        return json.load(f)

def extract_chapter_entries(toc: List[Dict], patterns: List[str]) -> List[Dict]:
    """
    Extract chapter entries from TOC based on common patterns.

    Args:
        toc: Table of contents array from book.json
        patterns: List of regex patterns to match chapter titles

    Returns:
        List of chapter entries with title and page_id
    """
    chapters = []

    for entry in toc:
        title = entry.get('title', '')

        # Check if title matches any chapter pattern
        for pattern in patterns:
            if re.search(pattern, title):
                chapters.append({
                    'title': title,
                    'pdf_page': entry.get('page_id'),
                    'toc_index': len(chapters)
                })
                break

    return chapters

def find_chapter_files(source_slug: str) -> List[Dict]:
    """
    Find all chapter markdown files and extract their metadata.

    Returns:
        List of chapter file info with number, title, PDF page, file path
    """
    source_dir = Path(f"sources-md/{source_slug}")
    chapter_files = []

    # Find all files matching pattern NN-*.md (excluding 00-frontmatter.md)
    for file_path in sorted(source_dir.glob("[0-9][0-9]-*.md")):
        if file_path.name == "00-frontmatter.md":
            continue

        # Extract chapter number from filename
        match = re.match(r'(\d+)-(.+)\.md', file_path.name)
        if not match:
            continue

        chapter_num = int(match.group(1))
        slug = match.group(2)

        # Read file to extract metadata
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        # Extract title from first heading
        title_match = re.search(r'^#\s+(.+)$', content, re.MULTILINE)
        title = title_match.group(1) if title_match else "Unknown"

        # Extract PDF page from metadata comment
        pdf_page = None
        metadata_match = re.search(r'PDF Page:\s*(\d+)', content)
        if metadata_match:
            pdf_page = int(metadata_match.group(1))

        chapter_files.append({
            'number': chapter_num,
            'slug': slug,
            'title': title,
            'pdf_page': pdf_page,
            'file_path': file_path
        })

    return chapter_files

def validate_chapters(book_data: Dict, chapter_files: List[Dict], source_slug: str):
    """
    Validate chapter files against book.json.

    Checks:
    - All chapters from book.json are represented as files
    - No extra chapter files that aren't in book.json
    - PDF page numbers match
    """
    print(f"\n{'='*70}")
    print(f"VALIDATION REPORT: {source_slug}")
    print(f"{'='*70}\n")

    # Extract chapter patterns (common ones)
    toc = book_data.get('table_of_contents', [])

    # Try common chapter patterns
    patterns = [
        r'^Chapter\s+\d+',
        r'^Abstract and Keywords$',
        r'^\d+\.',  # Numbered chapters
        r'^Part\s+\d+',
        r'^Section\s+\d+'
    ]

    # Try each pattern and use the one that finds the most chapters
    best_chapters = []
    best_pattern = None

    for pattern in patterns:
        chapters = extract_chapter_entries(toc, [pattern])
        if len(chapters) > len(best_chapters):
            best_chapters = chapters
            best_pattern = pattern

    toc_chapters = best_chapters

    print(f"📖 book.json Analysis:")
    print(f"   Total TOC entries: {len(toc)}")
    print(f"   Chapter pattern used: {best_pattern}")
    print(f"   Chapters found in TOC: {len(toc_chapters)}\n")

    print(f"📁 Chapter Files:")
    print(f"   Total chapter files: {len(chapter_files)}\n")

    # Compare counts
    if len(toc_chapters) == len(chapter_files):
        print(f"✅ Chapter count matches: {len(chapter_files)} chapters\n")
    else:
        print(f"⚠️  Chapter count mismatch!")
        print(f"   Expected (from book.json): {len(toc_chapters)}")
        print(f"   Found (chapter files): {len(chapter_files)}\n")

    # Detailed comparison
    print(f"📋 Detailed Comparison:\n")
    print(f"{'Ch':<4} {'File':<35} {'PDF Page':<10} {'TOC Match':<10}")
    print(f"{'-'*4} {'-'*35} {'-'*10} {'-'*10}")

    all_valid = True

    for i, chapter_file in enumerate(chapter_files):
        ch_num = chapter_file['number']
        slug = chapter_file['slug'][:30]  # Truncate for display
        pdf_page = chapter_file['pdf_page']

        # Try to find corresponding TOC entry
        toc_match = "❌"
        if i < len(toc_chapters):
            toc_chapter = toc_chapters[i]
            toc_pdf = toc_chapter['pdf_page']

            if pdf_page == toc_pdf:
                toc_match = "✅"
            elif pdf_page is None:
                toc_match = "⚠️  No PDF"
                all_valid = False
            else:
                toc_match = f"⚠️  ({toc_pdf})"
                all_valid = False
        else:
            all_valid = False

        pdf_str = str(pdf_page) if pdf_page is not None else "None"
        print(f"{ch_num:<4} {slug:<35} {pdf_str:<10} {toc_match:<10}")

    # Check for missing chapters
    if len(toc_chapters) > len(chapter_files):
        print(f"\n⚠️  Missing chapters in files:")
        for i in range(len(chapter_files), len(toc_chapters)):
            toc_chapter = toc_chapters[i]
            print(f"   Ch {i+1}: {toc_chapter['title'][:50]} (PDF p.{toc_chapter['pdf_page']})")

    print(f"\n{'='*70}")
    if all_valid and len(toc_chapters) == len(chapter_files):
        print("✅ VALIDATION PASSED: All chapters extracted correctly with matching PDF pages")
    else:
        print("⚠️  VALIDATION WARNINGS: See details above")
    print(f"{'='*70}\n")

    # Summary statistics
    with_pdf = sum(1 for c in chapter_files if c['pdf_page'] is not None)
    without_pdf = len(chapter_files) - with_pdf

    print(f"📊 Summary:")
    print(f"   Chapters with PDF pages: {with_pdf}/{len(chapter_files)}")
    print(f"   Chapters without PDF pages: {without_pdf}/{len(chapter_files)}")

    if without_pdf > 0:
        print(f"\n💡 Tip: Re-run the split script using book.json to add PDF page metadata")

def main():
    """Main validation function."""
    source_slug = parse_args()

    print(f"\n🔍 Validating chapter split for: {source_slug}\n")

    # Load book.json
    try:
        book_data = load_book_json(source_slug)
    except Exception as e:
        print(f"❌ ERROR loading book.json: {e}")
        sys.exit(1)

    # Find chapter files
    try:
        chapter_files = find_chapter_files(source_slug)
        if not chapter_files:
            print(f"❌ ERROR: No chapter files found in sources-md/{source_slug}/")
            print(f"   Expected files matching pattern: NN-*.md (e.g., 01-introduction.md)")
            sys.exit(1)
    except Exception as e:
        print(f"❌ ERROR finding chapter files: {e}")
        sys.exit(1)

    # Run validation
    try:
        validate_chapters(book_data, chapter_files, source_slug)
    except Exception as e:
        print(f"❌ ERROR during validation: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)

if __name__ == "__main__":
    main()
