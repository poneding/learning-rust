// 第一种方式
// mod struct1;
// fn main() {
//     let _s11 = struct1::Struct1 {};
//     print!("Hello, world!")
// }

// 第二种方式
mod mod2;
mod mod3;
mod mod4;
mod struct1;

use crate::mod2::struct2_1::Struct2_1;
use crate::mod3::struct3_1::Struct3_1;
use crate::mod3::struct3_2::Struct3_2;
use crate::mod4::Struct4;
use crate::mod4::Struct4_1;
use crate::struct1::Struct1;
fn main() {
    let _s1 = Struct1 {};

    let _s2 = mod2::Struct2 {};
    // let _s2_1 = mod2::struct2_1::Struct2_1 {};
    let _s2_1 = Struct2_1 {};

    let _s3 = mod3::Struct3 {};

    // let _s3_1 = mod3::struct3_1::Struct3_1 {};
    // let _s3_2 = mod3::struct3_2::Struct3_2 {};

    let _s3_1 = Struct3_1 {};
    let _s3_2 = Struct3_2 {};

    let _s4 = Struct4 {};
    let _s4_1 = Struct4_1 {};
    print!("Hello, world!")
}
