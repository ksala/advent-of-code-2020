use std::fs;

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    ch: char,
    password: String,
}

impl Password {
    fn new(password: String, ch: char, min: usize, max: usize) -> Password {
        Password {
            password,
            ch,
            min,
            max,
        }
    }

    fn new_from_line(line: &str) -> Option<Password> {
        let pieces = line.split(" ").collect::<Vec<_>>();
        if pieces.len() != 3 {
            return None
        }
        let minmax = pieces[0].split("-").collect::<Vec<_>>();
        let min = minmax[0].parse::<usize>().unwrap();
        let max = minmax[1].parse::<usize>().unwrap();
        let ch = pieces[1].chars().nth(0).unwrap();
        let password = pieces[2];
        Some(Password::new(password.to_string(), ch, min, max))
    }

    fn is_valid(&self) -> bool {
        let matches = self.password.matches(self.ch).count();
        if (matches > self.max) || (matches < self.min) {
            return false;
        }
        true
    }

    fn is_valid_solution_2(&self) -> bool {
        if (self.password.chars().nth(self.min-1).unwrap() == self.ch) ^
            (self.password.chars().nth(self.max-1).unwrap() == self.ch) {
            return true;
        }
        false
    }
}

fn main() {
    solution();
}

fn solution() {
    let mut passwords: Vec<Password> = vec![];
    let content = fs::read_to_string("src/input.txt").expect("could not read file");
    for row in content.lines() {
        passwords.push(Password::new_from_line(row).unwrap());
    }
    let mut count_solution_1 = 0;
    let mut count_solution_2 = 0;
    for password in passwords {
        if password.is_valid() {
            count_solution_1 = count_solution_1 + 1;
        }
        if password.is_valid_solution_2() {
            count_solution_2 = count_solution_2 + 1;
        }
    }
    println!("solution 1: {}", count_solution_1);
    println!("solution 2: {}", count_solution_2);
}