use std::fs;

fn main() {
    solution();
}

fn solution() {
    let content = fs::read_to_string("src/input.txt").expect("could not read file");
    let board = content.lines().collect::<Vec<&str>>();

    let solution_1 = calculate_downslope(&board, 3, 1);
    println!("First solution: {}", solution_1);

    let moves = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut solution_2: i64 = 1;
    for (right, down) in moves {
        solution_2 *= calculate_downslope(&board, right, down) as i64;
    }
    println!("Second solution: {}", solution_2);
}

fn calculate_downslope(board: &Vec<&str>, right: usize, down: usize) -> i32 {
    let mut y = 0;
    let mut tree_found = 0;
    for line in board.iter().skip(down).step_by(down) {
        y = (y + right) % line.len();
        if line.chars().nth(y).unwrap() == '#' {
            tree_found += 1;
        }
    }
    tree_found
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_board() -> Vec<&'static str> {
        let content = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        return content.lines().collect::<Vec<&str>>();
    }

    #[test]
    fn it_calculate_solution_1() {
        assert_eq!(7, calculate_downslope(&make_board(), 3, 1));
    }

    #[test]
    fn it_calculate_solution_2() {
        let moves = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut solution_2: i64 = 1;
        for (right, down) in moves {
            solution_2 *= calculate_downslope(&make_board(), right, down) as i64;
        }
        assert_eq!(336, solution_2);
    }
}
