use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub project: Project,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub entry_point: String,
}