use roxmltree::Node;

use crate::arg::Arg;

pub struct CodeNode {
    /// The name of the node.
    pub name: String,
    /// The main argument is the de facto text of the node.
    pub main_arg: String,
    /// The actual arguments of the node.
    pub args: Vec<Arg>,
    /// The address of the code node.
    pub addr: u64,
}

impl CodeNode {
    pub fn new(node: Node, addr: u64) -> Self {
        let attr_args = node.attributes().iter().map(|a| (a.name().to_string(), a.value().to_string())).collect::<Vec<_>>();
        let mut args: Vec<Arg> = vec![];

        for arg in attr_args {
            args.push(Arg::new(arg.0, arg.1, false));
        }
        
        // < and > are forbidden in XML args, so we shouldn't have issues.
        args.push(Arg::new("CLADE<<MAIN_ARG>>".to_string(), node.text().unwrap().to_string(), true));

        CodeNode {
            name: node.tag_name().name().to_string(),
            main_arg: node.text().unwrap().to_string(),
            args,
            addr
        }
    }
}