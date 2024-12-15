use itertools::Position;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Rem;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Default, Clone)]
struct quadrants {
    first: i32,
    second: i32,
    third: i32,
    fourth: i32,
    grid_size: (i32, i32),
}

impl quadrants {
    fn get_sum(self) -> i32 {
        // println!(
        //     "{}, {}, {}, {}",
        //     self.first, self.second, self.third, self.fourth
        // );
        self.first * self.second * self.third * self.fourth
    }

    // adds position to related quadrant based on grid_size
    // grid quadrants look as follows
    // 1 2
    // 3 4
    fn add_to_quadrant(&mut self, position: (i32, i32), velocity: (i32, i32)) {
        let new_position = self.clone().calculate_position(position, velocity);
        // println!(
        //     "pos: {}, {}, velocity: {}, {}, new_pos: {}, {}, grid: {}, {}",
        //     position.0,
        //     position.1,
        //     velocity.0,
        //     velocity.1,
        //     new_position.0,
        //     new_position.1,
        //     self.grid_size.0,
        //     self.grid_size.1
        // );

        // check first quadrant
        if new_position.0 < self.grid_size.0 / 2 && new_position.1 < self.grid_size.1 / 2 {
            self.first += 1;
        }

        // check second quadrant
        if new_position.0 > self.grid_size.0 / 2 && new_position.1 < self.grid_size.1 / 2 {
            self.second += 1;
        }

        // check third quadrant
        if new_position.0 < self.grid_size.0 / 2 && new_position.1 > self.grid_size.1 / 2 {
            self.third += 1;
        }

        // check fourth quadrant
        if new_position.0 > self.grid_size.0 / 2 && new_position.1 > self.grid_size.1 / 2 {
            self.fourth += 1;
        }
    }

    fn calculate_position_by_second(
        self,
        position: (i32, i32),
        velocity: (i32, i32),
        seconds: i32,
    ) -> (i32, i32) {
        let mut new_position = (0, 0);

        new_position.0 = (seconds * velocity.0 + position.0).rem_euclid(self.grid_size.0);
        new_position.1 = (seconds * velocity.1 + position.1).rem_euclid(self.grid_size.1);

        new_position
    }

    fn calculate_position(self, position: (i32, i32), velocity: (i32, i32)) -> (i32, i32) {
        let mut new_position = (0, 0);

        new_position.0 = (100 * velocity.0 + position.0).rem_euclid(self.grid_size.0);
        new_position.1 = (100 * velocity.1 + position.1).rem_euclid(self.grid_size.1);

        new_position
    }
}

pub fn run_day_14() {
    let mut quadrants = quadrants::default();
    quadrants.grid_size = (101, 103);
    solve_part_02();
}

fn solve_part_02() {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 103]; 101];
    let mut counter = 0;
    let mut quadrants = quadrants::default();
    quadrants.grid_size = (101, 103);

    loop {
        thread::sleep(Duration::from_millis(500));
        // clearscreen::clear().expect("failed to clear screen");
        let file = File::open("src/day_14/input.txt").expect("could not open file");
        let reader = BufReader::new(file);

        let re =
            Regex::new(r"^p=(?P<px>-?\d+),(?P<py>-?\d+)\s+v=(?P<vx>-?\d+),(?P<vy>-?\d+)$").unwrap();

        for line_res in reader.lines() {
            let line = line_res.expect("could not read line");
            if let Some(((px, py), (vx, vy))) = parse_line(&line, &re) {
                let position =
                    quadrants
                        .clone()
                        .calculate_position_by_second((px, py), (vx, vy), counter);
                grid[position.0 as usize][position.1 as usize] = 'X'
            } else {
                eprintln!("Failed to parse line: {}", line);
            }
        }
        print_grid(&mut grid);
        println!("Iteration: {}", counter);
        counter += 1;
    }
}

fn print_grid(grid: &mut Vec<Vec<char>>) {
    // Print the grid (borrow immutably while printing)
    for row in &*grid {
        for &digit in row {
            print!("{}", digit);
        }
        println!();
    }

    // Now modify the values
    for row in grid.iter_mut() {
        for digit in row {
            *digit = '.'; // For example, increment each digit
        }
    }
}

fn solve_part_01(mut quadrants: quadrants) {
    let file = File::open("src/day_14/input.txt").expect("could not open file");
    let reader = BufReader::new(file);

    let re =
        Regex::new(r"^p=(?P<px>-?\d+),(?P<py>-?\d+)\s+v=(?P<vx>-?\d+),(?P<vy>-?\d+)$").unwrap();

    for line_res in reader.lines() {
        let line = line_res.expect("could not read line");
        if let Some(((px, py), (vx, vy))) = parse_line(&line, &re) {
            quadrants.add_to_quadrant((px, py), (vx, vy));
        } else {
            eprintln!("Failed to parse line: {}", line);
        }
    }

    println!("part one: {}", quadrants.get_sum());
}

fn parse_line(line: &str, re: &Regex) -> Option<((i32, i32), (i32, i32))> {
    if let Some(caps) = re.captures(line) {
        let px: i32 = caps["px"].parse().ok()?;
        let py: i32 = caps["py"].parse().ok()?;
        let vx: i32 = caps["vx"].parse().ok()?;
        let vy: i32 = caps["vy"].parse().ok()?;

        Some(((px, py), (vx, vy)))
    } else {
        None
    }
}
