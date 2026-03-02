use box2md::boxnote::BoxNoteDocument;
use box2md::convert::boxnote_to_md;
use std::time::Instant;

#[test]
fn convert_100_line_boxnote_within_one_second() {
    let mut content = Vec::new();
    for i in 0..100 {
        content.push(serde_json::json!({
            "type": "paragraph",
            "content": [{"type": "text", "text": format!("Line {} with some content to make it realistic", i)}]
        }));
    }
    let boxnote = serde_json::json!({
        "version": 1,
        "schema_version": 1,
        "doc": {
            "type": "doc",
            "content": content
        }
    });
    let json_str = serde_json::to_string(&boxnote).unwrap();
    let doc: BoxNoteDocument = serde_json::from_str(&json_str).unwrap();

    let start = Instant::now();
    let _md = boxnote_to_md::convert(&doc).unwrap();
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_secs() < 1,
        "Conversion took {:?}, expected less than 1 second",
        elapsed
    );
}
