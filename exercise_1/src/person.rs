pub struct Person {
    pub name: String,
    pub age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}
