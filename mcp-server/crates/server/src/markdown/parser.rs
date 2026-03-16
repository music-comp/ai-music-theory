//! Markdown content extraction using pulldown-cmark.

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};

/// Extract the first heading from markdown content along with its level.
///
/// # Arguments
///
/// * `content` - The markdown content to parse
///
/// # Returns
///
/// Returns `Some((level, text))` if a heading is found, where:
/// - `level` is the heading level (H1, H2, etc.)
/// - `text` is the heading text content
///
/// Returns `None` if no heading is found.
///
/// # Examples
///
/// ```
/// use music_theory_mcp::markdown::extract_first_heading;
///
/// let markdown = "# Main Title\n\n## Subtitle\n\nContent";
/// let (level, text) = extract_first_heading(markdown).unwrap();
/// // level is H1, text is "Main Title"
/// ```
pub fn extract_first_heading(content: &str) -> Option<(HeadingLevel, String)> {
    let parser = Parser::new(content);

    let mut in_heading = false;
    let mut heading_level = None;
    let mut heading_text = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading(level, _, _)) => {
                in_heading = true;
                heading_level = Some(level);
            }
            Event::Text(text) if in_heading => {
                heading_text.push_str(&text);
            }
            Event::End(Tag::Heading(_, _, _)) if in_heading => {
                return heading_level.map(|level| (level, heading_text));
            }
            _ => {}
        }
    }

    None
}

/// Extract the first paragraph after headings, up to a maximum number of characters.
///
/// # Arguments
///
/// * `content` - The markdown content to parse
/// * `max_chars` - Maximum number of characters to extract (truncates if exceeded)
///
/// # Returns
///
/// Returns `Some(text)` containing the first paragraph text, or `None` if no paragraph is found.
/// The returned text may contain content from inline formatting elements (bold, italic, links, etc.).
///
/// # Examples
///
/// ```
/// use music_theory_mcp::markdown::extract_first_paragraph;
///
/// let markdown = "# Title\n\nThis is the first paragraph.\n\nSecond paragraph.";
/// let para = extract_first_paragraph(markdown, 200).unwrap();
/// assert_eq!(para, "This is the first paragraph.");
/// ```
pub fn extract_first_paragraph(content: &str, max_chars: usize) -> Option<String> {
    let parser = Parser::new(content);

    let mut in_paragraph = false;
    let mut paragraph_text = String::new();
    let mut skip_headings = true;

    for event in parser {
        match event {
            Event::Start(Tag::Paragraph) if !skip_headings => {
                in_paragraph = true;
            }
            Event::Text(text) | Event::Code(text) if in_paragraph => {
                // Add text respecting max_chars
                let remaining = max_chars.saturating_sub(paragraph_text.len());
                if remaining > 0 {
                    let to_add = if text.len() > remaining {
                        &text[..remaining]
                    } else {
                        &text
                    };
                    paragraph_text.push_str(to_add);
                    if !to_add.is_empty() && !to_add.ends_with(' ') {
                        paragraph_text.push(' ');
                    }
                }
            }
            Event::End(Tag::Paragraph) if in_paragraph => {
                return Some(paragraph_text.trim().to_string());
            }
            Event::End(Tag::Heading(_, _, _)) => {
                skip_headings = false; // Start looking for paragraphs after first heading
            }
            _ => {}
        }
    }

    if paragraph_text.is_empty() {
        None
    } else {
        Some(paragraph_text.trim().to_string())
    }
}

/// Extract all text content from markdown (strips all formatting).
///
/// This function parses the markdown and extracts only the text content,
/// removing all formatting markers (bold, italic, links, etc.).
///
/// # Arguments
///
/// * `content` - The markdown content to parse
///
/// # Returns
///
/// Returns a string containing all text content with formatting removed.
///
/// # Examples
///
/// ```no_run
/// # fn extract_text_content(content: &str) -> String { String::new() }
/// let markdown = "# Heading\n\n**Bold** and *italic* text with [link](url).";
/// let text = extract_text_content(markdown);
/// assert!(text.contains("Bold"));
/// assert!(text.contains("italic"));
/// assert!(!text.contains("**"));
/// assert!(!text.contains("["));
/// ```
pub fn extract_text_content(content: &str) -> String {
    let parser = Parser::new(content);
    let mut text = String::new();

    for event in parser {
        if let Event::Text(t) | Event::Code(t) = event {
            text.push_str(&t);
            text.push(' ');
        }
    }

    text.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_first_heading_h1() {
        let content = "# Main Title\n\n## Subtitle\n\nContent";
        let result = extract_first_heading(content);
        assert!(result.is_some());
        let (level, text) = result.unwrap();
        assert_eq!(level, HeadingLevel::H1);
        assert_eq!(text, "Main Title");
    }

    #[test]
    fn test_extract_first_heading_h2() {
        let content = "Some intro text\n\n## First Heading\n\nContent";
        let result = extract_first_heading(content);
        assert!(result.is_some());
        let (level, text) = result.unwrap();
        assert_eq!(level, HeadingLevel::H2);
        assert_eq!(text, "First Heading");
    }

    #[test]
    fn test_extract_first_heading_no_heading() {
        let content = "Just some text without headings.";
        let result = extract_first_heading(content);
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_first_paragraph_basic() {
        let content = "# Title\n\nThis is the first paragraph.\n\nSecond paragraph.";
        let para = extract_first_paragraph(content, 200);
        assert!(para.is_some());
        assert_eq!(para.unwrap(), "This is the first paragraph.");
    }

    #[test]
    fn test_extract_first_paragraph_with_formatting() {
        let content = "# Title\n\nThis has **bold** and *italic* text.\n\nSecond.";
        let para = extract_first_paragraph(content, 200);
        assert!(para.is_some());
        let text = para.unwrap();
        assert!(text.contains("bold"));
        assert!(text.contains("italic"));
    }

    #[test]
    fn test_extract_first_paragraph_max_chars() {
        let content =
            "# Title\n\nThis is a very long paragraph that should be truncated.\n\nSecond.";
        let para = extract_first_paragraph(content, 20);
        assert!(para.is_some());
        let text = para.unwrap();
        assert!(text.len() <= 25); // Allow some buffer for word boundaries
    }

    #[test]
    fn test_extract_first_paragraph_no_paragraph() {
        let content = "# Just a heading";
        let para = extract_first_paragraph(content, 200);
        assert!(para.is_none());
    }

    #[test]
    fn test_extract_first_paragraph_no_heading() {
        let content = "First paragraph without heading.\n\nSecond paragraph.";
        let para = extract_first_paragraph(content, 200);
        assert!(para.is_none()); // Skips headings, so waits for first heading
    }

    #[test]
    fn test_extract_text_content_basic() {
        let content = "# Heading\n\nSome content here.";
        let text = extract_text_content(content);
        assert!(text.contains("Heading"));
        assert!(text.contains("Some content here"));
    }

    #[test]
    fn test_extract_text_content_strips_formatting() {
        let content = "**Bold** and *italic* text with [link](url).";
        let text = extract_text_content(content);
        assert!(text.contains("Bold"));
        assert!(text.contains("italic"));
        assert!(text.contains("link"));
        assert!(!text.contains("**"));
        assert!(!text.contains("*"));
        assert!(!text.contains("["));
        assert!(!text.contains("]"));
        assert!(!text.contains("(url)"));
    }

    #[test]
    fn test_extract_text_content_empty() {
        let content = "";
        let text = extract_text_content(content);
        assert_eq!(text, "");
    }

    #[test]
    fn test_extract_text_content_with_code() {
        let content = "Some text with `inline code` here.";
        let text = extract_text_content(content);
        assert!(text.contains("Some text with"));
        assert!(text.contains("inline code"));
        assert!(text.contains("here"));
    }

    #[test]
    fn test_extract_first_paragraph_before_heading() {
        // Test that paragraphs before any heading are skipped
        let content = "Intro paragraph before heading.\n\n# Heading\n\nParagraph after heading.";
        let para = extract_first_paragraph(content, 200);
        assert!(para.is_some());
        assert_eq!(para.unwrap(), "Paragraph after heading.");
    }
}
