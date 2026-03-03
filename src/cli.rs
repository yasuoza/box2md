use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "box2md",
    version = env!("BOX2MD_VERSION"),
    about = "Convert Box Notes to Markdown and vice versa"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Convert Box Note (JSON file or clipboard HTML) to Markdown
    ToMd {
        #[command(flatten)]
        args: ToMdArgs,
    },
    /// Convert Markdown to GFM-compatible HTML
    ToHtml {
        #[command(flatten)]
        args: ToHtmlArgs,
    },
}

#[derive(Debug, Args)]
pub struct ToMdArgs {
    /// Box Note JSON file (.boxnote) to convert
    #[arg(short, long, conflicts_with = "from_clipboard")]
    pub input: Option<PathBuf>,
    /// Output Markdown file (default: stdout)
    #[arg(short, long, conflicts_with = "copy")]
    pub output: Option<PathBuf>,
    /// Read HTML from clipboard (as copied from Box Note in browser)
    #[arg(short = 'p', long)]
    pub from_clipboard: bool,
    /// Write Markdown result to clipboard
    #[arg(short, long)]
    pub copy: bool,
}

#[derive(Debug, Args)]
pub struct ToHtmlArgs {
    /// Markdown file to convert
    #[arg(short, long, conflicts_with = "from_clipboard")]
    pub input: Option<PathBuf>,
    /// Output HTML file (default: stdout)
    #[arg(short, long, conflicts_with = "copy")]
    pub output: Option<PathBuf>,
    /// Read Markdown from clipboard
    #[arg(short = 'p', long)]
    pub from_clipboard: bool,
    /// Write HTML result to clipboard as rich text
    #[arg(short, long)]
    pub copy: bool,
}
