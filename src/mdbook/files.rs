//! Parse SV project directory and collect SV files.
//!

//use std::fs;
//use std::path;
use std::path::{Path, PathBuf};

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
        //println!("input: {}", input);
        let path = Path::new(input);
        if !path.exists() {
            let include = options.includes.iter().find(|&x| Path::new(x).join(path).exists());
            match include {
                Some(inc) => inputs.push(Path::new(inc).join(path)),
                None => { println!("Warning: can't find '{}' in {:?}", input, &options.includes);
                          continue; },
            }
        }
    }

    let mut nodes = FsNode {
        name: String::from(""),
        children: Vec::new()
    };

    // Create FsNode with roots
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

    // Collect .sv and .md in root directories
    fn find_dirs_and_collect_files(node: &FsNode, path: &PathBuf) {
        println!("traverse {}: {:?}", node.name, path);
    }

    nodes.traverse(&mut PathBuf::from(""), find_dirs_and_collect_files);

    let src = SrcFiles {
        nodes: nodes,
    };

    Ok(src)
}