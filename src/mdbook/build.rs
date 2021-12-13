//! Build mdBook.
//!
//!
//!

use crate::args;
use std::fs;
use std::path::Path;
use mdbook::MDBook;

// See <https://github.com/rust-lang/mdBook#usage>
const MDBOOK_BOOK_DIR: &str = "book";

#[svgbobdoc::transform]
/// Build mdBook from mdBook sources.
///
/// ```svgbob
///     .--.---.
/// SV  |#  \_ | DOC
/// o-->||__(_)|*-->
///     |   \ \|
///     '----'-'
/// ```
pub fn build(options: &args::ParsedOptions) -> Result<(),String> {


    let mdbook_book_dir = Path::new(&options.output_dir).join(MDBOOK_BOOK_DIR);
    let mdbook_book_dir = mdbook_book_dir.to_str().unwrap();

    match fs::create_dir_all(mdbook_book_dir) {
        Err(e) => { println!("Can't create '{}' error: {}",
                        mdbook_book_dir, e);
                    return Err(e.to_string()); },
        Ok(_) => println!("Created directory '{}'", mdbook_book_dir),
    }


    let mut md = MDBook::load(&options.output_dir).expect("Unable to load the book");

    md.build().expect("Building failed");

    Ok(())
}