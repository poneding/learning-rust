// 最简单直接的智能指针：Box<T>
// 允许你将一个值放在堆上，而不是栈上，留在栈上的是指向堆数据的指针
fn main() {
    let b = Box::new(5); // 在堆上存储了一个 i32
    println!("b = {b}");
    // 离开作用域后，指向堆上的数据将被释放

    // let _list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
    let _list: List = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
}

// 嵌套列表 Cons List 伪代码：
// (1, (2, (3, Nil)))

// 使用枚举定义，报错提示
// recursive type `List` has infinite size
// 因为无法得知类型的大小
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// 使用 Box<T> 实现 Cons List
#[allow(unused)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
