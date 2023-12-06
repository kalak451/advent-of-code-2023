use std::fs;

mod day_01;
mod day_02;
mod day_03;
mod grid;
mod day_04;
mod day_05;
mod day_06;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn read_data_file(day: usize, name: &str) -> String {
    let path = format!("data/day{:02}/{}", day, name);

    return fs::read_to_string(path).unwrap();
}