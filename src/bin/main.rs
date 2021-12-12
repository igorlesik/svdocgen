

//extern crate svdocgen;

use svdocgen;
use svdocgen::mdbook::generate as generator;

#[svgbobdoc::transform]
///
///
/// ```svgbob
///     .--.---.
/// SV  |#  \_ | DOC
/// o-->||__(_)|*-->
///     |   \ \|
///     '----'-'
/// ```
fn main() {
    println!("Hello, world!");
    svdocgen::args::parse_args();
    generator::generate();
}
