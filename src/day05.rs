/* Using the maps, which describe how to convert numbers from a source category
into numbers in a destination category, find the lowest location number that
corresponds to any of the initial seeds.
To do this, you'll need to convert each seed number through other
categories until you can find its corresponding location number.
Solution part 1:  165788812
Solution part 2:
 */

use std::fs;

fn read_input() -> (Vec<i64>, Vec<Vec<Vec<i64>>>) {
    let path = "input/day05.txt";


    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let input = fs::read_to_string(path).unwrap();

    //let (first_line, remainder) = input.split_once("\n\n").unwrap();
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
    for map_as_string in remainder.split("\r\n\r\n"){
    //for map_as_string in remainder.split("\n\n"){
        let mut map_as_vec: Vec<Vec<i64>> = Vec::new();

        // iterate only over lines containing numbers
        for line in map_as_string.split_once(":\r\n").unwrap().1.lines() {
        //for line in map_as_string.split_once(":\n").unwrap().1.lines() {
            map_as_vec.push( line.split_whitespace()
                .map(|m| { m.parse::<i64>().unwrap() })
                .collect::<Vec<i64>>()
            );
        }
        maps.push(map_as_vec);
    };
    maps
}

fn convert(number: &i64, map: &Vec<Vec<i64>>) -> i64 {
    for sub_map in map{
        if (number >= &sub_map[1]) & (number <= &(sub_map[1] + sub_map[2])) {
            return number + sub_map[0] - sub_map[1]
        }
    }
    *number
}

fn get_location_of_seeds(seeds: Vec<i64>, maps: Vec<Vec<Vec<i64>>> ) -> Vec<i64>{
    let mut converted_seeds = Vec::new();

    for seed in seeds.iter(){
        let mut converted_seed = *seed;
        for map in maps.iter() {
            converted_seed = convert(&converted_seed, map);
        }
        converted_seeds.push(converted_seed);
    }

    converted_seeds
}

fn get_seeds_from_ranges(seed_ranges: Vec<i64>) -> Vec<i64>{
    let mut seeds = Vec::new();
    //println!("seed_ranges {:?}", seed_ranges);
    for i in (0..seed_ranges.len()).step_by(2){
        //println!("i {}", i);
        for s in seed_ranges[i]..seed_ranges[i]+seed_ranges[i+1]{
            seeds.push(s);
        }
    }
    //println!("seeds {:?}", seeds);
    seeds
}


pub fn solve_part1() -> i64 {
    let (seeds, maps) = read_input();

    let locations = get_location_of_seeds(seeds, maps);

    *locations.iter().min().unwrap()
}

pub fn solve_part2() -> i64 {
    let (seed_ranges, maps) = read_input();

    let seeds = get_seeds_from_ranges(seed_ranges);

    let locations = get_location_of_seeds(seeds, maps);

    *locations.iter().min().unwrap()
}