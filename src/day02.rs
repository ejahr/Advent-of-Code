use std::fs;
use regex::Regex;

fn read_input_to_string() -> String{
    let path = "input/day02.txt";
    return fs::read_to_string(path).unwrap();
}

fn game_is_possible(game: &str) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let matches: Vec<_> = re.captures_iter(game).map(|m| {
        let (_, [number, color]) = m.extract();
        (number, color)
    }).collect();

    for (number, color) in matches{
        match color {
            "red" => if number.parse::<i32>().unwrap() > max_red {return false},
            "green" => if number.parse::<i32>().unwrap() > max_green {return false},
            "blue" => if number.parse::<i32>().unwrap() > max_blue {return false},
            _=> print!("no color found")
        }
    }

    return true
}


pub fn solve_part1() -> u32 {

    let input = read_input_to_string();
    //let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n";

    let mut sum = 0;
    for game in input.lines(){
        if game_is_possible(game) {
            let re = Regex::new(r"\d+").unwrap();
            let game_id = re.find(game).unwrap().as_str();
            sum += game_id.parse::<u32>().unwrap();
        }
    }
    sum
}