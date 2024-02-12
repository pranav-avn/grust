struct Cli {
    pattern: String,
    path: std::path::PathBuf, //multi-plat Path Handling
}
fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path provided");

    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };
    println!("pattern: {:?}, path: {:?}", pattern, path);
}
