use crate::book::Book;

pub struct Library {
    pub books: Vec<Book>,
}

impl Library {
    pub fn new() -> Self {
        Self { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn list_books(&self) {
        println!("Library Catalog:");
        for (index, book) in self.books.iter().enumerate() {
            let availability = if book.is_available {
                "Available"
            } else {
                "Not Available"
            };
            println!(
                "Book {}: {} by {} ({})",
                index + 1,
                book.title,
                book.author,
                availability
            );
        }
    }

    // Borrow a book from the library
    pub fn borrow_book(&mut self, title: &str) -> bool {
        for book in &mut self.books {
            if book.title == title && book.is_available {
                book.borrow_book();
                return true;
            }
        }
        false
    }

    // Return a book to the library
    pub fn return_book(&mut self, title: &str) -> bool {
        for book in &mut self.books {
            if book.title == title && !book.is_available {
                book.return_book();
                return true;
            }
        }
        false
    }
}
