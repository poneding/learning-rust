use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file).expect("请指定可以读取的文件！");
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    query: String,
    file: String,
    ignore_case: bool,
}

impl Config {
    // pub fn build(args: &[String]) -> Result<Config, String> {
    //     if args.len() < 3 {
    //         // panic!("请传入足够的参数！")
    //         return Err(String::from("请传入足够的参数！"));
    //     }

    //     Ok(Config {
    //         query: args[1].clone(),
    //         file: args[2].clone(),
    //         ignore_case: env::var("IGNORE_CASE").is_ok(), // 是否设置了 IGNORE_CASE 环境变量
    //     })
    // }

    // impl Iterator for Args {
    //     type Item = String;
    // Args 实现了迭代器，可以获取其所有权，而借用 slice，我们无法获取所有权，也就只能使用 clone 获取参数值。
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("请传入查询关键字！"),
        };

        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("请传入查询文件路径！"),
        };

        Ok(Config {
            query,
            file,
            ignore_case: env::var("IGNORE_CASE").is_ok(), // 是否设置了 IGNORE_CASE 环境变量
        })
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }
    // result

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let query = query.to_lowercase();
    // let mut result = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line);
    //     }
    // }
    // result

    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
