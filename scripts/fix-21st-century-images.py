#!/usr/bin/env python3
"""
Update image references in 21st Century Classroom markdown file
to point to ./images/ subdirectory.
"""

import re
from pathlib import Path

# File to update
MD_FILE = Path("sources-md/21st-century-classroom/book.md")

def fix_image_paths():
    """Fix image paths in the markdown file."""
    print(f"Reading {MD_FILE}...")
    with open(MD_FILE, 'r', encoding='utf-8') as f:
        content = f.read()

    # Count original references
    original_count = content.count('![](_page_')
    print(f"Found {original_count} image references to update")

    # Replace ![](_page_X with ![](./images/_page_X
    updated_content = re.sub(
        r'!\[\]\(_page_',
        r'![](./images/_page_',
        content
    )

    # Write back
    with open(MD_FILE, 'w', encoding='utf-8') as f:
        f.write(updated_content)

    # Verify
    new_count = updated_content.count('![](./images/_page_')
    print(f"✓ Updated {new_count} image references")
    print(f"✓ All images now point to ./images/ subdirectory")

if __name__ == "__main__":
    fix_image_paths()
