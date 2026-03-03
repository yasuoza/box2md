use box2md::convert::html_to_md;

fn convert_html_fixture(name: &str) -> String {
    let input = std::fs::read_to_string(format!("tests/fixtures/{name}.html")).unwrap();
    html_to_md::convert(&input).unwrap()
}

fn expected_html(name: &str) -> String {
    std::fs::read_to_string(format!("tests/fixtures/{name}_html.md")).unwrap()
}

#[test]
fn simple_html() {
    assert_eq!(convert_html_fixture("simple"), expected_html("simple"));
}

#[test]
fn headings_html() {
    assert_eq!(convert_html_fixture("headings"), expected_html("headings"));
}

#[test]
fn lists_html() {
    assert_eq!(convert_html_fixture("lists"), expected_html("lists"));
}

#[test]
fn formatting_html() {
    assert_eq!(
        convert_html_fixture("formatting"),
        expected_html("formatting")
    );
}

#[test]
fn table_html() {
    assert_eq!(convert_html_fixture("table"), expected_html("table"));
}

#[test]
fn code_block_html() {
    let html = "<pre><code class=\"language-rust\">fn main() {}</code></pre>";
    let md = html_to_md::convert(html).unwrap();
    assert!(md.contains("```rust") || md.contains("```"));
    assert!(md.contains("fn main() {}"));
}

#[test]
fn blockquote_html() {
    let html = "<blockquote><p>quoted text</p></blockquote>";
    let md = html_to_md::convert(html).unwrap();
    assert!(md.contains("> quoted text"));
}

#[test]
fn horizontal_rule_html() {
    let html = "<hr>";
    let md = html_to_md::convert(html).unwrap();
    assert!(md.contains("---") || md.contains("***") || md.contains("___") || md.contains("* * *"));
}

#[test]
fn ordered_list_html() {
    let html = "<ol><li>First</li><li>Second</li></ol>";
    let md = html_to_md::convert(html).unwrap();
    assert!(md.contains("1.") || md.contains("1)"));
    assert!(md.contains("First"));
    assert!(md.contains("Second"));
}

#[test]
fn boxnote_nested_list_3_levels() {
    // Box Note uses non-standard nesting: <ul> as sibling of <li> instead of child
    let html = r#"<ul><li>A</li><ul><li>B</li><ul><li>C</li></ul><li>D</li></ul></ul>"#;
    let md = html_to_md::convert(html).unwrap();
    assert!(md.contains("- A\n"), "expected top-level item A, got: {md}");
    assert!(md.contains("  - B\n"), "expected nested item B, got: {md}");
    assert!(
        md.contains("    - C\n"),
        "expected deeply nested item C, got: {md}"
    );
    assert!(md.contains("  - D\n"), "expected nested item D, got: {md}");
}

#[test]
fn checklist_html_data_checked() {
    let html = r#"<ul data-check-list="true"><li data-checked="true">Done</li><li data-checked="false">Todo</li></ul>"#;
    let md = html_to_md::convert(html).unwrap();
    assert!(md.contains("[x] Done"), "expected [x] Done, got: {md}");
    assert!(md.contains("[ ] Todo"), "expected [ ] Todo, got: {md}");
}

#[test]
fn checklist_html_input_checkbox() {
    let html = r#"<ul><li><input type="checkbox" checked>Done</li><li><input type="checkbox">Todo</li></ul>"#;
    let md = html_to_md::convert(html).unwrap();
    assert!(md.contains("[x] Done"), "expected [x] Done, got: {md}");
    assert!(md.contains("[ ] Todo"), "expected [ ] Todo, got: {md}");
}

#[test]
fn strikethrough_s_tag() {
    let html = r#"<p><s>deleted</s></p>"#;
    let md = html_to_md::convert(html).unwrap();
    assert!(
        md.contains("~~deleted~~"),
        "expected ~~deleted~~, got: {md}"
    );
}

#[test]
fn strikethrough_del_tag() {
    let html = r#"<p><del>deleted</del></p>"#;
    let md = html_to_md::convert(html).unwrap();
    assert!(
        md.contains("~~deleted~~"),
        "expected ~~deleted~~, got: {md}"
    );
}

#[test]
fn checklist_boxnote_clipboard() {
    // Actual Box Note clipboard HTML uses class="check-list-item" with <p style="...">
    let html = r#"<ul class="check-list"><li class="check-list-item"><p style="font-size: 12pt;"><span>Todo</span></p></li><li class="check-list-item is-checked"><p style="font-size: 12pt;"><span>Done</span></p></li></ul>"#;
    let md = html_to_md::convert(html).unwrap();
    assert!(md.contains("[ ] Todo"), "expected [ ] Todo, got: {md}");
    assert!(md.contains("[x] Done"), "expected [x] Done, got: {md}");
}

#[test]
fn boxnote_clipboard_strikethrough_and_checklist() {
    // Real Box Note clipboard HTML
    let html = r#"<ul><li><p style="font-size: 12pt;"><span>normal</span></p></li><li><p style="font-size: 12pt;"><s><span>deleted</span></s></p></li></ul><ul class="check-list"><li class="check-list-item"><p style="font-size: 12pt;"><span>todo</span></p></li></ul>"#;
    let md = html_to_md::convert(html).unwrap();
    assert!(md.contains("normal"), "expected normal, got: {md}");
    assert!(
        md.contains("~~deleted~~"),
        "expected ~~deleted~~, got: {md}"
    );
    assert!(md.contains("[ ] todo"), "expected [ ] todo, got: {md}");
}

#[test]
fn checklist_comrak_html() {
    // comrak generates <input type="checkbox" checked="" disabled="" />
    let html = r#"<ul><li><input type="checkbox" disabled="" /> hello<ul><li><input type="checkbox" checked="" disabled="" /> world</li></ul></li></ul>"#;
    let md = html_to_md::convert(html).unwrap();
    assert!(md.contains("[ ] hello"), "expected [ ] hello, got: {md}");
    assert!(md.contains("[x] world"), "expected [x] world, got: {md}");
}
