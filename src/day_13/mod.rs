use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_day_13() {
    let (part_one_result, part_two_result) = solve_both_parts("src/day_13/input_big.txt");
    println!("part one: {}", part_one_result);
    println!("part two: {}", part_two_result);
}

// Reads and processes the file once, returning the sums for part 1 and part 2.
fn solve_both_parts(path: &str) -> (i32, u128) {
    let file = File::open(path).expect("could not open file");
    let reader = BufReader::new(file);

    let mut lines_iter = reader.lines().map(|l| l.expect("could not read line"));
    let mut buffer = Vec::new();

    let mut total_part1: i32 = 0;
    let mut total_part2: u128 = 0;

    while let Some(line) = lines_iter.next() {
        if line.trim().is_empty() {
            if buffer.len() == 3 {
                if let Some(value) = parse_and_process_block_part1(&buffer) {
                    total_part1 += value;
                }
                if let Some(value) = parse_and_process_block_part2(&buffer) {
                    total_part2 += value;
                }
            }
            buffer.clear();
        } else {
            buffer.push(line);
        }
    }

    // Handle the last block if there's no trailing newline
    if buffer.len() == 3 {
        if let Some(value) = parse_and_process_block_part1(&buffer) {
            total_part1 += value;
        }
        if let Some(value) = parse_and_process_block_part2(&buffer) {
            total_part2 += value;
        }
    }

    (total_part1, total_part2)
}

// Part 1 solve function
fn solve_part1(a11: f64, a12: f64, a21: f64, a22: f64, b1: f64, b2: f64) -> Option<(i32, i32)> {
    let max_a = 100.;
    let max_b = 100.;
    let det = a11 * a22 - a21 * a12;

    if det.abs() < f64::EPSILON {
        return None;
    }

    let a = (b1 * a22 - b2 * a12) / det;
    let b = (a11 * b2 - a21 * b1) / det;

    if a <= max_a && b <= max_b && a.fract() == 0. && b.fract() == 0. {
        Some((a as i32, b as i32))
    } else {
        None
    }
}

// Part 2 solve function
fn solve_part2(a11: f64, a12: f64, a21: f64, a22: f64, b1: f64, b2: f64) -> Option<(u128, u128)> {
    let det = a11 * a22 - a21 * a12;
    if det.abs() < f64::EPSILON {
        return None;
    }

    // Offset as per the original logic
    let adjusted_b1 = b1 + 10_000_000_000_000.;
    let adjusted_b2 = b2 + 10_000_000_000_000.;

    let a = (adjusted_b1 * a22 - adjusted_b2 * a12) / det;
    let b = (a11 * adjusted_b2 - a21 * adjusted_b1) / det;

    if a.fract() == 0. && b.fract() == 0. {
        Some((a as u128, b as u128))
    } else {
        None
    }
}

// Parses a block and calculates the part 1 score
fn parse_and_process_block_part1(block: &[String]) -> Option<i32> {
    let a_token_value = 3;
    let b_token_value = 1;

    let (a11, a21, a12, a22, b1, b2) = extract_values(block).ok()?;
    solve_part1(a11, a12, a21, a22, b1, b2).map(|(a, b)| a * a_token_value + b * b_token_value)
}

// Parses a block and calculates the part 2 score
fn parse_and_process_block_part2(block: &[String]) -> Option<u128> {
    let a_token_value = 3;
    let b_token_value = 1;

    let (a11, a21, a12, a22, b1, b2) = extract_values(block).ok()?;
    solve_part2(a11, a12, a21, a22, b1, b2).map(|(a, b)| a * a_token_value + b * b_token_value)
}

// Extracts the numeric values from a block of three lines
fn extract_values(
    block: &[String],
) -> Result<(f64, f64, f64, f64, f64, f64), Box<dyn std::error::Error>> {
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

    Ok((a11, a21, a12, a22, b1, b2))
}
