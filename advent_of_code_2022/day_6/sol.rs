use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args).unwrap();

    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut result: i32 = 0;
    for line in lines {
        for i in 0..line.len() {
            let end = i + 14;
            if end < line.len() {
                let slice = &line[i..end];
                println!("{}", slice);
                if !contains_duplicates(slice) {
                    result = end as i32;
                    break;
                }
            }
        }
    }

    println!("index {}", result);
}

fn contains_duplicates(str: &str) -> bool {
    let chars: Vec<char> = str.chars().collect();
    let seen: HashSet<&char> = chars.iter().collect();
    return seen.len() != chars.len();
}

struct Config {
    file_path: String,
}

fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 1 {
        return Err("not enough arguments");
    }
    let file_path = args[1].clone();
    let config: Config = Config { file_path };
    Ok(config)
}
