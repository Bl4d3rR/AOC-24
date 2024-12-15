use regex::Regex;

use std::f32::INFINITY;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        self.first * self.second * self.third * self.fourth
    }

    // adds position to related quadrant based on grid_size
    // grid quadrants look as follows
    // 1 2
    // 3 4
    fn add_to_quadrant(&mut self, position: (i32, i32), velocity: (i32, i32), counter: i32) {
        let new_position = self
            .clone()
            .calculate_position_by_second(position, velocity, counter);

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

    let sum = solve_part_01(quadrants.clone(), 100);
    println!("part one: {}", sum);
    solve_part_02(quadrants);
}

// i know its stupid and slow, but it works and it was late
fn solve_part_02(quadrants: quadrants) {
    let mut min_sum = INFINITY as i32;
    let mut min_counter = 0;

    for i in 0..10_000 {
        let sum = solve_part_01(quadrants.clone(), i);
        if sum < min_sum {
            min_sum = sum;
            min_counter = i;
        }
    }
    println!("part two: {}", min_counter)
}

fn solve_part_01(mut quadrants: quadrants, counter: i32) -> i32 {
    let file = File::open("src/day_14/input.txt").expect("could not open file");
    let reader = BufReader::new(file);

    let re =
        Regex::new(r"^p=(?P<px>-?\d+),(?P<py>-?\d+)\s+v=(?P<vx>-?\d+),(?P<vy>-?\d+)$").unwrap();

    for line_res in reader.lines() {
        let line = line_res.expect("could not read line");
        if let Some(((px, py), (vx, vy))) = parse_line(&line, &re) {
            quadrants.add_to_quadrant((px, py), (vx, vy), counter);
        } else {
            eprintln!("Failed to parse line: {}", line);
        }
    }

    quadrants.get_sum()
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
