use box2markdown::boxnote::BoxNoteDocument;
use box2markdown::convert::boxnote_to_md;

#[test]
fn empty_doc_content() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert!(md.trim().is_empty() || md == "\n");
}

#[test]
fn invalid_json_returns_error() {
    let result: Result<BoxNoteDocument, _> = serde_json::from_str("not json at all");
    assert!(result.is_err());
}

#[test]
fn malformed_json_missing_schema_version() {
    let json = r#"{"doc":{"type":"doc","content":[]}}"#;
    let result: Result<BoxNoteDocument, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn doc_type_validation_fails_for_non_doc() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"paragraph","content":[]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    assert!(doc.doc.validate().is_err());
}

#[test]
fn unknown_block_node_preserved_in_unknown_variant() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[{"type":"custom_widget","data":"something"}]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    match &doc.doc.content[0] {
        box2markdown::boxnote::BlockNode::Unknown { node_type, .. } => {
            assert_eq!(node_type, "custom_widget");
        }
        _ => panic!("expected Unknown variant"),
    }
}

#[test]
fn empty_paragraph_content() {
    let json =
        r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[{"type":"paragraph"}]}}"#;
    let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    assert!(md.len() <= 2);
}

#[test]
fn empty_markdown_to_html() {
    let html = box2markdown::convert::md_to_html::convert("").unwrap();
    assert!(html.trim().is_empty());
}

#[test]
fn empty_html_to_md() {
    let md = box2markdown::convert::html_to_md::convert("").unwrap();
    assert!(md.trim().is_empty());
}

#[test]
fn empty_markdown_to_boxnote() {
    let json = box2markdown::convert::md_to_boxnote::convert("").unwrap();
    let doc: BoxNoteDocument = serde_json::from_str(&json).unwrap();
    assert!(doc.doc.validate().is_ok());
    assert!(doc.doc.content.is_empty());
}
