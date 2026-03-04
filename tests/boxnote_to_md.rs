use box2md::boxnote::BoxNoteDocument;
use box2md::convert::boxnote_to_md;

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
fn table_with_list() {
    assert_eq!(
        convert_fixture("table_with_list"),
        expected("table_with_list")
    );
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

#[test]
fn image_placeholder() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[
        {"type":"image","attrs":{"src":"","alt":"","title":"","boxSharedLink":"","boxFileId":"","fileName":"screenshot.png","placeholderState":"","width":800,"height":600,"childId":"abc-123.png"},"marks":[{"type":"author_id","attrs":{"authorId":"123"}}]}
    ]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert_eq!(md, "![screenshot.png]()\n");
}

#[test]
fn image_without_filename_uses_child_id() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[
        {"type":"image","attrs":{"src":"","alt":"","title":"","boxSharedLink":"","boxFileId":"","fileName":"","placeholderState":"","width":null,"height":null,"childId":"abc-123.png"}}
    ]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert_eq!(md, "![abc-123.png]()\n");
}

#[test]
fn font_color_and_font_size_marks_preserved_text() {
    let json = r##"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[
        {"type":"paragraph","content":[
            {"type":"text","text":"colored","marks":[{"type":"font_color","attrs":{"color":"#ff0000"}}]},
            {"type":"text","text":" sized","marks":[{"type":"font_size","attrs":{"size":"18px"}}]}
        ]}
    ]}}"##;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert_eq!(md, "colored sized\n");
}

#[test]
fn bullet_list_nested_without_list_item_wrapper() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[
        {"type":"bullet_list","content":[
            {"type":"list_item","content":[
                {"type":"paragraph","content":[{"type":"text","text":"parent"}]},
                {"type":"bullet_list","content":[
                    {"type":"list_item","content":[
                        {"type":"paragraph","content":[{"type":"text","text":"child"}]}
                    ]}
                ]}
            ]}
        ]}
    ]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert!(md.contains("- parent"));
    assert!(md.contains("  - child"));
}

#[test]
fn bullet_list_sibling_nesting() {
    // Box Note format: nested lists are siblings of list_item, not children
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[
        {"type":"bullet_list","content":[
            {"type":"list_item","content":[
                {"type":"paragraph","content":[{"type":"text","text":"parent"}]}
            ]},
            {"type":"bullet_list","content":[
                {"type":"list_item","content":[
                    {"type":"paragraph","content":[{"type":"text","text":"child"}]}
                ]}
            ]},
            {"type":"list_item","content":[
                {"type":"paragraph","content":[{"type":"text","text":"sibling"}]}
            ]}
        ]}
    ]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert_eq!(md, "- parent\n  - child\n- sibling\n");
}

#[test]
fn ordered_list_sibling_nesting() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[
        {"type":"ordered_list","attrs":{"order":1},"content":[
            {"type":"list_item","content":[
                {"type":"paragraph","content":[{"type":"text","text":"First"}]}
            ]},
            {"type":"ordered_list","attrs":{"order":1},"content":[
                {"type":"list_item","content":[
                    {"type":"paragraph","content":[{"type":"text","text":"Nested"}]}
                ]}
            ]},
            {"type":"list_item","content":[
                {"type":"paragraph","content":[{"type":"text","text":"Second"}]}
            ]}
        ]}
    ]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert!(md.contains("1. First"));
    assert!(md.contains("   1. Nested"));
    assert!(md.contains("2. Second"));
}

#[test]
fn check_list_with_nested_bullet_list_sibling() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[
        {"type":"check_list","content":[
            {"type":"check_list_item","attrs":{"checked":false},"content":[
                {"type":"paragraph","content":[{"type":"text","text":"task"}]}
            ]},
            {"type":"bullet_list","content":[
                {"type":"list_item","content":[
                    {"type":"paragraph","content":[{"type":"text","text":"detail"}]}
                ]}
            ]}
        ]}
    ]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert!(md.contains("- [ ] task"));
    assert!(md.contains("  - detail"));
}

#[test]
fn tab_list_renders_as_indented_list() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[
        {"type":"tab_list","content":[
            {"type":"list_item","content":[
                {"type":"paragraph","content":[{"type":"text","text":"indented"}]}
            ]}
        ]}
    ]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert!(md.contains("- indented"));
}

#[test]
fn box_preview_renders_as_link() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[
        {"type":"box_preview","attrs":{"boxSharedLink":"https://example.box.com/s/abc123"}}
    ]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert_eq!(md, "[Box Preview](https://example.box.com/s/abc123)\n");
}

#[test]
fn annotation_id_mark_preserved_text() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[
        {"type":"paragraph","content":[
            {"type":"text","text":"annotated","marks":[{"type":"annotation_id","attrs":{"id":"123"}}]}
        ]}
    ]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert_eq!(md, "annotated\n");
}
