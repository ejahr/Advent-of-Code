// result part 1: 28538
// result part 2: 9425061

use std::fs;
use std::collections::HashSet;

fn read_input_to_string() -> String{
    let path = "input/day04.txt";
    fs::read_to_string(path).unwrap()
}

fn get_set_from_string(numbers: &str) -> HashSet<i32>{
    numbers.split_whitespace()
        .map(|m| {
            m.parse::<i32>().unwrap()
        }).collect()
}

fn winning_and_your_numbers(card: &str) -> (i32, HashSet<i32>, HashSet<i32>){
    // card 1: 2 | 2 3 -> [ card 1 ], [ 1 ], [ 2 3 ]
    let split: Vec<&str> = card.split(|c| c == ':' || c == '|').collect();
    let winning_numbers: HashSet<i32> = get_set_from_string(split[1]);
    let your_numbers: HashSet<i32> = get_set_from_string(split[2]);

    let card_id = split[0].split_whitespace()
        .collect::<Vec<_>>()[1]
        .parse::<i32>().unwrap();

    (card_id, winning_numbers, your_numbers)
}

fn calculate_matches(card: &str) -> (i32, i32) {
    let (card_id, winning, yours) = winning_and_your_numbers(card);
    let intersection:HashSet<_> = yours.intersection(&winning).collect();

    (card_id, intersection.len() as i32)
}

pub fn solve_part1() -> i32 {
    let input = read_input_to_string();

    let mut sum = 0;
    for card in input.lines(){
        let (_, num_matches) = calculate_matches(card);
        if num_matches > 0 {
            sum += i32::pow(2, num_matches as u32 - 1u32)
        }
    }
    sum
}

pub fn solve_part2() -> i32 {
    let input = read_input_to_string();

    let mut num_per_card = vec![1; input.lines().count()];
    for card in input.lines(){
        let (card_id, num_matches) = calculate_matches(card);
        // calculate scratch cards won
        for id in card_id+1..card_id+num_matches+1{
            num_per_card[(id-1) as usize] += num_per_card[(card_id-1) as usize];
        }
    }
    num_per_card.iter().sum()
}