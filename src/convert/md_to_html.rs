use anyhow::Result;
use comrak::{markdown_to_html, Options};

pub fn convert(markdown: &str) -> Result<String> {
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.render.r#unsafe = true;

    let html = markdown_to_html(markdown, &options);
    Ok(html)
}
