use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn read_file(path: &str) -> Result<i32, std::io::Error> {
    // Read file line by line
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // let mut parts: Vec<&str> = Vec::new();
    let mut total = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let card: Vec<&str> = line.split(":").collect();
                let card_parts: Vec<&str> = card[1].trim().split("|").collect();
                let my_numbers: Vec<&str> = card_parts[1].trim().split(" ").collect();
                let unique_numbers: HashSet<&str> = HashSet::from_iter(my_numbers);
                let winning_numbers = card_parts[0].trim().split(" ").collect::<Vec<&str>>();
                let mut output_numbers = Vec::new();
                let mut score: i32 = 0;
                for number in winning_numbers {
                    if number == "" {
                        continue;
                    }
                    if unique_numbers.contains(&number) {
                        output_numbers.push(number);
                        if score == 0 {
                            score = 1;
                        }else {
                            score *= 2;
                        }
                    }
                }
                println!("{} = {}", output_numbers.join(" "), score);
                output_numbers.clear();
                total += score
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    return Ok(total);
}
pub fn run() {
    match read_file("src/day_4/input.txt") {
        Ok(contents) => {
            println!("Calibration Value: {}", contents);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
