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
//const MDBOOK_SRC_DIR: &str = "src";

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

    //let mdbook_src_dir_p = Path::new(&options.output_dir).join(MDBOOK_SRC_DIR);
    //let mdbook_src_dir = mdbook_book_src_p.to_str().unwrap();

    let mdbook_book_dir_p = Path::new(&options.output_dir).join(MDBOOK_BOOK_DIR);
    let mdbook_book_dir = mdbook_book_dir_p.to_str().unwrap();

    match fs::create_dir_all(mdbook_book_dir) {
        Err(e) => { println!("Can't create '{}' error: {}",
                        mdbook_book_dir, e);
                    return Err(e.to_string()); },
        Ok(_) => println!("Created directory '{}'", mdbook_book_dir),
    }


    let /*mut*/ md = MDBook::load(&options.output_dir).expect("Unable to load the book");

    copy_assets_to_src(&Path::new(&options.output_dir)).expect("failed to copy assets");

    md.build().expect("Building failed");

    copy_assets(&mdbook_book_dir_p).expect("failed to copy assets");

    Ok(())
}

fn copy_assets(book_path: &Path)-> Result<(),String> {

    let asset_highlight_js = include_bytes!("../../assets/js/highlight.js");
    //include_flate::flate!(static asset_highlight_js: str from "../../assets/js/highlight.js");

    //println!("js: {}", String::from_utf8_lossy(asset_highlight_js));

    fs::write(
        book_path.join("highlight.js"),
        asset_highlight_js).expect("Unable to write file");

    Ok(())
}

fn copy_assets_to_src(book_src_path: &Path)-> Result<(),String> {

    let asset_loadwavedrom_js = include_bytes!("../../assets/js/loadwavedrom.js");
    fs::write(
        book_src_path.join("loadwavedrom.js"),
        asset_loadwavedrom_js).expect("Unable to write file");

    Ok(())
}