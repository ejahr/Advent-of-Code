/* Find the lowest location number that corresponds to any of the initial seeds.
To do this, you'll need to convert each seed number through other
categories until you can find its corresponding location number.
The values on the initial seeds: line come in pairs. Within each pair, the first
value is the start of the range and the second value is the length of the range.
Solution part 1: 165788812
Solution part 2: 1928058
 */

use std::fs;
use std::thread;
use std::thread::JoinHandle;

fn read_input() -> (Vec<i64>, Vec<Vec<Vec<i64>>>) {
    let path = "input/day05.txt";
    let input = fs::read_to_string(path).unwrap();

    let (first_line, remainder) = input.split_once("\r\n\r\n").unwrap();
    let seeds: Vec<i64> = get_seeds(first_line);
    let maps: Vec<Vec<Vec<i64>>> = get_maps(remainder);

    (seeds, maps)
}

fn get_seeds(first_line: &str) -> Vec<i64> {
    first_line.split_once(": ").unwrap().1       // get only string of numbers
        .split_whitespace()
        .map(|m| { m.parse::<i64>().unwrap() })
        .collect()
}

fn get_maps(remainder: &str) -> Vec<Vec<Vec<i64>>> {
    let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();
    // iterate over individual maps
    for map_as_string in remainder.split("\r\n\r\n") {
        let mut map_as_vec: Vec<Vec<i64>> = Vec::new();
        // iterate only over lines containing numbers
        for line in map_as_string.split_once(":\r\n").unwrap().1.lines() {
            map_as_vec.push(
                line.split_whitespace()
                    .map(|m| { m.parse::<i64>().unwrap() })
                    .collect::<Vec<i64>>()
            );
        }
        maps.push(map_as_vec);
    };
    maps
}

fn convert(number: &i64, map: &Vec<Vec<i64>>) -> i64 {
    for sub_map in map {
        if (number >= &sub_map[1]) && (number <= &(sub_map[1] + sub_map[2])) {
            return number + sub_map[0] - sub_map[1];
        }
    }
    *number
}

fn get_location_of_seed(seed: i64, maps: &Vec<Vec<Vec<i64>>>) -> i64 {
    let mut converted_seed = seed;
    for map in maps.iter() {
        converted_seed = convert(&converted_seed, map);
    }
    converted_seed
}

pub fn solve_part1() -> i64 {
    let (seeds, maps) = read_input();

    let locations: Vec<i64> = seeds.iter()
        .map(|i| get_location_of_seed(i.clone(), &maps))
        .collect();

    *locations.iter().min().unwrap()
}

pub fn solve_part2() -> i64 {
    let (seed_ranges, maps) = read_input();

    let mut threads: Vec<JoinHandle<i64>> = Vec::new();
    for i in (0..seed_ranges.len()).step_by(2) {
        let seed_range_copy = seed_ranges.clone();
        let maps_copy = maps.clone();
        threads.push(thread::spawn(move || {
            let mut min_location = i64::MAX;
            println!("start {i}");
            for seed in seed_range_copy[i]..seed_range_copy[i] + seed_range_copy[i + 1] {
                let location = get_location_of_seed(seed, &maps_copy);
                if location < min_location {
                    min_location = location;
                }
            }
            println!("end {i}");

            min_location
        }))
    }

    threads.into_iter()
        .map(|m| m.join().unwrap())
        .min().unwrap()
}