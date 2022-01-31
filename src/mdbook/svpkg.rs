//! Parse SV file and generate info about Verilog package(s).
//!
//!
//!

use sv_parser::{parse_sv, unwrap_node, /*unwrap_locate, Locate,*/ RefNode, SyntaxTree};
//use sv_parser::{PortDirection, NetType, IntegerVectorType};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
//use regex::Regex;
use crate::mdbook::svpar;

pub fn generate_sv_package_info(
    output_path: &str,
    file_path: &str
) -> (Vec<String>, Vec<(String, String, String)>)
{
    let mut text: Vec<String> = Vec::new();
    let mut list: Vec<(String, String, String)> = Vec::new();

    // The list of defined macros
    let defines = HashMap::new();
    // The list of include paths
    let includes: Vec<PathBuf> = Vec::new();

    // Parse
    let result = parse_sv(file_path, &defines, &includes, false, true);

    if let Ok((syntax_tree, _)) = result {
        let mut prev_node: Option<RefNode> = None;
        // &SyntaxTree is iterable
        for node in &syntax_tree {
            // The type of each node is RefNode
            match node {
                RefNode::PackageDeclaration(x) => {
                    let id = unwrap_node!(x, PackageIdentifier).unwrap();
                    let id = svpar::get_identifier(id).unwrap();
                    let id = syntax_tree.get_str(&id).unwrap();
                    let item = print_package(&mut text, output_path, file_path, id,
                        &syntax_tree, &node, &prev_node);
                    list.push(item)
                }
                RefNode::Description(_) => {
                }
                RefNode::WhiteSpace(_) => {
                }
                RefNode::Locate(_) => {
                }
                x => { prev_node = Some(x); () },
            }
        }
    } else {
        println!("parsing of '{}' failed\n", file_path);
    }

    (text, list)
}

fn print_package(
    top_text: &mut Vec<String>,
    output_path: &str,
    file_path: &str,
    pkg_name: &str,
    _syntax_tree: &SyntaxTree,
    _pkg_node: &RefNode,
    _prev_node: &Option<RefNode>
) -> (String, String, String)
{
    let mut text = String::new();

    text.push_str(format!("## Package `{}`\n\n", pkg_name).as_str());
    text.push_str(format!("File: `{}`\n\n", file_path).as_str());

    /*text.push_str(format!("### Ports: \n\n").as_str());
    if is_ansi {
        print_ansi_ports(&mut text, syntax_tree, module_node);
    }

    text.push_str(format!("\n\n### Instantiates modules: \n\n").as_str());
    print_instantiated_modules(&mut text, syntax_tree, module_node);*/

    //print_module_comments(&mut text, syntax_tree, module_node, prev_node);

    let mut pkg_path = Path::new(output_path).join("src").join(file_path);
    pkg_path.set_extension(format!("pkg.{}.md", pkg_name));
    let mut src_pkg_path = Path::new("src").join(file_path);
    src_pkg_path.set_extension(format!("pkg.{}.md", pkg_name));
    let src_pkg_path = src_pkg_path.to_str().unwrap();

    std::fs::write(pkg_path, text).expect("failed to write file");

    top_text.push(format!("- [`{}  :{}`]({})\n", pkg_name, file_path, src_pkg_path));

    (pkg_name.to_string(), file_path.to_string(), src_pkg_path.to_string())
}