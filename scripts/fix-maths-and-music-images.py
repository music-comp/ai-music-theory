#!/usr/bin/env python3
"""
Update image references in Mathematics and Music book.md
to point to ./images/ subdirectory.
"""

import re
from pathlib import Path

SOURCE_FILE = Path("sources-md/maths-and-music/book.md")


def fix_image_paths():
    """Fix image paths in book.md."""
    with open(SOURCE_FILE, 'r', encoding='utf-8') as f:
        content = f.read()

    original_count = content.count('![](_page_')

    updated_content = re.sub(
        r'!\[\]\(_page_',
        r'![](./images/_page_',
        content
    )

    if content != updated_content:
        with open(SOURCE_FILE, 'w', encoding='utf-8') as f:
            f.write(updated_content)

        new_count = updated_content.count('![](./images/_page_')
        print(f"Updated {new_count} image references in {SOURCE_FILE}")
    else:
        print("No image references to update.")


if __name__ == "__main__":
    fix_image_paths()
