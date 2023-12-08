use std::fs;
use regex::Regex;

fn read_input_to_string() -> String {
    let path = "input/day02.txt";
    fs::read_to_string(path).unwrap()
}

fn game_is_possible(game: &str) -> bool {
    // extract number of cubes and corresponding color
    let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let matches: Vec<_> = re.captures_iter(game).map(|m| {
        let (_, [number, color]) = m.extract();
        (number, color)
    }).collect();

    // check if any number of cubes exceeds corresponding max value
    for (number, color) in matches {
        match color {
            "red" => if number.parse::<i32>().unwrap() > 12 { return false; },
            "green" => if number.parse::<i32>().unwrap() > 13 { return false; },
            "blue" => if number.parse::<i32>().unwrap() > 14 { return false; },
            _ => print!("no color found")
        }
    }
    return true;
}

fn max_number_of_cubes(game: &str, color: &str) -> i32 {
    let expression = ["([0-9]+) ", color].concat();
    let re = Regex::new(&expression[..]).unwrap();

    // catch all occurrences of the expression and collect the numbers in a vec
    let cubes: Vec<_> = re.captures_iter(game).map(|m| {
        let (_, [number]) = m.extract();
        number.parse::<i32>().unwrap()
    }).collect();

    *cubes.iter().max().unwrap()
}

fn product_of_number_of_cubes(game: &str) -> i32 {
    max_number_of_cubes(game, "red")
        * max_number_of_cubes(game, "green")
        * max_number_of_cubes(game, "blue")
}

pub fn solve_part1() -> i32 {
    let input = read_input_to_string();

    let mut sum = 0;
    for game in input.lines() {
        if game_is_possible(game) {
            let re = Regex::new(r"\d+").unwrap();
            let game_id = re.find(game).unwrap().as_str();
            sum += game_id.parse::<i32>().unwrap();
        }
    }
    sum
}

pub fn solve_part2() -> i32 {
    let input = read_input_to_string();

    let mut sum = 0;
    for game in input.lines() {
        sum += product_of_number_of_cubes(game);
    }
    sum
}