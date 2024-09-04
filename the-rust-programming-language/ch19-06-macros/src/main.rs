/// 宏：Macro，Rust 中一系列的功能
/// - 使用 macro_rules! 的声明宏
/// - 三种过程宏：
///     1. 自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
///     2. 类属性（Attribute-like）宏定义可用于任意项的自定义属性
///     3. 类函数宏，看起来像函数，但是作用于作为参数传递的 token
fn main() {
    test_myvec();
}

// vec![1,2,3]
// 函数：我们无法预先知道参数值的数量和类型，所以通过 vec! 宏来完成这样的初始化
// 实现 myvec
#[macro_export]
macro_rules! myvec {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

fn test_myvec() {
    let v = myvec![1, 2, 3];
    for i in v {
        println!("{:?}", i);
    }
}
