use std::fs;
use std::collections::HashMap;

fn read_input_to_string() -> String{
    let path = "input/day01.txt";
    return fs::read_to_string(path).unwrap();
}

fn find_first_digit(string: &str) -> u32{
    string.chars()
        .find(|a| a.is_digit(10))
        .and_then(|a| a.to_digit(10))
        .unwrap()
}

fn find_calibration_value(line: &str) -> u32 {
    let first_digit = find_first_digit(line);

    let last_digit = find_first_digit(
        &line.chars().rev().collect::<String>()     // reverse line
    );

    (first_digit * 10 + last_digit) as u32
}


fn find_true_calibration_value(line: &str) -> u32 {
    let numbers:HashMap<&str, u32> = HashMap::from([
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
        ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ]);

    // find all occurrences of a number as a digit or word
    let mut digits:Vec<u32> = vec![];
    for (index,c) in line.chars().enumerate(){
        // check if c is digit or if substring starts with a written number
        if c.is_digit(10){
            digits.push(c.to_digit(10).unwrap());
        } else {
            for (number, digit) in numbers.iter() {
                if line[index..].starts_with(number){
                    digits.push(*digit);
                    break
                }
            }
        }
    }

    // take first and last digit to form calibration value
    (digits[0] * 10 + digits[digits.len() - 1]) as u32
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
        sum += find_true_calibration_value(line);
    }

    sum
}