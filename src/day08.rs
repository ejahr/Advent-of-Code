use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn read_input_to_string() -> String {
    let path = "input/day08.txt";
    fs::read_to_string(path).unwrap()
}

fn convert_input_to_graph(input: &str) -> HashMap<&str, (&str, &str)> {
    // extract nodes & edges from input
    let re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();
    let matches: Vec<_> = re.captures_iter(input)
        .map(|m| {
            let (_, [node, left, right]) = m.extract();
            (node, left, right)
        }).collect();

    // build network from key: node, value: (left edge, right edge)
    let mut network = HashMap::new();
    for (node, left, right) in matches {
        network.insert(
            node,
            (left, right)
        );
    }

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
                _ => println!("no node found")
            }
            println!("node {:?}", edges);
            num_steps += 1
        }
    }
    num_steps
}

pub fn solve_part1() -> i32{
    let input = "LLR
    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";
    // input = &read_input_to_string();

    let network = convert_input_to_graph(input);
    walk_path(input.lines().next().unwrap(), network)
}