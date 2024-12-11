use std::iter;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run_day_11() {
    let numbers = vec![17639, 47, 3858, 0, 470624, 9467423, 5, 188];
    let sum = Arc::new(Mutex::new(numbers.len() as u128));
    let mut handles = vec![];

    for &number in &numbers {
        let sum_clone = Arc::clone(&sum);
        let handle = thread::spawn(move || {
            solve_part_01(number, 0, &sum_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    let final_sum = sum.lock().unwrap();
    println!("part one: {}", *final_sum);
}

fn solve_part_01(number: u128, iteration: u128, sum: &Arc<Mutex<u128>>) {
    if iteration >= 75 {
        return;
    }

    let sum_guard = sum.lock().unwrap();
    if *sum_guard % 10_000_000 == 0 {
        println!("Sum: {}\nIteration: {}", sum_guard, iteration);
    }
    drop(sum_guard);

    if number == 0 {
        solve_part_01(1, iteration + 1, sum);
        return;
    }

    let num_str = number.to_string();
    if num_str.len() % 2 == 0 {
        let mut sum_guard = sum.lock().unwrap();
        *sum_guard += 1;
        drop(sum_guard);

        let (left_half, right_half) = num_str.split_at(num_str.len() / 2);

        if let Ok(left_half_value) = left_half.parse::<u128>() {
            solve_part_01(left_half_value, iteration + 1, sum);
        } else {
            eprintln!("Failed to parse left side: {}", num_str);
        }

        if let Ok(right_half_value) = right_half.parse::<u128>() {
            solve_part_01(right_half_value, iteration + 1, sum);
        } else {
            eprintln!("Failed to parse right side: {}", num_str);
        }
    } else {
        let new_number = number * 2024;
        solve_part_01(new_number, iteration + 1, sum);
    }
}
