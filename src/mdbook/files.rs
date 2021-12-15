//! Parse SV project directory and collect SV files.
//!

use crate::args;

pub enum FsNode {
    File(String),
    Dir(Vec<FsNode>)
}

pub struct SrcFiles {
    pub roots: Vec<String>,
}

pub struct DstFiles {

}

/// Data about all the files.
///
pub struct Files {
    pub src: SrcFiles,
    pub dst: DstFiles,
}

pub fn collect_sources(options: &args::ParsedOptions) -> Result<(),String> {

    for input in &options.inputs{
        println!("input: {}", input);
    }

    Ok(())
}