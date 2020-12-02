use aoc2020::input;

use std::collections::HashSet;

fn main() {
    println!("Loop:");
    loop_solution();
    println!("Set:");
    hashmap_solution();
}

const YEAR: i32 = 2020;

fn hashmap_solution() {
    let nums: Vec<i32> = input::to_vec("src/input.txt", "\n");
    let hashnums: HashSet<i32> = input::to_set("src/input.txt", "\n");
    let mut found_two = false;
    let mut found_three = false;
    for i in 0..nums.len() {
        // Sum of 2 numbers == 2020
        let a = 2020-nums.get(i).unwrap();
        match hashnums.get(&(YEAR-a)) {
            None => {},
            Some(b) => { 
                if !found_two {
                    found_two = true;
                    println!("{}, {}, {}", a, b, a * b);
                }
            },
        }

        // Sum of 3 numbers == 2020
        for j in (i+1)..nums.len() {
            let a = nums.get(i).unwrap();
            let b = nums.get(j).unwrap();
            if a + b < YEAR {
                match hashnums.get(&(YEAR-(a+b))) {
                    None => {},
                    Some(c) => { 
                        if !found_three {
                            found_three = true;
                            println!("{}, {}, {}, {}", a, b, c, a * b * c);
                        }
                    },
                }
            }
        }
    }
}

fn loop_solution() {
    let nums: Vec<i32> = input::to_vec("src/input.txt", "\n");
    for x in 0..nums.len() {
        // We can start at x to avoid processing numbers already processed
        for y in (x+1)..nums.len() {
            let num1 = nums.get(x).unwrap();
            let num2 = nums.get(y).unwrap();
            let solution_1 = num1 + num2;
            if solution_1 == YEAR {
                println!("{}, {}: {}", num1, num2, num1 * num2)
            }
            // Try to get solution 2 only if the sum of the current numbers is below 2020
            if solution_1 < YEAR {
                for z in (y+1)..nums.len() {
                    let num3 = nums.get(z).unwrap();
                    let solution_2 = solution_1 + num3;
                    if solution_2 == YEAR {
                        println!("{}, {}, {}: {}", num1, num2, num3, num1 * num2 * num3)
                    }
                }
            }
        }
    }
}
