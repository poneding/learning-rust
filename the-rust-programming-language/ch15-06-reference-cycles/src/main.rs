//! 引用循环与内存泄漏
//! 内存泄漏：当程序持续分配内存但不再使用时，会导致内存泄漏
//! 引用循环
fn main() {
    // 循环引用示例
    // main1();
    // 使用 Weak<T> 取代 Rc<T> 避免循环引用
    main2();
}

/// 制造一个引用循环
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
#[derive(Debug)]
#[allow(unused)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

#[allow(unused)]
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[allow(unused)]
fn main1() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initianl rc count: {}", Rc::strong_count(&a)); // 1
    println!("a next item: {:?}", a.tail()); // Some(RefCell { value: Nil })

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a)))); // clone +1
    println!("a rc count after b created: {}", Rc::strong_count(&a)); // 2
    println!("b initianl rc count: {}", Rc::strong_count(&b)); // 1
    println!("b next item: {:?}", b.tail()); // Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); // 修改 a 指向 b，形成了循环引用，b 引用 a，a 也引用了 b
    }
    println!("a rc count after a changed: {}", Rc::strong_count(&a)); // 2
    println!("b rc count after a changed: {}", Rc::strong_count(&a)); // 2

    // 如果运行下面这行语句，栈内存将溢出
    // println!("a next item: {:?}", a.tail()); // stack overflow
}

/// 使用 Weak<T> 取代 Rc<T> 避免循环引用
/// Rc::clone 会增加 Rc<T> 实例的 strong_count，在其 strong_count 为 0 时才会被清理
fn main2() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong: {}, weak: {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf)    // 0
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong: {}, weak: {}",
            Rc::strong_count(&branch), // 1
            Rc::weak_count(&branch)    // 1
        );

        println!(
            "leaf strong: {}, weak: {}",
            Rc::strong_count(&leaf), // 2
            Rc::weak_count(&leaf)    // 0
        );

        println!("leaf parent: {:?}", leaf.parent.borrow().upgrade()); // Some(...)
    }

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade()); // None

    println!(
        "leaf strong: {}, weak: {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf)    // 0
    );
}

// 一个带有子节点的 Node
#[derive(Debug)]
#[allow(unused)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>, // 使用 RefCell 就是希望能修改子节点
}
