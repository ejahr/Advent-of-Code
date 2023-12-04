// result 28538
use std::fs;
use std::collections::HashSet;

fn read_input_to_string() -> String{
    let path = "input/day04.txt";
    fs::read_to_string(path).unwrap()
}

fn winning_and_your_numbers(card: &str) -> (HashSet<i32>, HashSet<i32>){
    // card 1: 2 | 2 3 -> [ card 1 ], [ 1 ], [ 2 3 ]
    let parts: Vec<&str> = card.split(|c| c == ':' || c == '|').collect();

    let winning_numbers: HashSet<i32> = parts[1].split_whitespace()
        .map(|m| {
            m.parse::<i32>().unwrap()
        }).collect();

    let your_numbers: HashSet<i32> = parts[2].split_whitespace()
        .map(|m| {
            m.parse::<i32>().unwrap()
        }).collect();

    (winning_numbers, your_numbers)
}

fn calculate_points(card: &str) -> i32 {
    let (winning, yours) = winning_and_your_numbers(card);

    let intersection:HashSet<_> = yours.intersection(&winning).collect();

    let power = intersection.len() as u32;
    if power > 0 {
        return i32::pow(2, power - 1u32)
    }
    0
}

pub fn solve_part1() -> i32 {
    let input = read_input_to_string();

    let mut sum = 0;
    for card in input.lines(){
        sum += calculate_points(card);
    }
    sum
}