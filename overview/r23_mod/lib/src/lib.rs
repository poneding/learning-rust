pub mod chinese {
    use crate::privatemod::print;

    pub fn hello() {
        print("chinese");
        println!("你好，世界！");
        hello_again();
    }


    fn hello_again() {
        println!("再次你好!");
    }
}

pub mod english {
    pub fn hello() {
        crate::privatemod::print("english");
        println!("Hello, world!");
    }
}

mod privatemod {
    pub fn print(from: &str) {
        println!("greeting from {}", from);
    }
}