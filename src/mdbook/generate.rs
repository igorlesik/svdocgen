//! Parse SV files and generate mdBook sources.
//!
//!
//!

use std::fs;
use std::path::{Path, PathBuf};
use std::io::{BufWriter, Write};

use crate::args;
use crate::mdbook;
use crate::mdbook::files::SrcFiles;
use crate::fsnode::FsNode;

const MDBOOK_SRC_DIR: &str = "src";
const MDBOOK_SUMMARY_MD: &str = "SUMMARY.md";
const MDBOOK_BOOK_TOML: &str = "book.toml";

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

    let src_files = mdbook::files::collect_sources(options)?;

    copy_src_files(mdbook_src_dir, &src_files)?;

    create_summary_md(mdbook_src_dir, &src_files)?;

    create_book_toml(&options.output_dir)?;

    Ok(())
}

/// Create mdBook SUMMARY.md file.
///
/// The summary file is used by mdBook to know what chapters to include,
/// in what order they should appear, what their hierarchy is
/// and where the source files are. Without this file, there is no book.
///
fn create_summary_md(mdbook_src_dir: &str, src_files: &SrcFiles) -> Result<(),String> {

    let summary_fname = Path::new(&mdbook_src_dir).join(MDBOOK_SUMMARY_MD);
    let summary_fname = summary_fname.to_str().unwrap();

    let file = match fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(summary_fname) {
            Err(e) => return Err(e.to_string()),
            Ok(file) => file,
    };

    let mut text_buf: Vec<String> = Vec::new();
    text_buf.push("# Summary\n".to_string());
    let mut svtext = create_sv_docs(mdbook_src_dir, &src_files)?;
    text_buf.append(&mut svtext);

    text_buf.push("\n---\n\n- [User's Documentation]()\n".to_string());
    let users_md_docs = list_users_md_docs(mdbook_src_dir, src_files)?;
    for users_doc in &users_md_docs {
        text_buf.push(format!("{:indent$}- [{}]({})\n", " ",
            users_doc.1, users_doc.2, indent=users_doc.0*2));
    }

    let mut writer = BufWriter::new(file);

    for text in &text_buf {
        match writer.write_all(text.as_bytes()) {
            Err(e) => return Err(e.to_string()),
            Ok(_) => (),
        }
    }

    Ok(())
}

/// Create mdBook book.toml file.
///
/// The `book.toml` file is used by mdBook to know the configuration.
///
fn create_book_toml(path: &str) -> Result<(),String> {

    let book_toml_fname = Path::new(&path).join(MDBOOK_BOOK_TOML);
    let book_toml_fname = book_toml_fname.to_str().unwrap();

    let file = match fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(book_toml_fname) {
            Err(e) => return Err(e.to_string()),
            Ok(file) => file,
    };

    let data = r#"
[book]
title = "Documentation: Project X"
authors = ["Godzilla"]

[output.html]

# cargo install mdbook-linkcheck
# [output.linkcheck]  # enable the "mdbook-linkcheck" renderer

"#;

    let mut writer = BufWriter::new(file);

    match writer.write_all(data.as_bytes()) {
        Err(e) => return Err(e.to_string()),
        Ok(_) => (),
    }

    Ok(())
}

/// Copy all input files into mdBook `src` directory.
///
///
fn copy_src_files(path: &str, files: &SrcFiles) -> Result<(),String> {

    let target_dir = Path::new(&path).join("src");
    let target_dir = target_dir.to_str().unwrap();

    match fs::create_dir_all(target_dir) {
        Err(e) => return Err(e.to_string()),
        Ok(_) => (),
    }

    let mut/*env*/ create_dirs = |_node: &FsNode, path: &PathBuf, _level: usize| {
        if path.is_dir() {
            if let Some(path_str) = path.to_str() {
                if let Some(target_str) = Path::new(&target_dir).join(&path_str).to_str() {
                    match fs::create_dir_all(target_str) {
                        Err(e) => println!("error {}", e.to_string()),
                        Ok(_) => println!("create dir: {}", target_str),
                    }
                }
            }
        }
    };

    files.nodes.traverse_top(&mut create_dirs);

    let mut/*env*/ copy_files = |_node: &FsNode, path: &PathBuf, _level: usize| {
        if path.is_file() { if  let Some(fname) = path.file_name() {
            let target = if let Some(parent) = path.parent() {
                Path::new(&target_dir).join(parent).join(fname)
            }
            else {
                Path::new(&target_dir).join(fname)
            };
            match fs::copy(&path, &target) {
                Err(e) => println!("error {:?} copying {:?} {:?}", e, &path, &target),
                Ok(nr_bytes) => println!("copied {} bytes from {:?} to {:?}", nr_bytes, &path, &target),
            }
        }}
    };

    files.nodes.traverse_top(&mut copy_files);

    Ok(())
}

/// Create files.md file that lists all input files.
///
///
fn create_files_md(path: &str, files: &FsNode) -> Result<(),String> {

    let fname = Path::new(&path).join("files.md");
    let fname = fname.to_str().unwrap();

    let file = match fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(fname) {
            Err(e) => return Err(e.to_string()),
            Ok(file) => file,
    };

    fn show_src(data: &mut Vec<String>, path: &str) {
        let path = Path::new("src").join(path);
        data.push(format!("\n```verilog\n"));
        data.push(format!("{{{{#rustdoc_include {}::10}}}}\n", path.to_str().unwrap()));
        data.push(format!("```\n\n"));
    }

    let mut data: Vec<String> = Vec::new();
    data.push("# Files\n\n".to_string());

    let mut/*env*/ print_files = |_node: &FsNode, path: &PathBuf, _level: usize| {
        if path.is_file() {
            if let Some(path_str) = path.to_str() {
                data.push(format!("- {}\n", path_str));
                show_src(&mut data, path_str);
            }
        }
    };

    files.traverse_top(&mut print_files);

    let mut writer = BufWriter::new(file);

    for d in &data {
        match writer.write_all(d.as_bytes()) {
            Err(e) => return Err(e.to_string()),
            Ok(_) => (),
        }
    }

    Ok(())
}

fn list_users_md_docs(
    _mdbook_src_dir: &str,
    src_files: &SrcFiles
) -> Result<Vec<(usize,String,String)>,String>
{
    let md_files = mdbook::files::get_md_files(&src_files.nodes)?;

    let mut list: Vec<(usize, String, String)> = Vec::new();

    let mut/*env*/ list_md_files = |node: &FsNode, path: &PathBuf, level: usize| {
        if path.is_file() {
            let mdbook_path = Path::new("src").join(path);
            let mdbook_path_str = mdbook_path.to_str().unwrap_or("");
            list.push((level, node.name.clone(), mdbook_path_str.to_string()));
        }
        else {
            list.push((level, node.name.clone(), "".to_string()));
        }
    };

    md_files.traverse(&mut PathBuf::from(""), 1, &mut list_md_files);

    Ok(list)
}

fn create_sv_docs(
    mdbook_src_dir: &str,
    all_files: &SrcFiles
) -> Result<Vec<String>,String>
{

    let sv_files = mdbook::files::get_sv_files(&all_files.nodes)?;

    let mut text_buf: Vec<String> = Vec::new();

    create_files_md(mdbook_src_dir, &sv_files)?;
    text_buf.push("- [Files](files.md)\n".to_string());

    text_buf.push("- [Modules]()\n".to_string());
    text_buf.push("- [Functions]()\n".to_string());
    text_buf.push("- [Packages]()\n".to_string());
    text_buf.push("- [Interfaces]()\n".to_string());
    text_buf.push("- [Classes]()\n".to_string());

    Ok(text_buf)
}
