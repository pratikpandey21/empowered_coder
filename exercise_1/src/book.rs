pub struct Book {
    pub title: String,
    pub author: String,
    pub is_available: bool,
}

impl Book {
    pub fn new(title: String, author: String) -> Self {
        Self {
            title,
            author,
            is_available: true,
        }
    }

    pub fn borrow_book(&mut self) {
        self.is_available = false;
    }

    pub fn return_book(&mut self) {
        self.is_available = true;
    }
}
