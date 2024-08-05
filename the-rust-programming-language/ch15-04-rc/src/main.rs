use std::rc::Rc;
/// Rc<T> 引用计数智能指针
/// 引用计数意味着记录一个值的引用数量来知晓这个值是否仍在被使用。
/// 如果某个值有零个引用，就代表没有任何有效引用并可以被清理。

#[allow(unused)]
fn main() {
    // Rc<T> 可以同享数据
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

#[allow(unused)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
