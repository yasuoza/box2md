use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BoxNoteDocument {
    #[serde(default)]
    pub version: u64,
    pub schema_version: u64,
    pub doc: DocNode,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DocNode {
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(default)]
    pub content: Vec<BlockNode>,
    #[serde(default)]
    pub attrs: Option<Value>,
}

impl DocNode {
    pub fn validate(&self) -> Result<(), String> {
        if self.node_type != "doc" {
            return Err(format!(
                "expected root node type \"doc\", got \"{}\"",
                self.node_type
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum BlockNode {
    Paragraph {
        content: Option<Vec<InlineNode>>,
    },
    Heading {
        attrs: HeadingAttrs,
        content: Option<Vec<InlineNode>>,
    },
    BulletList {
        content: Vec<BlockNode>,
    },
    OrderedList {
        attrs: Option<OrderedListAttrs>,
        content: Vec<BlockNode>,
    },
    ListItem {
        content: Vec<BlockNode>,
    },
    CheckList {
        content: Vec<BlockNode>,
    },
    CheckListItem {
        attrs: CheckListItemAttrs,
        content: Vec<BlockNode>,
    },
    CodeBlock {
        attrs: Option<CodeBlockAttrs>,
        content: Option<Vec<InlineNode>>,
    },
    Blockquote {
        content: Vec<BlockNode>,
    },
    Table {
        content: Vec<BlockNode>,
    },
    TableRow {
        content: Vec<BlockNode>,
    },
    TableCell {
        attrs: Option<TableCellAttrs>,
        content: Vec<BlockNode>,
    },
    HorizontalRule,
    HardBreak,
    Unknown {
        node_type: String,
        raw: Value,
    },
}

impl BlockNode {
    fn from_value(raw: Value) -> Result<Self, String> {
        let node_type = raw
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string();

        match node_type.as_str() {
            "paragraph" => Ok(Self::Paragraph {
                content: parse_opt(&raw, "content")?,
            }),
            "heading" => Ok(Self::Heading {
                attrs: parse_req(&raw, "attrs")?,
                content: parse_opt(&raw, "content")?,
            }),
            "bullet_list" => Ok(Self::BulletList {
                content: parse_vec(&raw, "content")?,
            }),
            "ordered_list" => Ok(Self::OrderedList {
                attrs: parse_opt(&raw, "attrs")?,
                content: parse_vec(&raw, "content")?,
            }),
            "list_item" => Ok(Self::ListItem {
                content: parse_vec(&raw, "content")?,
            }),
            "check_list" => Ok(Self::CheckList {
                content: parse_vec(&raw, "content")?,
            }),
            "check_list_item" => Ok(Self::CheckListItem {
                attrs: parse_or_default(&raw, "attrs")?,
                content: parse_vec(&raw, "content")?,
            }),
            "code_block" => Ok(Self::CodeBlock {
                attrs: parse_opt(&raw, "attrs")?,
                content: parse_opt(&raw, "content")?,
            }),
            "blockquote" => Ok(Self::Blockquote {
                content: parse_vec(&raw, "content")?,
            }),
            "table" => Ok(Self::Table {
                content: parse_vec(&raw, "content")?,
            }),
            "table_row" => Ok(Self::TableRow {
                content: parse_vec(&raw, "content")?,
            }),
            "table_cell" => Ok(Self::TableCell {
                attrs: parse_opt(&raw, "attrs")?,
                content: parse_vec(&raw, "content")?,
            }),
            "horizontal_rule" => Ok(Self::HorizontalRule),
            "hard_break" => Ok(Self::HardBreak),
            _ => Ok(Self::Unknown { node_type, raw }),
        }
    }
}

impl<'de> Deserialize<'de> for BlockNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = Value::deserialize(deserializer)?;
        Self::from_value(raw).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum InlineNode {
    Text {
        text: String,
        marks: Vec<InlineMark>,
    },
    HardBreak,
    Unknown {
        node_type: String,
        raw: Value,
    },
}

impl InlineNode {
    fn from_value(raw: Value) -> Result<Self, String> {
        let node_type = raw
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string();

        match node_type.as_str() {
            "text" => Ok(Self::Text {
                text: raw
                    .get("text")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                marks: parse_vec(&raw, "marks")?,
            }),
            "hard_break" => Ok(Self::HardBreak),
            _ => Ok(Self::Unknown { node_type, raw }),
        }
    }
}

impl<'de> Deserialize<'de> for InlineNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = Value::deserialize(deserializer)?;
        Self::from_value(raw).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum InlineMark {
    Strong,
    Em,
    Code,
    Underline,
    Strikethrough,
    Link { attrs: LinkAttrs },
    Unknown { mark_type: String },
}

impl InlineMark {
    fn from_value(raw: Value) -> Result<Self, String> {
        let mark_type = raw
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string();

        match mark_type.as_str() {
            "strong" => Ok(Self::Strong),
            "em" => Ok(Self::Em),
            "code" => Ok(Self::Code),
            "underline" => Ok(Self::Underline),
            "strikethrough" => Ok(Self::Strikethrough),
            "link" => Ok(Self::Link {
                attrs: parse_req(&raw, "attrs")?,
            }),
            _ => Ok(Self::Unknown { mark_type }),
        }
    }
}

impl<'de> Deserialize<'de> for InlineMark {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = Value::deserialize(deserializer)?;
        Self::from_value(raw).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct HeadingAttrs {
    pub level: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct OrderedListAttrs {
    #[serde(default = "default_order")]
    pub order: u64,
}

fn default_order() -> u64 {
    1
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct CheckListItemAttrs {
    #[serde(default)]
    pub checked: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CodeBlockAttrs {
    #[serde(default)]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TableCellAttrs {
    #[serde(default)]
    pub colspan: Option<u32>,
    #[serde(default)]
    pub rowspan: Option<u32>,
    #[serde(default)]
    pub colwidth: Option<Vec<u32>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct LinkAttrs {
    pub href: String,
}

// ── Helper functions ──

fn parse_opt<T: DeserializeOwned>(raw: &Value, key: &str) -> Result<Option<T>, String> {
    raw.get(key)
        .map(|v| serde_json::from_value(v.clone()).map_err(|e| e.to_string()))
        .transpose()
}

fn parse_vec<T: DeserializeOwned>(raw: &Value, key: &str) -> Result<Vec<T>, String> {
    Ok(parse_opt::<Vec<T>>(raw, key)?.unwrap_or_default())
}

fn parse_req<T: DeserializeOwned>(raw: &Value, key: &str) -> Result<T, String> {
    let value = raw
        .get(key)
        .cloned()
        .ok_or_else(|| format!("missing required field: {key}"))?;
    serde_json::from_value(value).map_err(|e| e.to_string())
}

fn parse_or_default<T: DeserializeOwned + Default>(raw: &Value, key: &str) -> Result<T, String> {
    match raw.get(key).cloned() {
        Some(v) => serde_json::from_value(v).map_err(|e| e.to_string()),
        None => Ok(T::default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_paragraph_with_text() {
        let json = r#"{
            "version": 1,
            "schema_version": 1,
            "doc": {
                "type": "doc",
                "content": [
                    { "type": "paragraph", "content": [{ "type": "text", "text": "hello" }] }
                ]
            }
        }"#;
        let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();

        assert_eq!(
            doc.doc.content,
            vec![BlockNode::Paragraph {
                content: Some(vec![InlineNode::Text {
                    text: "hello".to_string(),
                    marks: vec![]
                }])
            }]
        );
    }

    #[test]
    fn parses_heading_with_level_attr() {
        let json = r#"{
            "version": 1,
            "schema_version": 1,
            "doc": {
                "type": "doc",
                "content": [
                    { "type": "heading", "attrs": { "level": 2 }, "content": [{ "type": "text", "text": "title" }] }
                ]
            }
        }"#;
        let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();

        assert_eq!(
            doc.doc.content,
            vec![BlockNode::Heading {
                attrs: HeadingAttrs { level: 2 },
                content: Some(vec![InlineNode::Text {
                    text: "title".to_string(),
                    marks: vec![]
                }])
            }]
        );
    }

    #[test]
    fn parses_nested_bullet_list() {
        let json = r#"{
            "version": 1,
            "schema_version": 1,
            "doc": {
                "type": "doc",
                "content": [
                    {
                        "type": "bullet_list",
                        "content": [{
                            "type": "list_item",
                            "content": [{
                                "type": "paragraph",
                                "content": [{ "type": "text", "text": "item" }]
                            }]
                        }]
                    }
                ]
            }
        }"#;
        let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();

        assert_eq!(
            doc.doc.content,
            vec![BlockNode::BulletList {
                content: vec![BlockNode::ListItem {
                    content: vec![BlockNode::Paragraph {
                        content: Some(vec![InlineNode::Text {
                            text: "item".to_string(),
                            marks: vec![]
                        }])
                    }]
                }]
            }]
        );
    }

    #[test]
    fn parses_text_with_marks() {
        let json = r#"{
            "version": 1,
            "schema_version": 1,
            "doc": {
                "type": "doc",
                "content": [{
                    "type": "paragraph",
                    "content": [{
                        "type": "text",
                        "text": "marked",
                        "marks": [
                            { "type": "strong" },
                            { "type": "em" },
                            { "type": "code" },
                            { "type": "link", "attrs": { "href": "https://example.com" } }
                        ]
                    }]
                }]
            }
        }"#;
        let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();

        match &doc.doc.content[0] {
            BlockNode::Paragraph {
                content: Some(content),
            } => match &content[0] {
                InlineNode::Text { marks, .. } => {
                    assert_eq!(marks[0], InlineMark::Strong);
                    assert_eq!(marks[1], InlineMark::Em);
                    assert_eq!(marks[2], InlineMark::Code);
                    assert_eq!(
                        marks[3],
                        InlineMark::Link {
                            attrs: LinkAttrs {
                                href: "https://example.com".to_string()
                            }
                        }
                    );
                }
                _ => panic!("expected text node"),
            },
            _ => panic!("expected paragraph node"),
        }
    }

    #[test]
    fn preserves_unknown_block_node_type() {
        let json = r#"{
            "version": 1,
            "schema_version": 1,
            "doc": { "type": "doc", "content": [{ "type": "mystery_block", "foo": 1 }] }
        }"#;
        let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();

        match &doc.doc.content[0] {
            BlockNode::Unknown { node_type, raw } => {
                assert_eq!(node_type, "mystery_block");
                assert_eq!(raw["foo"], 1);
            }
            _ => panic!("expected unknown block node"),
        }
    }

    #[test]
    fn preserves_unknown_inline_node_type() {
        let json = r#"{
            "version": 1,
            "schema_version": 1,
            "doc": {
                "type": "doc",
                "content": [{
                    "type": "paragraph",
                    "content": [{ "type": "mystery_inline", "foo": "bar" }]
                }]
            }
        }"#;
        let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();

        match &doc.doc.content[0] {
            BlockNode::Paragraph {
                content: Some(content),
            } => match &content[0] {
                InlineNode::Unknown { node_type, raw } => {
                    assert_eq!(node_type, "mystery_inline");
                    assert_eq!(raw["foo"], "bar");
                }
                _ => panic!("expected unknown inline node"),
            },
            _ => panic!("expected paragraph"),
        }
    }

    #[test]
    fn preserves_unknown_mark_type() {
        let json = r#"{
            "version": 1,
            "schema_version": 1,
            "doc": {
                "type": "doc",
                "content": [{
                    "type": "paragraph",
                    "content": [{
                        "type": "text",
                        "text": "x",
                        "marks": [{ "type": "mystery_mark" }]
                    }]
                }]
            }
        }"#;
        let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();

        match &doc.doc.content[0] {
            BlockNode::Paragraph {
                content: Some(content),
            } => match &content[0] {
                InlineNode::Text { marks, .. } => {
                    assert_eq!(
                        marks[0],
                        InlineMark::Unknown {
                            mark_type: "mystery_mark".to_string()
                        }
                    );
                }
                _ => panic!("expected text"),
            },
            _ => panic!("expected paragraph"),
        }
    }

    #[test]
    fn doc_node_validate_success_and_failure() {
        let ok = DocNode {
            node_type: "doc".to_string(),
            attrs: None,
            content: vec![],
        };
        let ng = DocNode {
            node_type: "paragraph".to_string(),
            attrs: None,
            content: vec![],
        };

        assert!(ok.validate().is_ok());
        assert_eq!(
            ng.validate(),
            Err("expected root node type \"doc\", got \"paragraph\"".to_string())
        );
    }

    #[test]
    fn parses_hard_break_as_block_and_inline() {
        let json = r#"{
            "version": 1,
            "schema_version": 1,
            "doc": {
                "type": "doc",
                "content": [
                    { "type": "hard_break" },
                    {
                        "type": "paragraph",
                        "content": [{ "type": "hard_break" }]
                    }
                ]
            }
        }"#;
        let doc: BoxNoteDocument = serde_json::from_str(json).unwrap();

        assert_eq!(doc.doc.content[0], BlockNode::HardBreak);
        assert_eq!(
            doc.doc.content[1],
            BlockNode::Paragraph {
                content: Some(vec![InlineNode::HardBreak])
            }
        );
    }
}
