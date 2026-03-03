use anyhow::{Context, Result};
use clap::Parser;

fn main() -> Result<()> {
    let cli = box2md::cli::Cli::parse();

    if cli.version {
        println!("{}", env!("BOX2MD_VERSION"));
        return Ok(());
    }

    let command = cli.command.unwrap_or_else(|| {
        box2md::cli::Cli::parse_from(["box2md", "--help"]);
        unreachable!()
    });

    match command {
        box2md::cli::Commands::ToMd { args } => {
            let md = if args.from_clipboard {
                let html = box2md::clipboard::read_html()?
                    .ok_or_else(|| anyhow::anyhow!("no HTML content in clipboard"))?;
                box2md::convert::html_to_md::convert(&html)?
            } else {
                let input = box2md::io::read_input(args.input.as_deref())?;
                let json = input.find('{').map(|i| &input[i..]).unwrap_or(&input);
                let doc: box2md::boxnote::BoxNoteDocument =
                    serde_json::from_str(json).context("failed to parse Box Note JSON")?;
                doc.doc.validate().map_err(anyhow::Error::msg)?;
                box2md::convert::boxnote_to_md::convert(&doc)?
            };
            if args.copy {
                box2md::clipboard::write_text(&md)?;
                eprintln!("Markdown copied to clipboard");
            } else {
                box2md::io::write_output(args.output.as_deref(), &md)?;
            }
            Ok(())
        }
        box2md::cli::Commands::ToHtml { args } => {
            let md = if args.from_clipboard {
                box2md::clipboard::read_text()?
                    .ok_or_else(|| anyhow::anyhow!("no text content in clipboard"))?
            } else {
                box2md::io::read_input(args.input.as_deref())?
            };
            let html = box2md::convert::md_to_html::convert(&md)?;
            if args.copy {
                let plain = html.replace('\n', " ");
                box2md::clipboard::write_html(&html, &plain)?;
                eprintln!("HTML copied to clipboard");
            } else {
                box2md::io::write_output(args.output.as_deref(), &html)?;
            }
            Ok(())
        }
    }
}
