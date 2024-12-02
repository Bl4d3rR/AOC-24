use itertools::Itertools;
use sortedlist_rs::SortedList;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run_day_01() {
    let input_lines = match read_lines("src/day_01/input.txt") {
        Ok(lines) => lines.filter_map(Result::ok).collect_vec(),
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };

    let sum_part_01 = solve_part_01(&input_lines);
    let sum_part_02 = solve_part_02(&input_lines);

    println!("part one: {}", sum_part_01);
    println!("part two: {}", sum_part_02);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn solve_part_01(lines: &[String]) -> u32 {
    let mut list_left = SortedList::new();
    let mut list_right = SortedList::new();
    let mut sum: u32 = 0;

    lines.iter().for_each(|line| {
        line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .tuple_windows::<(i32, i32)>()
            .for_each(|(a, b)| {
                list_left.insert(a);
                list_right.insert(b);
            });
    });

    list_left
        .to_vec()
        .iter()
        .zip(list_right.to_vec().iter())
        .for_each(|(a, b)| {
            sum += a.abs_diff(*b);
        });

    sum
}

fn solve_part_02(lines: &[String]) -> i32 {
    let mut list_left = HashMap::new();
    let mut list_right = HashMap::new();
    let mut sum: i32 = 0;

    lines.iter().for_each(|line| {
        line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .tuple_windows::<(i32, i32)>()
            .for_each(|(a, b)| {
                list_left
                    .entry(a)
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
                list_right
                    .entry(b)
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
            });
    });

    for key in list_left.keys() {
        if let Some(right_value) = list_right.get(key) {
            sum += key * right_value;
        }
    }

    sum
}
