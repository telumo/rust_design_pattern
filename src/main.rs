mod iterator;

#[derive(Debug)]
struct Book {
    name: String
}

impl Book {
    fn new(name: &str) -> Self {
        Book {
            name: name.into()
        }
    }

    fn getName(&self) -> String {
        (&self.name).into()
    }
}

trait Iterator<T> {
    fn hasNext() -> bool;
    fn next() -> T;
}

// TODO: Tの制約をイテレーターにしたい
// dyn？
trait Aggregate<T>
{
    fn iterator() -> T;
}

struct BookShelfIterator {
    bookShelf: BookShelf,
    index: i32,
}

// impl<T> Iterator<T> for BookShelfIterator {
//     fn hasNext() -> bool {
//         true
//     }
//     fn next() -> T {
//         Book::new("")
//     }
// }

struct BookShelf {
    books: Vec<Book>
}

impl BookShelf {
    fn new() -> Self {
        BookShelf {
            books: Vec::new()
        }
    }

    fn getBookAt(&self, index: usize) -> &Book {
        &(self.books[index])
    }

    fn appendBook(&mut self, book: Book) {
        self.books.push(book)
    }

    fn getLength(&self) -> usize {
        self.books.len()
    }
}

// impl Aggregate<BookShelfIterator> for BookShelf {
//     fn iterator() -> BookShelfIterator {

//     }
// }

fn main() {
    let book = Book::new("ハリーポッター");
    println!("book: {:#?}", book);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_book_1() {
        let book = Book::new("ハリーポッター");
        assert_eq!(book.getName(), "ハリーポッター")
    }


    #[test]
    fn test_book_shelf_1() {
        let book1 = Book::new("ハリーポッター1");
        let book2 = Book::new("ハリーポッター2");
        let book3 = Book::new("ハリーポッター3");
        let book4 = Book::new("ハリーポッター4");

        let mut book_shelf = BookShelf::new();
        book_shelf.appendBook(book1);
        book_shelf.appendBook(book2);
        book_shelf.appendBook(book3);
        book_shelf.appendBook(book4);

        let book = book_shelf.getBookAt(0);
        assert_eq!(book.getName(), "ハリーポッター1");

        let book = book_shelf.getBookAt(2);
        assert_eq!(book.getName(), "ハリーポッター3");

        let length = book_shelf.getLength();
        assert_eq!(length, 4);

        let book5 = Book::new("Rustで学ぶデザインパターン入門");
        book_shelf.appendBook(book5);

        let book = book_shelf.getBookAt(4);
        assert_eq!(book.getName(), "Rustで学ぶデザインパターン入門");

        let length = book_shelf.getLength();
        assert_eq!(length, 5);
    }


}