use std::{fs, error::Error};

pub struct Config {
    pub keyword: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let keyword = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { keyword, file_path })
    } 
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    println!("With text:\n{contents}");

    Ok(())
}
