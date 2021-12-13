//! Parse `svdocgen` command line arguments.
//!
//! Most of the work is done by <https://docs.rs/clap/latest/clap/>.

use clap::{Arg, App/*, SubCommand*/};

/// All configuration options in one place.
///
pub struct ParsedOptions {
    pub output_dir: String,
}

/// Parse command line arguments and return ParsedOptions struct.
///
/// Uses `clap` crate to parse command line arguments.
///
pub fn parse_args() -> ParsedOptions {
    println!("i parse args");
    let matches = App::new("SystemVerilog Documentation Generator")
        .version("0.1.0")
        .author("Igor Lesik <xxx@xxx.com>")
        .about("Finds .sv and .md files in SV project directory and generates documentation.")
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("DIR")
            .takes_value(true)
            .help("Set output directory for generated artifacts."))
        .get_matches();

    let output_dir = matches.value_of("output").unwrap_or("svdoc");

    let options = ParsedOptions {
        output_dir: String::from(output_dir),
    };

    return options;
}