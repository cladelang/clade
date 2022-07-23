use std::{env, path::PathBuf, io::Write};
use clap::Parser;
use crate::util::{try_create_folder, try_create_file};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action
}

#[derive(clap::Subcommand)]
enum Action {
    New(NewAction)
}

#[derive(Parser)]
struct NewAction {
    project_name: String,
}

pub fn run() {
    let args = Args::parse();
    match args.action {
        Action::New(NewAction { project_name }) => {
            let project_path = env::current_dir().unwrap().join(project_name);
            match std::fs::create_dir(&project_path) {
                Ok(x) => x,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::AlreadyExists {
                        println!("Project folder already exists.");
                        return;
                    }
                    return;
                }
            };

            create_files(&project_path);
        }
    }
}

pub fn create_files(project_path: &PathBuf) {
    let src_path = project_path.join("src");
    try_create_folder(&src_path);
    let main_path = project_path.join("src").join("Main.xml");
    try_create_file(&main_path);
    let target_path = project_path.join("target");
    try_create_folder(&target_path);

    // write to src/Main.xml
    let mut main_file = std::fs::File::create(&main_path).unwrap();
    main_file.write_all(b"<Main>\n    <Println>Hello, world!</Println>\n</Main>").unwrap();
}