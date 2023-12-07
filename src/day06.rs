// part 1: 316800
// part 2: 45647654

fn num_of_ways_to_beat(time: &f64, distance: &f64) -> f64{
    // find roots of x^2 - t*x + d = 0;
    let mut min_t = (time - (f64::powf(*time, 2.0) - 4.0*distance).sqrt() ) / 2.0;
    let mut max_t = (time + (f64::powf(*time, 2.0) - 4.0*distance).sqrt() ) / 2.0;

    if min_t.fract() == 0.0 {
        min_t += 1.0;
    }
    if max_t.fract() == 0.0 {
        max_t -= 1.0;
    }
    max_t.floor() - min_t.ceil() + 1.0
}

pub fn solve_part1() -> f64{
    let time = [61.0, 67.0, 75.0, 71.0];
    let distance = [430.0, 1036.0, 1307.0, 1150.0];

    let mut product = 1.0;
    for (t, d) in time.iter().zip(distance.iter()) {
        product *= num_of_ways_to_beat(t, d);
    }
    product
}

pub fn solve_part2() -> f64{
    let time = 61677571.0;
    let distance = 430103613071150.0;

    num_of_ways_to_beat(&time, &distance)
}