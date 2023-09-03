use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args).unwrap();

    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let guess_to_score: HashMap<char, i32> = HashMap::from([
        ('A', 1),
        ('B', 2), 
        ('C', 3),
        ('X', 1),
        ('Y', 2),
        ('Z', 3),
    ]);

    let guess_to_value: HashMap<char, String> = HashMap::from([
        ('A', String::from("Rock")), 
        ('B', String::from("Paper")), 
        ('C', String::from("Scissors")),
        ('X', String::from("Rock")), 
        ('Y', String::from("Paper")), 
        ('Z', String::from("Scissors")),
    ]);

    let value_to_guess: HashMap<String, char> = HashMap::from([
        (String::from("Rock"), 'X'), 
        (String::from("Paper"), 'Y'), 
        (String::from("Scissors"), 'Z'),
    ]);

    let result_to_score: HashMap<char, i32> = HashMap::from([
        ('w', 6),
        ('l', 0),
        ('d', 3),
    ]);

    let x_beats_y: HashMap<String, String> = HashMap::from([
        (String::from("Rock"), String::from("Scissors")),
        (String::from("Paper"), String::from("Rock")),
        (String::from("Scissors"), String::from("Paper")),
    ]);

    let x_loses_y: HashMap<String, String> = HashMap::from([
        (String::from("Scissors"), String::from("Rock")),
        (String::from("Rock"), String::from("Paper")),
        (String::from("Paper"), String::from("Scissors")),
    ]);

    let mut score_sum: i32 = 0;
    println!("{:?}", lines.len());

    
    for line in lines {
        let guesses: Vec<char> = line.split(" ").into_iter().filter_map(|s| s.parse::<char>().ok()).collect();

        // println!("{:?}", guesses);
        let enemy_guess: char = guesses[0];
        let my_guess = match guesses[1] {
            'X' => {
               value_to_guess[&x_beats_y[&guess_to_value[&enemy_guess]]]
            },
            'Y' => enemy_guess,
            'Z' => {
                value_to_guess[&x_loses_y[&guess_to_value[&enemy_guess]]]
            },
            _ => {
                'z'
            }
        };

        println!("{} {}", enemy_guess, my_guess);

        println!("{} {}", guess_to_value[&enemy_guess], guess_to_value[&my_guess]);
        
        if guess_to_value[&enemy_guess] == guess_to_value[&my_guess] {
            let score1:&i32 = result_to_score.get(&'d').unwrap();
            let score2:&i32 = guess_to_score.get(&my_guess).unwrap();
            score_sum += score1 + score2;
        }
        else if x_beats_y[&guess_to_value[&enemy_guess]] != guess_to_value[&my_guess] {
            let score1:&i32 = result_to_score.get(&'w').unwrap();
            let score2:&i32 = guess_to_score.get(&my_guess).unwrap();
            score_sum += score1 + score2;
        }
        else {
            score_sum += result_to_score[&'l'] + guess_to_score[&my_guess];
        }
    }

    println!("Score: {}", score_sum);

    // get top three values and add them together
    // if let Some(result) = sums.iter().max() {
    //     println!("Maximum value: {}", result);
    // } else {
    //     println!("The vector is empty.");
    // }
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
