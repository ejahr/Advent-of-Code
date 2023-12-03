
mod day01;
mod day02;

fn main() {
    let sum = day01::solve_part1();
    println!("result day 1 part 1: {}", sum);
    let sum = day01::solve_part2();
    println!("result day 1 part 2: {}", sum);

    let sum = day02::solve_part1();
    println!("result day 2 part 1: {}", sum);
}
