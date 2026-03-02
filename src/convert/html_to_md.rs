use anyhow::Result;

pub fn convert(html: &str) -> Result<String> {
    let html = html.replace("<del>", "~~").replace("</del>", "~~");
    let converter = htmd::HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style"])
        .build();
    let md = converter.convert(&html)?;
    Ok(normalize_output(&md))
}

fn normalize_output(md: &str) -> String {
    let trimmed = md.trim_end();
    if trimmed.is_empty() {
        String::new()
    } else {
        format!("{trimmed}\n")
    }
}
