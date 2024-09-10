use std::mem;
fn _mainv1() {
    let r1 = _gcd2(48, 36);
    println!("The greatest common divisor of 48 and 36 is {}", r1);

    let r2 = _gcd2(15, 14);
    println!("The greatest common divisor of 15 and 14 is {}", r2);
}

// 计算两个数的最大公约数
pub fn _gcd(mut n: u64, mut m: u64) -> u64 {
    // assert!(n != 0 && m != 0); // assertion failed: n != 0 && m != 0
    assert!(n != 0 && m != 0, "n or m can't be zero");
    print!("The greatest common divisor of {} and {} is ", n, m);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n; // 大数对小数取余，如果余数不为0，继续循环
    }
    print!("{}\n", n);
    n
}

// 计算两个数的最大公约数
pub fn _gcd2(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0, "n or m can't be zero");
    print!("The greatest common divisor of {} and {} is ", n, m);
    loop {
        if m == 0 {
            print!("{}\n", n);
            return n;
        }
        if m < n {
            mem::swap(&mut m, &mut n);
        }
        m = m % n;
    }
}

// cargo test
#[test]
fn test_gcd() {
    assert_eq!(_gcd(14, 15), 1);
    assert_eq!(_gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
    print!("test_gcd passed\n")
}

#[test]
fn test_gcd2() {
    assert_eq!(_gcd2(14, 15), 1);
    assert_eq!(_gcd2(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
    print!("test_gcd2 passed\n")
}
