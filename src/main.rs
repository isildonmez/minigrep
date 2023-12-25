use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let keyword = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", keyword);
    println!("In file {}", file_path);
}
