use std::fs::File;
use std::io::Read;

pub fn run_day_04() {
    let grid = getGrid();
    let mut counter = 0;

    // Iterate over the 2D vector row by row using iterators
    grid.iter().enumerate().for_each(|(row_index, row)| {
        // println!("Row {}: {:?}", row_index, row);

        // Iterate over the columns within the row using iterators
        row.iter().enumerate().for_each(|(col_index, &char)| {
            // check horizontally XMAS
            if col_index < row.len() - 3
                && char == 'X'
                && grid[row_index][col_index + 1] == 'M'
                && grid[row_index][col_index + 2] == 'A'
                && grid[row_index][col_index + 3] == 'S'
            {
                counter += 1;
            }
            // check horizontally SAMX
            if col_index < row.len() - 3
                && char == 'S'
                && grid[row_index][col_index + 1] == 'A'
                && grid[row_index][col_index + 2] == 'M'
                && grid[row_index][col_index + 3] == 'X'
            {
                counter += 1;
            }

            // check vertically XMAS
            if row_index < grid.len() - 3
                && char == 'X'
                && grid[row_index + 1][col_index] == 'M'
                && grid[row_index + 2][col_index] == 'A'
                && grid[row_index + 3][col_index] == 'S'
            {
                counter += 1;
            }
            // check vertically SAMX
            if row_index < grid.len() - 3
                && char == 'S'
                && grid[row_index + 1][col_index] == 'A'
                && grid[row_index + 2][col_index] == 'M'
                && grid[row_index + 3][col_index] == 'X'
            {
                counter += 1;
            }

            // check diagonally right XMAS
            if row_index < grid.len() - 3
                && col_index < row.len() - 3
                && char == 'X'
                && grid[row_index + 1][col_index + 1] == 'M'
                && grid[row_index + 2][col_index + 2] == 'A'
                && grid[row_index + 3][col_index + 3] == 'S'
            {
                counter += 1;
            }
            // check diagonally right SAMX
            if row_index < grid.len() - 3
                && col_index < row.len() - 3
                && char == 'S'
                && grid[row_index + 1][col_index + 1] == 'A'
                && grid[row_index + 2][col_index + 2] == 'M'
                && grid[row_index + 3][col_index + 3] == 'X'
            {
                counter += 1;
            }

            // check diagonally left XMAS
            if row_index < grid.len() - 3
                && col_index > 2
                && char == 'X'
                && grid[row_index + 1][col_index - 1] == 'M'
                && grid[row_index + 2][col_index - 2] == 'A'
                && grid[row_index + 3][col_index - 3] == 'S'
            {
                counter += 1;
            }
            // check diagonally left SAMX
            if row_index < grid.len() - 3
                && col_index > 2
                && char == 'S'
                && grid[row_index + 1][col_index - 1] == 'A'
                && grid[row_index + 2][col_index - 2] == 'M'
                && grid[row_index + 3][col_index - 3] == 'X'
            {
                counter += 1;
            }
        });
    });

    println!("part one: {}", counter);
}

fn getGrid() -> Vec<Vec<char>> {
    // Open the file
    let mut file = File::open("src/day_04/input_big.txt").expect("Failed to open file");

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    // Split the string by newline character
    let lines: Vec<&str> = contents.split('\n').collect();

    // Convert each line into a vector of characters
    let mut result: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let char_vec: Vec<char> = line.chars().collect();
        result.push(char_vec);
    }

    result
}
