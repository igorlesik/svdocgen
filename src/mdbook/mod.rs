//! Parse SV files and generate documentation in mdBook format.
//!
//! Most of the work is done by <https://rust-lang.github.io/mdBook/index.html>.

pub mod files;    // collect SV files
pub mod generate; // generate mdBook source files
pub mod svmodule; // generate md file with SV module info
pub mod build;    // build mdBook
