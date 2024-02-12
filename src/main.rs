use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]

struct Cli {
    pattern: String,          //the pattern to look for
    path: std::path::PathBuf, //multi-plat Path Handling
}
fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file '{}'", args.path.display()))?;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
