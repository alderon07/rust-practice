use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args).unwrap();

    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();


    let mut sum:u32 = 0;
    let mut group: String = String::new();
    for (line_number, line) in lines.iter().enumerate() {
        // let length: usize = line.len();
        // let index: usize = length / 2;
        let temp_str: String = line.to_string() + &'\n'.to_string();
        group.push_str(&temp_str);
        println!("{}", group);
 
        if (line_number + 1) % 3 != 0 {
            continue;
        }
        
        // let (first_compartment, second_compartment): (&str, &str)  = temp_str.split_at(index);
        println!("line: {}", line);
        let strings: Vec<&str> = group.split('\n').into_iter().collect();
        let first: &str = strings[0];
        let second: &str = strings[1];
        let third: &str = strings[2];

        let common_char: char = get_common_character(first, second, third).unwrap();

        let char_ascii_value: u8 = common_char as u8;

        let mut priority: u32;
        
        // a-z = 97-122
        if char_ascii_value >= 97 && char_ascii_value <= 122 {
            priority = (char_ascii_value - 96) as u32;
            sum += priority;
            // println!("compartment1: {}, compartmen2: {}, common_char: {}, priority: {}", first_compartment, second_compartment, common_char, priority);
        }

        // A-Z = 65-90
        if char_ascii_value >= 65 && char_ascii_value <= 90 {
            priority = (char_ascii_value - 38) as u32;
            sum += priority;
            // println!("compartment1: {}, compartmen2: {}, common_char: {}, priority: {}", first_compartment, second_compartment, common_char, priority);
        }

        group.clear();
    }
    
    println!("sum: {}", sum);
    
}

fn get_common_character(first: &str, second: &str, third: &str) -> Option<char> {
    let set: HashSet<char> = second.chars().collect();
    let set2: HashSet<char> = third.chars().collect();
    for char in first.chars() {
        if set.contains(&char) && set2.contains(&char) {
            return Some(char);
        }
    }
    None
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
