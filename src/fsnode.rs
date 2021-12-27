//! Tree structure to keep FS paths.
//!
//! FsNodeIter implement external iterator for FsNode tree.
//! Implementation ideas:
//!
//! - <https://aloso.github.io/2021/03/09/creating-an-iterator>
//!

use std::path;
use std::path::{/*Path,*/ PathBuf};

/// File System node as path to file or directory.
///
///#[derive(Clone)]
pub struct FsNode {
    pub name: String,
    pub children: Vec<FsNode>,
}


/// Tuple struct wrapping FsNode.
pub struct FsNodeIter<'a>(&'a FsNode);


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

    /// Traverse (iterate over) depth-first.
    ///
    pub fn traverse(
        &self,
        parent_path: &mut PathBuf,
        level: usize,
        f: &mut impl FnMut(&FsNode, &PathBuf, usize)
    ) {
        let path = parent_path;
        for child in &self.children {
            path.push(child.name.clone());
            f(child, &path, level);
            child.traverse(path, level + 1, f);
            path.pop();
        }
    }

    pub fn traverse_top(
        &self,
        f: &mut impl FnMut(&FsNode, &PathBuf, usize))
    {
        self.traverse(&mut PathBuf::from(""), 0, f)
    }

    pub fn iter(&self) -> FsNodeIter<'_> {
        FsNodeIter(self)
    }

}

impl Clone for FsNode {

    fn clone(&self) -> Self {
        FsNode {
            name: self.name.clone(),
            children: self.children.clone()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.name = source.name.clone();
        self.children = source.children.clone();
    }
}


impl<'a> Iterator for FsNodeIter<'a> {
    type Item = &'a FsNode;

    fn next(&mut self) -> Option<Self::Item> {
        //todo!()
        None
    }
}

