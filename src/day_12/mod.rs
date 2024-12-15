use geo::algorithm::concave_hull::ConcaveHull;
use geo::{line_string, polygon, ConvexHull, MultiPoint, Point};
use std::{collections::HashMap, fs::File, io::Read};

use indexmap::IndexMap;

static FENCES_LOOKUP: [usize; 5] = [4, 0, 2, 0, 0];

pub fn run_day_12() {
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 0.0),
        Point::new(2.0, 0.0),
        Point::new(3.0, 0.0),
        Point::new(4.0, 0.0),
        Point::new(5.0, 0.0),
        Point::new(0.0, 1.0),
        Point::new(1.0, 1.0),
        Point::new(2.0, 1.0),
        Point::new(5.0, 1.0),
        Point::new(0.0, 2.0),
        Point::new(1.0, 2.0),
        Point::new(2.0, 2.0),
        Point::new(5.0, 2.0),
        Point::new(0.0, 3.0),
        Point::new(3.0, 3.0),
        Point::new(4.0, 3.0),
        Point::new(5.0, 3.0),
        Point::new(0.0, 4.0),
        Point::new(3.0, 4.0),
        Point::new(4.0, 4.0),
        Point::new(5.0, 4.0),
        Point::new(0.0, 5.0),
        Point::new(1.0, 5.0),
        Point::new(2.0, 5.0),
        Point::new(3.0, 5.0),
        Point::new(4.0, 5.0),
        Point::new(5.0, 5.0),
    ];

    let mp: MultiPoint<f64> = MultiPoint(points);

    let hull = mp.convex_hull();
    let conc_hull = mp.concave_hull(2.);
    // hull is now a Polygon
    let num_sides = hull.exterior().0.len() - 1;
    println!("Convex hull sides: {}", num_sides);
    println!("Concave hull sides: {}", conc_hull.exterior().0.len());

    // let mut board = get_input();

    // solve_part_01(board);
}

fn solve_part_01(mut board: Vec<Vec<char>>) {
    let mut flood_indices: IndexMap<(usize, usize), bool> = IndexMap::new();
    let mut price = 0;
    let mut num_iterations = 0;

    flood_indices.insert((0, 0), true);

    loop {
        // println!(
        //     "indices: {:?}, num_iter: {}, len_indices: {}",
        //     flood_indices,
        //     num_iterations,
        //     flood_indices.len()
        // );
        if flood_indices.len() == 0 {
            break;
        }

        let start = flood_indices.get_index(0).expect("could not get map entry");
        price += flood_search(&mut board, (start.0 .0, start.0 .1), &mut flood_indices);
        flood_indices.swap_remove_index(0);
        num_iterations += 1;
    }

    println!("price: {}", price);
}

fn flood_search(
    board: &mut Vec<Vec<char>>,
    start: (usize, usize),
    flood_indices: &mut IndexMap<(usize, usize), bool>,
) -> i32 {
    let rows = board.len();
    let cols = board[0].len();
    let field = board[start.0][start.1];

    if field == '.' {
        return 0;
    }
    // println!("field: {}", field);
    // println!("board: {:?}", board);

    let mut num_fields = 0;
    let mut fences = 0;
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    let mut stack = vec![start];

    while let Some((row, col)) = stack.pop() {
        // skip if out of bounds or the color doesn't match the target
        if row >= rows || col >= cols {
            continue;
        }

        // skip if already visited
        if visited.contains_key(&(row, col)) {
            continue;
        }

        // add flood_indices
        if board[row][col] != field {
            if board[row][col] != '.' {
                flood_indices.insert((row, col), true);
            }
            continue;
        }

        // save visited to not add again
        visited.insert((row, col), true);
        num_fields += 1;
        fences += FENCES_LOOKUP[get_num_neighbours(board, (row, col))];
        // println!("fences: {}", fences);

        // Push neighboring cells onto the stack
        if row > 0 {
            stack.push((row - 1, col)); // Up
        }
        if row < rows - 1 {
            stack.push((row + 1, col)); // Down
        }
        if col > 0 {
            stack.push((row, col - 1)); // Left
        }
        if col < cols - 1 {
            stack.push((row, col + 1)); // Right
        }
    }

    for visited_coord in visited {
        board[visited_coord.0 .0][visited_coord.0 .1] = '.';
    }

    // println!(
    //     "Field: {}, Num fields: {}, fences: {}",
    //     field, num_fields, fences
    // );
    num_fields * fences as i32
}

fn get_num_neighbours(board: &mut Vec<Vec<char>>, position: (usize, usize)) -> usize {
    let mut num_neighbours = 0;
    let x = position.0;
    let y = position.1;
    let field = board[x][y];
    let width = board[0].len();
    let height = board.len();

    // lookup right
    if x < width - 1 && board[x + 1][y] == field {
        num_neighbours += 1;
    }
    // lookup down
    if y < height - 1 && board[x][y + 1] == field {
        num_neighbours += 1;
    }
    // lookup left
    if x > 0 && board[x - 1][y] == field {
        num_neighbours += 1;
    }
    // lookup up
    if y > 0 && board[x][y - 1] == field {
        num_neighbours += 1;
    }

    num_neighbours
}

fn get_input() -> Vec<Vec<char>> {
    // Open the file
    let mut file = File::open("src/day_12/input_part2.txt").expect("Failed to open file");

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
