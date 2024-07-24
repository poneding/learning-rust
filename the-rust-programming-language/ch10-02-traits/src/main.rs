use std::{fmt::Display, iter::Sum};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&article);
    notify2(&article);
}

// Trait 特征
//
// 定义 Trait
pub trait Summary {
    // fn summarize(&self) -> String; // 默认不实现

    // 默认实现
    // fn summarize(&self) -> String {
    //     String::from("(Read more...)")
    // }

    // 默认实现调用
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 为结构实现特征
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// trait 作为参数
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound 语法糖
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news from notify2! {}", item.summarize());
}

// 通过 + 指定多个 trait
pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news from notify3! {}", item.summarize());
}

pub fn notify4<T: Summary + Display>(item: &T) {
    println!("Breaking news from notify4! {}", item.summarize());
}

// where 方式定义 trait bound
pub fn notify5<T>(item: &T)
where
    T: Display + Summary,
{
    println!("Breaking new from notify5! {}", item.summarize())
}

// 返回实现 trait
fn _return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
