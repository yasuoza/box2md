use anyhow::{Context, Result};
use clap::Parser;

fn main() -> Result<()> {
    let cli = box2markdown::cli::Cli::parse();

    match cli.command {
        box2markdown::cli::Commands::ToMd { args } => {
            let input = box2markdown::io::read_input(args.input.as_deref())?;
            let doc: box2markdown::boxnote::BoxNoteDocument =
                serde_json::from_str(&input).context("failed to parse Box Note JSON")?;
            doc.doc.validate().map_err(anyhow::Error::msg)?;
            let md = box2markdown::convert::boxnote_to_md::convert(&doc)?;
            box2markdown::io::write_output(args.output.as_deref(), &md)?;
            Ok(())
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
