fn main() {
    let x: u64 = 4_294_967_296;
    let y = x as u32; // u32 的范围是 0 到 4_294_967_295，总共 4_294_967_296 个数字
                      // u32 最大值为 4_294_967_295, 超出范围，会发生截断, 4294967296 % (4294967295 + 1) = 0

    if x == y as u64 {
        println!("x equals y.")
    } else {
        println!("x does not equal y.")
    }

    println!(" x: {}, y: {}", x, y);

    if x > u32::MAX as u64 {
        println!("conversion error: x > u32::MAX")
    }
}
