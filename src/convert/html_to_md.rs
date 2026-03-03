use anyhow::Result;
use regex::Regex;

pub fn convert(html: &str) -> Result<String> {
    let html = fix_boxnote_list_nesting(html);
    // Use placeholders before htmd conversion to avoid escaping
    let html = html
        .replace("<del>", "STKOPN")
        .replace("</del>", "STKCLS")
        .replace("<s>", "STKOPN")
        .replace("</s>", "STKCLS");
    // Checkbox: match checked first, then unchecked (handles checked="", disabled="", />)
    let re = Regex::new(r#"<input\s+type="checkbox"[^>]*\bchecked\b[^>]*/?\s*>"#).unwrap();
    let html = re.replace_all(&html, "CHKDON ").to_string();
    let re = Regex::new(r#"<input\s+type="checkbox"[^>]*/?\s*>"#).unwrap();
    let html = re.replace_all(&html, "CHKTOD ").to_string();
    let html = html
        .replace(r#"<li data-checked="true">"#, "<li>CHKDON ")
        .replace(r#"<li data-checked="false">"#, "<li>CHKTOD ");
    // Box Note clipboard: <li class="check-list-item is-checked"><p style="...">
    // Inject placeholder inside <p> tag so htmd keeps it inline with content
    let re = Regex::new(r#"<li class="check-list-item is-checked"><p[^>]*>"#).unwrap();
    let html = re.replace_all(&html, "<li><p>CHKDON ").to_string();
    let re = Regex::new(r#"<li class="check-list-item"><p[^>]*>"#).unwrap();
    let html = re.replace_all(&html, "<li><p>CHKTOD ").to_string();
    let converter = htmd::HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style"])
        .build();
    let md = converter.convert(&html)?;
    let md = md
        .replace("STKOPN", "~~")
        .replace("STKCLS", "~~")
        .replace("CHKDON ", "[x] ")
        .replace("CHKTOD ", "[ ] ");
    Ok(normalize_output(&md))
}

/// Fix Box Note's non-standard list nesting.
///
/// Box Note outputs `<li>A</li><ul><li>B</li></ul>` (sibling)
/// instead of standard `<li>A<ul><li>B</li></ul></li>` (child).
/// This restructures the HTML so htmd can recognize nested lists.
fn fix_boxnote_list_nesting(html: &str) -> String {
    // Step 1: Remove </li> directly before <ul — makes <ul> a child of the preceding <li>
    let mut html = html.replace("</li><ul", "<ul");

    // Step 2: Insert </li> between </ul> and <li> — close the <li> we left open
    html = html.replace("</ul><li", "</ul></li><li");

    // Step 3: Insert </li> between </ul></ul> — close <li> at end of nested list
    loop {
        let fixed = html.replace("</ul></ul>", "</ul></li></ul>");
        if fixed == html {
            break;
        }
        html = fixed;
    }

    html
}

fn normalize_output(md: &str) -> String {
    let mut lines: Vec<String> = md.lines().map(normalize_list_marker).collect();

    // Remove blank lines between consecutive list items
    remove_blank_lines_between_list_items(&mut lines);

    let result = lines.join("\n").trim_end().to_string();
    if result.is_empty() {
        String::new()
    } else {
        format!("{result}\n")
    }
}

/// Convert `*   text` → `- text`, handling indentation for nested lists.
/// Only matches htmd's list format (`*` + 2-or-more spaces), NOT thematic breaks like `* * *`.
fn normalize_list_marker(line: &str) -> String {
    let trimmed = line.trim_start();
    let indent = line.len() - trimmed.len();
    // htmd uses `*   ` (star + 3 spaces) for unordered list markers
    if let Some(rest) = trimmed.strip_prefix("*  ") {
        // Ensure it's not a thematic break (e.g., `* * *`)
        let content = rest.trim_start();
        if content.is_empty() || content.chars().all(|c| c == '*' || c == ' ') {
            return line.to_string();
        }
        // Re-indent: htmd uses 4-space indent per level, normalize to 2-space
        let new_indent = indent / 2;
        format!("{}- {content}", " ".repeat(new_indent))
    } else {
        line.to_string()
    }
}

/// Remove blank lines that appear between two list item lines.
fn remove_blank_lines_between_list_items(lines: &mut Vec<String>) {
    let mut i = 0;
    while i < lines.len() {
        if lines[i].trim().is_empty() && is_between_list_items(lines, i) {
            lines.remove(i);
        } else {
            i += 1;
        }
    }
}

fn is_list_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("- ")
        || trimmed.starts_with("* ")
        || trimmed
            .split_once(". ")
            .is_some_and(|(num, _)| num.chars().all(|c| c.is_ascii_digit()))
}

fn is_between_list_items(lines: &[String], blank_idx: usize) -> bool {
    // Look backward for a list line
    let has_prev = (0..blank_idx).rev().any(|j| {
        let l = lines[j].trim();
        if l.is_empty() {
            return false; // skip consecutive blanks
        }
        is_list_line(&lines[j])
    });
    // Look forward for a list line
    let has_next = ((blank_idx + 1)..lines.len()).any(|j| {
        let l = lines[j].trim();
        if l.is_empty() {
            return false;
        }
        is_list_line(&lines[j])
    });
    has_prev && has_next
}
