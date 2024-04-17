fn main() {
    let mon = Weekday::Monday;
    println!("{:?}", mon);

    match_enum();
    let opt = Option::Some("Hello");
    println!("{:?}", opt);
    let age1 = 30;
    let result1 = old_man(age1);
    println!("{:?}", result1);

    let age2 = 150;
    let result2 = old_man(age2);
    println!("{:?}", result2);
    match_option();

    if_let();
    enum_with_type();
}

#[derive(Debug)]
#[allow(dead_code)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

enum Book {
    Papery { index: u32 },
    EBook { url: String },
}

// match 判断枚举类型
fn match_enum() {
    let mon = Weekday::Monday;
    match mon {
        Weekday::Monday => println!("Monday Matched"),
        Weekday::Tuesday => println!("Tuesday Matched"),
        Weekday::Wednesday => println!("Wednesday Matched"),
        Weekday::Thursday => println!("Thursday Matched"),
        Weekday::Friday => println!("Friday Matched"),
        Weekday::Saturday => println!("Saturday Matched"),
        Weekday::Sunday => println!("Sunday Matched"),
    }

    let book = Book::Papery { index: 1 };
    #[allow(unused_variables)]
    let eb = Book::EBook {
        url: "www.ebook.com/hello".to_string(),
    };
    match book {
        Book::Papery { index } => {
            println!("Papery index is {}", index);
        }
        Book::EBook { url } => {
            println!("EBook url is {}", url);
        }
    }

    // match 除了能够对枚举类进行分支选择以外，
    // 还可以对整数、浮点数、字符和字符串切片引用（&str）
    // 类型的数据进行分支选择。
    // 其中，浮点数类型被分支选择虽然合法，但不推荐这样使用，
    // 因为精度问题可能会导致分支错误。

    // 对非枚举类进行分支选择时必须注意处理例外情况，
    // 即使在例外情况下没有任何要做的事,
    // 例外情况用下划线 _ 表示：
    let s = "hello";
    match s {
        "hello" => {
            println!("HELLO WORLD")
        }
        _ => {}
    }

    let s2 = Option::Some(30);
    match s2 {
        // Option::Some(30) => { // YES
        Option::Some(32) => {
            // NO
            println!("YES")
        }
        _ => println!("No"),
    }
}

#[derive(Debug)]
// Option 枚举
enum Option<T> {
    // 返回一个值
    Some(T),
    // 用于返回 null，没有值的情况
    None,
}

fn old_man(age: i32) -> Option<bool> {
    if age > 100 {
        Option::Some(true)
    } else {
        Option::None
    }
}

fn match_option() {
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(some) => {
            println!("{}", some);
        }
        Option::None => {
            println!("opt is null");
        }
    }
}

// if let
fn if_let() {
    let i = 0;
    if let 0 = i {
        println!("zero");
    }

    let b = Book::EBook {
        url: "www.ebook.com/hello".to_string(),
    };
    if let Book::Papery { index } = b {
        println!("Papery index is {}", index);
    } else {
        println!("Not a Papery book");
    }
}

// 带数据类型的枚举
#[derive(Debug)]
enum Hello {
    Name(String),
}

fn enum_with_type() {
    let hello_jay = Hello::Name("Jay".to_string());
    match hello_jay {
        Hello::Name(name) => {
            println!("Hello, {}", name);
        }
    }
}
