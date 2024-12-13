use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_day_13() {
    let (part_one_result, part_two_result) = solve_both_parts("src/day_13/input_big.txt");
    println!("part one: {}", part_one_result);
    println!("part two: {}", part_two_result);
}

// Reads and processes the file once, returning the sums for part 1 and part 2.
fn solve_both_parts(path: &str) -> (i64, i64) {
    let file = File::open(path).expect("could not open file");
    let reader = BufReader::new(file);

    let mut lines_iter = reader.lines().map(|l| l.expect("could not read line"));
    let mut buffer = Vec::new();

    let mut total_part1: i64 = 0;
    let mut total_part2: i64 = 0;

    while let Some(line) = lines_iter.next() {
        if line.trim().is_empty() {
            if buffer.len() == 3 {
                if let Some(value) = parse_and_process_block(&buffer, false) {
                    total_part1 += value;
                }
                if let Some(value) = parse_and_process_block(&buffer, true) {
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
        if let Some(value) = parse_and_process_block(&buffer, false) {
            total_part1 += value;
        }
        if let Some(value) = parse_and_process_block(&buffer, true) {
            total_part2 += value;
        }
    }

    (total_part1, total_part2)
}

// Parses a block and calculates the num of tokens
fn parse_and_process_block(block: &[String], part2: bool) -> Option<i64> {
    let a_token_value = 3;
    let b_token_value = 1;

    let (a11, a21, a12, a22, b1, b2) = extract_values(block).ok()?;
    solve_eq(a11, a12, a21, a22, b1, b2, part2).map(|(a, b)| a * a_token_value + b * b_token_value)
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

// solves LS
fn solve_eq(
    a11: f64,
    a12: f64,
    a21: f64,
    a22: f64,
    b1: f64,
    b2: f64,
    part2: bool,
) -> Option<(i64, i64)> {
    let max_a = if !part2 { 100. } else { f64::INFINITY };
    let max_b = if !part2 { 100. } else { f64::INFINITY };

    let det = a11 * a22 - a21 * a12;

    if det.abs() < f64::EPSILON {
        return None;
    }

    let b1 = if part2 { b1 + 10_000_000_000_000. } else { b1 };
    let b2 = if part2 { b2 + 10_000_000_000_000. } else { b2 };

    let a = (b1 * a22 - b2 * a12) / det;
    let b = (a11 * b2 - a21 * b1) / det;

    if a <= max_a && b <= max_b && a.fract() == 0. && b.fract() == 0. {
        Some((a as i64, b as i64))
    } else {
        None
    }
}

// fn solve_more_efficient(
//     a11: f64,
//     a12: f64,
//     a21: f64,
//     a22: f64,
//     b1: f64,
//     b2: f64,
//     part2: bool,
// ) -> Option<(i64, i64)> {
//     use nalgebra::{Matrix2, Vector2};

//     let a = Matrix2::from_row_slice(&[a11, a12, a21, a22]);

//     let b1 = if part2 { b1 + 10_000_000_000_000. } else { b1 };
//     let b2 = if part2 { b2 + 10_000_000_000_000. } else { b2 };

//     let b = Vector2::from_row_slice(&[b1, b2]);

//     // Solve the system using LU decomposition
//     match a.lu().solve(&b) {
//         Some(solution) => {
//             if solution.x <= max_a
//                 && solution.y <= max_b
//                 && solution.x.fract() <= 1e-13
//                 && solution.y.fract() <= 1e-13
//             {
//                 Some((solution.x as i64, solution.y as i64))
//             } else {
//                 None
//             }
//         }
//         None => None,
//     }
// }
