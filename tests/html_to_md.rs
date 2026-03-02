use box2markdown::convert::html_to_md;

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
