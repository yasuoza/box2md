use anyhow::Result;
use comrak::{markdown_to_html, Options};
use regex::Regex;

pub fn convert(markdown: &str) -> Result<String> {
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.render.r#unsafe = true;
    options.render.hardbreaks = true;

    let html = markdown_to_html(markdown, &options);
    // Insert blank-line separators on raw comrak output (before checklist
    // splitting) so that only original markdown blank lines are reflected.
    let html = insert_blank_line_separators(&html);
    Ok(convert_checklist_to_boxnote(&html))
}

/// Convert GFM checklist HTML to Box Note format.
///
/// comrak: `<ul><li><input type="checkbox" checked="" disabled="" /> text</li></ul>`
/// Box Note: `<ul class="check-list"><li class="check-list-item is-checked"><p>text</p></li></ul>`
fn convert_checklist_to_boxnote(html: &str) -> String {
    // Step 1: Replace checked checkbox → Box Note class (don't add <p>)
    let re =
        Regex::new(r#"<li>\s*<input\s+type="checkbox"[^>]*\bchecked\b[^>]*/?\s*>\s*"#).unwrap();
    let html = re
        .replace_all(html, r#"<li class="check-list-item is-checked">"#)
        .to_string();
    // Step 2: Replace unchecked checkbox
    let re = Regex::new(r#"<li>\s*<input\s+type="checkbox"[^>]*/?\s*>\s*"#).unwrap();
    let html = re
        .replace_all(&html, r#"<li class="check-list-item">"#)
        .to_string();
    // Step 3a: Wrap bare text in <p> — simple items: <li class="...">text</li>
    let re = Regex::new(r#"(<li class="check-list-item[^"]*">)([^<\n]+)</li>"#).unwrap();
    let html = re.replace_all(&html, "$1<p>$2</p></li>").to_string();
    // Step 3b: Wrap bare text in <p> — items with nested content: <li class="...">text\n
    let re = Regex::new(r#"(<li class="check-list-item[^"]*">)([^<\n]+)\n"#).unwrap();
    let html = re.replace_all(&html, "$1<p>$2</p>\n").to_string();
    // Step 4: Split <ul> blocks containing both check-list-items and regular items
    let html = split_mixed_check_lists(&html);
    // Step 5: Mark <ul> containing check-list-items as check-list
    let re = Regex::new(r#"<ul>\s*\n?\s*(<li class="check-list-item)"#).unwrap();
    let html = re
        .replace_all(&html, r#"<ul class="check-list">$1"#)
        .to_string();
    html
}

/// Split `<ul>` blocks that contain both `<li class="check-list-item">` and
/// regular `<li>` items into separate `<ul>` elements.
fn split_mixed_check_lists(html: &str) -> String {
    if !html.contains("check-list-item") {
        return html.to_string();
    }

    let lines: Vec<&str> = html.split('\n').collect();
    let mut result: Vec<String> = Vec::new();
    // Per <ul> nesting: None = no items yet, Some(true) = had check items,
    // Some(false) = had regular items
    let mut item_type: Vec<Option<bool>> = Vec::new();

    for line in &lines {
        let trimmed = line.trim();

        if trimmed.starts_with("<ul") {
            item_type.push(None);
        }

        let is_check = trimmed.starts_with("<li class=\"check-list-item");
        let is_regular = !is_check && (trimmed.starts_with("<li>") || trimmed.starts_with("<li "));

        if is_check || is_regular {
            if let Some(state) = item_type.last_mut() {
                if let Some(was_check) = *state {
                    if was_check != is_check {
                        // Transition between check-list and regular items — split
                        let is_loose = trimmed == "<li>";
                        result.push("</ul>".to_string());
                        if is_loose {
                            result.push("<p></p>".to_string());
                        }
                        result.push("<ul>".to_string());
                    }
                }
                *state = Some(is_check);
            }
        }

        result.push(line.to_string());

        if trimmed == "</ul>" {
            item_type.pop();
        }
    }

    result.join("\n")
}

/// Insert `<p></p>` between adjacent top-level block elements so that Box Note
/// renders visible blank-line spacing.  Only operates outside of list
/// containers (`<ul>`/`<ol>`) to avoid inserting separators inside `<li>`.
fn insert_blank_line_separators(html: &str) -> String {
    const BLOCK_END: &[&str] = &[
        "</p>",
        "</ul>",
        "</ol>",
        "</blockquote>",
        "</table>",
        "</pre>",
    ];
    const BLOCK_START: &[&str] = &[
        "<p>",
        "<p ",
        "<ul>",
        "<ul ",
        "<ol>",
        "<ol ",
        "<blockquote>",
        "<table>",
        "<pre>",
    ];

    let lines: Vec<&str> = html.split('\n').collect();
    let mut result: Vec<String> = Vec::new();
    let mut list_depth: i32 = 0;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Track list nesting (count tags on the current line)
        list_depth += trimmed.matches("<ul").count() as i32;
        list_depth += trimmed.matches("<ol").count() as i32;
        list_depth -= trimmed.matches("</ul>").count() as i32;
        list_depth -= trimmed.matches("</ol>").count() as i32;

        result.push(line.to_string());

        if list_depth == 0 && i + 1 < lines.len() {
            let next = lines[i + 1].trim();

            let ends_block = BLOCK_END.iter().any(|t| trimmed.ends_with(t));
            let next_starts_block = BLOCK_START.iter().any(|t| next.starts_with(t));
            let already_separated = next == "<p></p>";
            let current_is_separator = trimmed == "<p></p>";

            if ends_block && next_starts_block && !already_separated && !current_is_separator {
                result.push("<p></p>".to_string());
            }
        }
    }

    result.join("\n")
}
