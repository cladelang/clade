use crate::{config, util, code_node};
use crate::compiler::Compiler;
use roxmltree::Document;

pub fn run(release: bool) {
    let config: config::Config = toml::from_str(&std::fs::read_to_string("Clade.toml").unwrap()).unwrap();
    let mut compiler = Compiler::new(release);
    
    let input = std::fs::read_to_string(util::current_dir().join("src").join(&config.project.entry_point)).unwrap();
    
    let doc = Document::parse(&input).unwrap();
    let main_method = match doc.descendants().find(|e| e.tag_name().name() == "Main") {
        Some(e) => e,
        None => panic!("No Main method found"),
    };
    compiler.push_line_str("fn main() {");
    for child in main_method.children() {
        let tag_name = child.tag_name().name();
        let code_node = code_node::CodeNode::new(child);

        if tag_name == "Println" {
            compiler.push_with_indent_str(&format!("println!(\"{}\");", code_node.main_arg.replace("\"", "\\\"")));
        }
    }
    compiler.push_line_str("}");
    compiler.compile(&config);
    
    let exe = util::current_dir().join("bin").join(format!("{}.exe", config.project.name));
    std::process::Command::new(exe.to_str().unwrap()).spawn().unwrap();
}