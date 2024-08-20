//! 引用循环与内存泄漏
//! 内存泄漏：当程序持续分配内存但不再使用时，会导致内存泄漏
//! 引用循环
fn main() {}

/// 制造一个引用循环
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
