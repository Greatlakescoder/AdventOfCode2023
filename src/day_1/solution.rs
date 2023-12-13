use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Result<u32, std::io::Error> {
    // Read entire file in memory
    // fs::read_to_string(path)

    // Read file line by line
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // let mut parts: Vec<&str> = Vec::new();
    let mut total = 0;
    let mut combined_number: String = String::from("");
    let re =
        regex::Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)")
            .unwrap();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let parts: Vec<&str> = re.find_iter(&line).map(|m| m.as_str()).collect();
                let first_number = parts.first().unwrap().to_string();
                if first_number.len() > 1 {
                    match first_number.as_str() {
                        "one" => combined_number += "1",
                        "two" => combined_number += "2",
                        "three" => combined_number += "3",
                        "four" => combined_number += "4",
                        "five" => combined_number += "5",
                        "six" => combined_number += "6",
                        "seven" => combined_number += "7",
                        "eight" => combined_number += "8",
                        "nine" => combined_number += "9",
                        _ => {}
                        
                    }
                }else {
                    combined_number += first_number.as_str();
                }
                let second_number = parts.last().unwrap().to_string();
                if second_number.len() > 1 {
                    match second_number.as_str() {
                        "one" => combined_number += "1",
                        "two" => combined_number += "2",
                        "three" => combined_number += "3",
                        "four" => combined_number += "4",
                        "five" => combined_number += "5",
                        "six" => combined_number += "6",
                        "seven" => combined_number += "7",
                        "eight" => combined_number += "8",
                        "nine" => combined_number += "9",
                        _ => {}
                        
                    }
                }else {
                    combined_number += second_number.as_str();
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        total += combined_number.parse::<u32>().unwrap();
        combined_number.clear();
    }
    return Ok(total);
}

mod test {
    #[test]
    fn test_case_1() {
        match super::read_file("src/day_1/test_case.txt") {
            Ok(contents) => {
                assert_eq!(contents, 142);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    #[test]
    fn test_case_2() {
        match super::read_file("src/day_1/test_case_2.txt") {
            Ok(contents) => {
                assert_eq!(contents, 281);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    #[test]
    fn test_case_3() {
        match super::read_file("src/day_1/input.txt") {
            Ok(contents) => {
                assert_eq!(contents, 54076);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

pub fn run() {
    match read_file("src/day_1/input.txt") {
        Ok(contents) => {
            println!("Calibration Value: {}", contents);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
