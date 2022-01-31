//! Parse SV file and generate info about Verilog module(s).
//!
//!
//!

use sv_parser::{parse_sv, unwrap_node, unwrap_locate, /*Locate,*/ RefNode, SyntaxTree};
use sv_parser::{PortDirection, NetType, IntegerVectorType};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use regex::Regex;
use crate::mdbook::svpar;

pub fn generate_sv_module_info(
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
                RefNode::ModuleDeclarationNonansi(x) => {
                    // unwrap_node! gets the nearest ModuleIdentifier from x
                    let id = unwrap_node!(x, ModuleIdentifier).unwrap();

                    let id = svpar::get_identifier(id).unwrap();

                    // Original string can be got by SyntaxTree::get_str(self, locate: &Locate)
                    let id = syntax_tree.get_str(&id).unwrap();
                    let item = print_module(&mut text, output_path, file_path, id, false,
                        &syntax_tree, &node, &prev_node);
                    list.push(item)
                }
                RefNode::ModuleDeclarationAnsi(x) => {
                    let id = unwrap_node!(x, ModuleIdentifier).unwrap();
                    let id = svpar::get_identifier(id).unwrap();
                    let id = syntax_tree.get_str(&id).unwrap();
                    let item = print_module(&mut text, output_path, file_path, id, true,
                        &syntax_tree, &node, &prev_node);
                    list.push(item)
                }
                RefNode::ModuleDeclaration(_) => {
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



fn print_module(
    top_text: &mut Vec<String>,
    output_path: &str,
    file_path: &str,
    module_name: &str,
    is_ansi: bool,
    syntax_tree: &SyntaxTree,
    module_node: &RefNode,
    prev_node: &Option<RefNode>
) -> (String, String, String)
{
    let mut text = String::new();

    text.push_str(format!("## Module `{}`\n\n", module_name).as_str());
    text.push_str(format!("File: `{}`\n\n", file_path).as_str());

    text.push_str(format!("### Ports: \n\n").as_str());
    if is_ansi {
        print_ansi_ports(&mut text, syntax_tree, module_node);
    }

    text.push_str(format!("\n\n### Instantiates modules: \n\n").as_str());
    print_instantiated_modules(&mut text, syntax_tree, module_node);

    print_module_comments(&mut text, syntax_tree, module_node, prev_node);

    let mut module_path = Path::new(output_path).join("src").join(file_path);
    module_path.set_extension(format!("module.{}.md", module_name));
    let mut src_module_path = Path::new("src").join(file_path);
    src_module_path.set_extension(format!("module.{}.md", module_name));
    let src_module_path = src_module_path.to_str().unwrap();

    std::fs::write(module_path, text).expect("failed to write file");

    top_text.push(format!("- [`{}  :{}`]({})\n", module_name, file_path, src_module_path));

    (module_name.to_string(), file_path.to_string(), src_module_path.to_string())
}

fn print_ansi_ports(
    text: &mut String,
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
                text.push_str(format!("- {}\n", id).as_str());

                let dir = unwrap_node!(x, PortDirection);
                let dir_str = match dir {
                    Some(RefNode::PortDirection(PortDirection::Input(_))) => "➔ input",
                    Some(RefNode::PortDirection(PortDirection::Output(_))) => "output ➔",
                    Some(RefNode::PortDirection(PortDirection::Inout(_))) => "inout",
                    _ => "?",
                };
                text.push_str(format!("  * direction: {}\n", dir_str).as_str());

                let net_type = unwrap_node!(x, NetType);
                match net_type {
                    Some(RefNode::NetType(NetType::Wire(_))) =>
                        text.push_str("  * type: wire\n"),
                    _ => (),
                }

                let vnet_type = unwrap_node!(x, IntegerVectorType);
                match vnet_type {
                    Some(RefNode::IntegerVectorType(IntegerVectorType::Reg(_))) =>
                        text.push_str("  * type: reg\n"),
                    Some(RefNode::IntegerVectorType(IntegerVectorType::Logic(_))) =>
                        text.push_str("  * type: logic\n"),
                    Some(RefNode::IntegerVectorType(IntegerVectorType::Bit(_))) =>
                        text.push_str("  * type: bit\n"),
                    _ => (),
                }

                let width = unwrap_node!(x, PackedDimensionRange);
                if let Some(width) = width {
                    //text.push(format!("{:?}\n", &width));
                    text.push_str(format!("  * width: {}\n", svpar::get_whole_str(syntax_tree, &width)).as_str());
                }
            }
            _ => (),
        }
    }

}

fn print_instantiated_modules(
    text: &mut String,
    syntax_tree: &SyntaxTree,
    module_node: &RefNode
)
{
    let mut mod_instances: HashMap<String, Vec<String>> = HashMap::new();

    for node in module_node.clone().into_iter() {
        match node {
            RefNode::ModuleInstantiation(x) => {
                let mod_name = unwrap_node!(x, ModuleIdentifier).unwrap();
                let mod_name = svpar::get_identifier(mod_name).unwrap();
                let mod_name = syntax_tree.get_str(&mod_name).unwrap();

                let inst_name = unwrap_node!(x, InstanceIdentifier).unwrap();
                let inst_name = svpar::get_identifier(inst_name).unwrap();
                let inst_name = syntax_tree.get_str(&inst_name).unwrap();

                let /*mut*/ m = match mod_instances.get_mut(mod_name) {
                    Some(m) => m,
                    None => {mod_instances.insert(mod_name.to_string(), Vec::<String>::new());
                        mod_instances.get_mut(mod_name).unwrap()},
                };

                m.push(inst_name.to_string());
            }
            _ => (),
        }
    }

    for (mname, inames) in mod_instances.iter() {
        text.push_str(format!("- {}\n", mname).as_str());
        for iname in inames.iter() {
            text.push_str(format!("  - {}\n", iname).as_str());
        }
    }
}

fn print_module_comments(
    text: &mut String,
    syntax_tree: &SyntaxTree,
    _module_node: &RefNode,
    prev_node: &Option<RefNode>,
)
{
    for node in prev_node.clone().into_iter() {
        match node {
            RefNode::Comment(x) => {
                let comment = unwrap_node!(x, Comment).unwrap();
                //text.push_str(format!("\ncomment:\n {}\n", get_whole_str(syntax_tree, &comment)).as_str());
                let comment = unwrap_locate!(comment).unwrap();
                let comment = syntax_tree.get_str(comment).unwrap();
                text.push_str(format!("\n\n### Description:\n\n").as_str());
                text.push_str(extract_text_from_comment(comment).as_str());
            }
            _ => (),
        }
    }
}

fn extract_text_from_comment(raw_text: &str) -> String
{
    let re = Regex::new(r"^\s*/(\*)+").unwrap();
    let text = re.replace_all(raw_text, "");

    let re = Regex::new(r"(\*/)+\s*$").unwrap();
    let text = re.replace_all(&text, "");

    // Ugly workaround for:
    // "Note that ^ matches after new lines, even at the end of input"
    let re = Regex::new(r"(?m)^\s*(\*)+\s*$").unwrap();
    let text = re.replace_all(&text, "<!--empty_line-->");

    let re = Regex::new(r"(?m)^\s*(\*)+").unwrap();
    let text = re.replace_all(&text, "");

    // Fixup for the ugly workaround
    let re = Regex::new(r"(?m)^<!--empty_line-->$").unwrap();
    let text = re.replace_all(&text, "");

    text.to_string()
}