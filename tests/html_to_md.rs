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
    assert_eq!(convert_html_fixture("formatting"), expected_html("formatting"));
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
    assert!(
        md.contains("---") || md.contains("***") || md.contains("___") || md.contains("* * *")
    );
}

#[test]
fn ordered_list_html() {
    let html = "<ol><li>First</li><li>Second</li></ol>";
    let md = html_to_md::convert(html).unwrap();
    assert!(md.contains("1.") || md.contains("1)"));
    assert!(md.contains("First"));
    assert!(md.contains("Second"));
}
