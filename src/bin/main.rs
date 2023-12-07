#![warn(missing_docs)]

//! This is the entry point for the regular executable
use simple_math::operations::addition::add;

/// This is the entry point for the `main` executable
pub fn main() {
    println!("main: 2 + 2 is {}", add(2, 2));
}
