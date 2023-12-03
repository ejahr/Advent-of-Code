use std::fs;

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

pub fn solve_part1() -> u32 {
    let input = read_input_to_string();

    let mut sum = 0;
    for line in input.lines(){
        sum += find_calibration_value(line);
    }
    sum
}



