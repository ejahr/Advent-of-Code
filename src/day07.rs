// part 1 : 251927063

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs;
use std::io::Read;

const ORDER: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

fn get_hands_and_bets(input: &str) -> Vec<([char; 5], i32)> {
    input.lines()
        .map(|m| m.split_once(' ').unwrap())
        .map(|(m1, m2)| {
            let mut buffer = [0; 5];
            m1.as_bytes().read(&mut buffer).expect("read didn't work");
            (buffer.map(char::from), m2.parse::<i32>().unwrap())
        })
        .collect()
}

fn check_type(hand: &[char; 5]) -> i32 {
    let mut occurrences: [u8; 13] = [0; 13];

    for c in hand {
        let index = ORDER.iter().position(|a| a == c).unwrap();
        occurrences[index] += 1;
    }

    occurrences.sort();
    occurrences.reverse();

    if occurrences[0] == 5 { return 7; }
    if occurrences[0] == 4 { return 6; }
    if (occurrences[0] == 3) && (occurrences[1] == 2) { return 5; }
    if occurrences[0] == 3 { return 4; }
    if (occurrences[0] == 2) && (occurrences[1] == 2) { return 3; }
    if occurrences[0] == 2 { return 2; }

    1
}

fn compare_hands(first: &([char; 5], i32), second: &([char; 5], i32)) -> Ordering {
    if check_type(&first.0) < check_type(&second.0) {
        return Less;
    } else if check_type(&first.0) > check_type(&second.0) {
        return Greater;
    }

    for i in 0..5 {
        let index_first = ORDER.iter().position(|a| a == &first.0[i]).unwrap();
        let index_second = ORDER.iter().position(|a| a == &second.0[i]).unwrap();
        if index_first > index_second { return Greater; }
        if index_first < index_second { return Less; }
    }

    Equal
}

pub fn solve_part1() -> i32 {
    let _input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let path = "input/day07.txt";
    let input = &fs::read_to_string(path).unwrap();

    let mut hands_and_bets = get_hands_and_bets(input);

    hands_and_bets.sort_by(compare_hands);

    let mut sum = 0;
    for i  in 0..hands_and_bets.len()  {
        sum += (i as i32 + 1) * hands_and_bets[i].1;
    }
    sum
}
