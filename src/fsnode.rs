//! Tree structure to keep FS paths.
//!

use std::path;
use std::path::{/*Path,*/ PathBuf};

/// File System node as path to file or directory.
///
pub struct FsNode {
    pub name: String,
    pub children: Vec<FsNode>,
}

impl FsNode {

    pub fn push(&mut self, path: &PathBuf) {
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

    pub fn exists(&self, path: &PathBuf) -> bool {
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

    pub fn traverse(&self, parent_path: &mut PathBuf, f: /*impl Fn*/fn(&FsNode, &PathBuf)) {
        let path = parent_path;
        for child in &self.children {
            path.push(child.name.clone());
            f(child, &path);
            child.traverse(path, f);
            path.pop();
        }
    }
}