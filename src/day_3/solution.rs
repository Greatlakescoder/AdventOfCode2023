use std::fs::File;
use std::io::{BufRead, BufReader};
fn read_file(path: &str) -> Result<u32, std::io::Error> {
    // Read file line by line
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut schematic: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let new_row: Vec<char> = line.chars().collect();
                schematic.push(new_row);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    let mut i = 0;
    let mut j = 0;
    let mut numbers: Vec<String> = Vec::new();
    while i < schematic.len() {
        while j < schematic[i].len() {
            if schematic[i][j] == '.' {
                j += 1;
                continue;
            }
            if schematic[i][j].is_ascii_punctuation() {
                let symbol = schematic[i][j];
                // If we find a symbol we need to check the 8 surrounding squares
                let x = i.clone();
                let y = j.clone();
                let mut value: String = "".to_string();
                // First two we check either side of X
                if schematic[x][y + 1].is_ascii_digit() {
                    let mut z = y.clone() + 1;
                    while z < schematic[x].len() && schematic[x][z].is_ascii_digit() {
                        value.push(schematic[x][z]);
                        schematic[x][z] = '.';
                        z += 1;
                    }
                    if value != "" {
                        println!("Found Value {} near Symbol {}", value, symbol);
                        numbers.push(value);
                    }
                }
                value = "".to_string();
                if schematic[x][y - 1].is_ascii_digit() {
                    let mut z = y.clone() - 1;
                    while z != 0 && schematic[x][z].is_ascii_digit() {
                        value.push(schematic[x][z]);
                        schematic[x][z] = '.';
                        z -= 1;
                    }
                    if z == 0 {
                        if schematic[x][z].is_ascii_digit() {
                            value.push(schematic[x][z]);
                            schematic[x][z] = '.';
                        }
                    }
                    if value != "" {
                        value = value.chars().rev().collect();
                        println!("Found Value {} near Symbol {}", value, symbol);
                        numbers.push(value);
                    }
                }

                // Third we Check the 3 above
                if schematic[x - 1][y].is_ascii_digit() {
                    let mut z = y.clone();
                    let mut left_hand_side = "".to_string();
                    let mut right_hand_side: String = "".to_string();
                    while schematic[x - 1][z].is_ascii_digit() {
                        right_hand_side.push(schematic[x - 1][z]);
                        schematic[x - 1][z] = '.';
                        z += 1;
                    }
                    // Subtract one since we are now going to left and we marked the right as .
                    let mut z = y.clone() - 1;
                    while z < schematic[x - 1].len() && schematic[x - 1][z].is_ascii_digit() {
                        left_hand_side.push(schematic[x - 1][z]);
                        schematic[x - 1][z] = '.';
                        z -= 1;
                    }
                    left_hand_side = left_hand_side.chars().rev().collect();
                    value = format!("{}{}", left_hand_side, right_hand_side);
                    if value != "" {
                        println!("Found Value {} near Symbol {}", value, symbol);
                        numbers.push(value);
                    }
                }

                // Check the top right diagonal
                if y + 1 < schematic[x - 1].len() {
                    let mut z = y.clone() + 1;
                    let mut value: String = "".to_string();
                    while z < schematic[x - 1].len() && schematic[x - 1][z].is_ascii_digit() {
                        value.push(schematic[x - 1][z]);
                        schematic[x - 1][z] = '.';
                        z += 1;
                    }
                    if value != "" {
                        println!("Found Value {} near Symbol {}", value, symbol);
                        numbers.push(value);
                    }
                }
                // Check the top left diagonal
                if y - 1 > 0 {
                    let mut z = y.clone() - 1;
                    let mut value: String = "".to_string();
                    while z > 0 && schematic[x - 1][z].is_ascii_digit() {
                        value.push(schematic[x - 1][z]);
                        schematic[x - 1][z] = '.';
                        z -= 1;
                    }
                    if z == 0 {
                        if schematic[x - 1][z].is_ascii_digit() {
                            value.push(schematic[x - 1][z]);
                            schematic[x - 1][z] = '.';
                        }
                    }
                    value = value.chars().rev().collect();
                    if value != "" {
                        println!("Found Value {} near Symbol {}", value, symbol);
                        numbers.push(value);
                    }
                }

                // Second we Check the 3 below
                if x + 1 >= schematic.len() {
                    j += 1;
                    continue;
                }
                if schematic[x + 1][y].is_ascii_digit() {
                    let mut z = y.clone();
                    let mut left_hand_side = "".to_string();
                    let mut right_hand_side: String = "".to_string();
                    while z < schematic[x + 1].len() && schematic[x + 1][z].is_ascii_digit() {
                        right_hand_side.push(schematic[x + 1][z]);
                        schematic[x + 1][z] = '.';
                        z += 1;
                    }
                    let mut z = y.clone() - 1;
                    while z != 0 && schematic[x + 1][z].is_ascii_digit() {
                        left_hand_side.push(schematic[x + 1][z]);
                        schematic[x + 1][z] = '.';
                        z -= 1;
                    }
                    left_hand_side = left_hand_side.chars().rev().collect();
                    value = format!("{}{}", left_hand_side, right_hand_side);
                    if value != "" {
                        println!("Found Value {} near Symbol {}", value, symbol);
                        numbers.push(value);
                    }
                }
                // Check the bottom right diagonal
                if y + 1 < schematic[x + 1].len() {
                    let mut z = y.clone() + 1;
                    let mut value: String = "".to_string();
                    while z < schematic[x + 1].len() && schematic[x + 1][z].is_ascii_digit() {
                        value.push(schematic[x + 1][z]);
                        schematic[x + 1][z] = '.';
                        z += 1;
                    }
                    if value != "" {
                        println!("Found Value {} near Symbol {}", value, symbol);
                        numbers.push(value);
                    }
                }
                // Check the bottom left diagonal
                if y - 1 > 0 {
                    let mut z = y.clone() - 1;
                    let mut value: String = "".to_string();
                    while z > 0 && schematic[x + 1][z].is_ascii_digit() {
                        value.push(schematic[x + 1][z]);
                        schematic[x + 1][z] = '.';
                        z -= 1;
                    }
                    if z == 0 {
                        if schematic[x + 1][z].is_ascii_digit() {
                            value.push(schematic[x + 1][z]);
                            schematic[x + 1][z] = '.';
                        }
                    }
                    value = value.chars().rev().collect();
                    if value != "" {
                        println!("Found Value {} near Symbol {}", value, symbol);
                        numbers.push(value);
                    }
                }
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }
    let total = numbers
        .iter()
        .fold(0, |acc, x| acc + x.parse::<u32>().unwrap());
    return Ok(total);
}
pub fn run() {
    match read_file("src/day_3/input.txt") {
        Ok(contents) => {
            println!("Calibration Value: {}", contents);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
