#!/usr/bin/env python3
"""
Upgrade Chapter 4 and 5 concept cards from v2 to v3 template format.
This script handles the mechanical frontmatter and heading changes.
"""

import os
import re
import yaml

CARD_DIR = os.path.join(os.path.dirname(os.path.dirname(__file__)),
                        "concept-cards", "post-tonal-theory")

# Category/subcategory mappings based on section
CATEGORY_MAP = {
    # Chapter 4
    "4.1": ("motive", "composing-out"),
    "4.2": ("harmony", "interval cycles"),
    "4.2.1": ("harmony", "interval cycles"),
    "4.2.2": ("harmony", "cyclic harmonies"),
    "4.2.3": ("harmony", "interval cycles"),
    "4.2.4": ("harmony", "interval cycles"),
    "4.3": ("voice-leading", "smooth voice leading"),
    "4.3.1": ("voice-leading", "smooth voice leading"),
    "4.3.2": ("voice-leading", "fuzzy T/I"),
    "4.4": ("voice-leading", "smooth voice leading"),
    "4.4.1": ("voice-leading", "smooth voice leading"),
    "4.4.2": ("voice-leading", "smooth voice leading"),
    "4.4.3": ("voice-leading", "smooth voice leading"),
    "4.5": ("harmony", "triadic post-tonality"),
    "4.5.1": ("harmony", "triadic post-tonality"),
    "4.5.2": ("harmony", "triadic post-tonality"),
    "4.6": ("harmony", "triadic post-tonality"),
    "4.6.1": ("harmony", "triadic post-tonality"),
    "4.6.2": ("harmony", "triadic post-tonality"),
    # Chapter 5
    "5.1": ("centricity", "axes"),
    "5.2": ("centricity", "axes"),
    "5.2.1": ("centricity", "axes"),
    "5.2.2": ("centricity", "inversional wedges"),
    "5.2.3": ("centricity", "axes"),
    "5.2.4": ("centricity", "axes"),
    "5.3": ("collections", "diatonic"),
    "5.4": ("collections", "octatonic"),
    "5.5": ("collections", "whole-tone"),
    "5.6": ("collections", "hexatonic"),
    "5.7": ("collections", None),
}

# Section mapping from Source Reference lines
SECTION_PATTERNS = {
    "4.1": "4.1 Composing-Out",
    "4.2,": "4.2 Interval Cycles",
    "4.2.1": "4.2.1 Cyclic linear motion",
    "4.2.2": "4.2.2 Cyclic harmonies",
    "4.2.3": "4.2.3 Maximal evenness",
    "4.2.4": "4.2.4 Combination cycles",
    "4.3": "4.3 Voice Leading",
    "4.3.1": "4.3.1 Transposition and inversion",
    "4.3.2": "4.3.2 Fuzzy transposition and inversion",
    "4.4": "4.4 Set-Class Space",
    "4.4.1": "4.4.1 Voice-leading space for trichords",
    "4.4.2": "4.4.2 Voice-leading space for tetrachords",
    "4.4.3": "4.4.3 Harmonic quality",
    "4.5": "4.5 Contextual Inversion",
    "4.5.1": "4.5.1 Chain and space for (014)",
    "4.5.2": "4.5.2 Chain and space for B-G-A-Bb",
    "4.6": "4.6 Triadic Post-Tonality",
    "4.6.1": "4.6.1 Triadic transformation",
    "4.6.2": "4.6.2 Other progressions of triads",
    "5.1": "5.1 Tonality and Centricity",
    "5.2": "5.2 Inversional Axis",
    "5.2.1": "5.2.1 Inversional symmetry in pitch space",
    "5.2.2": "5.2.2 Inversional wedges",
    "5.2.3": "5.2.3 Inversional symmetry in pitch-class space",
    "5.2.4": "5.2.4 Motion from axis to axis",
    "5.3": "5.3 Diatonic Collection",
    "5.4": "5.4 Octatonic Collection",
    "5.5": "5.5 Whole-Tone Collection",
    "5.6": "5.6 Hexatonic Collection",
    "5.7": "5.7 Collectional and Centric Interaction",
}


def extract_section_number(source_ref):
    """Extract section number from Source Reference text."""
    m = re.search(r'Section\s+(\d+\.\d+(?:\.\d+)?)', source_ref)
    if m:
        return m.group(1)
    # Try from "Sections X.Y"
    m = re.search(r'Sections?\s+(\d+\.\d+)', source_ref)
    if m:
        return m.group(1)
    return None


def make_slug(concept_name):
    """Convert concept name to slug."""
    # Remove quotes and parentheses content
    name = re.sub(r'\(.*?\)', '', concept_name).strip()
    name = re.sub(r'"', '', name)
    # Convert to lowercase hyphenated
    name = name.lower()
    name = re.sub(r"[^a-z0-9\s-]", "", name)
    name = re.sub(r"\s+", "-", name).strip("-")
    name = re.sub(r"-+", "-", name)
    return name


def upgrade_card(filepath):
    """Upgrade a single card from v2 to v3 format."""
    with open(filepath, 'r') as f:
        content = f.read()

    # Check if already v3 (has slug field)
    if 'slug:' in content.split('---')[1] if content.startswith('---') else False:
        return False  # Already upgraded

    # Parse frontmatter
    parts = content.split('---', 2)
    if len(parts) < 3:
        return False

    try:
        fm = yaml.safe_load(parts[1])
    except:
        return False

    if not fm or 'chapter_number' not in fm:
        return False

    ch_num = fm['chapter_number']
    if ch_num not in (4, 5):
        return False

    concept_name = fm.get('concept', '')
    body = parts[2]

    # Extract section number from Source Reference
    source_ref_match = re.search(r'# Source Reference\n(.+)', body)
    source_ref = source_ref_match.group(1) if source_ref_match else ''
    section_num = extract_section_number(source_ref)

    # Determine category and subcategory
    cat, subcat = "theory", None
    if section_num and section_num in CATEGORY_MAP:
        cat, subcat = CATEGORY_MAP[section_num]
    elif section_num:
        # Try parent section
        parent = '.'.join(section_num.split('.')[:2])
        if parent in CATEGORY_MAP:
            cat, subcat = CATEGORY_MAP[parent]

    # Determine section name
    section_name = None
    if section_num:
        for key, val in SECTION_PATTERNS.items():
            if section_num.startswith(key.rstrip(',')):
                section_name = val
                break

    slug = make_slug(concept_name)
    filename = os.path.basename(filepath).replace('.md', '')

    # Build new frontmatter
    new_fm = {
        'concept': concept_name,
        'slug': filename,  # Use filename as slug for consistency
        'category': cat,
    }
    if subcat:
        new_fm['subcategory'] = subcat
    else:
        new_fm['subcategory'] = None

    new_fm.update({
        'tier': 'advanced',
        'source': '"Introduction to Post-Tonal Theory"',
        'source_slug': 'post-tonal-theory',
        'authors': '"Joseph N. Straus"',
        'chapter': fm.get('chapter', ''),
        'chapter_number': ch_num,
        'pdf_page': fm.get('pdf_page', 175 if ch_num == 4 else 244),
        'section': section_name,
        'extraction_confidence': 'high',
        'aliases': [],
        'prerequisites': [],
        'extends': [],
        'related': [],
        'contrasts_with': [],
        'answers_questions': [f'"What is {concept_name.lower()}?"'],
    })

    # Rename headings in body
    body = body.replace('# Formal Definition', '# Core Definition')
    body = body.replace('# Mathematical Formulation/Recognition', '# Construction / Recognition')
    body = body.replace('# Musical Context/Application', '# Context & Application')

    # Add verification note if not present
    if '# Verification Notes' not in body:
        body = body.rstrip() + '\n\n# Verification Notes\nUpgraded from v2 to v3 template format.\n'

    # Build YAML manually for cleaner output
    lines = ['---']
    lines.append(f'concept: {concept_name}')
    lines.append(f'slug: {filename}')
    lines.append(f'category: {cat}')
    lines.append(f'subcategory: {subcat if subcat else "null"}')
    lines.append(f'tier: advanced')
    lines.append(f'source: "Introduction to Post-Tonal Theory"')
    lines.append(f'source_slug: post-tonal-theory')
    lines.append(f'authors: "Joseph N. Straus"')
    lines.append(f'chapter: "{fm.get("chapter", "")}"')
    lines.append(f'chapter_number: {ch_num}')
    lines.append(f'pdf_page: {fm.get("pdf_page", 175 if ch_num == 4 else 244)}')
    lines.append(f'section: {f"{section_name}" if section_name else "null"}')
    lines.append(f'extraction_confidence: high')
    lines.append(f'aliases: []')
    lines.append(f'prerequisites: []')
    lines.append(f'extends: []')
    lines.append(f'related: []')
    lines.append(f'contrasts_with: []')
    lines.append(f'answers_questions:')
    lines.append(f'  - "What is {concept_name.lower()}?"')
    lines.append('---')

    new_content = '\n'.join(lines) + body

    with open(filepath, 'w') as f:
        f.write(new_content)

    return True


def main():
    upgraded = 0
    skipped = 0
    errors = 0

    for filename in sorted(os.listdir(CARD_DIR)):
        if not filename.endswith('.md'):
            continue

        filepath = os.path.join(CARD_DIR, filename)

        # Read to check if it's a Ch 4 or 5 card
        with open(filepath, 'r') as f:
            content = f.read()

        if 'chapter_number: 4' not in content and 'chapter_number: 5' not in content:
            continue

        # Skip already upgraded cards (those with slug: in frontmatter)
        fm_section = content.split('---')[1] if content.startswith('---') and content.count('---') >= 2 else ''
        if 'slug:' in fm_section:
            skipped += 1
            continue

        try:
            result = upgrade_card(filepath)
            if result:
                upgraded += 1
                print(f"  Upgraded: {filename}")
            else:
                skipped += 1
        except Exception as e:
            errors += 1
            print(f"  ERROR: {filename}: {e}")

    print(f"\nDone: {upgraded} upgraded, {skipped} skipped, {errors} errors")


if __name__ == '__main__':
    main()
