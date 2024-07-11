fn main() {
    let mut count = 0;
    // 必须要以 ' 开头
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // rev() 反转 range
    for number in (1..11).rev() {
        println!("number: {number}")
    }

    let mut a = [10, 20, 30, 40, 50];
    a.reverse(); // 反转 vec
    for element in a {
        println!("element: {element}");
    }
}
