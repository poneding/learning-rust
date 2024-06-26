fn main() {
    let s = String::from("Hello world!");

    // let word = _first_word_1(&s); // 5

    // s.clear();
    // word 不再具有意义

    let word = first_word(&s);

    println!("first word: {word}");
}

// 返回第一个单词末尾的 index
fn _first_word_1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // iter() 返回每个元素
        // enumerate() 返回索引和元素的元组
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// 返回第一个单词字符串字面值
fn _first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // iter() 返回每个元素
        // enumerate() 返回索引和元素的元组
        if item == b' ' {
            return &s[..i];
        }
    }
    // s.as_str()
    &s[..]
}

// 参数为 s: &str，既可以传入 &String，也可以传入 &str
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // iter() 返回每个元素
        // enumerate() 返回索引和元素的元组
        if item == b' ' {
            return &s[..i];
        }
    }
    // s.as_str()
    &s[..]
}
