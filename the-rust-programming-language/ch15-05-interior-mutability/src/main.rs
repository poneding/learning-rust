use std::cell::RefCell;
use std::rc::Rc;

/// 内部可变性（Interior mutability）是 Rust 中的一个设计模式，
/// 它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的。
/// 需要用到 unsafe 代码来绕过可变性借用规则。
fn main() {
    // 内部可变性，不可变值的可变借用
    let a = RefCell::<Vec<&str>>::new(vec!["Hello"]);
    a.borrow_mut().push("World");
    let b = a.borrow().join(", ");

    println!("b: {b}"); // b: Hello, World

    main2();
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: you are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!")
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self.sent_messages.push(String::from(message)) // 不可变借用

            // let mut one_brorrow = self.sent_messages.borrow_mut();
            // let mut two_brorrow = self.sent_messages.borrow_mut(); // already borrowed: BorrowMutError
            // one_brorrow.push(String::from(message));
            // two_brorrow.push(String::from(message));

            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_wanrning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// 结合 RefCell<T> 和 Rc<T> 来拥有多个可变数据所有者
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main2() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a after = {a:?}");
    if let Cons(ref head, ref tail) = *a {
        println!("head = {head:?}");
        println!("tail = {tail:?}");
    }

    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
