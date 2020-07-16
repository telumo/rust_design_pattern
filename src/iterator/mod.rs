#[derive(Debug, Clone)]
struct Book {
    name: String,
}

impl Book {
    fn new(name: impl Into<String>) -> Self {
        Book { name: name.into() }
    }

    fn get_name(&self) -> String {
        (&self.name).into()
    }
}

trait Iterator {
    type Item; // implでtype Item = Book;
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Self::Item;
}

// TODO: Tの制約をイテレーターにしたい
// dyn？
trait Aggregate {
    type Item;
    fn iterator(self) -> Self::Item;
}

struct BookShelfIterator {
    book_shelf: BookShelf,
    index: usize,
}

impl BookShelfIterator {
    fn new(book_shelf: BookShelf) -> Self {
        BookShelfIterator {
            book_shelf,
            index: 0,
        }
    }
}

impl Iterator for BookShelfIterator {
    type Item = Book;

    fn has_next(&self) -> bool {
        // println!("Bookshelf len: {:?}", self.bookShelf);
        self.index < self.book_shelf.get_length()
    }
    fn next(&mut self) -> Book {
        let book = self.book_shelf.get_book_at(self.index);
        self.index += 1;
        book
    }
}

#[derive(Debug, Clone)]
struct BookShelf {
    books: Vec<Book>,
}

impl BookShelf {
    fn new() -> Self {
        BookShelf { books: Vec::new() }
    }

    fn get_book_at(&self, index: usize) -> Book {
        self.books[index].clone()
    }

    fn append_book(&mut self, book: Book) {
        self.books.push(book)
    }

    fn get_length(&self) -> usize {
        self.books.len()
    }
}

impl Aggregate for BookShelf {
    type Item = BookShelfIterator;
    fn iterator(self) -> BookShelfIterator {
        BookShelfIterator::new(self)
    }
}

pub fn run() {
    let mut bookshelf = BookShelf::new();
    bookshelf.append_book(Book::new("ハリーポッター"));
    bookshelf.append_book(Book::new("ハリーポッター2"));
    let mut it = bookshelf.iterator();
    while it.has_next() {
        let book = it.next();
        println!("Index: {}", it.index);
        println!("This book is {:?}", book.get_name());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_book_1() {
        let book = Book::new("ハリーポッター");
        assert_eq!(book.get_name(), "ハリーポッター")
    }

    #[test]
    fn test_book_shelf_1() {
        let book1 = Book::new("ハリーポッター1");
        let book2 = Book::new("ハリーポッター2");
        let book3 = Book::new("ハリーポッター3");
        let book4 = Book::new("ハリーポッター4");

        let mut book_shelf = BookShelf::new();
        book_shelf.append_book(book1);
        book_shelf.append_book(book2);
        book_shelf.append_book(book3);
        book_shelf.append_book(book4);

        let book = book_shelf.get_book_at(0);
        assert_eq!(book.get_name(), "ハリーポッター1");

        let book = book_shelf.get_book_at(2);
        assert_eq!(book.get_name(), "ハリーポッター3");

        let length = book_shelf.get_length();
        assert_eq!(length, 4);

        let book5 = Book::new("Rustで学ぶデザインパターン入門");
        book_shelf.append_book(book5);

        let book = book_shelf.get_book_at(4);
        assert_eq!(book.get_name(), "Rustで学ぶデザインパターン入門");

        let length = book_shelf.get_length();
        assert_eq!(length, 5);
    }
}
