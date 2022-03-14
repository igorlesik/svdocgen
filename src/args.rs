//! Parse `svdocgen` command line arguments.
//!
//! Most of the work is done by <https://docs.rs/clap/latest/clap/>.
//! After the parsing all information is put in to `struct ParsedOptions`.
//!

use clap::{Arg, App/*, SubCommand*/};

/// All configuration options and input info in one place.
///
/// TODO: think to use <https://lib.rs/crates/structopt>
///
pub struct ParsedOptions {
    pub output_dir: String,
    pub inputs: Vec<String>,
    pub includes: Vec<String>,
    pub project_name: String
}

/// Parse command line arguments and return ParsedOptions struct.
///
/// Uses `clap` crate to parse command line arguments.
///
pub fn parse_args() -> ParsedOptions {

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
        .arg(Arg::with_name("INPUT")
            .help("Set the input file or directory")
            .required(true)
            .multiple(true)
            .index(1))
        .arg(Arg::with_name("include")
            .short("i")
            .long("include")
            .help("Include path where input files and directories are located")
            .required(false)
            .takes_value(true)
            .multiple(true)
            .number_of_values(1))
        .arg(Arg::with_name("project-name")
            .long("project-name")
            .takes_value(true)
            .help("Project name string."))
        .get_matches();

    let output_dir = matches.value_of("output").unwrap_or("svdoc");

    let inputs: Vec<&str> = matches.values_of("INPUT").unwrap().collect();

    let includes: Vec<&str> = match matches.values_of("include") {
        Some(values) => values.collect(),
        None => Vec::new(),
    };

    let project_name = matches.value_of("project-name").unwrap_or("");

    ParsedOptions {
        output_dir: String::from(output_dir),
        inputs: inputs.iter().map(|&x| String::from(x)).collect(),
        includes: includes.iter().map(|&x| String::from(x)).collect(),
        project_name: String::from(project_name)
    }
}