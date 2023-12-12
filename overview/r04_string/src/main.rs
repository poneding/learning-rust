fn main() {
    // &str: 字符串字面量，编译时就知道值的字符串类型，是字符char的集合，静态
    // String: 字符串对象，pub 结构体，存储在堆上
    let name ="Jay";

    let s1 = String::new();
    let s2 = String::from("Hello, 世界");

    println!("s1: {}, len of s1: {}",s1,s1.len());
    println!("s2: {}, len of s2: {}",s2,s2.len());

    let mut s3=String::new();
    s3.push_str("Hello");
    println!("s3: {}",s3);

    s3.push(',');
    s3.push_str("World");
    println!("s3: {}",s3);

    let mut s4=String::from("Hello Jay");
    let result = s4.replace("Jay","周杰伦");
    println!("result: {}",result);

    // &str 转 String
    let s5="Hello".to_string();
    println!("s5: {}",s5);

    // String 转 &str
    let s=String::from("Halo");
    show_name(s.as_str());

    str_trim();

    str_split();
    str_chars();
    str_add();

}

fn show_name(name:&str){
    println!("name is {}",name);
}

fn str_trim(){
    println!("{}","  hello world  ".trim());
}

fn str_split() {
    let s="Hello Jay Chou!";
    for item in s.split(' '){
        println!("item: {}",item);
    }
}

fn str_chars(){
    let s="Hello Jay Chou!";
    for c in s.chars(){
        println!("c: {}",c);
    }
}

fn str_add(){
    let s1="Hello".to_string();
    let s2=" World".to_string();
    let s=s1+&s2;
    println!("s :{}",s);
}