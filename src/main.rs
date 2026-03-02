use anyhow::{Context, Result};
use clap::Parser;

fn main() -> Result<()> {
    let cli = box2markdown::cli::Cli::parse();

    match cli.command {
        box2markdown::cli::Commands::ToMd { args } => {
            let input = if args.paste {
                box2markdown::clipboard::read_text()?
                    .ok_or_else(|| anyhow::anyhow!("no text content in clipboard"))?
            } else {
                box2markdown::io::read_input(args.input.as_deref())?
            };
            let json = input
                .find('{')
                .map(|i| &input[i..])
                .unwrap_or(&input);
            let doc: box2markdown::boxnote::BoxNoteDocument =
                serde_json::from_str(json).context("failed to parse Box Note JSON")?;
            doc.doc.validate().map_err(anyhow::Error::msg)?;
            let md = box2markdown::convert::boxnote_to_md::convert(&doc)?;
            if args.copy {
                box2markdown::clipboard::write_text(&md)?;
                eprintln!("Markdown copied to clipboard");
            } else {
                box2markdown::io::write_output(args.output.as_deref(), &md)?;
            }
            Ok(())
        }
        box2markdown::cli::Commands::FromHtml { args } => {
            let html = if args.paste {
                box2markdown::clipboard::read_html()?
                    .ok_or_else(|| anyhow::anyhow!("no HTML content in clipboard"))?
            } else {
                box2markdown::io::read_input(args.input.as_deref())?
            };
            let md = box2markdown::convert::html_to_md::convert(&html)?;
            if args.copy {
                box2markdown::clipboard::write_text(&md)?;
                eprintln!("Markdown copied to clipboard");
            } else {
                box2markdown::io::write_output(args.output.as_deref(), &md)?;
            }
            Ok(())
        }
        box2markdown::cli::Commands::ToHtml { args } => {
            let md = if args.paste {
                box2markdown::clipboard::read_text()?
                    .ok_or_else(|| anyhow::anyhow!("no text content in clipboard"))?
            } else {
                box2markdown::io::read_input(args.input.as_deref())?
            };
            let html = box2markdown::convert::md_to_html::convert(&md)?;
            if args.copy {
                let plain = html.replace('\n', " ");
                box2markdown::clipboard::write_html(&html, &plain)?;
                eprintln!("HTML copied to clipboard");
            } else {
                box2markdown::io::write_output(args.output.as_deref(), &html)?;
            }
            Ok(())
        }
    }
}
