// create a struct Novel with title, author, genre (all String)
struct Novel {
  title: String,
  author: String,
  genre: String
}

// create a struct NonFiction with title, author, topic (all String)
struct NonFiction {
  title: String,
  author: String,
  topic: String
}

// create a trait Book
trait Book {
  fn get_summary(&self) -> ();
}

// implement Book trait for Novel
   // define get_summary which takes in &self
       // prints "<Book Title> is a <Book Genre> written by <Book Author>"
impl Book for Novel {
  fn get_summary(&self) {
      println!("{} is a {} written by {}", self.title, self.genre, self.author);
  }
}

// implement Book trait for NonFiction
   // define get_summary which takes in &self
       // prints "<Book Title> is a <Book Topic> written by <Book Author>"
impl Book for NonFiction {
  fn get_summary(&self) {
      println!("{} is a {} written by {}", self.title, self.topic, self.author);
  }
}

// define function which takes in a generic that that implements Book trait
   // call get_summary with the book
fn print_book_summary<T:Book> (book: T) {
  book.get_summary();
}

fn main() {
  // create book_1 of instance Novel
  let book_1 = Novel {
    title: String::from("Sherlock Holmes"),
    author: "Sir Conan Doyle".to_owned(),
    genre: "Mystery".to_string(),
  };
  
  // create book_2 of instance NonFiction
  let book_2 = NonFiction {
    title: String::from("Rust Book"),
    author: "God of Rust".to_owned(),
    topic: "Programming".to_string(),
  };

  // call print_book_summary with book_1
  // call print_book_summary with book_2
  print_book_summary(book_1);
  print_book_summary(book_2);
}