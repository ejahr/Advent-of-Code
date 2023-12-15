// solution part 1: 19783
// solution part 2: 9177460370549

use regex::Regex;
use std::collections::HashMap;
use std::fs;
use num_integer::lcm;

fn read_input_to_string() -> String {
    let path = "input/day08.txt";
    fs::read_to_string(path).unwrap()
}

fn get_nodes_and_edges(input: &str) -> Vec<(&str, &str, &str)>{
    let re = Regex::new(r"([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)").unwrap();
    re.captures_iter(input)
        .map(|m| {
            let (_, [node, left, right]) = m.extract();
            (node, left, right)
        })
        .collect()
}

fn parse_input_to_graph(input: &str) -> HashMap<&str, (&str, &str)> {
    let nodes_and_edges = get_nodes_and_edges(input);
    let mut network = HashMap::new();
    for (node, left, right) in nodes_and_edges {
        // key: node, value: (left edge, right edge)
        network.insert(node, (left, right));
    }
    network
}

fn walk_path(directions: &str, network: HashMap<&str, (&str, &str)>) -> i32 {
    let mut node = "AAA";
    let mut num_steps = 0;

    while node != "ZZZ" {
        for direction in directions.chars() {
            node = go_to_next_node(node, direction, &network);
            num_steps += 1
        }
    }
    num_steps
}

fn go_to_next_node<'a>(mut node: &'a str, direction: char, network: &'a HashMap<&str, (&str, &str)>) -> &'a str {
    let edges = network.get(node).unwrap();
    match direction {
        'L' => node = edges.0,
        'R' => node = edges.1,
        _ => println!("no node found"),
    }
    node
}

fn find_starting_points(network: &HashMap<&str, (&str, &str)>) -> Vec<String> {
    let mut starting_points = Vec::new();

    for (&node, _) in network.iter(){
        if node.chars().nth(2) == Some('A'){
            starting_points.push(node.to_string());
        }
    }
    starting_points
}

fn walk_all_paths(directions: &str, network: HashMap<&str, (&str, &str)>) -> Vec<i64> {
    let starting_points = find_starting_points(&network);
    let mut num_steps_for_each_path = Vec::new();

    for start in starting_points.iter() {
        let mut node: &str = start;
        let mut num_steps = 0;

        while node.chars().nth(2) != Some('Z') {
            for direction in directions.chars() {
                node = go_to_next_node(node, direction, &network);
                num_steps += 1
            }
        }
        num_steps_for_each_path.push(num_steps);
    }
    num_steps_for_each_path
}


pub fn solve_part1() -> i32 {
    let input = read_input_to_string();
    let network = parse_input_to_graph(&input);
    walk_path(input.lines().next().unwrap(), network)
}

pub fn solve_part2() -> i64 {
    let input = read_input_to_string();
    let network = parse_input_to_graph(&input);
    // first line gives directions
    let num_steps = walk_all_paths(input.lines().next().unwrap(), network);
    let mut total_num_steps = 1;
    for num in num_steps{
        total_num_steps = lcm(total_num_steps, num)
    }
    total_num_steps
}