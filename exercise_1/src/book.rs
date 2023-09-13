use crate::person::Person;

pub struct Book<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub is_available: bool,
    pub borrowed_by: Option<&'a Person<'a>>,
}

impl<'a> Book<'a> {
    pub fn new(title: &'a str, author: &'a str) -> Book<'a> {
        Book {
            title,
            author,
            is_available: true,
            borrowed_by: None,
        }
    }

    pub fn borrow_book(&mut self, person: &'a Person<'a>) {
        self.is_available = false;
        self.borrowed_by = Option::from(person);
    }

    pub fn return_book(&mut self) {
        self.is_available = true;
        self.borrowed_by = None;
    }
}
