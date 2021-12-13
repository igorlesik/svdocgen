//! SvDocGen executable that reads SV files
//! and generates documentation in mdBook format.
//!
//!

use svdocgen;
use svdocgen::mdbook::generate as generator;
use svdocgen::mdbook::build as builder;


fn main() -> Result<(), i32>{

    let options = svdocgen::args::parse_args();

    match generator::generate(&options) {
        Err(e) => { println!("Error during generation: {}", e);
                    return Err(2); },
        _ => (),
    }

    match builder::build(&options) {
        Err(e) => { println!("Error while building mdBook: {}", e);
                    return Err(3); },
        _ => (),
    }

    Ok(())
}
