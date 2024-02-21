use std::fmt::Display;
use std::ptr::addr_of_mut;

fn main() {
    println!("Hello, world!");
    generic_collection();
    trait_demo();
    generic_func();
}

struct Data<T> {
    value: T,
}

fn generic_collection() {
    let d1: Data<i32> = Data { value: 100 };
    println!("d1: {}", d1.value);
    let d2: Data<f32> = Data { value: 99.99 };
    println!("d2: {}", d2.value);
}

// 特质：Trait
struct Book {
    name: String,
    id: u32,
    author: String,
}

trait ShowBook {
    fn Show(&self);
}

impl ShowBook for Book {
    fn Show(&self) {
        println!("Book Name: {}, Author: {}", self.name, self.author);
    }
}

fn trait_demo() {
    let book = Book {
        name: "Greatest Works of Art".to_string(),
        id: 1,
        author: "Jay".to_string(),
    };
    book.Show();
}

// 范型函数
fn show2<T: Display>(t: T) {
    println!("{}", t);
}

impl Display for Book {
    // fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    //     write!(f, "--- Book Name: {}, Author: {}", self.name, self.author)
    // }

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("*** Book Name: {}, Author: {}", self.name, self.author);
        let r = Result::Ok(());
        return r;
    }
}

fn generic_func() {
    let book = Book {
        name: "Greatest Works of Art".to_string(),
        id: 1,
        author: "Jay".to_string(),
    };
    show2(book);
}
