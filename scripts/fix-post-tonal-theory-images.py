#!/usr/bin/env python3
"""
Update image references in post-tonal-theory book.md file
to point to ./images/ subdirectory.
"""

import re
from pathlib import Path

# File path
BOOK_FILE = Path("sources-md/post-tonal-theory/book.md")

def fix_image_paths():
    """Fix image paths in book.md."""
    with open(BOOK_FILE, 'r', encoding='utf-8') as f:
        content = f.read()

    # Count original references
    original_count = content.count('![](_page_')

    # Replace ![](_page_X with ![](./images/_page_X
    updated_content = re.sub(
        r'!\[\]\(_page_',
        r'![](./images/_page_',
        content
    )

    # Write back
    with open(BOOK_FILE, 'w', encoding='utf-8') as f:
        f.write(updated_content)

    # Count new references
    new_count = updated_content.count('![](./images/_page_')

    return original_count, new_count

def main():
    """Process book.md file."""
    print(f"Processing {BOOK_FILE}")
    print()

    original, updated = fix_image_paths()

    print(f"✓ Updated {updated} image references")
    print(f"  (changed from {original} references)")

if __name__ == "__main__":
    main()
