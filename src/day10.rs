use std::fs;
use array2d::Array2D;

fn array2d_from_input() ->  Array2D<char> {
    let input = fs::read_to_string("input/day10.txt").unwrap();
    let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    // add buffer around the maze to avoid indexing out of bound
    let input = format!(".{}.", input);
    let maze = input.replace("\n", "..");

    let num_rows = input.lines().count();
    let num_col = input.lines().next().unwrap().len() + 1;

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
    let direction = (row - previous.0 as i32,
                              col - previous.1 as i32);

    if (maze[current] == '-') || (maze[current] == '|'){
        row += direction.0 ;
        col += direction.1 ;
    }
    else if direction == (0,1){     // right
        if maze[current] == '7' { row += 1; }
        else if maze[current] == 'J' { row -= 1; }
    }
    else if direction == (1,0){     // down
        if maze[current] == 'L' { col += 1; }
        else if maze[current] == 'J' { col -= 1; }
    }
    else if direction == (0,-1){    // left
        if maze[current] == 'F' { row += 1; }
        else if maze[current] == 'L' { row -= 1; }
    }
    else if direction == (-1,0){    // up
        if maze[current] == '7' { col -= 1; }
        else if maze[current] == 'F' { col += 1; }
    }

    //println!("current {:?}", (current, maze[current], row, col) );

    return (row as usize, col as usize);
}

fn go_through_loop(start: (usize, usize), maze: &Array2D<char>) -> u64 {
    let mut previous = start;
    let mut current = (0,0);
    let left = (start.0, start.1-1);
    let down = (start.0+1, start.1);
    let right = (start.0, start.1+1);
    let up = (start.0-1, start.1);

    if (maze[left] == '-') ||
        (maze[left] == 'L') ||
        (maze[left] == 'F') {
        current = left;
    }
    else if (maze[down] == '|') ||
        (maze[down] == 'J') ||
        (maze[down] == 'L') {
        current = down;
    }
    else if (maze[right] == '-') ||
        (maze[right] == 'J') ||
        (maze[right] == '7') {
        current = right;
    }
    else if (maze[up] == '|') ||
        (maze[up] == 'F') ||
        (maze[up] == '7') {
        current = up;
    }
    //println!("current {:?}", maze[current]);
    let mut num_steps = 0;

    while current != start {
        let next = go_to_next_pipe(previous, current, &maze);
        previous = current;
        current = next;
        num_steps += 1;
    }
    num_steps
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
    //println!("maze {:?}", maze);
    println!("array from input");

    let start = find_start(&maze);
    //println!("start {:?}", start);
    println!("find start");

    let size_loop = go_through_loop(start, &maze);

    size_loop/2 + 1
}