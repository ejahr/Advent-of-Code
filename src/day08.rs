// solution part 1: 19783

use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn read_input_to_string() -> String {
    let path = "input/day08.txt";
    fs::read_to_string(path).unwrap()
}

fn convert_input_to_graph(input: &str) -> HashMap<&str, (&str, &str)> {
    // extract nodes & edges from input
    let re = Regex::new(r"([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)").unwrap();
    let matches: Vec<_> = re
        .captures_iter(input)
        .map(|m| {
            let (_, [node, left, right]) = m.extract();
            (node, left, right)
        })
        .collect();

    // build network from key: node, value: (left edge, right edge)
    let mut network = HashMap::new();
    for (node, left, right) in matches {
        network.insert(node, (left, right));
    }
    println!("network {:?}", network);
    network
}

fn walk_path(directions: &str, network: HashMap<&str, (&str, &str)>) -> i32 {
    let mut node = "AAA";
    let mut num_steps = 0;

    while node != "ZZZ" {
        for d in directions.chars() {
            let edges = network.get(node).unwrap();
            match d {
                'L' => node = edges.0,
                'R' => node = edges.1,
                _ => println!("no node found"),
            }
            num_steps += 1
        }
    }
    num_steps
}

fn walk_all_paths(directions: &str, network: HashMap<&str, (&str, &str)>) -> i32 {
    let mut node = "11A";
    let mut num_steps = 0;

    while node.chars().nth(2) != Some('Z') {
        for d in directions.chars() {
            println!("node {:?}", node);
            let edges = network.get(node).unwrap();
            match d {
                'L' => node = edges.0,
                'R' => node = edges.1,
                _ => println!("no node found"),
            }
            num_steps += 1
        }
    }
    num_steps
}

pub fn solve_part1() -> i32 {
    let input = "LLR
    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";
    // input = &read_input_to_string();

    let network = convert_input_to_graph(input);
    walk_path(input.lines().next().unwrap(), network)
}

pub fn solve_part2() -> i32 {
    let input = "LR
11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    // input = &read_input_to_string();

    let network = convert_input_to_graph(input);
    walk_all_paths(input.lines().next().unwrap(), network)
}
