use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "box2markdown")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    ToMd {
        #[command(flatten)]
        args: ToMdArgs,
    },
    FromHtml {
        #[command(flatten)]
        args: FromHtmlArgs,
    },
    ToHtml {
        #[command(flatten)]
        args: ToHtmlArgs,
    },
    ToBoxnote {
        #[command(flatten)]
        args: ToBoxnoteArgs,
    },
}

#[derive(Debug, Args)]
pub struct ToMdArgs {
    #[arg(short, long)]
    pub input: Option<PathBuf>,
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct FromHtmlArgs {
    #[arg(short, long, conflicts_with = "paste")]
    pub input: Option<PathBuf>,
    #[arg(short, long)]
    pub output: Option<PathBuf>,
    #[arg(long)]
    pub paste: bool,
}

#[derive(Debug, Args)]
pub struct ToHtmlArgs {
    #[arg(short, long)]
    pub input: Option<PathBuf>,
    #[arg(short, long, conflicts_with = "copy")]
    pub output: Option<PathBuf>,
    #[arg(long)]
    pub copy: bool,
}

#[derive(Debug, Args)]
pub struct ToBoxnoteArgs {
    #[arg(short, long)]
    pub input: Option<PathBuf>,
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}
