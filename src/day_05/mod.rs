use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn run_day_05() {
    let (rules, order_map, updates) = read_file();
    solve_part_01(rules, order_map, updates);
}

fn solve_part_01(
    rules: HashMap<String, HashMap<String, usize>>,
    order_map: HashMap<i32, Vec<i32>>,
    updates: Vec<String>,
) {
    let mut sum_part_01 = 0;
    let mut sum_part_02 = 0;

    updates.into_iter().for_each(|update| {
        let mut valid = true;
        let update_vec: Vec<&str> = update.split(",").collect();

        for (index, item) in update_vec.clone().into_iter().enumerate() {
            if index < update_vec.len() - 1 {
                if let Some(inner_map) = rules.get(&item.to_owned()) {
                    if !inner_map.contains_key(update_vec[index + 1]) {
                        valid = false;
                        break;
                    }
                }
            }
        }

        if valid {
            sum_part_01 += update_vec[update_vec.len() / 2].parse::<i32>().unwrap();
        } else {
            // Convert Vec<&str> to Vec<i32> and unwrap each parse result
            let mut update_vec_int: Vec<i32> = update_vec
                .iter()
                .map(|s| s.parse::<i32>().unwrap()) // Use unwrap to get the parsed value, will panic if parsing fails
                .collect();

            update_vec_int.sort_by(|a, b| {
                if is_before(*a, *b, &order_map) {
                    std::cmp::Ordering::Less
                } else if is_before(*b, *a, &order_map) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });

            sum_part_02 += update_vec_int[update_vec_int.len() / 2];
        }
    });

    println!("part one: {}", sum_part_01);
    println!("part two: {}", sum_part_02);
}

fn is_before(a: i32, b: i32, order_map: &HashMap<i32, Vec<i32>>) -> bool {
    if let Some(after_list) = order_map.get(&a) {
        after_list.contains(&b)
    } else {
        false
    }
}

fn read_file() -> (
    HashMap<String, HashMap<String, usize>>,
    HashMap<i32, Vec<i32>>,
    Vec<String>,
) {
    let mut file = File::open("src/day_05/input.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    // Split the contents into two parts
    let parts: Vec<&str> = contents.split("\n\n").collect();

    // Process the rules into a HashMap
    let mut rules_map: HashMap<String, HashMap<String, usize>> = HashMap::new();
    parts[0].lines().for_each(|line| {
        let mut split = line.splitn(2, '|');
        let key = split.next().unwrap().to_string();
        let value = split.next().unwrap_or("").to_string();
        let entry = rules_map.entry(key).or_insert_with(HashMap::new);

        *entry.entry(value).or_insert(0) += 1;
    });

    let mut order_map: HashMap<i32, Vec<i32>> = HashMap::new();

    // Parse each constraint and store the order relationships
    for constraint in parts[0].lines() {
        let parts: Vec<i32> = constraint
            .split('|')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        // Ensure the order is represented (parts[0] must be before parts[1])
        order_map
            .entry(parts[0])
            .or_insert(Vec::new())
            .push(parts[1]);
    }

    // Process the updates into a Vec<String>
    let updates: Vec<String> = parts[1].lines().map(String::from).collect();

    (rules_map, order_map, updates)
}
