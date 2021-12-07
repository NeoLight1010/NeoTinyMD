mod compiler;
mod cli;

use cli::print_long_banner;
use compiler::parse_markdown_file;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => print_long_banner(),
    }
}
