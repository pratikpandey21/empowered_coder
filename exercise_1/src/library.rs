use crate::book::Book;
use crate::person::Person;

pub struct Library<'a> {
    pub books: Vec<Book<'a>>,
}

impl<'a> Library<'a> {
    pub fn new() -> Library<'a> {
        Library { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: Book<'a>) {
        self.books.push(book);
    }

    pub fn list_all_available_books(&self) {
        println!("Available Library Catalog:");
        for (index, book) in self.books.iter().enumerate() {
            if book.is_available {
                println!("Book {}: {} by {}", index + 1, book.title, book.author);
            }
        }
    }

    pub fn list_all_checkout_books(&self) {
        println!("Checkout Library Catalog:");
        for (index, book) in self.books.iter().enumerate() {
            if !book.is_available {
                let borrowed_by = book.borrowed_by.unwrap_or_default();
                println!(
                    "Book {}: {} by {} borrowed by {}",
                    index + 1,
                    book.title,
                    book.author,
                    borrowed_by
                );
            }
        }
    }

    // Borrow a book from the library
    pub fn borrow_book(&mut self, title: &str, borrower: &'a Person<'a>) -> bool {
        for book in &mut self.books {
            if book.title == title && book.is_available {
                book.borrow_book(borrower);
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
