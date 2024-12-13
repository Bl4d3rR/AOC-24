use core::num;
use std::{collections::HashMap, fs::File, io::Read, sync::atomic::fence};
use std::{thread, time};

use indexmap::IndexMap;

static FENCES_LOOKUP: [usize; 4] = [0, 3, 2, 1];

pub fn run_day_12() {
    let mut board = get_input();

    solve_part_01(board);
}

fn solve_part_01(mut board: Vec<Vec<char>>) {
    let mut flood_indices: IndexMap<(usize, usize), bool> = IndexMap::new();
    let mut price = 0;

    flood_indices.insert((0, 0), true);

    loop {
        let ten_millis = time::Duration::from_millis(100);
        let now = time::Instant::now();

        thread::sleep(ten_millis);

        println!("indices: {:?}", flood_indices);
        if flood_indices.len() <= 0 {
            break;
        }

        let start = flood_indices.get_index(0).expect("could not get map entry");
        price += flood_search(&mut board, (start.0 .0, start.0 .1), &mut flood_indices);
        flood_indices.swap_remove_index(0);
    }
}

fn flood_search(
    board: &mut Vec<Vec<char>>,
    start: (usize, usize),
    flood_indices: &mut IndexMap<(usize, usize), bool>,
) -> i32 {
    let rows = board.len();
    let cols = board[0].len();
    let field = board[start.0][start.1];

    let mut num_fields = 0;
    let mut fences = 0;
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    let mut stack = vec![start];

    while let Some((x, y)) = stack.pop() {
        // skip if out of bounds or the color doesn't match the target
        if x >= rows || y >= cols {
            continue;
        }

        // skip if already visited
        if visited.contains_key(&(x, y)) {
            continue;
        }

        // add flood_indices
        if board[x][y] != field {
            flood_indices.insert((x, y), true);
            continue;
        }

        // save visited to not add again
        visited.insert((x, y), true);
        num_fields += 1;
        fences += FENCES_LOOKUP[get_num_neighbours(board, (x, y))];

        // Push neighboring cells onto the stack
        if x > 0 {
            stack.push((x - 1, y)); // Up
        }
        if x < rows - 1 {
            stack.push((x + 1, y)); // Down
        }
        if y > 0 {
            stack.push((x, y - 1)); // Left
        }
        if y < cols - 1 {
            stack.push((x, y + 1)); // Right
        }
    }

    num_fields * fences as i32
}

fn get_num_neighbours(board: &mut Vec<Vec<char>>, position: (usize, usize)) -> usize {
    let mut num_neighbours = 0;
    let x = position.0;
    let y = position.1;
    let field = board[x][y];
    let width = board[0].len();
    let height = board.len();

    // lookup right
    if x < width - 1 && board[x + 1][y] == field {
        num_neighbours += 1;
    }
    // lookup down
    if y < height - 1 && board[x][y + 1] == field {
        num_neighbours += 1;
    }
    // lookup left
    if x > 0 && board[x - 1][y] == field {
        num_neighbours += 1;
    }
    // lookup up
    if y > 0 && board[x][y - 1] == field {
        num_neighbours += 1;
    }

    num_neighbours
}

fn get_input() -> Vec<Vec<char>> {
    // Open the file
    let mut file = File::open("src/day_12/input.txt").expect("Failed to open file");

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    // Split the string by newline character
    let lines: Vec<&str> = contents.split('\n').collect();

    // Convert each line into a vector of characters
    let mut result: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let char_vec: Vec<char> = line.chars().collect();
        result.push(char_vec);
    }

    result
}
