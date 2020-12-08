#![allow(dead_code)]

use std::collections::HashSet;
use std::collections::HashMap;

fn group_to_answers(group: &str) -> usize {
    let mut map: HashSet<char> = HashSet::new();

    for c in group.chars() {
        if c == '\n' { continue; }

        map.insert(c);
    }

    map.len()
}

fn group_to_answers_for_all(group: &str) -> usize {
    let mut answers: HashMap<char, usize> = HashMap::new();
    let people: Vec<&str> = group.split("\n").filter(|&n| n != "").collect();

    for person in people.iter() {
        for c in person.chars() {
            let increment = answers.get(&c).unwrap_or(&0).to_owned();
            answers.insert(c, increment + 1);
        }
    }

    answers.iter().fold(0, |acc, (_, &val)| {
        if val == people.len() {
            acc + 1
        } else {
            acc
        }
    })
}

fn part_one(groups: Vec<&str>) -> usize {
    groups.iter().fold(0, |acc, n| acc + group_to_answers(n))
}

fn part_two(groups: Vec<&str>) -> usize {
    groups.iter().fold(0, |acc, n| acc + group_to_answers_for_all(n))
}

#[cfg(test)]
mod tests {
    use crate::input::{read_file};
    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("input/day6.txt");
        let groups: Vec<&str> = input
            .split("\n\n")
            .collect();

        assert_eq!(part_one(groups), 6775);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("input/day6.txt");
        let groups: Vec<&str> = input
            .split("\n\n")
            .collect();

        assert_eq!(part_two(groups), 3356);
    }
}
