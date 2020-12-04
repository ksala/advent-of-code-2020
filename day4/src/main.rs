use std::fs;

use regex::Regex;

#[derive(Debug, Default)]
struct Passport {
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    byr: Option<String>, // (Birth Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>, // (Country ID)
}

impl Passport {
    fn new() -> Passport {
        Passport::default()
    }

    fn add_value(&mut self, value: &str) {
        let pieces = value.split(":").collect::<Vec<_>>();
        if pieces.len() != 2 {
            panic!("Invalid piece!")
        }
        match pieces[0] {
            "iyr" => self.iyr = Some(pieces[1].to_string()),
            "eyr" => self.eyr = Some(pieces[1].to_string()),
            "byr" => self.byr = Some(pieces[1].to_string()),
            "hgt" => self.hgt = Some(pieces[1].to_string()),
            "hcl" => self.hcl = Some(pieces[1].to_string()),
            "ecl" => self.ecl = Some(pieces[1].to_string()),
            "pid" => self.pid = Some(pieces[1].to_string()),
            "cid" => self.cid = Some(pieces[1].to_string()),
            _ => panic!("Invalid field!"),
        }
    }

    fn is_valid_solution_1(&self) -> bool {
        self.iyr.is_some()
            && self.eyr.is_some()
            && self.byr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }   

    fn is_valid_solution_2(&self) -> bool {
        if !self.is_valid_solution_1() {
            return false
        }
        if is_valid_digit(self.byr.as_ref().unwrap(), 1920, 2002).err().is_some() {
            return false
        }
        if is_valid_digit(self.iyr.as_ref().unwrap(), 2010, 2020).err().is_some() {
            return false
        }
        if is_valid_digit(self.eyr.as_ref().unwrap(), 2020, 2030).err().is_some() {
            return false
        }
        if ! Regex::new(r"^#[a-f0-9]{6}$").unwrap().is_match(self.hcl.as_ref().unwrap().as_str()) {
            return false
        }
        let valid_eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if ! valid_eye_colors.contains(&self.ecl.as_ref().unwrap().as_str()) {
            return false
        }
        if is_valid_height(self.hgt.as_ref().unwrap()).err().is_some() {
            return false
        }
        if ! Regex::new(r"^\d{9}$").unwrap().is_match(self.pid.as_ref().unwrap().as_str()) {
            return false
        }
        true
    }
}

fn is_valid_digit(s: &String, min: i32, max: i32) -> Result<(), ()> {
    let digits = s.parse::<i32>().unwrap_or(-1);
    if min > digits || max < digits {
        return Err(());
    }
    Ok(())
}

fn is_valid_height(s: &String) -> Result<(), ()> {
    let height: String = s.chars().filter(|c| c.is_digit(10)).collect();
    let height = height.parse::<i32>().unwrap_or(-1);
    if s.contains("in") {
        if height < 59 || height > 76 {
            return Err(());
        }
        return Ok(())
    } else if s.contains("cm") {
        if height < 150 || height > 193 {
            return Err(());
        }
        return Ok(())
    } else {
        return Err(());
    }
}

fn main() {
    solution();
}

fn solution() {
    let content = fs::read_to_string("src/input.txt").expect("could not read file");
    let lines = content.lines().collect::<Vec<&str>>();

    let mut passports: Vec<Passport> = vec![];
    let mut passport = Passport::new();
    for line in lines {
        if line == "" {
            // If line is empty, push existing passport, start a new one and continue to the next line
            passports.push(passport);
            passport = Passport::new();
            continue
        }
        let values = line.split_whitespace();
        for value in values {
            passport.add_value(value);
        }
    }
    // Push last passpord
    passports.push(passport);
    println!("parsed: {}", passports.len());
    let solution_1 = passports.iter().filter(|p| p.is_valid_solution_1()).count();
    println!("Solution 1: {}", solution_1); // 228
    let solution_2 = passports.iter().filter(|p| p.is_valid_solution_2()).count();
    println!("Solution 2: {}", solution_2); // 175
}

