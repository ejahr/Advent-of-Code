use std::fs;
use num_integer::binomial;

fn get_sum_of_next_values(input: &str) -> i64 {
    let histories = get_histories(input);
    let coefficients = get_coefficients(histories[0].len() as i64);

    let mut sum = 0;
    for history in histories {
        sum += get_next_value(&history, &coefficients);
    }
    sum
}

fn get_sum_of_previous_values(input: &str) -> i64 {
    let histories = get_histories(input);

    let mut coefficients = get_coefficients(histories[0].len() as i64);
    coefficients.reverse();

    let mut sum = 0;
    for history in histories {
        sum += get_next_value(&history, &coefficients);
    }
    sum
}

fn get_histories(input: &str) -> Vec<Vec<i64>> {
    let mut histories = Vec::new();
    for line in input.lines(){
        histories.push(
            line.split_whitespace()
                .map(|m| { m.parse::<i64>().unwrap() })
                .collect()
        )
    }
    histories
}

fn get_coefficients(n: i64) -> Vec<i64>{
    // The coefficients with which the individual numbers have to be multiplied
    // are up to (-1)^(n-k-1) equal to the binomial numbers (see Pascal's Triangle)
    let mut coefficients = Vec::new();
    for k in 0..n {
        coefficients.push( i64::pow(-1, (n-k-1) as u32) * binomial(n, k))
    }
    coefficients
}

fn get_next_value(history: &Vec<i64>, coefficients: &Vec<i64>) -> i64 {
    // dot product of both vectors
    history.iter().zip(coefficients.iter())
        .map(|(x, y)| x * y)
        .sum()
}

pub fn solve_part1() -> i64 {
    let input = fs::read_to_string("input/day09.txt").unwrap();
    get_sum_of_next_values(&input)
}

pub fn solve_part2() -> i64 {
    let input = fs::read_to_string("input/day09.txt").unwrap();
    get_sum_of_previous_values(&input)
}