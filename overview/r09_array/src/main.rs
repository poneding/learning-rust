fn main() {
    // 数组：
    // 1. 相同类型，在编译时就可以确定类型
    // 2. 在内存中连续存储，在栈中分配

    let arr1: [&str; 2] = ["Hello", "Jay"];
    let arr2 = [1, 2, 3, 4, 5];

    let arr3: [i32; 3] = [60; 3]; // 默认值 60

    // len
    println!("len of arr1: {}", arr1.len());

    // iter
    for n1 in arr2 {
        println!("n1: {}", n1)
    }
    for n2 in arr2.iter() {
        println!("n2: {}", n2);
    }

    // 不可变数组不可修改，如果修改，数组声明时需要添加 mut 关键字
    let mut arr3 = [1, 2, 3, 4];
    let l = arr3.len();
    for i in 0..l {
        arr3[i] = arr3[i] * arr3[i];
    }
    println!("arr3: {:?}", arr3);

    let mut str_arr = ["Vue", "Golang", "Rust"];
    println!("before show_arr2: {:?}", str_arr);
    show_arr2(&mut str_arr);
    println!("after show_arr2: {:?}", str_arr);
}

// 引用传递
fn show_arr2(arr: &mut [&str; 3]) {
    let l = arr.len();
    for i in 0..l {
        arr[i] = "别卷了"
    }
    println!("in show_arr2: {:?}", arr)
}
