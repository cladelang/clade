pub struct Project {
    pub name: String,
}

impl Project {
    pub fn new(name: String) -> Self {
        Project {
            name,
        }
    }
}