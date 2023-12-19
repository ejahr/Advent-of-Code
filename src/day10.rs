use std::fs;
use array2d::Array2D;

fn array2d_from_input() -> Array2D<char> {

    let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    let input = fs::read_to_string("input/day10.txt").unwrap();
    let num_rows = input.lines().count();
    let num_col = input.lines().next().unwrap().len() + 2;

    println!("{:?}", input.lines().next().unwrap());

    // add buffer around the maze to avoid indexing out of bound
    let input = format!(".{}.", input);
    //let maze = input.replace("\n", "..");
    let maze = input.replace("\r\n", "..");

    Array2D::from_iter_row_major(
        maze.chars(),
        num_rows,
        num_col
    ).unwrap()
}

fn go_to_next_pipe(previous: (usize, usize), current: (usize, usize), maze: &Array2D<char>)
    -> (usize, usize) {
    let mut row = current.0 as i32;
    let mut col = current.1 as i32;
    // incoming direction needed identify where to go next
    let direction = (row - previous.0 as i32, col - previous.1 as i32);

    if (maze[current] == '-') || (maze[current] == '|'){
        row += direction.0 ;
        col += direction.1 ;
    }
    else if direction == (0,-1){    // left
        if maze[current] == 'L' { row -= 1 }
        else if maze[current] == 'F' { row += 1 }
    }
    else if direction == (1,0){     // down
        if maze[current] == 'J' { col -= 1 }
        else if maze[current] ==  'L' { col += 1 }
    }
    else if direction == (0,1){     // right
        if maze[current] == '7' { row += 1 }
        else if maze[current] == 'J' { row -= 1 }
    }
    else if direction == (-1,0){    // up
        if maze[current] ==  'F' { col += 1 }
        else if maze[current] == '7' { col -= 1 }
    }
    (row as usize, col as usize)
}

fn go_through_loop(maze: &Array2D<char>) -> u64 {
    let (start, mut current) = find_loop(&maze);
    let mut previous = start;
    let mut num_steps = 0;

    while current != start && num_steps < 10000000 {
        if num_steps%10000 == 0 {
            println!("\nstep {}", num_steps);
            println!("previous {:?}", (previous, maze[previous]));
            println!("current {:?}", (current, maze[current]));
        }
        let next = go_to_next_pipe(previous, current, &maze);
        previous = current;
        current = next;
        num_steps += 1;
    }
    num_steps
}

fn find_loop(maze: &Array2D<char>) -> ((usize, usize), (usize, usize)) {
    let start = find_start(&maze);

    let left = (start.0, start.1-1);
    let down = (start.0+1, start.1);
    let right = (start.0, start.1+1);
    let up = (start.0-1, start.1);

    if (maze[left] == '-') ||
        (maze[left] == 'L') ||
        (maze[left] == 'F') {
        return (start, left);
    }
    else if (maze[down] == '|') ||
        (maze[down] == 'J') ||
        (maze[down] == 'L') {
        return (start, down);
    }
    else if (maze[right] == '-') ||
        (maze[right] == 'J') ||
        (maze[right] == '7') {
        return (start, right);
    }
    else if (maze[up] == '|') ||
        (maze[up] == 'F') ||
        (maze[up] == '7') {
        return (start, up);
    }
    (start, (0,0))
}

fn find_start(maze: &Array2D<char>) -> (usize, usize) {
    for i in 0..maze.num_columns(){
        for j in 0..maze.num_rows(){
            if maze[(i,j)] == 'S' {
                return (i,j)
            }
        }
    }
    (0, 0)
}

pub fn solve_part1() -> u64 {
    let maze = array2d_from_input();

    println!("{:?}", maze.as_rows()[0]);

    let size_loop = go_through_loop(&maze);

    size_loop/2 + 1
}