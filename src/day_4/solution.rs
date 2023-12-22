use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Result<i32, std::io::Error> {
    // Read file line by line
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // let mut parts: Vec<&str> = Vec::new();
    let mut total = 0;
    let mut card_mapping: HashMap<u32, u32> = HashMap::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let card: Vec<&str> = line.split(":").collect();
                let card_parts: Vec<&str> = card[1].trim().split("|").collect();
                let my_numbers: Vec<&str> = card_parts[1].trim().split(" ").collect();
                let unique_numbers: HashSet<&str> = HashSet::from_iter(my_numbers);
                let winning_numbers = card_parts[0].trim().split(" ").collect::<Vec<&str>>();
                let mut score: u32 = 0;
                let card_number: u32 = card[0].split(" ").collect::<Vec<&str>>()[1]
                    .trim()
                    .parse::<u32>()
                    .unwrap();
                if card_mapping.contains_key(&card_number) {
                  
                    score = *card_mapping.get(&card_number).unwrap();
                    card_mapping.insert(card_number, score + 1);
                    for _i in 0..score {
                        fun_name(
                            winning_numbers.clone(),
                            unique_numbers.clone(),
                            &mut card_mapping,
                            card_number + 1,
                            score,
                        );
                    }
                    fun_name(
                        winning_numbers.clone(),
                        unique_numbers.clone(),
                        &mut card_mapping,
                        card_number + 1,
                        score,
                    );
                } else {
                    card_mapping.insert(card_number, 1);
                    fun_name(
                        winning_numbers,
                        unique_numbers,
                        &mut card_mapping,
                        card_number + 1,
                        score,
                    );
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    for (key, value) in card_mapping.iter() {
      
            total += *value as i32;
        
    }
    println!("{:?}", card_mapping);
    return Ok(total);
}

fn fun_name(
    winning_numbers: Vec<&str>,
    unique_numbers: HashSet<&str>,
    card_mapping: &mut HashMap<u32, u32>,
    mut card_number: u32,
    mut score: u32,
) {
    for number in winning_numbers {
        if number == "" {
            continue;
        }
        if unique_numbers.contains(&number) {
            if card_mapping.contains_key(&card_number) {
                score = *card_mapping.get(&card_number).unwrap();
                card_mapping.insert(card_number, score + 1);
            } else {
                card_mapping.insert(card_number, 1);
            }
            card_number += 1;
        }
    }
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
