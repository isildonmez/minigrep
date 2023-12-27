use std::{fs, error::Error, env};

pub struct Config {
    pub keyword: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let keyword = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a keyword string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { keyword, file_path, ignore_case })
    } 
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.keyword, &contents)
    } else {
        search(&config.keyword, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(keyword: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(keyword))
        .collect()
}

pub fn search_case_insensitive<'a>(keyword: &str, contents: &'a str) -> Vec<&'a str> {
    let keyword = keyword.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&keyword))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let keyword = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(keyword, contents));
    }

    #[test]
    fn case_insensitive() {
        let keyword = "RUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(keyword, contents));
    }

}

