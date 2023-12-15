use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn read_file(path: &str) -> Result<u32, std::io::Error> {
    // Read file line by line
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // let mut parts: Vec<&str> = Vec::new();
    let mut total = 0;
    let mut min_cubes_required: HashMap<String, u32> = HashMap::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let game_parts: Vec<&str> = line.split(":").collect();
                let game_rounds: Vec<&str> = game_parts[1].trim_start().split(";").collect();
                for round in game_rounds {
                    let parts: Vec<&str> = round.split(",").collect();
                    for part in parts {
                        let numbers: Vec<&str> = part.trim().split(" ").collect();
                        let key = numbers[1];
                        let num_value = numbers[0].trim_start().parse::<u32>().unwrap();
                        if min_cubes_required.contains_key(key) {
                            let current_value = min_cubes_required.get(key).unwrap();
                            if num_value > *current_value {
                                min_cubes_required.insert(key.to_string(), num_value);
                            }
                        } else {
                            min_cubes_required.insert(key.to_string(), num_value);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        let mut power_sum = 1;
        for (_key,value) in min_cubes_required.iter() {
            power_sum *= value;
        }
        min_cubes_required.clear();
        total += power_sum;
    }
    return Ok(total);
}
pub fn run() {
    match read_file("src/day_2/input.txt") {
        Ok(contents) => {
            println!("Calibration Value: {}", contents);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

// PART 1
// fn read_file(path: &str) -> Result<u32, std::io::Error> {
//     // Read file line by line
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);

//     // let mut parts: Vec<&str> = Vec::new();
//     let mut total = 0;
//     let mut min_cubes_required: HashMap<String, u32> = HashMap::new();
//     for line in reader.lines() {
//         match line {
//             Ok(line) => {
//                 let game_parts: Vec<&str> = line.split(":").collect();
//                 let game_rounds: Vec<&str> = game_parts[1].trim_start().split(";").collect();
//                 let mut is_valid_game = 3;
//                 for round in game_rounds {
//                     let parts: Vec<&str> = round.split(",").collect();
//                     for part in parts {
//                         let numbers: Vec<&str> = part.trim().split(" ").collect();
//                         let key = numbers[1];
//                         let num_value = numbers[0].trim_start().parse::<u32>().unwrap();

//                         match key {
//                             "red" => {
//                                 if num_value > 12 {
//                                     is_valid_game -= 1;
//                                 }
//                             }
//                             "blue" => {
//                                 if num_value > 14 {
//                                     is_valid_game -= 1;
//                                 }
//                             }
//                             "green" => {
//                                 if num_value > 13 {
//                                     is_valid_game -= 1
//                                 }
//                             }
//                             _ => {}
//                         }
//                     }
//                 }
//                 if is_valid_game == 3 {
//                     let game_id: Vec<&str> = game_parts[0].split(" ").collect();
//                     total += game_id[1].to_string().parse::<u32>().unwrap();
//                     println!("Valid Game: {}", game_id[1]);
//                 } else {
//                     let game_id: Vec<&str> = game_parts[0].split(" ").collect();
//                     println!("Invalid Game: Game Id: {}", game_id[1]);
//                 }
//             }
//             Err(e) => {
//                 println!("Error: {}", e);
//             }
//         }
//     }
//     return Ok(total);
