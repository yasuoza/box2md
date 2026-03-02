use anyhow::{Context, Result};
use std::io::{Read, Write};
use std::path::Path;

pub fn read_input(input: Option<&Path>) -> Result<String> {
    match input {
        Some(path) => std::fs::read_to_string(path)
            .with_context(|| format!("failed to read input file: {}", path.display())),
        None => {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .context("failed to read stdin")?;
            Ok(buf)
        }
    }
}

pub fn write_output(output: Option<&Path>, content: &str) -> Result<()> {
    match output {
        Some(path) => std::fs::write(path, content)
            .with_context(|| format!("failed to write output file: {}", path.display())),
        None => {
            let mut stdout = std::io::stdout();
            stdout
                .write_all(content.as_bytes())
                .context("failed to write stdout")?;
            stdout.flush().context("failed to flush stdout")
        }
    }
}
