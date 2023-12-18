
mod day01;
mod day02;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() {

    let day = 9;

    match day {
        1 => {
            println!("result day 1 part 1: {}", day01::solve_part1());
            println!("result day 1 part 2: {}", day01::solve_part2())
        },
        2 => {
            println!("result day 2 part 1: {}", day02::solve_part1());
            println!("result day 2 part 2: {}", day02::solve_part2());
        }
        4 => {
            println!("result day 4 part 1: {}", day04::solve_part1());
            println!("result day 4 part 2: {}", day04::solve_part2());
        }
        5 => {
            println!("result day 5 part 1: {}", day05::solve_part1());
            println!("result day 5 part 2: {}", day05::solve_part2());
        }
        6 => {
            println!("result day 6 part 1: {}", day06::solve_part1());
            println!("result day 6 part 2: {}", day06::solve_part2());
        }
        7 => {
            println!("result day 7 part 1: {}", day07::solve_part1());
        }
        8 => {
            println!("result day 8 part 1: {}", day08::solve_part1());
            println!("result day 8 part 2: {}", day08::solve_part2());
        }
        9 => {
            println!("result day 9 part 1: {}", day09::solve_part1());
            println!("result day 9 part 2: {}", day09::solve_part2());
        }

        _ => {println!("no day specified")}
    }

}
