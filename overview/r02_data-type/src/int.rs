pub fn int() {
    let _price = 100;
    let price2: u32 = 200;
    let price3: i32 = -300;
    let price4: isize = 400;
    let price5: usize = 500;

    let _idec_1 = 98222;
    let _idec = 98_222; // decimal，可以使用下划线分隔，以便于阅读
    let _ihex = 0xff; // hexadecimal，以0x或0X开头
    let _ioctal = 0o77; // octal, 以0o或0O开头
    let _ibinary = 0b1111_0000; // binary
    let _ibyte = b'A'; // byte(u8 only)

    println!("price2 is {} and price3 is {}", price2, price3);
    println!("price4 is {} and price5 is {}", price4, price5);
}
