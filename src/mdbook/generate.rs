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

    create_book_toml(&options.output_dir, &options.project_name)?;

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

    text_buf.push("\n- [User's Documentation]()\n".to_string());
    let users_md_docs = list_users_md_docs(mdbook_src_dir, src_files)?;
    for users_doc in &users_md_docs {
        text_buf.push(format!("{:indent$}- [{}]({})\n", " ",
            users_doc.1, users_doc.2, indent=users_doc.0*2));
    }

    text_buf.push("\n---\n\n".to_string());

    let mut svtext = create_sv_docs(mdbook_src_dir, &src_files)?;
    text_buf.append(&mut svtext);

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
fn create_book_toml(path: &str, project_name: &str) -> Result<(),String> {

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

    let data = format!(r#"
[book]
title = "{}"
authors = ["Godzilla"]

[output.html]
additional-js = ["loadwavedrom.js"]

# cargo install mdbook-linkcheck
# [output.linkcheck]  # enable the "mdbook-linkcheck" renderer

"#, project_name);

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

            let mut ext = target.extension().unwrap().to_os_string();
            if ext.eq("v") || ext.eq("sv") {
                ext.push(".md");
                let mut sv_md = target.clone();
                sv_md.set_extension(ext);
                println!("generate {:?}", sv_md);
                let fname_str = fname.to_str().unwrap();
                let txt = format!(
                    "## {}\n\n```verilog\n{{{{#include {}}}}}\n```\n",
                    fname_str, fname_str);
                fs::write(&sv_md, txt).expect("failed to create file");
            }
        }}
    };

    files.nodes.traverse_top(&mut copy_files);

    Ok(())
}

/// Create files.md file that lists all input files.
///
///
fn create_files_md(path: &str, files: &FsNode) -> Result<String,String> {

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
        let src_path = Path::new("src").join(path);
        let path_str = src_path.to_str().unwrap();
        let mut path_str_md = String::from(path_str);
        path_str_md.push_str(".md");
        data.push(format!("- [{}]({})\n", path, path_str_md));

        //data.push(format!("\n```verilog\n"));
        //data.push(format!("{{{{#include {}}}}}\n", src_path.to_str().unwrap()));
        //data.push(format!("```\n\n"));
    }

    fn add_to_list(list: &mut String, path: &str) {
        let src_path = Path::new("src").join(path);
        let path_str = src_path.to_str().unwrap();
        let mut path_str_md = String::from(path_str);
        path_str_md.push_str(".md");
        list.push_str(format!("  - [{}]({})\n", path, path_str_md).as_str());
    }

    let mut list = String::new();

    let mut data: Vec<String> = Vec::new();
    data.push("# Files\n\n".to_string());

    let mut/*env*/ print_files = |_node: &FsNode, path: &PathBuf, _level: usize| {
        if path.is_file() {
            if let Some(path_str) = path.to_str() {
                //data.push(format!("- {}\n", path_str));
                show_src(&mut data, path_str);
                add_to_list(&mut list, path_str);
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

    Ok(list)
}

/// Create modules.md file that lists all input files.
///
///
fn create_modules_md(
    output_path: &str,
    files: &FsNode
) -> Result<Vec<(String,String,String)>,String>
{

    let fname = Path::new(&output_path).join("modules.md");
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

    fn print_module_info(output_path: &str, path: &str)
    -> (Vec<String>, Vec<(String,String,String)>)
    {
        mdbook::svmodule::generate_sv_module_info(output_path, path)
    }

    let mut list_of_modules: Vec<(String,String,String)> = Vec::new();

    let mut text: Vec<String> = Vec::new();
    text.push("# Modules\n\n".to_string());

    let mut/*env*/ print_modules = |_node: &FsNode, path: &PathBuf, _level: usize| {
        if path.is_file() {
            if let Some(path_str) = path.to_str() {
                let (mut new_text_chunk, mut new_mod_chunk) = print_module_info(output_path, path_str);
                text.append(&mut new_text_chunk);
                list_of_modules.append(&mut new_mod_chunk);
            }
        }
    };

    files.traverse_top(&mut print_modules);

    let mut writer = BufWriter::new(file);

    for t in &text {
        match writer.write_all(t.as_bytes()) {
            Err(e) => return Err(e.to_string()),
            Ok(_) => (),
        }
    }

    Ok(list_of_modules)
}

fn create_ifaces_md(
    output_path: &str,
    files: &FsNode
) -> Result<Vec<(String,String,String)>,String>
{

    let fname = Path::new(&output_path).join("ifaces.md");
    let fname = fname.to_str().unwrap();

    let file = match fs::OpenOptions::new()
        .read(false).write(true).create(true).truncate(true)
        .open(fname) {
            Err(e) => return Err(e.to_string()),
            Ok(file) => file,
    };

    fn print_iface_info(output_path: &str, path: &str)
    -> (Vec<String>, Vec<(String,String,String)>)
    {
        mdbook::sviface::generate_sv_interface_info(output_path, path)
    }

    let mut list_of_ifaces: Vec<(String,String,String)> = Vec::new();

    let mut text: Vec<String> = Vec::new();
    text.push("# Interfaces\n\n".to_string());

    let mut/*env*/ print_ifaces = |_node: &FsNode, path: &PathBuf, _level: usize| {
        if path.is_file() {
            if let Some(path_str) = path.to_str() {
                let (mut new_text_chunk, mut new_iface_chunk) = print_iface_info(output_path, path_str);
                text.append(&mut new_text_chunk);
                list_of_ifaces.append(&mut new_iface_chunk);
            }
        }
    };

    files.traverse_top(&mut print_ifaces);

    let mut writer = BufWriter::new(file);

    for t in &text {
        match writer.write_all(t.as_bytes()) {
            Err(e) => return Err(e.to_string()),
            Ok(_) => (),
        }
    }

    Ok(list_of_ifaces)
}

fn create_classes_md(
    output_path: &str,
    files: &FsNode
) -> Result<Vec<(String,String,String)>,String>
{

    let fname = Path::new(&output_path).join("classes.md");
    let fname = fname.to_str().unwrap();

    let file = match fs::OpenOptions::new()
        .read(false).write(true).create(true).truncate(true)
        .open(fname) {
            Err(e) => return Err(e.to_string()),
            Ok(file) => file,
    };

    fn print_class_info(output_path: &str, path: &str)
    -> (Vec<String>, Vec<(String,String,String)>)
    {
        mdbook::svclass::generate_sv_class_info(output_path, path)
    }

    let mut list_of_classes: Vec<(String,String,String)> = Vec::new();

    let mut text: Vec<String> = Vec::new();
    text.push("# Classes\n\n".to_string());

    let mut/*env*/ print_classes = |_node: &FsNode, path: &PathBuf, _level: usize| {
        if path.is_file() {
            if let Some(path_str) = path.to_str() {
                let (mut new_text_chunk, mut new_class_chunk) = print_class_info(output_path, path_str);
                text.append(&mut new_text_chunk);
                list_of_classes.append(&mut new_class_chunk);
            }
        }
    };

    files.traverse_top(&mut print_classes);

    let mut writer = BufWriter::new(file);

    for t in &text {
        match writer.write_all(t.as_bytes()) {
            Err(e) => return Err(e.to_string()),
            Ok(_) => (),
        }
    }

    Ok(list_of_classes)
}

fn create_packages_md(
    output_path: &str,
    files: &FsNode
) -> Result<Vec<(String,String,String)>,String>
{

    let fname = Path::new(&output_path).join("packages.md");
    let fname = fname.to_str().unwrap();

    let file = match fs::OpenOptions::new()
        .read(false).write(true).create(true).truncate(true)
        .open(fname) {
            Err(e) => return Err(e.to_string()),
            Ok(file) => file,
    };

    fn print_pkg_info(output_path: &str, path: &str)
    -> (Vec<String>, Vec<(String,String,String)>)
    {
        mdbook::svpkg::generate_sv_package_info(output_path, path)
    }

    let mut list_of_pkgs: Vec<(String,String,String)> = Vec::new();

    let mut text: Vec<String> = Vec::new();
    text.push("# Packages\n\n".to_string());

    let mut/*env*/ print_pkgs = |_node: &FsNode, path: &PathBuf, _level: usize| {
        if path.is_file() {
            if let Some(path_str) = path.to_str() {
                let (mut new_text_chunk, mut new_pkg_chunk) = print_pkg_info(output_path, path_str);
                text.append(&mut new_text_chunk);
                list_of_pkgs.append(&mut new_pkg_chunk);
            }
        }
    };

    files.traverse_top(&mut print_pkgs);

    let mut writer = BufWriter::new(file);

    for t in &text {
        match writer.write_all(t.as_bytes()) {
            Err(e) => return Err(e.to_string()),
            Ok(_) => (),
        }
    }

    Ok(list_of_pkgs)
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

    let file_list = create_files_md(mdbook_src_dir, &sv_files)?;
    text_buf.push("- [Files](files.md)\n".to_string());
    text_buf.push(file_list);

    let mut module_list = create_modules_md(mdbook_src_dir, &sv_files)?;
    module_list.sort();
    text_buf.push("- [Modules](modules.md)\n".to_string());
    for module in &module_list {
        text_buf.push(format!("  - [`{}`  :{}]({})\n", module.0, module.1, module.2));
    }

    let mut pkg_list = create_packages_md(mdbook_src_dir, &sv_files)?;
    pkg_list.sort();
    text_buf.push("- [Packages](packages.md)\n".to_string());
    for pkg in &pkg_list {
        text_buf.push(format!("  - [`{}`  :{}]({})\n", pkg.0, pkg.1, pkg.2));
    }

    let mut iface_list = create_ifaces_md(mdbook_src_dir, &sv_files)?;
    iface_list.sort();
    text_buf.push("- [Interfaces](ifaces.md)\n".to_string());
    for iface in &iface_list {
        text_buf.push(format!("  - [`{}`  :{}]({})\n", iface.0, iface.1, iface.2));
    }

    let mut class_list = create_classes_md(mdbook_src_dir, &sv_files)?;
    class_list.sort();
    text_buf.push("- [Classes](classes.md)\n".to_string());
    for class in &class_list {
        text_buf.push(format!("  - [`{}`  :{}]({})\n", class.0, class.1, class.2));
    }

    text_buf.push("- [Functions]()\n".to_string());

    Ok(text_buf)
}
