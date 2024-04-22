fn main() {
    println!("Hello, world!");
}

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    #[allow(dead_code)]
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // string is a
            // TODO: Complete the function body. You can do it!
            match command {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().into()),
                Command::Append(n) => {
                    let mut appended: String = string.clone();
                    // for _ in 0..*n {
                    //     appended.push_str("bar");
                    // }
                    appended.push_str("bar".repeat(*n).as_str());
                    output.push(appended);

                    // 怎么使用 repeat 方法？
                    // output.push(format!("{}{}", string, "bar".repeat(*n)))
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
