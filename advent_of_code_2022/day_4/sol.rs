use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args).unwrap();

    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut count = 0;
    
    for line in lines {
        let elfs: Vec<&str> = line.split(",").collect();
        let elf1: &str = elfs[0];
        let elf2: &str = elfs[1];

        let range1: Vec<i32> = elf1.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        let range2: Vec<i32> = elf2.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        
        let vec1: Vec<i32> = (range1[0]..range1[1] + 1).collect();
        let vec2: Vec<i32> = (range2[0]..range2[1] + 1).collect();

        // take intersection of vectors
        let intersection: Vec<i32> = vec1.into_iter().filter(|x| vec2.contains(x)).collect();

        if intersection.len() > 0 {
            count += 1;
        }
    }

    println!("There are {} many sections that overlap", count);
    
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
