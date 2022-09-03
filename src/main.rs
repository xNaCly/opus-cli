use std::env;

use crate::cli::parse_cli;

mod cli;
mod types;

fn main() {
    parse_cli(env::args().collect());
}
