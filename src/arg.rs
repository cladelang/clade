#[derive(Debug)]
pub struct Arg {
    pub name: String,
    pub value: String,
    pub main_arg: bool
}

impl Arg {
    pub fn new(name: String, value: String, main_arg: bool) -> Self {
        Arg {
            name,
            value,
            main_arg
        }
    }
}