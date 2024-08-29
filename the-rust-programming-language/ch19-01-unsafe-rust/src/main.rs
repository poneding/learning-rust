//! 不安全的代码 unsafe
//! 1. 解引用裸指针
//! 2. 调用不安全的函数或者方法
//! 3. 访问或修改可变静态变量
//! 4. 实现不安全的 trait
//! 5. 访问 union 的字段

fn main() {
    m1();
    println!("--------------------------");
    m2();
    println!("--------------------------");
    m3();
    println!("--------------------------");
    m4();
    println!("--------------------------");
    m5();
}

fn m1() {
    // 裸指针：*const T、*mut T
    let mut num = 1;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 在 unsafe 代码块中解引用
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }
}

unsafe fn dangerous() {
    println!("dangerous function called.")
}

fn m2() {
    unsafe {
        dangerous();
    }
}

fn m3() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);

    // 自定义实现
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("a: {:?}", a);
    println!("b: {:?}", b);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    // 下面这行编译器检测到 values 被借用两次了，无法通过编译
    // (&mut values[..mid], &mut values[mid..])

    // 但其实这两部分不会重叠，我们使用 unsafe 实现同样的逻辑
    let ptr = values.as_mut_ptr();
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn m4() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

static mut HELLO: &str = "Hello, World";

fn change_hello(s: &'static str) {
    unsafe {
        HELLO = s;
    }
}

fn m5() {
    change_hello("Hello, Rust");
    unsafe {
        println!("HELLO is: {HELLO}");
    }
}
