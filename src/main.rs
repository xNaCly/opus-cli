use std::env;

use crate::cli::parse_cli;
use crate::types::Cli;

mod cli;
mod types;

fn main() {
    let result: Cli = parse_cli(env::args().collect());
    println!("{}", result.top_level_arg.to_string());
}
