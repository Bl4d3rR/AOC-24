use core::num;
use std::iter;

pub fn run_day_11() {
    let numbers: Vec<u128> = vec![17639, 47, 3858, 0, 470624, 9467423, 5, 188];
    let mut sum = numbers.len() as u128;
    let mut iteration = 0 as u128;

    for number in numbers {
        iteration = 0;
        solve_part_01(number, iteration, &mut sum);
    }

    println!("part one: {}", sum);
}

fn solve_part_01(number: u128, iteration: u128, sum: &mut u128) {
    // println!("Number: {}", number);
    if iteration == 75 {
        return;
    }

    let iter = iteration + 1;

    if number == 0 {
        solve_part_01(1, iter, sum);
        return;
    }

    let num_str = number.to_string();
    if num_str.len() % 2 == 0 {
        *sum += 1;
        // left half
        let left_half = &num_str[..num_str.len() / 2];
        let left_half_i32 = match left_half.parse::<u128>() {
            Ok(value) => value,
            Err(e) => {
                println!("Failed to parse left side: {}: {}", e, num_str);
                return;
            }
        };
        // println!("Number left: {}", left_half_i32);

        solve_part_01(left_half_i32, iter, sum);

        // right half
        let right_half = &num_str[num_str.len() / 2..];
        let right_half_i32 = match right_half.parse::<u128>() {
            Ok(value) => value,
            Err(e) => {
                println!("Failed to parse right side: {}: {}", e, num_str);
                return;
            }
        };
        // println!("Number right: {}", right_half_i32);

        solve_part_01(right_half_i32, iter, sum);

        return;
    }

    let mul_1024 = number * 2024;
    solve_part_01(mul_1024, iter, sum);
}
