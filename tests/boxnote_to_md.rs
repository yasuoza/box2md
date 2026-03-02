use box2markdown::boxnote::BoxNoteDocument;
use box2markdown::convert::boxnote_to_md;

fn convert_fixture(name: &str) -> String {
    let input = std::fs::read_to_string(format!("tests/fixtures/{name}.boxnote")).unwrap();
    let doc: BoxNoteDocument = serde_json::from_str(&input).unwrap();
    boxnote_to_md::convert(&doc).unwrap()
}

fn expected(name: &str) -> String {
    std::fs::read_to_string(format!("tests/fixtures/{name}.md")).unwrap()
}

#[test]
fn simple_paragraph() {
    assert_eq!(convert_fixture("simple"), expected("simple"));
}

#[test]
fn headings() {
    assert_eq!(convert_fixture("headings"), expected("headings"));
}

#[test]
fn lists_nested() {
    assert_eq!(convert_fixture("lists_nested"), expected("lists_nested"));
}

#[test]
fn formatting() {
    assert_eq!(convert_fixture("formatting"), expected("formatting"));
}

#[test]
fn table() {
    assert_eq!(convert_fixture("table"), expected("table"));
}

#[test]
fn code_block() {
    assert_eq!(convert_fixture("code_block"), expected("code_block"));
}

#[test]
fn blockquote() {
    assert_eq!(convert_fixture("blockquote"), expected("blockquote"));
}

#[test]
fn checklist() {
    assert_eq!(convert_fixture("checklist"), expected("checklist"));
}

#[test]
fn horizontal_rule() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[{"type":"horizontal_rule"}]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert!(md.contains("---"));
}

#[test]
fn hard_break_in_paragraph() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[{"type":"paragraph","content":[{"type":"text","text":"line1"},{"type":"hard_break"},{"type":"text","text":"line2"}]}]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert!(md.contains("line1\nline2") || md.contains("line1  \nline2"));
}

#[test]
fn ordered_list() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[{"type":"ordered_list","attrs":{"order":1},"content":[{"type":"list_item","content":[{"type":"paragraph","content":[{"type":"text","text":"First"}]}]},{"type":"list_item","content":[{"type":"paragraph","content":[{"type":"text","text":"Second"}]}]}]}]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert!(md.contains("1. First"));
    assert!(md.contains("2. Second"));
}
