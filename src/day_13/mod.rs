use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_day_13() {
    solve_part_01();
}

fn solve_part_01() {
    let file = File::open("src/day_13/input_big.txt").expect("could not open file");
    let reader = BufReader::new(file);
    let mut num_tokens = 0;

    let mut lines_iter = reader.lines().map(|l| l.unwrap());
    let mut buffer = Vec::new();

    while let Some(line) = lines_iter.next() {
        if line.trim().is_empty() {
            if buffer.len() == 3 {
                let result = parse_and_process_block(&buffer);

                match result {
                    Ok(value) => {
                        num_tokens += value;
                    }
                    Err(e) => {}
                }
            }
            buffer.clear();
        } else {
            buffer.push(line);
        }
    }

    // If there's a block at the end without a trailing newline
    if !buffer.is_empty() && buffer.len() == 3 {
        let result = parse_and_process_block(&buffer);

        match result {
            Ok(value) => {
                num_tokens += value;
            }
            Err(e) => {}
        }
    }

    println!("part one: {}", num_tokens);
}

fn solve(a11: f64, a12: f64, a21: f64, a22: f64, b1: f64, b2: f64) -> Option<(i32, i32)> {
    let max_a = 100.;
    let max_b = 100.;
    let det = a11 * a22 - a21 * a12;

    if det.abs() < f64::EPSILON {
        // No unique solution for the first two equations
        return None;
    }

    let a = (b1 * a22 - b2 * a12) / det;
    let b = (a11 * b2 - a21 * b1) / det;

    // Check additional constraints
    if a <= max_a && b <= max_b && a.fract() == 0. && b.fract() == 0. {
        Some(((a as i32), (b as i32)))
    } else {
        None
    }
}

fn parse_and_process_block(block: &[String]) -> Result<i32, Box<dyn std::error::Error>> {
    let a_token_value = 3;
    let b_token_value = 1;
    let re_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)")?;
    let re_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)")?;
    let re_p = Regex::new(r"Prize: X=(\d+), Y=(\d+)")?;

    let a_line = &block[0];
    let b_line = &block[1];
    let p_line = &block[2];

    let a_caps = re_a
        .captures(a_line)
        .ok_or("Failed to match Button A line")?;
    let b_caps = re_b
        .captures(b_line)
        .ok_or("Failed to match Button B line")?;
    let p_caps = re_p.captures(p_line).ok_or("Failed to match Prize line")?;

    let a11: f64 = a_caps[1].parse()?;
    let a21: f64 = a_caps[2].parse()?;
    let a12: f64 = b_caps[1].parse()?;
    let a22: f64 = b_caps[2].parse()?;
    let b1: f64 = p_caps[1].parse()?;
    let b2: f64 = p_caps[2].parse()?;

    match solve(a11, a12, a21, a22, b1, b2) {
        Some((a, b)) => Ok(a * a_token_value + b * b_token_value),
        None => Err("No solution found...".into()),
    }
}
