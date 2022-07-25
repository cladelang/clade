use std::{io::Write, fs::create_dir, fs::File};
use clap::{Parser, Subcommand};
use crate::{util, project::Project, parser};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    New(NewAction),
    Start(StartAction),
    Compile(CompileAction)
}

#[derive(Parser)]
struct NewAction {
    project_name: String,
}

#[derive(Parser)]
struct StartAction {
    #[clap(long, short)]
    release: bool,
}

#[derive(Parser)]
struct CompileAction {
    #[clap(long, short)]
    release: bool,
}

pub fn run() {
    let args = Args::parse();
    match args.action {
        Action::New(NewAction { project_name }) => {
            let project_path = util::current_dir().join(&project_name);
            match create_dir(&project_path) {
                Ok(x) => x,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::AlreadyExists {
                        println!("Project folder already exists.");
                    }
                    util::exit_ok();
                    return;
                }
            };

            create_files(Project::new(project_name.to_string()));
        },
        Action::Start(StartAction { release }) => {
            if util::in_project() {
                parser::run(false, release);
            } else {
                println!("This action can only be ran in the root of a project folder.");
                util::exit_ok();
            }
        },
        Action::Compile(CompileAction { release }) => {
            if util::in_project() {
                parser::run(true, release);
            } else {
                println!("This action can only be ran in the root of a project folder.");
                util::exit_ok();
            }
        }
    }
}

pub fn create_files(project: Project) {
    let src_path = util::get_src_dir();
    util::try_create_folder(&src_path);
    let main_path = util::get_src_dir().join("Main.xml");
    util::try_create_file(&main_path);
    let bin_path = util::get_bin_dir();
    util::try_create_folder(&bin_path);
    let clade_toml_path = util::get_clade_toml();
    util::try_create_file(&clade_toml_path);

    let mut clade_toml = File::create(&clade_toml_path).unwrap();
    writeln!(clade_toml, "[project]").unwrap();
    writeln!(clade_toml, "name = \"{}\"", project.name).unwrap();
    write!(clade_toml, "entry_point = \"Main.xml\"").unwrap();

    let mut main_file = File::create(&main_path).unwrap();
    writeln!(main_file, "<Main>").unwrap();
    writeln!(main_file, "    <Println>Hello, world!</Println>").unwrap();
    write!(main_file, "</Main>").unwrap();
}