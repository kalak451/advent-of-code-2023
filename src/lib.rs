use std::fs;

mod day_01;
mod day_02;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn read_data_file(day: usize, name: &str) -> String {
    let path = format!("data/day{:02}/{}", day, name);

    return fs::read_to_string(path).unwrap();
}