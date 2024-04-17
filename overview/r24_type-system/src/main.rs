fn main() {
    str2num();
    cast1();
    cast2();
    alias();
}

// 字符串转数字
fn str2num() {
    let s = "123";
    let n1: i32 = s.parse().unwrap();
    // 或者
    let n2 = s.parse::<i32>().unwrap();
    println!("n1: {}, n2: {}", n1, n2);
}

// 使用 as 关键字进行显示的类型转换
fn cast1() {
    let a = 1; // i32
    let b = a as f64;
    println!("b: {}", b);
}

// 使用 Trait 解决类型转换问题，一般使用到 From 和 Into 两个 Trait
fn cast2() {
    let s1 = "Hello World!";
    #[allow(unused)]
    let s2 = String::from(s1);

    let s: Student = Student {
        name: String::from("张三"),
        grade: 5,
        age: 12,
    };

    let p: Person = Person::from(s);
    println!("p: {:?}", p);

    let s2: Student = p.into(); // 但这里必须显示的指定 s2 的类型为 Student
    println!("s2: {:?}", s2);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Student {
    name: String,
    #[allow(unused)]
    grade: u8,
    age: u8,
}

// From
impl From<Person> for Student {
    fn from(person: Person) -> Student {
        Student {
            name: person.name,
            grade: 1,
            age: person.age,
        }
    }
}

impl From<Student> for Person {
    fn from(student: Student) -> Person {
        Person {
            name: student.name,
            age: student.age,
        }
    }
}

// Into
// 如果为类型实现了 From，那么则自动就已经有了 Into的实现了，
// 以下代码其实是可以不用写的
// impl Into<Person> for Student {
//     fn into(self) -> Person {
//         Person {
//             name: self.name,
//             age: self.age,
//         }
//     }
// }

// 字面量
#[allow(unused)]
fn literal() {
    let a = 1; // i32
    let b = 1.0; // f64
    let c = 1.0f32; // f32
    let d = 1.0f64; // f64
    let e = 1.0_f32; // f32
    let f = 1.0_f64; // f64
    let g = 1.0_f32 as f64; // f64
    let h = 1.0_f64 as f32; // f32
    let i = 1.0_f64 as i32; // i32
    let j = 1.0_f32 as i32; // i32
    let k = 1.0_f64 as i32; // i32
    let l = 1.0_f32 as i32; // i32
    let m = 1.0_f64 as i32; // i32
    let n = 1.0_f32 as i32; // i32
    let o = 1.0_f64 as i32; // i32
    let p = 1.0_f32 as i32; // i32
    let q = 1.0_f64 as i32; // i32
    let r = 1.0_f32 as i32; // i32
    let s = 1.0_f64 as i32; // i32
    let t = 1.0_f32 as i32; // i32
    let u = 1.0_f64 as i32; // i32
    let v = 1.0_f32 as i32; // i32
    let w = 1.0_f64 as i32; // i32
    let x = 1.0_f32 as i32; // i32
    let y = 1.0_f64 as i32; // i32
    let z = 1.0_f32 as i32; // i32
    let aa = 1.0_f64 as i32; // i32
    let bb = 1.0_f32 as i32; // i32
    let cc = 1.0_f64 as i32; // i32
}

// 类型推断
#[allow(unused)]
fn type_inference() {
    let mut vec = Vec::new(); // 空向量，目前不知道元素类型
    vec.push(1); // 向量中添加元素，类型推断为 i32
    println!("vec: {:?}", vec);
}

// 别名
#[allow(unused)]
fn alias() {
    type MyI32 = i32;
    type MyU64 = u64;
    type HisU64 = u64;
    let a: MyI32 = 1;
    let b: MyU64 = 1;
    let c: HisU64 = 1;
    println!("b + c = {}", b + c); // 底层同一类型可想加
}
