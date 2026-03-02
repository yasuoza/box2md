use anyhow::Result;

use crate::boxnote::{BlockNode, BoxNoteDocument, InlineMark, InlineNode};

enum ListContext {
    None,
    Bullet { depth: usize },
    Ordered { depth: usize, start: u64 },
    CheckList { depth: usize },
}

pub fn convert(document: &BoxNoteDocument) -> Result<String> {
    let mut output = String::new();
    let mut warnings = Vec::new();
    convert_blocks(
        &document.doc.content,
        &mut output,
        &mut warnings,
        0,
        &ListContext::None,
    );

    for warning in &warnings {
        eprintln!("{warning}");
    }

    Ok(output.trim_end().to_string() + "\n")
}

fn convert_blocks(
    blocks: &[BlockNode],
    output: &mut String,
    warnings: &mut Vec<String>,
    indent: usize,
    list_context: &ListContext,
) {
    for block in blocks {
        convert_block(block, output, warnings, indent, list_context);
    }
}

fn convert_block(
    block: &BlockNode,
    output: &mut String,
    warnings: &mut Vec<String>,
    indent: usize,
    list_context: &ListContext,
) {
    match block {
        BlockNode::Paragraph { content } => {
            let rendered = render_inline_nodes(content.as_deref().unwrap_or(&[]), warnings);
            write_indented_text(output, indent, &rendered);
            output.push('\n');
        }
        BlockNode::Heading { attrs, content } => {
            let level = attrs.level.clamp(1, 6) as usize;
            let rendered = render_inline_nodes(content.as_deref().unwrap_or(&[]), warnings);
            output.push_str(&" ".repeat(indent));
            output.push_str(&"#".repeat(level));
            output.push(' ');
            output.push_str(&rendered);
            output.push_str("\n\n");
        }
        BlockNode::BulletList { content } => {
            let depth = list_depth(list_context);
            let context = ListContext::Bullet { depth };
            for item in content {
                if let BlockNode::ListItem { content } = item {
                    render_list_item(
                        content,
                        output,
                        warnings,
                        &context,
                        &format!("{}- ", " ".repeat(depth * 2)),
                        depth * 2 + 2,
                    );
                } else {
                    warnings.push(format!(
                        "WARN: unknown node type \"{}\" skipped",
                        block_node_name(item)
                    ));
                }
            }
            if matches!(list_context, ListContext::None) {
                output.push('\n');
            }
        }
        BlockNode::OrderedList { attrs, content } => {
            let depth = list_depth(list_context);
            let start = attrs.as_ref().map_or_else(
                || match list_context {
                    ListContext::Ordered { start, .. } => *start,
                    _ => 1,
                },
                |a| a.order,
            );
            let context = ListContext::Ordered { depth, start };
            for (index, item) in content.iter().enumerate() {
                if let BlockNode::ListItem { content } = item {
                    let number = start + index as u64;
                    render_list_item(
                        content,
                        output,
                        warnings,
                        &context,
                        &format!("{}{}. ", " ".repeat(depth * 3), number),
                        depth * 3 + 3,
                    );
                } else {
                    warnings.push(format!(
                        "WARN: unknown node type \"{}\" skipped",
                        block_node_name(item)
                    ));
                }
            }
            if matches!(list_context, ListContext::None) {
                output.push('\n');
            }
        }
        BlockNode::ListItem { content } => {
            convert_blocks(content, output, warnings, indent, list_context);
        }
        BlockNode::CheckList { content } => {
            let depth = list_depth(list_context);
            let context = ListContext::CheckList { depth };
            for item in content {
                if let BlockNode::CheckListItem { attrs, content } = item {
                    let marker = if attrs.checked { "[x]" } else { "[ ]" };
                    render_list_item(
                        content,
                        output,
                        warnings,
                        &context,
                        &format!("{}- {} ", " ".repeat(depth * 2), marker),
                        depth * 2 + 6,
                    );
                } else {
                    warnings.push(format!(
                        "WARN: unknown node type \"{}\" skipped",
                        block_node_name(item)
                    ));
                }
            }
            if matches!(list_context, ListContext::None) {
                output.push('\n');
            }
        }
        BlockNode::CheckListItem { attrs, content } => {
            let marker = if attrs.checked { "- [x] " } else { "- [ ] " };
            render_list_item(content, output, warnings, list_context, marker, indent + 6);
        }
        BlockNode::CodeBlock { attrs, content } => {
            let mut text = String::new();
            for node in content.as_deref().unwrap_or(&[]) {
                match node {
                    InlineNode::Text { text: value, .. } => text.push_str(value),
                    InlineNode::HardBreak => text.push('\n'),
                    InlineNode::Unknown { node_type, .. } => {
                        warnings.push(format!("WARN: unknown node type \"{node_type}\" skipped"));
                    }
                }
            }

            output.push_str(&" ".repeat(indent));
            output.push_str("```");
            if let Some(language) = attrs
                .as_ref()
                .and_then(|attrs| attrs.language.as_ref())
                .filter(|language| !language.is_empty())
            {
                output.push_str(language);
            }
            output.push('\n');
            output.push_str(&text);
            if !text.ends_with('\n') {
                output.push('\n');
            }
            output.push_str(&" ".repeat(indent));
            output.push_str("```\n\n");
        }
        BlockNode::Blockquote { content } => {
            let mut inner = String::new();
            convert_blocks(content, &mut inner, warnings, 0, &ListContext::None);
            for line in inner.trim_end().lines() {
                output.push_str(&" ".repeat(indent));
                output.push_str("> ");
                output.push_str(line);
                output.push('\n');
            }
            output.push('\n');
        }
        BlockNode::Table { content } => {
            let mut rows: Vec<Vec<String>> = Vec::new();
            for row in content {
                if let BlockNode::TableRow { content } = row {
                    let mut cells = Vec::new();
                    for cell in content {
                        if let BlockNode::TableCell { content, .. } = cell {
                            cells.push(render_table_cell_text(content, warnings));
                        } else {
                            warnings.push(format!(
                                "WARN: unknown node type \"{}\" skipped",
                                block_node_name(cell)
                            ));
                        }
                    }
                    rows.push(cells);
                } else {
                    warnings.push(format!(
                        "WARN: unknown node type \"{}\" skipped",
                        block_node_name(row)
                    ));
                }
            }

            if let Some(header) = rows.first() {
                output.push_str(&" ".repeat(indent));
                output.push_str("| ");
                output.push_str(&header.join(" | "));
                output.push_str(" |\n");

                output.push_str(&" ".repeat(indent));
                output.push('|');
                for _ in 0..header.len() {
                    output.push_str(" --- |");
                }
                output.push('\n');

                for row in rows.iter().skip(1) {
                    let mut values = row.clone();
                    while values.len() < header.len() {
                        values.push(String::new());
                    }
                    output.push_str(&" ".repeat(indent));
                    output.push_str("| ");
                    output.push_str(&values[..header.len()].join(" | "));
                    output.push_str(" |\n");
                }
                output.push('\n');
            }
        }
        BlockNode::TableRow { .. } | BlockNode::TableCell { .. } => {
            warnings.push(format!(
                "WARN: unknown node type \"{}\" skipped",
                block_node_name(block)
            ));
        }
        BlockNode::HorizontalRule => {
            output.push_str(&" ".repeat(indent));
            output.push_str("---\n\n");
        }
        BlockNode::HardBreak => {
            output.push('\n');
        }
        BlockNode::Unknown { node_type, .. } => {
            warnings.push(format!("WARN: unknown node type \"{node_type}\" skipped"));
        }
    }
}

fn render_list_item(
    content: &[BlockNode],
    output: &mut String,
    warnings: &mut Vec<String>,
    list_context: &ListContext,
    marker: &str,
    continuation_indent: usize,
) {
    let mut consumed_first_paragraph = false;
    if let Some(BlockNode::Paragraph { content }) = content.first() {
        output.push_str(marker);
        output.push_str(&render_inline_nodes(content.as_deref().unwrap_or(&[]), warnings));
        output.push('\n');
        consumed_first_paragraph = true;
    } else {
        output.push_str(marker);
        output.push('\n');
    }

    let depth = list_depth(list_context);
    let start_index = if consumed_first_paragraph { 1 } else { 0 };
    for block in &content[start_index..] {
        match block {
            BlockNode::BulletList { .. } => {
                convert_block(
                    block,
                    output,
                    warnings,
                    0,
                    &ListContext::Bullet { depth: depth + 1 },
                );
            }
            BlockNode::OrderedList { attrs, .. } => {
                let start = attrs.as_ref().map_or(1, |a| a.order);
                convert_block(
                    block,
                    output,
                    warnings,
                    0,
                    &ListContext::Ordered {
                        depth: depth + 1,
                        start,
                    },
                );
            }
            BlockNode::CheckList { .. } => {
                convert_block(
                    block,
                    output,
                    warnings,
                    0,
                    &ListContext::CheckList { depth: depth + 1 },
                );
            }
            _ => convert_block(
                block,
                output,
                warnings,
                continuation_indent,
                &ListContext::None,
            ),
        }
    }
}

fn render_inline_nodes(nodes: &[InlineNode], warnings: &mut Vec<String>) -> String {
    let mut rendered = String::new();
    for node in nodes {
        match node {
            InlineNode::Text { text, marks } => rendered.push_str(&apply_marks(text, marks, warnings)),
            InlineNode::HardBreak => rendered.push('\n'),
            InlineNode::Unknown { node_type, .. } => {
                warnings.push(format!("WARN: unknown node type \"{node_type}\" skipped"));
            }
        }
    }
    rendered
}

fn apply_marks(text: &str, marks: &[InlineMark], warnings: &mut Vec<String>) -> String {
    let mut rendered = text.to_string();
    for mark in marks {
        rendered = match mark {
            InlineMark::Strong => format!("**{rendered}**"),
            InlineMark::Em => format!("*{rendered}*"),
            InlineMark::Code => format!("`{rendered}`"),
            InlineMark::Underline => rendered,
            InlineMark::Strikethrough => format!("~~{rendered}~~"),
            InlineMark::Link { attrs } => format!("[{rendered}]({})", attrs.href),
            InlineMark::Unknown { mark_type } => {
                warnings.push(format!("WARN: unknown mark type \"{mark_type}\" skipped"));
                rendered
            }
        };
    }
    rendered
}

fn render_table_cell_text(content: &[BlockNode], warnings: &mut Vec<String>) -> String {
    let mut value = String::new();
    for block in content {
        match block {
            BlockNode::Paragraph { content } => {
                if !value.is_empty() {
                    value.push(' ');
                }
                value.push_str(&render_inline_nodes(content.as_deref().unwrap_or(&[]), warnings));
            }
            BlockNode::HardBreak => value.push(' '),
            BlockNode::Unknown { node_type, .. } => {
                warnings.push(format!("WARN: unknown node type \"{node_type}\" skipped"));
            }
            _ => {
                let mut nested = String::new();
                convert_block(block, &mut nested, warnings, 0, &ListContext::None);
                let compact = nested.trim().replace('\n', " ");
                if !compact.is_empty() {
                    if !value.is_empty() {
                        value.push(' ');
                    }
                    value.push_str(&compact);
                }
            }
        }
    }
    value
}

fn write_indented_text(output: &mut String, indent: usize, text: &str) {
    let prefix = " ".repeat(indent);
    let mut lines = text.split('\n').peekable();
    while let Some(line) = lines.next() {
        output.push_str(&prefix);
        output.push_str(line);
        output.push('\n');
        if lines.peek().is_some() && line.is_empty() {
            output.push_str(&prefix);
        }
    }
}

fn list_depth(list_context: &ListContext) -> usize {
    match list_context {
        ListContext::None => 0,
        ListContext::Bullet { depth }
        | ListContext::Ordered { depth, .. }
        | ListContext::CheckList { depth } => *depth,
    }
}

fn block_node_name(block: &BlockNode) -> &str {
    match block {
        BlockNode::Paragraph { .. } => "paragraph",
        BlockNode::Heading { .. } => "heading",
        BlockNode::BulletList { .. } => "bullet_list",
        BlockNode::OrderedList { .. } => "ordered_list",
        BlockNode::ListItem { .. } => "list_item",
        BlockNode::CheckList { .. } => "check_list",
        BlockNode::CheckListItem { .. } => "check_list_item",
        BlockNode::CodeBlock { .. } => "code_block",
        BlockNode::Blockquote { .. } => "blockquote",
        BlockNode::Table { .. } => "table",
        BlockNode::TableRow { .. } => "table_row",
        BlockNode::TableCell { .. } => "table_cell",
        BlockNode::HorizontalRule => "horizontal_rule",
        BlockNode::HardBreak => "hard_break",
        BlockNode::Unknown { node_type, .. } => node_type,
    }
}
