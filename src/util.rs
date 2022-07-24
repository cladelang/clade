use std::path::PathBuf;

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
    let clade_toml = get_clade_toml();
    let bin_dir = get_bin_dir();
    let src_dir = get_src_dir();
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

pub fn get_src_dir() -> PathBuf {
    current_dir().join("src")
}

pub fn get_bin_dir() -> PathBuf {
    current_dir().join("bin")
}

pub fn get_clade_toml() -> PathBuf {
    current_dir().join("Clade.toml")
}