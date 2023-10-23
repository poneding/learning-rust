// 为自定义类型实现fmt::Display
#![ allow(dead_code)]
use std::fmt;
#[derive(Debug)]
struct Point2D{
    x:f64,
    y:f64,
}

impl fmt::Display for Point2D{
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"x:{}, y:{}",self.x,self.y)

    }
}

pub fn run_print(){
	let p = Point2D{x:22f64,y:23f64};
    println!("Debug: {:#?}",p);
    println!("Display: {}",p);

    let pi = 3.1415926;
    println!("pi: {:.3}",pi);
}

// 输出
// Debug: Point2D {
//     x: 22.0,
//     y: 23.0,
// }
// Display: x:22, y:23
// pi: 3.142
