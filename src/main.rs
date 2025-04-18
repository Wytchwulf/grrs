use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Pattern to search for
    pattern: String,
    /// Path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("Could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
