use std::{path::PathBuf, process::Command};

// Tries to create a folder and exits if there was an error.
pub fn try_create_folder(path: &PathBuf) {
    match std::fs::create_dir(path) {
        Ok(x) => x,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                println!("Folder already exists.");
            }
            exit_ok();
        }
    }
}

// Tries to create a file and exits if there was an error.
pub fn try_create_file(path: &PathBuf) {
    match std::fs::File::create(path) {
        Ok(x) => x,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                println!("File already exists.");
            }
            exit_ok();
            return;
        }
    };
}

pub fn exit_ok() {
    std::process::exit(0);
}

pub fn in_project() -> bool {
    let clade_toml = current_dir().join("Clade.toml");
    let bin_dir = current_dir().join("bin");
    let src_dir = current_dir().join("src");
    clade_toml.exists() && bin_dir.exists() && src_dir.exists()
}

pub fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

pub fn escape_str(s: &str) -> String {
    s.replace("\"", "\\\"")
}

pub fn create_dir_if_not_exists(path: &PathBuf) {
    if !path.exists() {
        std::fs::create_dir(path).unwrap();
    }
}

pub fn get_src_dir(proj_name: String) -> PathBuf {
    if !in_project() {
        return current_dir().join(proj_name).join("src");
    }
    current_dir().join("src")
}

pub fn get_bin_dir(proj_name: String) -> PathBuf {
    if !in_project() {
        return current_dir().join(proj_name).join("bin");
    }
    current_dir().join("bin")
}

pub fn get_clade_toml(proj_name: String) -> PathBuf {
    if !in_project() {
        return current_dir().join(proj_name).join("Clade.toml");
    }
    current_dir().join("Clade.toml")
}

pub fn get_cargo_dir(proj_name: String) -> PathBuf {
    if !in_project() {
        return current_dir().join(proj_name).join("cargo");
    }
    current_dir().join("cargo")
}

pub fn run_and_wait(command: &mut Command) {
    let mut child = command.spawn().unwrap();
    child.wait().unwrap();
}

pub fn get_clade_time() -> String {
    let now = chrono::Utc::now();
    let time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    time
}

pub fn get_ext() -> String {
    if cfg!(windows) {
        return ".exe".to_string();
    }
    else {
        return "".to_string();
    }
}