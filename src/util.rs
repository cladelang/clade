use std::path::PathBuf;

pub fn try_create_folder(path: &PathBuf) {
    match std::fs::create_dir(path) {
        Ok(x) => x,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                println!("Folder already exists.");
            }
            return;
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
            return;
        }
    };
}