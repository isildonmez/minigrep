use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.keyword);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    
    println!("With text:\n{contents}");
}

struct Config {
    keyword: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let keyword = args[1].clone();
        let file_path = args[2].clone();

        Config { keyword, file_path }
    }
    
}

