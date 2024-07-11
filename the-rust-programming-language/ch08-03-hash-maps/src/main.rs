// 哈希 map
// 引入
use std::{self, collections::HashMap, io};

#[allow(unused)]
fn main1() {
    // 定义
    let mut scores = HashMap::new();
    // 插入数据
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 访问数据
    let blue_score = scores.get("Blue").copied().unwrap_or(0);
    // get() 返回 Option<&i32>，通过 copied() 返回 Option<i32>
    println!("blue_score: {blue_score}"); // 50

    for (k, v) in &scores {
        println!("key: {k}, value: {v}")
    }

    // 更新数据
    scores.insert(String::from("Blue"), 25);
    let blue_score = scores.get("Blue").copied().unwrap_or(0);
    println!("blue_score: {blue_score}"); // 25

    // 不存在时插入数据，存在时略过
    scores.entry(String::from("Blue")).or_insert(10); // 插入无效，又因为 key：Blue 已经存在
    scores.entry(String::from("Red")).or_insert(30); // 有效
    println!("scores: {scores:?}"); // scores: {"Blue": 25, "Yellow": 50, "Red": 30}

    count_word();
}

fn main() {
    exercie3();
}

fn count_word() {
    let text = "hello world wonderful world";

    // let mut words = HashMap::new();
    let mut words = HashMap::<&str, i32>::new();
    for word in text.split_whitespace() {
        // let count = words.entry(word).or_insert(0); // 编译器无法判定 value 类型，所以需要显示默认为 0
        let count = words.entry(word).or_default();
        *count += 1;
    }
    println!("words: {words:?}")
    // words: {"hello": 1, "world": 2, "wonderful": 1}
}

// 练习 1：
// 给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）
// 和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
// cargo test exercie1
#[test]
fn exercie1() {
    let nums = vec![1, 2, 3, 4, 4, 4, 5, 5, 5, 5, 7, 8];
    // 中位数是 4.5，众数是 5

    match median(&nums) {
        Some(m) => assert_eq!(4.5, m),
        None => println!("no mediam num returned."),
    }

    match mode(&nums) {
        Some(m) => assert_eq!(5, m),
        None => println!("no mode num returned."),
    }
}

#[allow(unused)]
fn median(nums: &Vec<i32>) -> Option<f32> {
    let nums_len = nums.len();
    if nums_len == 0 {
        return None;
    }

    let mut sorted_nums = nums.clone();
    sorted_nums.sort();

    if nums_len % 2 == 0 {
        let m1 = sorted_nums[nums_len / 2 - 1];
        let m2 = sorted_nums[nums_len / 2];
        Some((m1 + m2) as f32 / 2.0)
    } else {
        Some(sorted_nums[nums_len / 2] as f32)
    }
}

#[allow(unused)]
fn mode(nums: &Vec<i32>) -> Option<i32> {
    if nums.len() == 0 {
        return None;
    }

    let mut num_counts = HashMap::new();
    for n in nums.iter() {
        let count = num_counts.entry(n).or_insert(0);
        *count += 1;
    }

    num_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(val, _)| *val)
}

// 练习 2：
// 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词
// 的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。元音字母开头的单
// 词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
// cargo test exercie2 -- --nocapture
#[test]
fn exercie2() {
    let text = "first apple string example";
    let latin = to_pig_latin(text);
    println!("latin: {latin}");
    assert_eq!("irst-fay apple-hay tring-say example-hay", latin.as_str());
}

#[allow(unused)]
fn to_pig_latin(text: &str) -> String {
    text.split_whitespace()
        .into_iter()
        .map(|word| {
            let first_char = word.chars().next().unwrap();
            if is_vowel(first_char) {
                format!("{word}-hay")
            } else {
                let mut chars = word.chars();
                let first_char = chars.next().unwrap();
                format!("{}-{}ay", chars.collect::<String>(), first_char)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

// is_vowel 判断字符是否是元音字符 a,e,i,o,u
#[allow(unused)]
fn is_vowel(c: char) -> bool {
    matches!(
        c.to_lowercase().next().unwrap(),
        'a' | 'e' | 'i' | 'o' | 'u'
    )
}

// 练习 3：
// 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工
// 的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。
// 接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
// #[test]
fn exercie3() {
    let mut company = HashMap::new();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("Bye~");
            break;
        }

        let parts = input.split_whitespace().collect::<Vec<&str>>();
        if parts.len() < 2 {
            println!("Invalid command.");
            continue;
        }

        match parts[0].to_lowercase().as_str() {
            "add" => {
                if parts.len() != 4 || parts[2].to_lowercase() != "to" {
                    println!(
                        "Invalid command for adding employee. Use 'Add <name> to <department>'"
                    )
                }
                let name = parts[1].to_string();
                let department = parts[3].to_string();
                let employees = company.entry(department).or_insert(Vec::<String>::new());
                if !employees.contains(&name) {
                    employees.push(name)
                }
            }
            "list" => {
                if parts[1].to_lowercase().as_str() == "all" {
                    for (department, department_employees) in &company {
                        list_department_employee(department, &department_employees);
                    }
                } else {
                    let department = parts[1].to_string();
                    if let Some(employees) = company.get(&department) {
                        list_department_employee(&department, &employees)
                    }
                }
            }
            _ => {
                println!("Unknown command.");
                continue;
            }
        }
    }
}

fn list_department_employee(department: &String, department_employees: &Vec<String>) {
    let mut sorted_department_employees = department_employees.clone();
    sorted_department_employees.sort();
    for employee in sorted_department_employees {
        println!("Employee in {department}: {employee:?}");
    }
}
