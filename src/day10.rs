use std::fs;
use array2d::Array2D;

fn array2d_from_input() -> Array2D<char> {
    let input = fs::read_to_string("input/day10.txt").unwrap();
    let num_rows = input.lines().count();
    let num_col = input.lines().next().unwrap().len() + 2;

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

fn go_through_loop(maze: &Array2D<char>) -> u64 {
    let (start, mut current) = find_loop(&maze);
    let mut previous = start;
    let mut num_steps = 0;

    while current != start {
        let next = go_to_next_pipe(previous, current, &maze);
        previous = current;
        current = next;
        num_steps += 1;
    }
    num_steps
}

fn go_to_next_pipe(previous: (usize, usize), current: (usize, usize),
                   maze: &Array2D<char>) -> (usize, usize) {
    let mut row = current.0 as i32;
    let mut col = current.1 as i32;
    // incoming direction of current pipe
    let direction = (row - previous.0 as i32, col - previous.1 as i32);

    if (maze[current] == '-') || (maze[current] == '|'){
        row += direction.0 ;
        col += direction.1 ;
    }
    else if maze[current] == 'L' {
        if direction == (0,-1) { row -= 1 }         // from right
        else if direction == (1,0) { col += 1 }     // from up
    }
    else if maze[current] == 'F' {
        if direction == (-1,0) { col += 1 }         // from down
        else if direction == (0,-1) { row += 1 }    // from right
    }
    else if maze[current] == 'J' {
        if direction == (0,1) { row -= 1 }          // from left
        else if direction == (1,0) { col -= 1 }     // from up
    }
    else if maze[current] == '7' {
        if direction == (0,1) { row += 1 }          // from left
        else if direction == (-1,0) { col -= 1 }    // from down
    }

    (row as usize, col as usize)
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
    let size_loop = go_through_loop(&maze);
    size_loop/2 + 1
}