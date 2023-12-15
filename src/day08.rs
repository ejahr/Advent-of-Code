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
    //println!("network {:?}", network);
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
    // find all starting nodes
    let mut current_nodes = Vec::new();
    for (node, _) in network.iter(){
        if node.chars().nth(2) == Some('A'){
            current_nodes.push(node);
        }
    }

    println!("start nodes {:?}", current_nodes);
    println!("num of start nodes {}", current_nodes.len());
    // walk all paths simultaneously
    let mut num_steps = 0;
    let mut finished = false;

    // loop as long as not all nodes end with Z
    while finished == false {
        for d in directions.chars() {
            for i in 0..current_nodes.len(){
                //println!("nodes {:?}", current_nodes);
                let edges = network.get(current_nodes[i]).unwrap();
                let mut next_node : &&str = &"AAA";
                match d {
                    'L' => next_node = &edges.0,
                    'R' => next_node = &edges.1,
                    _ => println!("no next node found"),
                }
                current_nodes.push(next_node);
                current_nodes.swap_remove(i);
            }
            num_steps += 1;
        }
        // check if all nodes end with "Z"
        finished = true;
        for node in current_nodes.iter() {
            if node.chars().nth(2) != Some('Z'){
                finished = false;
                break
            }
        }
        //println!("finished: {}", finished)
    }
    num_steps
}

/*
fn find_starting_points(network: HashMap<&str, (&str, &str)>) -> Vec<&str> {
    let mut starting_points = Vec::new();

    for (&node, _) in network.iter(){
        if node.chars().nth(2) != Some('A'){
            starting_points.push(node);
        }
    }
    starting_points
}
*/


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
    let input = read_input_to_string();

    let network = convert_input_to_graph(&input);
    walk_all_paths(input.lines().next().unwrap(), network)
}
