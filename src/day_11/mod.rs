use memoize::memoize;

pub fn run_day_11() {
    let numbers: Vec<u128> = vec![17639, 47, 3858, 0, 470624, 9467423, 5, 188];
    // We start sum from the length of the numbers (as in your original code)
    let mut sum_part_01 = numbers.len() as u128;
    let mut sum_part_02 = numbers.len() as u128;

    for number in numbers.clone() {
        sum_part_01 += solve(number, 0, 25);
        sum_part_02 += solve(number, 0, 75);
    }

    println!("part one: {}", sum_part_01);
    println!("part two: {}", sum_part_02);
}

#[memoize]
fn solve(number: u128, iteration: u16, max_depth: u16) -> u128 {
    if iteration >= max_depth {
        return 0;
    }
    let iter = iteration + 1;

    if number == 0 {
        return solve(1, iter, max_depth);
    }

    let num_str = number.to_string();
    if num_str.len() % 2 == 0 {
        // If length is even, add 1 to sum and recurse into the left and right halves.
        let mut total = 1;

        let left_half = &num_str[..num_str.len() / 2];
        let left_half_val = left_half.parse::<u128>().unwrap();
        total += solve(left_half_val, iter, max_depth);

        let right_half = &num_str[num_str.len() / 2..];
        let right_half_val = right_half.parse::<u128>().unwrap();
        total += solve(right_half_val, iter, max_depth);

        return total;
    }

    // If length is odd, multiply by 2024 and recurse
    let mul_1024 = number * 2024;
    solve(mul_1024, iter, max_depth)
}
