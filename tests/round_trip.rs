use box2markdown::boxnote::*;
use box2markdown::convert::{boxnote_to_md, md_to_boxnote};

fn round_trip(boxnote_json: &str) -> BoxNoteDocument {
    let doc: BoxNoteDocument = serde_json::from_str(boxnote_json).unwrap();
    let md = boxnote_to_md::convert(&doc).unwrap();
    let json_out = md_to_boxnote::convert(&md).unwrap();
    serde_json::from_str(&json_out).unwrap()
}

#[test]
fn round_trip_paragraph() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[{"type":"paragraph","content":[{"type":"text","text":"Hello"}]}]}}"#;
    let result = round_trip(json);
    match &result.doc.content[0] {
        BlockNode::Paragraph {
            content: Some(inlines),
        } => match &inlines[0] {
            InlineNode::Text { text, .. } => assert_eq!(text, "Hello"),
            _ => panic!("expected text"),
        },
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn round_trip_heading() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[{"type":"heading","attrs":{"level":2},"content":[{"type":"text","text":"Title"}]}]}}"#;
    let result = round_trip(json);
    match &result.doc.content[0] {
        BlockNode::Heading {
            attrs,
            content: Some(inlines),
        } => {
            assert_eq!(attrs.level, 2);
            match &inlines[0] {
                InlineNode::Text { text, .. } => assert_eq!(text, "Title"),
                _ => panic!("expected text"),
            }
        }
        _ => panic!("expected heading"),
    }
}

#[test]
fn round_trip_bullet_list() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[{"type":"bullet_list","content":[{"type":"list_item","content":[{"type":"paragraph","content":[{"type":"text","text":"Item"}]}]}]}]}}"#;
    let result = round_trip(json);
    match &result.doc.content[0] {
        BlockNode::BulletList { content } => {
            assert_eq!(content.len(), 1);
        }
        _ => panic!("expected bullet_list"),
    }
}

#[test]
fn round_trip_bold() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[{"type":"paragraph","content":[{"type":"text","text":"bold","marks":[{"type":"strong"}]}]}]}}"#;
    let result = round_trip(json);
    match &result.doc.content[0] {
        BlockNode::Paragraph {
            content: Some(inlines),
        } => {
            let has_bold = inlines.iter().any(|n| {
                matches!(n,
                    InlineNode::Text { text, marks } if text == "bold" && marks.contains(&InlineMark::Strong)
                )
            });
            assert!(has_bold);
        }
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn round_trip_code_block() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[{"type":"code_block","attrs":{"language":"rust"},"content":[{"type":"text","text":"fn main() {}"}]}]}}"#;
    let result = round_trip(json);
    match &result.doc.content[0] {
        BlockNode::CodeBlock { attrs, content } => {
            let lang = attrs.as_ref().and_then(|a| a.language.as_deref());
            assert_eq!(lang, Some("rust"));
            assert!(content.is_some());
        }
        _ => panic!("expected code_block"),
    }
}

#[test]
fn round_trip_horizontal_rule() {
    let json = r#"{"version":1,"schema_version":1,"doc":{"type":"doc","content":[{"type":"horizontal_rule"}]}}"#;
    let result = round_trip(json);
    let has_hr = result
        .doc
        .content
        .iter()
        .any(|n| matches!(n, BlockNode::HorizontalRule));
    assert!(has_hr);
}
