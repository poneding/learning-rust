fn main() {
    // 元组： tuple 是复合类型，存储多个不同类型的数据
    let t1:(&str,&str)=("Hello","World");
    println!("{:?}",t1);

    println!("t1.0: {}",t1.0);
    println!("t1.1: {}",t1.1);
    // println!("t1.2: {}",t1.2); // error[E0609]: no field `2` on type `(&str, &str)`

    show_tuple(t1);

    // 元组结构
    let (s1,s2)=t1;
    println!("s1: {}",s1);
    println!("s2: {}",s2);
}

fn show_tuple(t:(&str,&str)){
    println!("show_tuple print: {:?}",t)
}

