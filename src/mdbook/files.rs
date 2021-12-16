//! Parse SV project directory and collect SV files.
//!

//use std::fs;
use std::path;
use std::path::{Path, PathBuf};

use crate::args;

/// File System node as File or Directory.
///
pub struct FsNode {
    name: String,
    children: Vec<FsNode>
}

impl FsNode {
    fn push(&mut self, path: &PathBuf) {
        let mut node: &mut FsNode = self;
        for component in path.components() {
            //println!("component {:?}", component);
            match component {
                path::Component::Normal(_) => {
                    //println!("normal component {:?}", component);
                    let name = component.as_os_str().to_string_lossy();
                    let pos = node.children.iter().position(|child| child.name.eq(&name));
                    node = match pos {
                        Some(pos) => node.children.get_mut(pos).unwrap(),
                        None => {
                            let new_node = FsNode {name: name.to_string(), children: Vec::new()};
                            node.children.push(new_node);
                            node.children.last_mut().unwrap()
                        },
                    }
                },
                _ => (),
            }
        }
    }

    fn exists(&self, path: &PathBuf) -> bool {
        let mut node: &FsNode = self;
        for component in path.components() {
            //println!("component {:?}", component);
            match component {
                path::Component::Normal(_) => {
                    //println!("normal component {:?}", component);
                    let name = component.as_os_str().to_string_lossy();
                    let child = node.children.iter().find(|child| child.name.eq(&name));
                    node = match child {
                        Some(existing_node) => existing_node,
                        None => return false,
                    }
                },
                _ => (),
            }
        }
        true
    }
}

/// Info about user's source files.
///
pub struct SrcFiles {
    //pub roots: Vec<String>,
    pub nodes: FsNode,
}

pub struct DstFiles {

}

/// Data about all the files.
///
pub struct Files {
    pub src: SrcFiles,
    pub dst: DstFiles,
}

/// Collect info about user's source files.
///
pub fn collect_sources(options: &args::ParsedOptions) -> Result<SrcFiles,String> {

    let mut inputs: Vec<PathBuf> = Vec::new();

    // Create vector of valid/existing input files|dirs.
    for input in &options.inputs {
        println!("input: {}", input);
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

    let /*mut*/ src = SrcFiles {
        //roots: options.inputs.clone(),
        nodes: nodes, //Vec::new()
    };

    Ok(src)
}