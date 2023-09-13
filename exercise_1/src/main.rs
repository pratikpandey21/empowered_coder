use crate::book::Book;
use crate::library::Library;
use crate::person::Person;

mod book;
mod library;
mod person;

fn main() {
    let person = Person::new("Pratik", 31);
    println!("Welcome, {}!", person.name);

    let mut library = Library::new();

    // Add books to the library
    library.add_book(Book::new("The Hobbit", "J.R.R. Tolkien"));
    library.add_book(Book::new("To Kill a Mockingbird", "Harper Lee"));
    library.add_book(Book::new("1984", "George Orwell"));

    // List the books in the library catalog
    library.list_all_available_books();

    // Borrow a book
    let borrowed = library.borrow_book("1984", &person);
    if borrowed {
        println!("You've borrowed '1984'.");
    } else {
        println!("Sorry, '1984' is not available for borrowing.");
    }

    // List the updated library catalog
    library.list_all_available_books();

    // Return a book
    // Using Shadowing here.
    let borrowed = library.borrow_book("1984", &person);
    if borrowed {
        println!("You've borrowed '1984'.");
    } else {
        println!("Sorry, '1984' is not available for borrowing.");
    }

    library.list_all_checkout_books();

    let returned = library.return_book("1984");
    if returned {
        println!("You've returned '1984'.");
    } else {
        println!("Something went wrong");
    }

    // List the final library catalog
    library.list_all_available_books();
}
