use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "box2md")]
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
    ToHtml {
        #[command(flatten)]
        args: ToHtmlArgs,
    },
}

#[derive(Debug, Args)]
pub struct ToMdArgs {
    #[arg(short, long, conflicts_with = "paste")]
    pub input: Option<PathBuf>,
    #[arg(short, long, conflicts_with = "copy")]
    pub output: Option<PathBuf>,
    #[arg(long)]
    pub paste: bool,
    #[arg(long)]
    pub copy: bool,
}

#[derive(Debug, Args)]
pub struct ToHtmlArgs {
    #[arg(short, long, conflicts_with = "paste")]
    pub input: Option<PathBuf>,
    #[arg(short, long, conflicts_with = "copy")]
    pub output: Option<PathBuf>,
    #[arg(long)]
    pub paste: bool,
    #[arg(long)]
    pub copy: bool,
}
