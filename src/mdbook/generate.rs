//! Parse SV files and generate mdBook sources.
//!
//!
//!

use crate::args;
use std::fs;
use std::path::Path;
use std::io::{BufWriter, Write};


const MDBOOK_SRC_DIR: &str = "src";
const MDBOOK_SUMMARY_MD: &str = "SUMMARY.md";

#[svgbobdoc::transform]
/// Generate mdBook sources.
///
/// ```svgbob
///     .--.---.
/// SV  |#  \_ | DOC
/// o-->||__(_)|*-->
///     |   \ \|
///     '----'-'
/// ```
pub fn generate(options: &args::ParsedOptions) -> Result<(),String> {


    match fs::create_dir_all(&options.output_dir) {
        Err(e) => { println!("Can't create output directory '{}' error: {}",
                        &options.output_dir, e);
                    return Err(e.to_string()); },
        Ok(_) => println!("Created output directory '{}'", &options.output_dir),
    }

    let mdbook_src_dir = Path::new(&options.output_dir).join(MDBOOK_SRC_DIR);
    let mdbook_src_dir = mdbook_src_dir.to_str().unwrap();

    match fs::create_dir_all(mdbook_src_dir) {
        Err(e) => { println!("Can't create '{}' error: {}",
                        mdbook_src_dir, e);
                    return Err(e.to_string()); },
        Ok(_) => println!("Created directory '{}'", mdbook_src_dir),
    }

    create_summary_md(mdbook_src_dir)?;

    Ok(())
}

/// Create mdBook SUMMARY.md file.
///
/// The summary file is used by mdBook to know what chapters to include,
/// in what order they should appear, what their hierarchy is
/// and where the source files are. Without this file, there is no book.
///
fn create_summary_md(path: &str) -> Result<(),String> {

    let summary_fname = Path::new(&path).join(MDBOOK_SUMMARY_MD);
    let summary_fname = summary_fname.to_str().unwrap();

    let file = match fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(summary_fname) {
            Err(e) => return Err(e.to_string()),
            Ok(file) => file,
    };

    let data = "# Summary\n- [test](SUMMARY.md)";

    let mut writer = BufWriter::new(file);

    match writer.write_all(data.as_bytes()) {
        Err(e) => return Err(e.to_string()),
        Ok(_) => (),
    }

    Ok(())
}