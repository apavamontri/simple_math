#![warn(missing_docs)]

//! This is the entry point for the command-line interface (CLI)
use simple_math::operations::addition::add;

/// This is the main try point for the `cli` executable
pub fn main() {
    println!("CLI: 2 + 2 is {}", add(2, 2));
}
