use memoize::memoize;

pub fn run_day_11() {
    let numbers: Vec<u128> = vec![17639, 47, 3858, 0, 470624, 9467423, 5, 188];
    // We start sum from the length of the numbers (as in your original code)
    let mut sum = numbers.len() as u128;
    for number in numbers {
        // Start iteration at 0 for each number
        sum += solve_part_01(number, 0);
    }
    println!("part one: {}", sum);
}

#[memoize]
fn solve_part_01(number: u128, iteration: u128) -> u128 {
    if iteration >= 75 {
        return 0;
    }
    let iter = iteration + 1;

    if number == 0 {
        return solve_part_01(1, iter);
    }

    let num_str = number.to_string();
    if num_str.len() % 2 == 0 {
        // If length is even, add 1 to sum and recurse into the left and right halves.
        let mut total = 1;

        let left_half = &num_str[..num_str.len() / 2];
        let left_half_val = left_half.parse::<u128>().unwrap();
        total += solve_part_01(left_half_val, iter);

        let right_half = &num_str[num_str.len() / 2..];
        let right_half_val = right_half.parse::<u128>().unwrap();
        total += solve_part_01(right_half_val, iter);

        total
    } else {
        // If length is odd, multiply by 2024 and recurse
        let mul_1024 = number * 2024;
        solve_part_01(mul_1024, iter)
    }
}
