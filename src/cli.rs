use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Puzzle file path
    #[arg(short, long)]
    pub path: String,

    /// Maximum iterations before giving up
    #[arg(short, long, default_value_t = 100)]
    pub max_iterations: u32,

    /// Enable debug logging
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}