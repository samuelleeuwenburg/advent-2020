#![allow(dead_code)]

use std::fs;

pub fn read_nums(filename: &str) -> Vec<usize> {
    let input = fs::read_to_string(filename)
        .expect("err");

    input.split("\n")
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect()
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let input = fs::read_to_string(filename)
        .expect("err");

    input.split("\n")
        .filter_map(|s| s.trim().parse().ok())
        .filter(|s| s != "")
        .collect()
}

