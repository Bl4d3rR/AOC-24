use core::num;
use std::{
    collections::{HashMap, VecDeque},
    fs, thread, time,
};

#[derive(Clone, Copy, Debug)]
struct Guard {
    guard: char,
    path: char,
    col: i32,
    row: i32,
    advance_row: i32,
    advance_col: i32,
}

impl Guard {
    pub fn rotate_90_deg(&mut self) {
        match self.guard {
            '<' => {
                self.guard = '^';
                self.path = 'X'
            }
            '^' => {
                self.guard = '>';
                self.path = 'X'
            }
            '>' => {
                self.guard = 'v';
                self.path = 'X'
            }
            'v' => {
                self.guard = '<';
                self.path = 'X'
            }
            _ => return,
        }
        let tmp_row = self.advance_row;
        self.advance_row = self.advance_col;
        self.advance_col = tmp_row * -1;
    }

    pub fn rotate_neg_90_deg(&mut self) {
        match self.guard {
            '<' => {
                self.guard = 'v';
                self.path = 'X'
            }
            '^' => {
                self.guard = '<';
                self.path = 'X'
            }
            '>' => {
                self.guard = '^';
                self.path = 'X'
            }
            'v' => {
                self.guard = '>';
                self.path = 'X'
            }
            _ => return,
        }
    }

    pub fn walk(&mut self) {
        self.col += self.advance_col;
        self.row += self.advance_row;
    }

    pub fn get_col(&self) -> usize {
        self.col as usize
    }
    pub fn get_row(&self) -> usize {
        self.row as usize
    }
    pub fn get_advance_col(&self) -> usize {
        (self.col + self.advance_col) as usize
    }
    pub fn get_advance_row(&self) -> usize {
        (self.row + self.advance_row) as usize
    }
}

pub fn run_day_06() {
    let ten_millis = time::Duration::from_millis(70);
    // clearscreen::clear().expect("failed to clear screen");

    // Parse the file into a 2D grid
    let (mut grid, start) = parse_file_to_grid("src/day_06/input.txt");

    let mut guard = Guard {
        guard: '^',
        path: 'X',
        col: start.1.try_into().unwrap(),
        row: start.0.try_into().unwrap(),
        advance_row: -1,
        advance_col: 0,
    };

    let grid_width: i32 = grid[0].len() as i32;
    let grid_heigth: i32 = grid.len() as i32;

    let mut n_visited = 0;

    grid[start.0][start.1] = 'X';

    let mut walls: HashMap<(usize, usize), i32> = HashMap::new();
    let mut num_walls = 0;

    loop {
        // hits a wall
        if grid[guard.get_advance_row()][guard.get_advance_col()] == '#' {
            guard.rotate_90_deg();
            continue;
        }

        if grid[guard.get_advance_row()][guard.get_advance_col()] != 'X'
            && check_bullshit_cycle(grid.clone(), guard.clone())
        {
            if !walls.contains_key(&(guard.get_advance_row(), guard.get_advance_col())) {
                num_walls += 1;
            } else {
                walls.insert((guard.get_advance_row(), guard.get_advance_col()), 0);
            }
        }

        if grid[guard.get_row()][guard.get_col()] != 'X' {
            n_visited += 1;
        }

        grid[guard.get_row()][guard.get_col()] = guard.guard;
        // print_grid(grid.clone(), n_visited, num_walls, guard.row, guard.col);
        grid[guard.get_row()][guard.get_col()] = guard.path;

        // UNCOMMENT EVERYTHING FOR ANIMATION!
        // thread::sleep(ten_millis);
        // clearscreen::clear().expect("failed to clear screen");

        if guard.col + guard.advance_col == grid_width - 1
            || guard.col + guard.advance_col == -1
            || guard.row + guard.advance_row == grid_heigth - 1
            || guard.row + guard.advance_row == -1
        {
            break;
        }
        guard.walk();
    }
    // print_grid(grid.clone(), n_visited + 2, num_walls, guard.row, guard.col);

    println!("part one: {}", n_visited + 2);
    println!("part two: {}", num_walls);
}

fn check_bullshit_cycle(mut grid: Vec<Vec<char>>, mut guard: Guard) -> bool {
    let grid_width: i32 = grid[0].len() as i32;
    let grid_heigth: i32 = grid.len() as i32;

    let mut visited: HashMap<(i32, i32), HashMap<char, usize>> = HashMap::new();

    grid[guard.get_advance_row()][guard.get_advance_col()] = '#';

    loop {
        grid[guard.get_row()][guard.get_col()] = guard.path;

        if check_value_or_insert(&mut visited, &guard) {
            return true;
        }

        // hits a wall
        if grid[guard.get_advance_row()][guard.get_advance_col()] == '#' {
            guard.rotate_90_deg();
            continue;
        }

        guard.walk();

        if guard.col + guard.advance_col == grid_width
            || guard.col + guard.advance_col == -1
            || guard.row + guard.advance_row == grid_heigth
            || guard.row + guard.advance_row == -1
        {
            return false;
        }
    }
}

fn check_value_or_insert(
    visited: &mut HashMap<(i32, i32), HashMap<char, usize>>,
    guard: &Guard,
) -> bool {
    if let Some(inner_map) = visited.get_mut(&(guard.row, guard.col)) {
        if let Some(_) = inner_map.get_mut(&guard.guard) {
            return true;
        } else {
            inner_map.insert(guard.guard, 0);
        }
    } else {
        visited
            .entry((guard.row, guard.col))
            .or_insert_with(HashMap::new)
            .insert(guard.guard, 0);
    }
    false
}

fn print_grid(grid: Vec<Vec<char>>, n_visited: i32, num_walls: i32, x: i32, y: i32) {
    let mut result = String::new();
    let grid_rows = grid.len() as i32;
    let grid_cols = if grid.is_empty() {
        0
    } else {
        grid[0].len() as i32
    };

    // Define the window bounds (clamp to grid size)
    let start_row = (x - 20).max(0);
    let end_row = (x + 20).min(grid_rows);
    let start_col = (y - 20).max(0);
    let end_col = (y + 20).min(grid_cols);

    // Construct the windowed grid
    for row_idx in start_row..end_row {
        for col_idx in start_col..end_col {
            let char = grid[row_idx as usize][col_idx as usize];
            if char == '>' || char == '<' || char == '^' || char == 'v' {
                result.push_str(&format!(" \x1b[31m{}\x1b[0m", char)); // change color to red
            } else {
                result.push_str(&format!(" {}", char));
            }
        }
        result.push('\n'); // Add a newline at the end of each row
    }

    result.push_str(&format!("\nMoved {} distinct positions\n", n_visited));
    result.push_str(&format!(
        "Could place {} walls to force a loop\n",
        num_walls
    ));

    // Print the windowed grid as a single string
    print!("{}", result);
}

fn parse_file_to_grid(filename: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    // Attempt to read the file
    let content = fs::read_to_string(filename).expect("Failed to read file");

    let mut grid = Vec::new();
    let mut start_pos = None;

    // Parse the content into a 2D Vec and locate '^'
    for (row_idx, line) in content.lines().enumerate() {
        let row: Vec<char> = line.chars().collect();
        if let Some(col_idx) = row.iter().position(|&ch| ch == '^') {
            start_pos = Some((row_idx, col_idx));
        }
        grid.push(row);
    }

    (grid, start_pos.expect("could not find the start position"))
}
