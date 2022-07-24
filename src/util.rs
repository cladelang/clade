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
    let clade_toml = current_dir().join("Clade.toml");
    clade_toml.exists()
}

pub fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

pub fn escape_str(s: &str) -> String {
    s.replace("\"", "\\\"")
}