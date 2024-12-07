use core::num;
use std::{collections::VecDeque, fs, thread, time};

#[derive(Clone, Copy)]
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
        self.col.try_into().unwrap()
    }
    pub fn get_row(&self) -> usize {
        self.row.try_into().unwrap()
    }
    pub fn get_advance_col(&self) -> usize {
        (self.col + self.advance_col).try_into().unwrap()
    }
    pub fn get_advance_row(&self) -> usize {
        (self.row + self.advance_row).try_into().unwrap()
    }
}

pub fn run_day_06() {
    let ten_millis = time::Duration::from_millis(400);
    clearscreen::clear().expect("failed to clear screen");

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

    let grid_width: i32 = grid[0].len().try_into().unwrap();
    let grid_heigth: i32 = grid.len().try_into().unwrap();

    let mut n_visited = 0;

    grid[start.0][start.1] = 'X';

    let mut walls: VecDeque<(i32, i32)> = VecDeque::new();
    let mut num_walls = 0;
    let mut num_fields_to_walk = 0;
    let mut walked = 0;
    let mut row_wall = true;
    let mut checked = false;
    let mut cnt = 0;

    loop {
        // cnt += 1;
        // if cnt == 4 {
        //     break;
        // }
        guard.walk();
        if check_bullshit_cycle(grid.clone(), guard.clone(), row_wall) {
            num_walls += 1;
            checked = true;
        }

        // if walls.len() == 3 {
        //     println!("walked: {} num_fields: {}", walked, num_fields_to_walk);
        //     if walked >= num_fields_to_walk && num_fields_to_walk != 0 && !checked {
        //         num_walls += 1;
        //     }
        //     checked = false;

        //     num_fields_to_walk = check_possible_loop(&walls, guard.guard);

        //     walls.pop_front();
        //     walked = 0
        // }

        // hits a wall in a row
        if grid[guard.get_row()][guard.get_advance_col()] == '#' {
            walls.push_back((guard.row, guard.get_advance_col().try_into().unwrap()));

            guard.rotate_90_deg();
            guard.advance_row = guard.advance_col;
            guard.advance_col = 0;
            row_wall = true;
        }

        // hits a wall in a column
        if grid[guard.get_advance_row()][guard.get_col()] == '#' {
            walls.push_back((guard.get_advance_row().try_into().unwrap(), guard.col));

            guard.rotate_90_deg();
            guard.advance_col = guard.advance_row * -1;
            guard.advance_row = 0;
            row_wall = false;
        }
        if grid[guard.get_row()][guard.get_col()] != 'X' {
            n_visited += 1;
        }

        let ridx: usize = guard.get_row();
        let cidx: usize = guard.get_col();

        grid[ridx][cidx] = guard.guard;
        print_grid(grid.clone(), n_visited, num_walls, guard.row, guard.col);
        grid[ridx][cidx] = guard.path;

        if guard.col + guard.advance_col == grid_width - 1
            || guard.col + guard.advance_col == 0
            || guard.row + guard.advance_row == grid_heigth - 1
            || guard.row + guard.advance_row == 0
        {
            // thread::sleep(ten_millis);
            // clearscreen::clear().expect("failed to clear screen");

            guard.walk();
            walked += 1;
            if check_bullshit_cycle(grid.clone(), guard.clone(), row_wall) {
                num_walls += 1;
                checked = true;
            }

            if grid[guard.get_row()][guard.get_col()] != 'X' {
                n_visited += 1;
            }

            let ridx: usize = guard.get_row();
            let cidx: usize = guard.get_col();

            grid[ridx][cidx] = guard.guard;
            print_grid(grid.clone(), n_visited, num_walls, guard.row, guard.col);
            grid[ridx][cidx] = guard.path;

            n_visited += 1;

            // if check_bullshit_cycle(grid.clone(), guard.clone(), row_wall) {
            //     num_walls += 1;
            //     checked = true;
            // }
            // thread::sleep(ten_millis);
            // clearscreen::clear().expect("failed to clear screen");

            println!("walked: {} num_fields: {}", walked, num_fields_to_walk);

            print_grid(grid, n_visited, num_walls, guard.row, guard.col);

            break;
        }
        walked += 1;

        // thread::sleep(ten_millis);
        // clearscreen::clear().expect("failed to clear screen");
    }
}

fn check_bullshit_cycle(mut grid: Vec<Vec<char>>, mut guard: Guard, row: bool) -> bool {
    let grid_width: i32 = grid[0].len().try_into().unwrap();
    let grid_heigth: i32 = grid.len().try_into().unwrap();

    // println!("\nBEGIN --------------");

    // println!("{}, {}", guard.advance_row, guard.advance_col);

    if !row {
        guard.advance_row = guard.advance_col;
        guard.advance_col = 0;
    } else {
        guard.advance_col = guard.advance_row * -1;
        guard.advance_row = 0;
    }
    // println!("{}, {}", guard.advance_row, guard.advance_col);

    loop {
        if grid[guard.get_row()][guard.get_col()] == 'X' {
            if grid[guard.get_advance_row()][guard.get_advance_col()] == '#' {
                // println!("END TURE--------------\n\n");
                return true;
            }
        }
        if guard.col + guard.advance_col == grid_width - 1
            || guard.col + guard.advance_col == 0
            || guard.row + guard.advance_row == grid_heigth - 1
            || guard.row + guard.advance_row == 0
        {
            // println!("END--------------\n\n");
            return false;
        }
        if grid[guard.get_row()][guard.get_advance_col()] == '#' {
            guard.advance_row = if guard.advance_col != 0 {
                guard.advance_col
            } else {
                guard.advance_row
            };
            guard.advance_col = 0;
        }

        // hits a wall in a column
        if grid[guard.get_advance_row()][guard.get_col()] == '#' {
            guard.advance_col = if guard.advance_row * -1 != 0 {
                guard.advance_row * -1
            } else {
                guard.advance_col
            };
            guard.advance_row = 0;
        }

        grid[guard.get_row()][guard.get_col()] = guard.path;
        // print_grid(grid.clone(), 0, 0, guard.row, guard.col);
        guard.walk();
        // println!("{}, {}", guard.advance_row, guard.advance_col);
    }
}

fn check_possible_loop(walls: &VecDeque<(i32, i32)>, direction: char) -> i32 {
    let mut possible = false;
    let mut num_fields_to_walk = 0;

    if (walls[0].0 - walls[1].0).abs() == 1
        || (walls[0].0 - walls[2].0).abs() == 1
        || (walls[1].0 - walls[2].0).abs() == 1
    {
        if (walls[0].1 - walls[1].1).abs() == 1
            || (walls[0].1 - walls[2].1).abs() == 1
            || (walls[1].1 - walls[2].1).abs() == 1
        {
            possible = true;
        }
    }

    // get num fields to walk, to check if there is an obstacle in the way
    num_fields_to_walk = match direction {
        '<' => (walls[0].1 - walls[1].1).abs(),
        '^' => (walls[0].0 - walls[1].0).abs(),
        '>' => (walls[0].1 - walls[1].1).abs(),
        'v' => (walls[0].0 - walls[1].0).abs(),
        _ => 0,
    };

    println!("num field to walk: {}", num_fields_to_walk);
    if possible {
        num_fields_to_walk
    } else {
        0
    }
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
                result.push_str(&format!(" \x1b[31m{}\x1b[0m", char));
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
