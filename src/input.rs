use std::{hash::Hash, fs};
use std::str::FromStr;
use std::collections::HashSet;

pub fn to_vec<T: Default + FromStr>(path: &str, split_on: &str) -> Vec<T> {
    let mut rows: Vec<T> = vec![];
    let content = fs::read_to_string(path).expect("could not read file");
    for row in content.split(split_on) {
        // Should probably not use default and either return a Result or crash
        rows.push(row.parse::<T>().unwrap_or_default());
    }
    return rows;
}

pub fn to_set<T: Default + FromStr + Eq + Hash>(path: &str, split_on: &str) -> HashSet<T> {
    let mut set = HashSet::new();
    let content = fs::read_to_string(path).expect("could not read file");
    for row in content.split(split_on) {
        // Should probably not use default and either return a Result or crash
        set.insert(row.parse::<T>().unwrap_or_default());
    }
    return set;
}