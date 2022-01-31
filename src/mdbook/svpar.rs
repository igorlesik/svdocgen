//! Helpers to parse SV.
//!
//!
//!

use sv_parser::{unwrap_node, unwrap_locate, Locate, RefNode, SyntaxTree};

pub fn get_identifier(node: RefNode) -> Option<Locate> {
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

pub fn get_whole_str(
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