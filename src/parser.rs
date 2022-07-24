use crate::config::Config;
use crate::util;
use crate::code_node::CodeNode;
use crate::compiler::Compiler;

use roxmltree::Document;
use std::process::Command;

pub fn run(compile_only: bool, release: bool) {
    let config: Config = toml::from_str(&std::fs::read_to_string("Clade.toml").unwrap()).unwrap();
    let mut compiler = Compiler::new(release);
    
    let input = std::fs::read_to_string(util::get_src_dir().join(&config.project.entry_point)).unwrap();

    let doc = Document::parse(&input).unwrap();
    let main_method = match doc.descendants().find(|e| e.tag_name().name() == "Main") {
        Some(e) => e,
        None => panic!("No Main method found"),
    };
    compiler.push_line_str("fn main() {");
    let mut code_nodes: Vec<CodeNode> = vec![];

    for child in main_method.children() {
        let code_node = CodeNode::new(child);
        code_nodes.push(code_node);
    }

    for code_node in &code_nodes {
        for arg in &code_node.args {
            compiler.push_with_indent_str(&format!("let _{}_arg = \"{}\";", arg.0, util::escape_str(&arg.1)));
        }
    }

    compiler.push_line_str("");

    // TODO: implement runnable
    for code_node in &code_nodes {
        if code_node.name == "" {
            continue;
        }

        #[cfg(debug_assertions)]
        println!("Arg: {:?}", code_node.args);

        if code_node.name == "Println" {
            compiler.push_with_indent_str(&format!("println!(\"{}\");", util::escape_str(&code_node.main_arg)));
        }
    }
    
    compiler.push_line_str("}");
    let out_path = compiler.compile(&config);
    
    if !compile_only {
        let exe_path = out_path.join(format!("{}.exe", config.project.name));
        util::run_and_wait(&mut Command::new(exe_path.to_str().unwrap()));
    }
}