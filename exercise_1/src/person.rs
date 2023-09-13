use std::fmt::Display;

pub struct Person<'a> {
    pub name: &'a str,
    pub age: u8,
}

impl<'a> Person<'a> {
    pub fn new(name: &str, age: u8) -> Person {
        Person { name, age }
    }
}

impl Display for Person<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for &Person<'_> {
    fn default() -> Self {
        &Person { name: "", age: 0 }
    }
}
