use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = box2markdown::cli::Cli::parse();

    match cli.command {
        box2markdown::cli::Commands::ToMd { .. } => {
            todo!("Phase 3: boxnote_to_md")
        }
        box2markdown::cli::Commands::FromHtml { .. } => {
            todo!("Phase 4: html_to_md")
        }
        box2markdown::cli::Commands::ToHtml { .. } => {
            todo!("Phase 5: md_to_html")
        }
        box2markdown::cli::Commands::ToBoxnote { .. } => {
            todo!("Phase 6: md_to_boxnote")
        }
    }
}
