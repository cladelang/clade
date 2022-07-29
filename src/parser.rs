use crate::{config::Config, util, code_node::CodeNode, compiler::Compiler};

use roxmltree::Document;
use std::process::Command;
use rand::Rng;

pub fn run(compile_only: bool, release: bool) {
    let config: Config = toml::from_str(&std::fs::read_to_string("Clade.toml").unwrap()).unwrap();
    let mut compiler = Compiler::new(release);
    
    let input = std::fs::read_to_string(util::get_src_dir().join(&config.project.entry_point)).unwrap();

    let doc = Document::parse(&input).unwrap();
    let main_method = match doc.descendants().find(|e| e.tag_name().name() == "Main") {
        Some(e) => e,
        None => panic!("No Main method found"),
    };

    compiler.push_line_str("// Clade generated code");
    compiler.push_line_str(&format!("// At {}", util::get_clade_time()));
    compiler.push_line_str("// The Clade project: https://github.com/cladelang/clade");
    compiler.push_empty_line();

    compiler.push_line_str("#![allow(dead_code)]");
    compiler.push_empty_line();

    // helper methods
    compiler.push_line_str("fn get_arg<'a>(args: &'a Vec<(u64, Vec<(&str, &str)>)>, addr: u64, name: &'a str) -> &'a str {");
    compiler.push_with_indent_str("for arg in args {");
    compiler.push_with_x_indent("if arg.0 == addr {", 2);
    compiler.push_with_x_indent("for (arg_name, arg_value) in &arg.1 {", 3);
    compiler.push_with_x_indent("if arg_name == &name {", 4);
    compiler.push_with_x_indent("return arg_value;", 5);
    for x in (1..5).rev() {
        compiler.push_with_x_indent("}", x);
    }
    compiler.push_with_x_indent("panic!(\"Arg not found: {}\", name);", 1);
    compiler.push_line_str("}");
    compiler.push_empty_line();
    compiler.push_empty_line();
    // end helper methods

    compiler.push_line_str("fn main() {");
    compiler.push_with_indent_str("let mut args: Vec<(u64, Vec<(&str, &str)>)> = vec![];");
    compiler.push_empty_line();
    let mut code_nodes: Vec<CodeNode> = vec![];

    let mut rng = rand::thread_rng();

    for child in main_method.children() {
        if child.tag_name().name() == "" {
            continue;
        }

        let mut node_addr: u64 = rng.gen_range(0..999999999);
        // avoid duplicate addresses
        while code_nodes.iter().map(|n| n.addr).any(|a| a == node_addr) {
            node_addr = rng.gen_range(0..999999999);
        }
        let code_node = CodeNode::new(child, node_addr);
        code_nodes.push(code_node);
    }

    for code_node in &code_nodes {
        let mut args: Vec<(String, String)> = vec![];
        for arg in &code_node.args {
            args.push((arg.0.clone(), arg.1.clone()));
        }
        compiler.push_with_indent_str(&format!("args.push(({}, vec![", code_node.addr));
        for arg in &args {
            compiler.push_with_x_indent(&format!("(\"{}\", \"{}\"),", util::escape_str(&arg.0), util::escape_str(&arg.1)), 2);
        }
        compiler.push_with_indent_str("]));");
    }

    compiler.push_empty_line();

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