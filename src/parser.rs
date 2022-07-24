use crate::config::Config;
use crate::util;
use crate::code_node::CodeNode;
use crate::compiler::Compiler;
use roxmltree::Document;

pub fn run(release: bool) {
    let config: Config = toml::from_str(&std::fs::read_to_string("Clade.toml").unwrap()).unwrap();
    let mut compiler = Compiler::new(release);
    
    let input = std::fs::read_to_string(util::current_dir().join("src").join(&config.project.entry_point)).unwrap();

    let doc = Document::parse(&input).unwrap();
    let main_method = match doc.descendants().find(|e| e.tag_name().name() == "Main") {
        Some(e) => e,
        None => panic!("No Main method found"),
    };
    compiler.push_line_str("fn main() {");
    // TODO: implement runnable
    for child in main_method.children() {
        let code_node = CodeNode::new(child);
        // roxmltree gives us empty named nodes, so we need to filter them out.
        if code_node.name == "" {
            continue;
        }

        #[cfg(debug_assertions)]
        println!("Arg: {:?}", code_node.args);
        
        if code_node.name == "Println" {
            compiler.push_with_indent_str(&format!("println!(\"{}\");", code_node.main_arg.replace("\"", "\\\"")));
        }
    }
    compiler.push_line_str("}");
    compiler.compile(&config);
    
    let exe_path = util::current_dir().join("bin").join(format!("{}.exe", config.project.name));
    std::process::Command::new(exe_path.to_str().unwrap()).spawn().unwrap().wait().unwrap();
}