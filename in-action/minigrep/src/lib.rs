use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file).expect("请指定可以读取的文件！");
    println!("contents: \n{contents}");
    Ok(())
}

pub struct Config {
    query: String,
    file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            // panic!("请传入足够的参数！")
            return Err(String::from("请传入足够的参数！"));
        }

        Ok(Config {
            query: args[1].clone(),
            file: args[2].clone(),
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // TODO
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
