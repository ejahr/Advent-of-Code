use std::fs;
use std::collections::HashMap;

fn read_input_to_string() -> String{
    let path = "input/day01.txt";
    return fs::read_to_string(path).unwrap();
}

fn find_calibration_value(line: &str) -> u32 {
    // Find first digit.
    let first_digit = line.chars()
        .find(|a| a.is_digit(10))
        .and_then(|a| a.to_digit(10))
        .unwrap();

    // Find last digit.
    let reverse_line = line.chars().rev().collect::<String>();
    let last_digit = reverse_line.chars()
        .find(|a| a.is_digit(10))
        .and_then(|a| a.to_digit(10))
        .unwrap();

    (first_digit * 10 + last_digit) as u32
}

fn find_first_digit(string: &str){
    string.chars().find(|a| a.is_digit(10))
        .and_then(|a| a.to_digit(10))
        .unwrap();
}



fn find_calibration_value_part2(line: &str) -> u32 {

    let numbers:HashMap<&str, u32> = HashMap::from([
        ("zero", 0), ("one", 1), ("two", 2), ("three", 3),
        ("four", 4), ("five", 5), ("six", 6), ("seven", 7),
        ("eight", 8), ("nine", 9),
    ]);

    let mut first_digit:(usize, u32) = (line.len(), 0);
    let mut last_digit = (0, 0);
    for (i,c) in line.chars().enumerate(){
        for (word, value) in numbers.iter(){
            if line[i..].starts_with(word) || line[i..].starts_with(char::from_digit(*value, 10).unwrap()) {
                if i < first_digit.0 {
                    first_digit = (i, *value)
                }
                else if i > last_digit.0 {
                    last_digit = (i, *value)
                }
            }
        }
    }

    (first_digit.1 * 10 + last_digit.1) as u32
}

pub fn solve_part1() -> u32 {
    let input = read_input_to_string();

    let mut sum = 0;
    for line in input.lines(){
        sum += find_calibration_value(line);
    }
    sum
}

pub fn solve_part2() -> u32 {
    let input = read_input_to_string();

    let mut sum = 0;
    for line in input.lines(){
        sum += find_calibration_value_part2(line);
    }
    sum
}

