use anyhow::Result;
use comrak::nodes::{AstNode, ListType, NodeValue};
use comrak::{parse_document, Arena, Options};
use serde_json::{Map, Value};

use crate::boxnote::*;

pub fn convert(markdown: &str) -> Result<String> {
    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;

    let root = parse_document(&arena, markdown, &options);
    let blocks = convert_children(root);

    let doc = BoxNoteDocument {
        version: 1,
        schema_version: 1,
        doc: DocNode {
            node_type: "doc".to_string(),
            content: blocks,
            attrs: None,
        },
    };

    let json = serde_json::to_string_pretty(&document_to_value(&doc))?;
    Ok(format!("{json}\n"))
}

fn document_to_value(doc: &BoxNoteDocument) -> Value {
    let mut root = Map::new();
    root.insert("version".to_string(), Value::from(doc.version));
    root.insert(
        "schema_version".to_string(),
        Value::from(doc.schema_version),
    );

    let mut doc_obj = Map::new();
    doc_obj.insert("type".to_string(), Value::from(doc.doc.node_type.clone()));
    doc_obj.insert(
        "content".to_string(),
        Value::Array(doc.doc.content.iter().map(block_to_value).collect()),
    );
    if let Some(attrs) = &doc.doc.attrs {
        doc_obj.insert("attrs".to_string(), attrs.clone());
    }
    root.insert("doc".to_string(), Value::Object(doc_obj));

    Value::Object(root)
}

fn block_to_value(block: &BlockNode) -> Value {
    let mut value = Map::new();
    match block {
        BlockNode::Paragraph { content } => {
            value.insert("type".to_string(), Value::from("paragraph"));
            if let Some(content) = content {
                value.insert(
                    "content".to_string(),
                    Value::Array(content.iter().map(inline_to_value).collect()),
                );
            }
        }
        BlockNode::Heading { attrs, content } => {
            value.insert("type".to_string(), Value::from("heading"));
            let mut attrs_value = Map::new();
            attrs_value.insert("level".to_string(), Value::from(attrs.level));
            value.insert("attrs".to_string(), Value::Object(attrs_value));
            if let Some(content) = content {
                value.insert(
                    "content".to_string(),
                    Value::Array(content.iter().map(inline_to_value).collect()),
                );
            }
        }
        BlockNode::BulletList { content } => {
            value.insert("type".to_string(), Value::from("bullet_list"));
            value.insert(
                "content".to_string(),
                Value::Array(content.iter().map(block_to_value).collect()),
            );
        }
        BlockNode::OrderedList { attrs, content } => {
            value.insert("type".to_string(), Value::from("ordered_list"));
            if let Some(attrs) = attrs {
                let mut attrs_value = Map::new();
                attrs_value.insert("order".to_string(), Value::from(attrs.order));
                value.insert("attrs".to_string(), Value::Object(attrs_value));
            }
            value.insert(
                "content".to_string(),
                Value::Array(content.iter().map(block_to_value).collect()),
            );
        }
        BlockNode::ListItem { content } => {
            value.insert("type".to_string(), Value::from("list_item"));
            value.insert(
                "content".to_string(),
                Value::Array(content.iter().map(block_to_value).collect()),
            );
        }
        BlockNode::CheckList { content } => {
            value.insert("type".to_string(), Value::from("check_list"));
            value.insert(
                "content".to_string(),
                Value::Array(content.iter().map(block_to_value).collect()),
            );
        }
        BlockNode::CheckListItem { attrs, content } => {
            value.insert("type".to_string(), Value::from("check_list_item"));
            let mut attrs_value = Map::new();
            attrs_value.insert("checked".to_string(), Value::from(attrs.checked));
            value.insert("attrs".to_string(), Value::Object(attrs_value));
            value.insert(
                "content".to_string(),
                Value::Array(content.iter().map(block_to_value).collect()),
            );
        }
        BlockNode::CodeBlock { attrs, content } => {
            value.insert("type".to_string(), Value::from("code_block"));
            if let Some(attrs) = attrs {
                let mut attrs_value = Map::new();
                match &attrs.language {
                    Some(language) => {
                        attrs_value.insert("language".to_string(), Value::from(language.clone()));
                    }
                    None => {
                        attrs_value.insert("language".to_string(), Value::Null);
                    }
                }
                value.insert("attrs".to_string(), Value::Object(attrs_value));
            }
            if let Some(content) = content {
                value.insert(
                    "content".to_string(),
                    Value::Array(content.iter().map(inline_to_value).collect()),
                );
            }
        }
        BlockNode::Blockquote { content } => {
            value.insert("type".to_string(), Value::from("blockquote"));
            value.insert(
                "content".to_string(),
                Value::Array(content.iter().map(block_to_value).collect()),
            );
        }
        BlockNode::Table { content } => {
            value.insert("type".to_string(), Value::from("table"));
            value.insert(
                "content".to_string(),
                Value::Array(content.iter().map(block_to_value).collect()),
            );
        }
        BlockNode::TableRow { content } => {
            value.insert("type".to_string(), Value::from("table_row"));
            value.insert(
                "content".to_string(),
                Value::Array(content.iter().map(block_to_value).collect()),
            );
        }
        BlockNode::TableCell { attrs, content } => {
            value.insert("type".to_string(), Value::from("table_cell"));
            if let Some(attrs) = attrs {
                let mut attrs_value = Map::new();
                if let Some(colspan) = attrs.colspan {
                    attrs_value.insert("colspan".to_string(), Value::from(colspan));
                }
                if let Some(rowspan) = attrs.rowspan {
                    attrs_value.insert("rowspan".to_string(), Value::from(rowspan));
                }
                if let Some(colwidth) = &attrs.colwidth {
                    attrs_value.insert(
                        "colwidth".to_string(),
                        Value::Array(colwidth.iter().copied().map(Value::from).collect()),
                    );
                }
                value.insert("attrs".to_string(), Value::Object(attrs_value));
            }
            value.insert(
                "content".to_string(),
                Value::Array(content.iter().map(block_to_value).collect()),
            );
        }
        BlockNode::HorizontalRule => {
            value.insert("type".to_string(), Value::from("horizontal_rule"));
        }
        BlockNode::HardBreak => {
            value.insert("type".to_string(), Value::from("hard_break"));
        }
        BlockNode::Unknown { raw, .. } => return raw.clone(),
    }
    Value::Object(value)
}

fn inline_to_value(inline: &InlineNode) -> Value {
    let mut value = Map::new();
    match inline {
        InlineNode::Text { text, marks } => {
            value.insert("type".to_string(), Value::from("text"));
            value.insert("text".to_string(), Value::from(text.clone()));
            if !marks.is_empty() {
                value.insert(
                    "marks".to_string(),
                    Value::Array(marks.iter().map(mark_to_value).collect()),
                );
            }
        }
        InlineNode::HardBreak => {
            value.insert("type".to_string(), Value::from("hard_break"));
        }
        InlineNode::Unknown { raw, .. } => return raw.clone(),
    }
    Value::Object(value)
}

fn mark_to_value(mark: &InlineMark) -> Value {
    let mut value = Map::new();
    match mark {
        InlineMark::Strong => {
            value.insert("type".to_string(), Value::from("strong"));
        }
        InlineMark::Em => {
            value.insert("type".to_string(), Value::from("em"));
        }
        InlineMark::Code => {
            value.insert("type".to_string(), Value::from("code"));
        }
        InlineMark::Underline => {
            value.insert("type".to_string(), Value::from("underline"));
        }
        InlineMark::Strikethrough => {
            value.insert("type".to_string(), Value::from("strikethrough"));
        }
        InlineMark::Link { attrs } => {
            value.insert("type".to_string(), Value::from("link"));
            let mut attrs_value = Map::new();
            attrs_value.insert("href".to_string(), Value::from(attrs.href.clone()));
            value.insert("attrs".to_string(), Value::Object(attrs_value));
        }
        InlineMark::Unknown { mark_type } => {
            value.insert("type".to_string(), Value::from(mark_type.clone()));
        }
    }
    Value::Object(value)
}

fn convert_children<'a>(node: &'a AstNode<'a>) -> Vec<BlockNode> {
    node.children().filter_map(convert_node).collect()
}

fn convert_node<'a>(node: &'a AstNode<'a>) -> Option<BlockNode> {
    match &node.data().value {
        NodeValue::Paragraph => {
            let inlines = collect_inlines(node);
            Some(BlockNode::Paragraph {
                content: if inlines.is_empty() {
                    None
                } else {
                    Some(inlines)
                },
            })
        }
        NodeValue::Heading(heading) => {
            let inlines = collect_inlines(node);
            Some(BlockNode::Heading {
                attrs: HeadingAttrs {
                    level: heading.level,
                },
                content: if inlines.is_empty() {
                    None
                } else {
                    Some(inlines)
                },
            })
        }
        NodeValue::List(list) => {
            let items = convert_children(node);
            if list.list_type == ListType::Bullet {
                if list.is_task_list {
                    Some(BlockNode::CheckList { content: items })
                } else {
                    Some(BlockNode::BulletList { content: items })
                }
            } else {
                Some(BlockNode::OrderedList {
                    attrs: Some(OrderedListAttrs {
                        order: list.start as u64,
                    }),
                    content: items,
                })
            }
        }
        NodeValue::Item(_) => {
            let blocks = convert_children(node);
            Some(BlockNode::ListItem { content: blocks })
        }
        NodeValue::TaskItem(task_item) => {
            let blocks = convert_children(node);
            Some(BlockNode::CheckListItem {
                attrs: CheckListItemAttrs {
                    checked: task_item.symbol.is_some(),
                },
                content: blocks,
            })
        }
        NodeValue::CodeBlock(code) => {
            let language = if code.info.trim().is_empty() {
                None
            } else {
                Some(code.info.trim().to_string())
            };
            let text = code.literal.trim_end_matches('\n').to_string();
            Some(BlockNode::CodeBlock {
                attrs: Some(CodeBlockAttrs { language }),
                content: if text.is_empty() {
                    None
                } else {
                    Some(vec![InlineNode::Text {
                        text,
                        marks: vec![],
                    }])
                },
            })
        }
        NodeValue::BlockQuote => {
            let blocks = convert_children(node);
            Some(BlockNode::Blockquote { content: blocks })
        }
        NodeValue::ThematicBreak => Some(BlockNode::HorizontalRule),
        NodeValue::Table(_) => Some(BlockNode::Table {
            content: convert_table_rows(node),
        }),
        NodeValue::HtmlBlock(html) => {
            if html.literal.trim().is_empty() {
                None
            } else {
                Some(BlockNode::Paragraph {
                    content: Some(vec![InlineNode::Text {
                        text: html.literal.trim().to_string(),
                        marks: vec![],
                    }]),
                })
            }
        }
        _ => None,
    }
}

fn convert_table_rows<'a>(table_node: &'a AstNode<'a>) -> Vec<BlockNode> {
    table_node
        .children()
        .filter_map(|row| match &row.data().value {
            NodeValue::TableRow(_) => Some(BlockNode::TableRow {
                content: convert_table_cells(row),
            }),
            _ => None,
        })
        .collect()
}

fn convert_table_cells<'a>(row_node: &'a AstNode<'a>) -> Vec<BlockNode> {
    row_node
        .children()
        .filter_map(|cell| match &cell.data().value {
            NodeValue::TableCell => {
                let inlines = collect_inlines(cell);
                Some(BlockNode::TableCell {
                    attrs: None,
                    content: vec![BlockNode::Paragraph {
                        content: if inlines.is_empty() {
                            None
                        } else {
                            Some(inlines)
                        },
                    }],
                })
            }
            _ => None,
        })
        .collect()
}

fn collect_inlines<'a>(node: &'a AstNode<'a>) -> Vec<InlineNode> {
    let mut inlines = Vec::new();
    collect_inlines_recursive(node, &mut inlines, &[]);
    inlines
}

fn collect_inlines_recursive<'a>(
    node: &'a AstNode<'a>,
    inlines: &mut Vec<InlineNode>,
    current_marks: &[InlineMark],
) {
    for child in node.children() {
        match &child.data().value {
            NodeValue::Text(text) => {
                inlines.push(InlineNode::Text {
                    text: text.to_string(),
                    marks: current_marks.to_vec(),
                });
            }
            NodeValue::Code(code) => {
                let mut marks = current_marks.to_vec();
                marks.push(InlineMark::Code);
                inlines.push(InlineNode::Text {
                    text: code.literal.clone(),
                    marks,
                });
            }
            NodeValue::Strong => {
                let mut marks = current_marks.to_vec();
                marks.push(InlineMark::Strong);
                collect_inlines_recursive(child, inlines, &marks);
            }
            NodeValue::Emph => {
                let mut marks = current_marks.to_vec();
                marks.push(InlineMark::Em);
                collect_inlines_recursive(child, inlines, &marks);
            }
            NodeValue::Strikethrough => {
                let mut marks = current_marks.to_vec();
                marks.push(InlineMark::Strikethrough);
                collect_inlines_recursive(child, inlines, &marks);
            }
            NodeValue::Link(link) => {
                let mut marks = current_marks.to_vec();
                marks.push(InlineMark::Link {
                    attrs: LinkAttrs {
                        href: link.url.clone(),
                    },
                });
                collect_inlines_recursive(child, inlines, &marks);
            }
            NodeValue::SoftBreak => {
                inlines.push(InlineNode::Text {
                    text: " ".to_string(),
                    marks: current_marks.to_vec(),
                });
            }
            NodeValue::LineBreak => {
                inlines.push(InlineNode::HardBreak);
            }
            _ => collect_inlines_recursive(child, inlines, current_marks),
        }
    }
}
