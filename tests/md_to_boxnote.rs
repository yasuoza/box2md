use box2markdown::boxnote::*;
use box2markdown::convert::md_to_boxnote;

fn convert_to_boxnote(md: &str) -> BoxNoteDocument {
    let json = md_to_boxnote::convert(md).unwrap();
    serde_json::from_str(&json).unwrap()
}

#[test]
fn simple_paragraph() {
    let doc = convert_to_boxnote("Hello, world!\n");
    assert_eq!(doc.version, 1);
    assert_eq!(doc.schema_version, 1);
    assert_eq!(doc.doc.node_type, "doc");
    match &doc.doc.content[0] {
        BlockNode::Paragraph {
            content: Some(inlines),
        } => match &inlines[0] {
            InlineNode::Text { text, marks } => {
                assert_eq!(text, "Hello, world!");
                assert!(marks.is_empty());
            }
            _ => panic!("expected text"),
        },
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn heading() {
    let doc = convert_to_boxnote("## Title\n");
    match &doc.doc.content[0] {
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
fn bold_and_italic() {
    let doc = convert_to_boxnote("**bold** *italic*\n");
    match &doc.doc.content[0] {
        BlockNode::Paragraph {
            content: Some(inlines),
        } => {
            let has_bold = inlines.iter().any(|n| {
                matches!(n, InlineNode::Text { text, marks } if text == "bold" && marks.contains(&InlineMark::Strong))
            });
            assert!(has_bold, "expected bold text");

            let has_italic = inlines.iter().any(|n| {
                matches!(n, InlineNode::Text { text, marks } if text == "italic" && marks.contains(&InlineMark::Em))
            });
            assert!(has_italic, "expected italic text");
        }
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn bullet_list() {
    let doc = convert_to_boxnote("- Item 1\n- Item 2\n");
    match &doc.doc.content[0] {
        BlockNode::BulletList { content } => {
            assert_eq!(content.len(), 2);
            match &content[0] {
                BlockNode::ListItem { content: items } => assert!(!items.is_empty()),
                _ => panic!("expected list_item"),
            }
        }
        _ => panic!("expected bullet_list"),
    }
}

#[test]
fn ordered_list() {
    let doc = convert_to_boxnote("1. First\n2. Second\n");
    match &doc.doc.content[0] {
        BlockNode::OrderedList { content, .. } => assert_eq!(content.len(), 2),
        _ => panic!("expected ordered_list"),
    }
}

#[test]
fn code_block_with_language() {
    let doc = convert_to_boxnote("```rust\nfn main() {}\n```\n");
    match &doc.doc.content[0] {
        BlockNode::CodeBlock { attrs, content } => {
            let lang = attrs.as_ref().and_then(|a| a.language.as_deref());
            assert_eq!(lang, Some("rust"));
            let has_code = content.as_ref().is_some_and(|c| {
                c.iter()
                    .any(|n| matches!(n, InlineNode::Text { text, .. } if text.contains("fn main()")))
            });
            assert!(has_code);
        }
        _ => panic!("expected code_block"),
    }
}

#[test]
fn blockquote() {
    let doc = convert_to_boxnote("> quoted text\n");
    match &doc.doc.content[0] {
        BlockNode::Blockquote { content } => assert!(!content.is_empty()),
        _ => panic!("expected blockquote"),
    }
}

#[test]
fn horizontal_rule() {
    let doc = convert_to_boxnote("---\n");
    let has_hr = doc
        .doc
        .content
        .iter()
        .any(|n| matches!(n, BlockNode::HorizontalRule));
    assert!(has_hr);
}

#[test]
fn link() {
    let doc = convert_to_boxnote("[example](https://example.com)\n");
    match &doc.doc.content[0] {
        BlockNode::Paragraph {
            content: Some(inlines),
        } => {
            let has_link = inlines.iter().any(|n| {
                matches!(n,
                    InlineNode::Text { marks, .. } if marks.iter().any(|m| matches!(m,
                        InlineMark::Link { attrs } if attrs.href == "https://example.com"
                    ))
                )
            });
            assert!(has_link, "expected link mark");
        }
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn table() {
    let doc = convert_to_boxnote("| A | B |\n| --- | --- |\n| 1 | 2 |\n");
    match &doc.doc.content[0] {
        BlockNode::Table { content } => {
            assert_eq!(content.len(), 2);
            match &content[0] {
                BlockNode::TableRow { content: cells } => assert_eq!(cells.len(), 2),
                _ => panic!("expected table_row"),
            }
        }
        _ => panic!("expected table"),
    }
}

#[test]
fn strikethrough() {
    let doc = convert_to_boxnote("~~deleted~~\n");
    match &doc.doc.content[0] {
        BlockNode::Paragraph {
            content: Some(inlines),
        } => {
            let has_strike = inlines.iter().any(|n| {
                matches!(n, InlineNode::Text { marks, .. } if marks.contains(&InlineMark::Strikethrough))
            });
            assert!(has_strike, "expected strikethrough mark");
        }
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn inline_code() {
    let doc = convert_to_boxnote("`code`\n");
    match &doc.doc.content[0] {
        BlockNode::Paragraph {
            content: Some(inlines),
        } => {
            let has_code = inlines.iter().any(|n| {
                matches!(n, InlineNode::Text { text, marks } if text == "code" && marks.contains(&InlineMark::Code))
            });
            assert!(has_code, "expected code mark");
        }
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn checklist() {
    let doc = convert_to_boxnote("- [x] Done\n- [ ] Todo\n");
    assert!(matches!(doc.doc.content[0], BlockNode::CheckList { .. }));
    match &doc.doc.content[0] {
        BlockNode::CheckList { content } => {
            assert_eq!(content.len(), 2);
            assert!(matches!(
                content[0],
                BlockNode::CheckListItem {
                    attrs: CheckListItemAttrs { checked: true },
                    ..
                }
            ));
            assert!(matches!(
                content[1],
                BlockNode::CheckListItem {
                    attrs: CheckListItemAttrs { checked: false },
                    ..
                }
            ));
        }
        _ => panic!("expected check_list"),
    }
}

#[test]
fn valid_boxnote_structure() {
    let json = md_to_boxnote::convert("Test\n").unwrap();
    let doc: BoxNoteDocument = serde_json::from_str(&json).unwrap();
    assert_eq!(doc.version, 1);
    assert_eq!(doc.schema_version, 1);
    assert!(doc.doc.validate().is_ok());
}
