use anyhow::{Context, Result};
use arboard::Clipboard;

pub fn read_html() -> Result<Option<String>> {
    let mut clipboard = Clipboard::new().context("failed to access clipboard")?;
    match clipboard.get().html() {
        Ok(html) => Ok(Some(html)),
        Err(arboard::Error::ContentNotAvailable) => Ok(None),
        Err(err) => Err(err).context("failed to read HTML from clipboard"),
    }
}

pub fn read_text() -> Result<Option<String>> {
    let mut clipboard = Clipboard::new().context("failed to access clipboard")?;
    match clipboard.get().text() {
        Ok(text) => Ok(Some(text)),
        Err(arboard::Error::ContentNotAvailable) => Ok(None),
        Err(err) => Err(err).context("failed to read text from clipboard"),
    }
}

pub fn write_text(text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new().context("failed to access clipboard")?;
    clipboard
        .set()
        .text(text)
        .context("failed to write text to clipboard")
}

pub fn write_html(html: &str, alt_text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new().context("failed to access clipboard")?;
    clipboard
        .set()
        .html(html, Some(alt_text))
        .context("failed to write HTML to clipboard")
}
