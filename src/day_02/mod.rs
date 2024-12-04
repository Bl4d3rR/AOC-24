use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static DECREASING_SET: Lazy<HashSet<i32>> = Lazy::new(|| [1, 2, 3].into_iter().collect());
static INCREASING_SET: Lazy<HashSet<i32>> = Lazy::new(|| [-1, -2, -3].into_iter().collect());

pub fn run_day_02() {
    let mut num_safe_part_01: u32 = 0;
    let mut num_safe_part_02: u32 = 0;

    if let Ok(lines) = read_lines("src/day_02/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let line_vec = line_to_vec(&line);

            let safe_01 = solve_part_01(&line_vec);
            if safe_01 {
                num_safe_part_01 += 1;
            }

            let safe_02 = solve_part_02(&line_vec);
            if safe_02 {
                num_safe_part_02 += 1;
            }
        }
    }

    println!("part one: {}", num_safe_part_01);
    println!("part two: {}", num_safe_part_02);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn solve_part_01(line: &Vec<i32>) -> bool {
    let mut line_set: HashSet<i32> = HashSet::new();

    line.iter()
        .copied()
        .tuple_windows::<(i32, i32)>() // this processes the array in tuple windows
        .for_each(|(a, b)| {
            // the "|" are an anonymous function in Rust
            line_set.insert(a - b);
        });

    return line_set.is_subset(&DECREASING_SET) || line_set.is_subset(&INCREASING_SET);
}

fn solve_part_02(line: &Vec<i32>) -> bool {
    let mut levels_to_delete: HashSet<i32> = HashSet::new();

    // let inc = (line[0] - line[1]) > 0;

    if solve_part_01(line) {
        return true;
    }

    // mega dumm, aber kb mehr edgecases zu suchen, lul
    for i in 0..line.len() - 1 {
        let diff = line[i] - line[i + 1];
        if diff == 0 || diff < -3 || diff > 3 {
            levels_to_delete.insert(i.try_into().unwrap());
            levels_to_delete.insert((i + 1).try_into().unwrap());
        }

        // check for monotony change (from decreasing to increasing)
        // e.g. 2 5 3 4 6 <- here the 5 needs to be removed
        // e.g. 2 5 3 8 9 <- here the 3 needs to be removed
        // e.g. 2 0 1 2 5 <- here the 2 needs to be remobed
        if i < line.len() && i > 0 {
            if line[i] < line[i + 1] && line[i] < line[i - 1] {
                levels_to_delete.insert(i.try_into().unwrap());
                levels_to_delete.insert((i + 1).try_into().unwrap());
                levels_to_delete.insert((i - 1).try_into().unwrap());
            }

            if line[i] > line[i + 1] && line[i] > line[i - 1] {
                levels_to_delete.insert(i.try_into().unwrap());
                levels_to_delete.insert((i + 1).try_into().unwrap());
                levels_to_delete.insert((i - 1).try_into().unwrap());
            }
        }
    }

    // if there are more than 4 indices
    if levels_to_delete.len() > 4 {
        return false;
    }
    // if levels_to_delete.len() == 4 {
    //     println!("Levels: {:?} ", line);
    //     println!("Levels to delete: {:?} ", levels_to_delete);
    //     println!("");
    // }

    for level_to_delete in levels_to_delete {
        let mut cloned_vec = line.clone();
        _ = cloned_vec.remove(level_to_delete.try_into().unwrap());

        let is_safe = solve_part_01(&cloned_vec);
        if is_safe {
            return true;
        }
    }

    return false;
}

fn line_to_vec(raw_line: &str) -> Vec<i32> {
    return raw_line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect_vec();
}
