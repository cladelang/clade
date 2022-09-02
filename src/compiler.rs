use crate::{config::Config, util};
use std::{io::Write, path::PathBuf, fs};

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

    pub fn push_empty_line(&mut self, x: usize) {
        for _ in 0..x {
            self.lines.push(String::new());
        }
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
        let bin_dir = util::get_bin_dir(config.project.name.to_string());
        
        let rustcode_path = util::get_cargo_dir(config.project.name.to_string())
            .join("src").join("main.rs");

        let mut rustcode_file = std::fs::File::create(&rustcode_path).unwrap();
        for line in self.lines.iter() {
            rustcode_file.write_all(line.as_bytes()).unwrap();
            if line != &self.lines[self.lines.len() - 1] {
                rustcode_file.write_all("\n".as_bytes()).unwrap();
            }
        }

        let mut cargo = std::process::Command::new("cargo");

        let out_dir = if self.release {
            let out_dir = bin_dir.join("release");
            util::create_dir_if_not_exists(&out_dir);
            cargo.current_dir(format!("{}/{}", util::current_dir().display(), "cargo")).arg("build").arg("--release");
            cargo.spawn().unwrap().wait().unwrap();
            fs::copy(util::current_dir().join("cargo").join("target").join("release").join("cargo"), util::current_dir().join("bin").join("release").join(&config.project.name)).unwrap();
            out_dir
        } else {
            let out_dir = bin_dir.join("debug");
            util::create_dir_if_not_exists(&bin_dir.join("debug"));
            cargo.current_dir(format!("{}/{}", util::current_dir().display(), "cargo")).arg("build");
            cargo.spawn().unwrap().wait().unwrap();
            fs::copy(util::current_dir().join("cargo").join("target").join("debug").join("cargo"), util::current_dir().join("bin").join("debug").join(&config.project.name)).unwrap();
            out_dir
        };

        out_dir.clone()
    }
}