use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::hash::Hash;

mod day_01;
mod day_02;
mod day_03;
mod grid;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn read_data_file(day: usize, name: &str) -> String {
    let path = format!("data/day{:02}/{}", day, name);

    return fs::read_to_string(path).unwrap();
}

fn shortest_path<T: Ord + Copy + Hash, F: Fn(T) -> Vec<(T, i64)>>(start: T, adj: F) -> HashMap<T, i64> {
    let mut dist = HashMap::new();
    dist.insert(start, 0);

    let mut queue = BinaryHeap::new();
    queue.push((0i64, start));

    while let Some((_, t)) = queue.pop() {
        adj(t)
            .into_iter()
            .for_each(|(new_t, new_dist)| {
                let total_dist_to_new_t = *dist.get(&t).unwrap_or(&0i64) + new_dist;
                if total_dist_to_new_t < *dist.get(&new_t).unwrap_or(&i64::MAX) {
                    dist.insert(new_t, total_dist_to_new_t);
                    queue.push((-total_dist_to_new_t, new_t))
                }
            });
    }

    return dist;
}