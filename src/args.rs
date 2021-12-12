//! Parse `svdocgen` command line arguments.
//!
//! Most of the work is done by <https://docs.rs/clap/latest/clap/>.

use clap::{Arg, App/*, SubCommand*/};

/// Use `clap` crate to parse command line arguments.
///
pub fn parse_args() {
    println!("i parse args");
    let _matches = App::new("SystemVerilog Documentation Generator")
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
}