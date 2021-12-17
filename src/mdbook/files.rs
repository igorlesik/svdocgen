//! Parse SV project directory and collect SV files.
//!

use std::fs;
//use std::path;
use std::path::{Path, PathBuf};
use std::io;

use crate::args;
use crate::fsnode::FsNode;


/// Info about user's source files.
///
pub struct SrcFiles {
    pub nodes: FsNode,
}

//pub struct DstFiles {
//
//}

/// Data about all the files.
///
//pub struct Files {
//    pub src: SrcFiles,
//    pub dst: DstFiles,
//}

/// Collect info about user's source files.
///
pub fn collect_sources(options: &args::ParsedOptions) -> Result<SrcFiles,String> {

    let mut inputs: Vec<PathBuf> = Vec::new();

    // Create vector of valid/existing input files|dirs.
    for input in &options.inputs {
        //println!("user input: {}", input);
        let path = Path::new(input);
        if !path.exists() {
            let include = options.includes.iter().find(|&x| Path::new(x).join(path).exists());
            match include {
                Some(inc) => inputs.push(Path::new(inc).join(path)),
                None => { println!("Warning: can't find '{}' in {:?}", input, &options.includes);
                          continue; },
            }
        }
        else {
            inputs.push(path.to_path_buf());
        }
    }

    let mut nodes = FsNode {
        name: String::from(""),
        children: Vec::new()
    };

    // Create FsNode from user provided inputs.
    for input in &inputs {
        println!("input path: {:?}", input);
        let is_already_present = nodes.exists(input);
        if is_already_present {
            println!("Warning: duplicate {:?}", input);
        }
        else {
            nodes.push(input);
        }
    }

    let mut nodes_with_files = nodes.clone();

    // Collect .sv and .md in the input directories.
    let mut/*env*/ collect_files = |node: &FsNode, path: &PathBuf| {
        println!("traverse {}: {:?}", node.name, path);
        if path.is_dir() && node.children.is_empty() {
            println!("checking for files in {:?}", path);
            match visit_dir_and_search_files(&mut nodes_with_files, path) {
                Err(_) => println!("error"),
                _ => (),
            }
        }
    };

    nodes.traverse(&mut PathBuf::from(""), &mut collect_files);

    let src = SrcFiles {
        nodes: nodes_with_files,
    };

    Ok(src)
}

fn visit_dir_and_search_files(nodes: &mut FsNode, dir: &Path) -> io::Result<()> {

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dir_and_search_files(nodes, &path)?;
            } else {
                if let Some(ext) = path.extension() {
                    if ext.eq("sv") || ext.eq("v") || ext.eq("md") {
                        println!("add {:?}", path);
                        nodes.push(&path);
                    }
                }
            }
        }
    }

    Ok(())
}