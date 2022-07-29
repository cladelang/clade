use crate::{config::Config, util};
use std::{io::Write, path::PathBuf};

pub struct Compiler {
    pub lines: Vec<String>,
    pub release: bool,
}

impl Compiler {
    pub fn new(release: bool) -> Self {
        Compiler {
            lines: Vec::new(),
            release,
        }
    }

    pub fn push_empty_line(&mut self) {
        self.lines.push(String::new());
    }

    pub fn push_line_str(&mut self, line: &str) {
        self.lines.push(line.to_string());
    }

    pub fn push_with_x_indent(&mut self, line: &str, x: usize) {
        self.lines.push(format!("{}{}", " ".repeat(x * 4), line));
    }

    pub fn push_with_indent_str(&mut self, line: &str) {
        self.push_with_x_indent(line, 1);
    }

    pub fn compile(&mut self, config: &Config) -> PathBuf {
        let bin_dir = util::get_bin_dir();

        let rustcode_path = if self.release {
            util::create_dir_if_not_exists(&bin_dir.join("release"));
            bin_dir.join("release").join(format!("{}.rs", config.project.name))
        } else {
            util::create_dir_if_not_exists(&bin_dir.join("debug"));
            bin_dir.join("debug").join(format!("{}.rs", config.project.name))
        };

        let mut rustcode_file = std::fs::File::create(&rustcode_path).unwrap();
        for line in self.lines.iter() {
            rustcode_file.write_all(line.as_bytes()).unwrap();
            if line != &self.lines[self.lines.len() - 1] {
                rustcode_file.write_all("\n".as_bytes()).unwrap();
            }
        }

        let mut rustc = std::process::Command::new("rustc");

        let out_dir = if self.release {
            let out_dir = bin_dir.join("release");
            rustc.arg("--out-dir").arg(out_dir.to_str().unwrap());
            rustc.arg("-O");
            rustc.arg("-Cdebuginfo=0");
            rustc.arg("-Copt-level=3");
            rustc.arg("-Clink-arg=/DEBUG:NONE");
            out_dir
        } else {
            let out_dir = bin_dir.join("debug");
            rustc.arg("--out-dir").arg(out_dir.to_str().unwrap());
            out_dir
        };
        rustc.arg(rustcode_path.to_str().unwrap());

        rustc.spawn().unwrap().wait().unwrap();

        out_dir.clone()
    }
}