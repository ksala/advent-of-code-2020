use std::fs;

fn main() {
    solution();
}

fn solution() {
    let mut lines: Vec<&str> = vec![];
    let content = fs::read_to_string("src/input.txt").expect("could not read file");
    for row in content.lines() {
        lines.push(row);
    }

    let input_width = lines[0].len();
    let moves = vec![(3, 1), (1, 1), (5, 1), (7, 1), (1, 2)];
    let mut x;
    let mut y;
    let mut tree_found;
    let mut first_solution = 0;
    let mut total: i64 = 1; // Set to 1 so we can just multiply it with the result
    for (right, down) in &moves {
        tree_found = 0;
        x = 0;
        y = 0;
        loop {
            y = (y + right) % input_width; // Modulo so we wrap around the field
            x = x + down;
            if x >= lines.len() {
                // Save the result of the first set of moves
                if first_solution == 0 {
                    first_solution = tree_found;
                }
                break;
            }
            if lines[x].chars().nth(y).unwrap() == '#' {
                tree_found = tree_found + 1;
            }
        }
        total = total * tree_found;
    }
    println!("First solution: {}", first_solution);
    println!("Second solution: {}", total);
}
