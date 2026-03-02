use box2markdown::convert::md_to_html;

fn convert_md_fixture(name: &str) -> String {
    let input = std::fs::read_to_string(format!("tests/fixtures/{name}_tohtml.md")).unwrap();
    md_to_html::convert(&input).unwrap()
}

fn expected_tohtml(name: &str) -> String {
    std::fs::read_to_string(format!("tests/fixtures/{name}_tohtml.html")).unwrap()
}

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
    assert_eq!(convert_md_fixture("formatting"), expected_tohtml("formatting"));
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
    let md = "```rust\nfn main() {}\n```\n";
    let html = md_to_html::convert(md).unwrap();
    assert!(html.contains("<pre>") || html.contains("<code>"));
    assert!(html.contains("fn main() {}"));
}

#[test]
fn blockquote_md_to_html() {
    let md = "> quoted text\n";
    let html = md_to_html::convert(md).unwrap();
    assert!(html.contains("<blockquote>"));
    assert!(html.contains("quoted text"));
}

#[test]
fn horizontal_rule_md_to_html() {
    let md = "---\n";
    let html = md_to_html::convert(md).unwrap();
    assert!(html.contains("<hr"));
}

#[test]
fn checklist_md_to_html() {
    let md = "- [x] Done\n- [ ] Todo\n";
    let html = md_to_html::convert(md).unwrap();
    assert!(html.contains("checked"));
    assert!(html.contains("Done"));
    assert!(html.contains("Todo"));
}

#[test]
fn strikethrough_md_to_html() {
    let md = "~~deleted~~\n";
    let html = md_to_html::convert(md).unwrap();
    assert!(html.contains("<del>") || html.contains("strikethrough"));
}
