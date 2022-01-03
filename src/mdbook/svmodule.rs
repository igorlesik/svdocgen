//! Parse SV file and generate info about Verilog module(s).
//!
//!
//!

use sv_parser::{parse_sv, unwrap_node, unwrap_locate, Locate, RefNode, SyntaxTree};
use sv_parser::{PortDirection, NetType, IntegerVectorType};
use std::collections::HashMap;
use std::path::PathBuf;

pub fn generate_sv_module_info(
    file_path: &str
) -> Vec<String>
{
    let mut text: Vec<String> = Vec::new();

    // The list of defined macros
    let defines = HashMap::new();
    // The list of include paths
    let includes: Vec<PathBuf> = Vec::new();

    // Parse
    let result = parse_sv(file_path, &defines, &includes, false, true);

    if let Ok((syntax_tree, _)) = result {
        // &SyntaxTree is iterable
        for node in &syntax_tree {
            // The type of each node is RefNode
            match node {
                RefNode::ModuleDeclarationNonansi(x) => {
                    // unwrap_node! gets the nearest ModuleIdentifier from x
                    let id = unwrap_node!(x, ModuleIdentifier).unwrap();

                    let id = get_identifier(id).unwrap();

                    // Original string can be got by SyntaxTree::get_str(self, locate: &Locate)
                    let id = syntax_tree.get_str(&id).unwrap();
                    print_module(&mut text, file_path, id, false, &syntax_tree, &node);
                }
                RefNode::ModuleDeclarationAnsi(x) => {
                    let id = unwrap_node!(x, ModuleIdentifier).unwrap();
                    let id = get_identifier(id).unwrap();
                    let id = syntax_tree.get_str(&id).unwrap();
                    print_module(&mut text, file_path, id, true, &syntax_tree, &node);
                }
                _ => (),
            }
        }
    } else {
        println!("parsing of '{}' failed\n", file_path);
    }

    text
}

fn get_identifier(node: RefNode) -> Option<Locate> {
    // unwrap_node! can take multiple types
    match unwrap_node!(node, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => {
            return Some(x.nodes.0);
        }
        Some(RefNode::EscapedIdentifier(x)) => {
            return Some(x.nodes.0);
        }
        _ => None,
    }
}

fn get_whole_str(
    syntax_tree: &SyntaxTree,
    node: &RefNode
) -> String
{
    let mut s = String::new();

    for subnode in node.clone().into_iter() {
        if let RefNode::Locate(_text) = subnode {
            let text = unwrap_locate!(subnode);
            if let Some(text) = text {
                let text = syntax_tree.get_str(text);
                if let Some(text) = text {
                    s.push_str(text);
                }
            }
        }
    }

    s
}

fn print_module(
    text: &mut Vec<String>,
    file_path: &str,
    module_name: &str,
    is_ansi: bool,
    syntax_tree: &SyntaxTree,
    module_node: &RefNode
)
{
    text.push(format!("\n## Module `{}`\n\n", module_name));
    text.push(format!("File: `{}`\n\n", file_path));
    //text.push(format!("File: `{:?}`\n\n", _syntax_tree.get_origin(unwrap_locate!(module_node)));

    text.push(format!("Ports: \n\n"));
    if is_ansi {
        print_ansi_ports(text, syntax_tree, module_node);
    }
}

fn print_ansi_ports(
    text: &mut Vec<String>,
    syntax_tree: &SyntaxTree,
    module_node: &RefNode
)
{
    // FIXME TODO check if it can be done  without cloning
    for node in module_node.clone().into_iter() {
        // The type of each node is RefNode
        match node {
            RefNode::AnsiPortDeclaration(x) => {
                //text.push(format!("{:?}\n", x));
                let id = unwrap_node!(x, PortIdentifier).unwrap();
                let id = unwrap_locate!(id).unwrap();
                let id = syntax_tree.get_str(id).unwrap();
                text.push(format!("- {}\n", id));

                let dir = unwrap_node!(x, PortDirection);
                let dir_str = match dir {
                    Some(RefNode::PortDirection(PortDirection::Input(_))) => "input",
                    Some(RefNode::PortDirection(PortDirection::Output(_))) => "output",
                    Some(RefNode::PortDirection(PortDirection::Inout(_))) => "inout",
                    _ => "?",
                };
                text.push(format!("  * direction: {}\n", dir_str));

                let net_type = unwrap_node!(x, NetType);
                match net_type {
                    Some(RefNode::NetType(NetType::Wire(_))) =>
                        text.push("  * type: wire\n".to_string()),
                    _ => (),
                }

                let vnet_type = unwrap_node!(x, IntegerVectorType);
                match vnet_type {
                    Some(RefNode::IntegerVectorType(IntegerVectorType::Reg(_))) =>
                        text.push("  * type: reg\n".to_string()),
                    Some(RefNode::IntegerVectorType(IntegerVectorType::Logic(_))) =>
                        text.push("  * type: logic\n".to_string()),
                    Some(RefNode::IntegerVectorType(IntegerVectorType::Bit(_))) =>
                        text.push("  * type: bit\n".to_string()),
                    _ => (),
                }

                let width = unwrap_node!(x, PackedDimensionRange);
                if let Some(width) = width {
                    //text.push(format!("{:?}\n", &width));
                    text.push(format!("  * width: {}\n", get_whole_str(syntax_tree, &width)));
                }
            }
            _ => (),
        }
    }

}

