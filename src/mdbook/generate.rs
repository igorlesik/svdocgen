//! Parse SV files and generate mdBook sources.
//!
//!
//!

use crate::args;
use std::fs;
//use mdbook::MDBook;

pub fn generate(options: &args::ParsedOptions) -> Result<(),String> {


    match fs::create_dir_all(&options.output_dir) {
        Err(e) => { println!("Can't create output directory '{}' error: {}",
                        &options.output_dir, e);
                    return Err("can't create output directory".to_string()); },
        Ok(_) => println!("Created output directory '{}'", &options.output_dir),
    };

    return Ok(());
}