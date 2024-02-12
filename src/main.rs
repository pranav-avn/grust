use clap::Parser;

#[derive(Parser)]

struct Cli {
    pattern: String,          //the pattern to look for
    path: std::path::PathBuf, //multi-plat Path Handling
}
fn main() {
    let args = Cli::parse();
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}
