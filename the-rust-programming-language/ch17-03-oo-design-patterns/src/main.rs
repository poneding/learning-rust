// use core::str;
use std::str;

// Rust 面向对象设计模式实现：状态模式
fn main() {
    let mut post = Post::new();
    post.add_text("昨夜雨疏风骤，浓睡不消残酒");
    // assert_eq!("", post.content()); // Draft 状态博客不能获取内容

    let post = post.request_review();
    // assert_eq!("", post.content()); // PendingReview 状态博客不能获取内容

    let post = post.approve();
    assert_eq!("昨夜雨疏风骤，浓睡不消残酒", post.content()) // Published 状态博客可以获取内容
}

// trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     // 默认实现
//     fn content<'a>(&self, _: &'a Post) -> &'a str {
//         ""
//     }
// }

// struct Draft {}

// impl State for Draft {
//     // self: Box<Self> 语法意味着该方法只可在持有这个类型的 Box 上被调用
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }

// struct PendingReview {}

// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }
// }

// struct Published {}

// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     // 返回了 post 一部分的引用，所以需要生命周期注解
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
// }
pub struct Post {
    // state: Option<Box<dyn State>>, // Option<T> 类型的 Trait 对象
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    // pub fn new() -> Post {
    pub fn new() -> DraftPost {
        // Post {
        //     state: Some(Box::new(Draft {})),
        //     content: String::new(),
        // }

        // 改为创建一个 DraftPost
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    // pub fn add_text(&mut self, text: &str) {
    //     self.content.push_str(text);
    // }

    // // content 占位
    // pub fn content(&self) -> &str {
    //     self.state.as_ref().unwrap().content(self)
    // }

    // pub fn request_review(&mut self) {
    //     // take() 取出 Some 值并留下一个 None
    //     if let Some(state) = self.state.take() {
    //         self.state = Some(state.request_review())
    //     }
    // }

    // pub fn approve(&mut self) {
    //     if let Some(state) = self.state.take() {
    //         self.state = Some(state.approve())
    //     }
    // }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
