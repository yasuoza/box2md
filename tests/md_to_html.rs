use box2md::convert::md_to_html;

fn convert(md: &str) -> String {
    md_to_html::convert(md).unwrap()
}

fn convert_md_fixture(name: &str) -> String {
    let input = std::fs::read_to_string(format!("tests/fixtures/{name}_tohtml.md")).unwrap();
    convert(&input)
}

fn expected_tohtml(name: &str) -> String {
    std::fs::read_to_string(format!("tests/fixtures/{name}_tohtml.html")).unwrap()
}

// ── Fixture-based tests ─────────────────────────────────────────────

#[test]
fn simple_md_to_html() {
    assert_eq!(convert_md_fixture("simple"), expected_tohtml("simple"));
}

#[test]
fn headings_md_to_html() {
    assert_eq!(convert_md_fixture("headings"), expected_tohtml("headings"));
}

#[test]
fn formatting_md_to_html() {
    assert_eq!(
        convert_md_fixture("formatting"),
        expected_tohtml("formatting")
    );
}

#[test]
fn lists_md_to_html() {
    assert_eq!(convert_md_fixture("lists"), expected_tohtml("lists"));
}

#[test]
fn table_md_to_html() {
    assert_eq!(convert_md_fixture("table"), expected_tohtml("table"));
}

#[test]
fn code_block_md_to_html() {
    let html = convert("```rust\nfn main() {}\n```\n");
    assert!(html.contains("<pre>") || html.contains("<code>"));
    assert!(html.contains("fn main() {}"));
}

#[test]
fn blockquote_md_to_html() {
    let html = convert("> quoted text\n");
    assert!(html.contains("<blockquote>"));
    assert!(html.contains("quoted text"));
}

#[test]
fn horizontal_rule_md_to_html() {
    let html = convert("---\n");
    assert!(html.contains("<hr"));
}

#[test]
fn strikethrough_md_to_html() {
    let html = convert("~~deleted~~\n");
    assert!(html.contains("<del>") || html.contains("strikethrough"));
}

// ── Hardbreaks ──────────────────────────────────────────────────────

#[test]
fn hardbreaks_single_newline() {
    let html = convert("Line 1\nLine 2\n");
    assert!(html.contains("<br"), "expected <br>, got: {html}");
    assert!(html.contains("Line 1"));
    assert!(html.contains("Line 2"));
}

// ── Checklist: Box Note format ──────────────────────────────────────

#[test]
fn checklist_boxnote_format_checked() {
    let html = convert("- [x] Done\n");
    assert!(
        html.contains(r#"<li class="check-list-item is-checked">"#),
        "expected is-checked class, got: {html}"
    );
    assert!(
        html.contains(r#"<ul class="check-list">"#),
        "expected check-list ul, got: {html}"
    );
    assert!(
        html.contains("<p>Done</p>"),
        "expected <p>Done</p>, got: {html}"
    );
}

#[test]
fn checklist_boxnote_format_unchecked() {
    let html = convert("- [ ] Todo\n");
    assert!(
        html.contains(r#"<li class="check-list-item">"#),
        "expected check-list-item class, got: {html}"
    );
    assert!(
        !html.contains("is-checked"),
        "unchecked should not have is-checked, got: {html}"
    );
    assert!(
        html.contains("<p>Todo</p>"),
        "expected <p>Todo</p>, got: {html}"
    );
}

#[test]
fn checklist_mixed_checked_unchecked() {
    let html = convert("- [x] A\n- [ ] B\n- [x] C\n");
    assert!(html.contains(r#"<li class="check-list-item is-checked"><p>A</p>"#), "got: {html}");
    assert!(html.contains(r#"<li class="check-list-item"><p>B</p>"#), "got: {html}");
    assert!(html.contains(r#"<li class="check-list-item is-checked"><p>C</p>"#), "got: {html}");
}

#[test]
fn checklist_nested() {
    let html = convert("- [ ] Parent\n  - [x] Child\n");
    // Both <ul> should be check-list
    let count = html.matches(r#"class="check-list""#).count();
    assert!(count >= 2, "expected ≥2 check-list <ul>, got {count}: {html}");
    assert!(html.contains("<p>Parent</p>"), "got: {html}");
    assert!(html.contains("<p>Child</p>"), "got: {html}");
}

#[test]
fn checklist_only_items_no_split() {
    let html = convert("- [x] A\n- [ ] B\n");
    // All items in one <ul>, no stray </ul><ul> split
    let ul_count = html.matches("<ul").count();
    assert_eq!(ul_count, 1, "expected single <ul>, got {ul_count}: {html}");
}

// ── Mixed checklist + regular: splitting ────────────────────────────

#[test]
fn mixed_checklist_then_regular_tight() {
    // No blank line → no <p></p> separator
    let html = convert("- [ ] Hello\n  - [x] World\n- Hi\n- Fi\n");
    assert!(
        html.contains(r#"<ul class="check-list">"#),
        "expected check-list ul, got: {html}"
    );
    // Regular items in separate <ul> (no class)
    assert!(
        html.contains("</ul>\n<ul>\n"),
        "expected split into two <ul>, got: {html}"
    );
    assert!(
        !html.contains("</ul>\n<p></p>\n<ul>"),
        "tight should NOT have <p></p> separator, got: {html}"
    );
    assert!(html.contains("<li>Hi</li>"), "got: {html}");
    assert!(html.contains("<li>Fi</li>"), "got: {html}");
}

#[test]
fn mixed_checklist_then_regular_loose() {
    // Blank line → <p></p> separator
    let html = convert("- [ ] Hello\n  - [x] World\n\n- Hi\n- Fi\n");
    assert!(
        html.contains(r#"<ul class="check-list">"#),
        "expected check-list ul, got: {html}"
    );
    assert!(
        html.contains("<p></p>"),
        "loose should have <p></p> separator, got: {html}"
    );
    assert!(html.contains("Hi"), "got: {html}");
    assert!(html.contains("Fi"), "got: {html}");
}

#[test]
fn regular_list_not_split() {
    // Regular nested list must NOT be split
    let html = convert("- Item 1\n  - Nested\n- Item 2\n");
    let ul_open_count = html.matches("<ul").count();
    assert_eq!(
        ul_open_count, 2,
        "expected 2 <ul> (parent + nested), got {ul_open_count}: {html}"
    );
    assert!(
        !html.contains("check-list"),
        "no check-list class for regular lists, got: {html}"
    );
}

#[test]
fn regular_only_items_no_split() {
    let html = convert("- A\n- B\n- C\n");
    let ul_count = html.matches("<ul").count();
    assert_eq!(ul_count, 1, "expected single <ul>, got {ul_count}: {html}");
    assert!(!html.contains("<p></p>"), "no separator for regular list, got: {html}");
}

#[test]
fn mixed_regular_then_checklist() {
    // Regular items first, then checklist — also needs split
    let html = convert("- Normal\n- [ ] Todo\n");
    // Should produce two <ul>: one regular, one check-list
    assert!(
        html.contains(r#"<ul class="check-list">"#),
        "expected check-list ul, got: {html}"
    );
    assert!(
        html.contains("<li>Normal</li>"),
        "expected regular item, got: {html}"
    );
    assert!(
        html.contains(r#"<li class="check-list-item"><p>Todo</p>"#),
        "expected check-list-item, got: {html}"
    );
}

#[test]
fn mixed_checklist_regular_checklist() {
    // check → regular → check: three <ul> sections
    let html = convert("- [x] A\n- Mid\n- [ ] B\n");
    assert!(
        html.contains("<li>Mid</li>"),
        "expected regular item Mid, got: {html}"
    );
    let check_list_count = html.matches(r#"class="check-list""#).count();
    assert!(
        check_list_count >= 2,
        "expected ≥2 check-list <ul>, got {check_list_count}: {html}"
    );
}

// ── Blank-line separators ───────────────────────────────────────────

#[test]
fn blank_line_between_paragraphs() {
    let html = convert("Hello\n\nWorld\n");
    assert!(
        html.contains("</p>\n<p></p>\n<p>"),
        "expected <p></p> between paragraphs, got: {html}"
    );
}

#[test]
fn blank_line_paragraph_to_list() {
    let html = convert("Hello\n\n- A\n");
    assert!(
        html.contains("</p>\n<p></p>\n<ul>"),
        "expected <p></p> between paragraph and list, got: {html}"
    );
}

#[test]
fn blank_line_list_to_paragraph() {
    let html = convert("- A\n\nHello\n");
    assert!(
        html.contains("</ul>\n<p></p>\n<p>"),
        "expected <p></p> between list and paragraph, got: {html}"
    );
}

#[test]
fn blank_line_paragraph_to_blockquote() {
    let html = convert("Hello\n\n> quoted\n");
    assert!(
        html.contains("</p>\n<p></p>\n<blockquote>"),
        "expected <p></p> before blockquote, got: {html}"
    );
}

#[test]
fn blank_line_blockquote_to_paragraph() {
    let html = convert("> quoted\n\nHello\n");
    assert!(
        html.contains("</blockquote>\n<p></p>\n<p>"),
        "expected <p></p> after blockquote, got: {html}"
    );
}

#[test]
fn blank_line_paragraph_to_code_block() {
    let html = convert("Hello\n\n```\ncode\n```\n");
    assert!(
        html.contains("</p>\n<p></p>\n<pre>"),
        "expected <p></p> before code block, got: {html}"
    );
}

#[test]
fn blank_line_paragraph_to_table() {
    let html = convert("Hello\n\n| A |\n|---|\n| 1 |\n");
    assert!(
        html.contains("</p>\n<p></p>\n<table>"),
        "expected <p></p> before table, got: {html}"
    );
}

#[test]
fn no_blank_line_after_heading() {
    // Heading → paragraph should NOT get <p></p>
    let html = convert("# Title\n\nContent\n");
    assert!(
        !html.contains("</h1>\n<p></p>"),
        "should not insert <p></p> after heading, got: {html}"
    );
}

#[test]
fn no_blank_line_inside_list() {
    // <p></p> must NOT appear inside a list between <p> and nested <ul>
    let html = convert("- [ ] Hello\n  - [x] World\n");
    // The only <p></p>-free region is inside the list
    let lines: Vec<&str> = html.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if *line == "<p></p>" {
            // Verify it's not between a check-list-item <p> and a nested <ul>
            let prev = if i > 0 { lines[i - 1] } else { "" };
            assert!(
                !prev.contains("</p>") || !lines.get(i + 1).is_some_and(|l| l.contains("<ul")),
                "<p></p> appeared inside list between <p> and <ul>: {html}"
            );
        }
    }
}

#[test]
fn no_double_separator() {
    // When split_mixed_check_lists already inserts <p></p>,
    // insert_blank_line_separators should not add another
    let html = convert("- [ ] Hello\n  - [x] World\n\n- Hi\n- Fi\n");
    assert!(
        !html.contains("<p></p>\n<p></p>"),
        "should not have double <p></p>, got: {html}"
    );
}

#[test]
fn single_block_no_separator() {
    // Single paragraph — no <p></p>
    let html = convert("Hello\n");
    assert!(!html.contains("<p></p>"), "got: {html}");
}

#[test]
fn single_list_no_separator() {
    // Single list — no <p></p>
    let html = convert("- A\n- B\n");
    assert!(!html.contains("<p></p>"), "got: {html}");
}

// ── Combined / end-to-end ───────────────────────────────────────────

#[test]
fn full_document_mixed() {
    let md = "\
# Title

Hello

- [ ] Todo
  - [x] Done

- Regular

Bye
";
    let html = convert(md);

    // Heading present, no separator after it
    assert!(html.contains("<h1>Title</h1>"), "got: {html}");
    assert!(!html.contains("</h1>\n<p></p>"), "got: {html}");

    // Paragraph "Hello"
    assert!(html.contains("Hello"), "got: {html}");

    // Checklist in check-list <ul>
    assert!(html.contains(r#"class="check-list""#), "got: {html}");
    assert!(html.contains(r#"check-list-item"><p>Todo</p>"#), "got: {html}");
    assert!(html.contains(r#"check-list-item is-checked"><p>Done</p>"#), "got: {html}");

    // Regular item separated from checklist
    assert!(html.contains("Regular"), "got: {html}");

    // Paragraph "Bye"
    assert!(html.contains("Bye"), "got: {html}");

    // Separators between top-level blocks
    let sep_count = html.matches("<p></p>").count();
    assert!(sep_count >= 2, "expected ≥2 separators, got {sep_count}: {html}");
}
