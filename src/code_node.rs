use roxmltree::Node;

pub struct CodeNode {
    /// The name of the node.
    pub name: String,
    /// The main argument is the de facto text of the node.
    pub main_arg: String,
    /// The actual arguments of the node.
    pub args: Vec<(String, String)>,
    /// The address of the code node.
    pub addr: u64,
}

impl CodeNode {
    pub fn new(node: Node, addr: u64) -> Self {
        let args = node.attributes().iter().map(|a| (a.name().to_string(), a.value().to_string())).collect::<Vec<_>>();

        CodeNode {
            name: node.tag_name().name().to_string(),
            main_arg: node.text().unwrap().to_string(),
            args,
            addr
        }
    }
}