#![allow(dead_code)]

use regex::Regex;

#[derive(Debug, PartialEq)]
struct Password {
    parameters: (usize, usize),
    symbol: char,
    password: String,
}

impl Password {
    fn new(symbol: char, password: &str, parameters: (usize, usize)) -> Self {
        Password { parameters, symbol, password: password.to_owned() }
    }

    fn from_string(input: &str) -> Self {
        let re = Regex::new(r"(\d+)-(\d+)\s([a-z]):\s([a-z]+)").unwrap();
        let matches = re.captures(input).unwrap();

        let parameters = (
            matches.get(1).unwrap().as_str().parse().unwrap(),
            matches.get(2).unwrap().as_str().parse().unwrap()
        );

        Self::new(
            matches.get(3).unwrap().as_str().chars().next().unwrap(),
            matches.get(4).unwrap().as_str(),
            parameters,
        )
    }
}

fn parse_input(lines: Vec<String>) -> Vec<Password> {
    lines.iter().map(|line| Password::from_string(line)).collect()
}

fn verify_password_by_occurance(p: &Password) -> bool {
    let occurances = p.password.chars().fold(0, |acc, c| {
        if c == p.symbol {
            acc + 1
        } else {
            acc
        }
    });

    let (min, max) = p.parameters;

    occurances >= min && occurances <= max
}

fn verify_password_by_position(p: &Password) -> bool {
    let (a, b) = p.parameters;

    let char_a = p.password.chars().collect::<Vec<_>>().get(a - 1).unwrap().to_owned();
    let char_b = p.password.chars().collect::<Vec<_>>().get(b - 1).unwrap().to_owned();

    (char_a == p.symbol) ^ (char_b == p.symbol)
}

fn part_one(lines: Vec<String>) -> usize {
    parse_input(lines)
        .iter()
        .map(|p| verify_password_by_occurance(p))
        .filter(|&b| b)
        .count()
}

fn part_two(lines: Vec<String>) -> usize {
    parse_input(lines)
        .iter()
        .map(|p| verify_password_by_position(p))
        .filter(|&b| b)
        .count()
}


#[cfg(test)]
mod tests {
    use crate::input::{read_lines};
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = vec![
            "1-3 a: aaabbbd".to_string(),
            "2-9 b: ccddbb".to_string(),
        ];

        assert_eq!(parse_input(input), vec![
           Password { parameters: (1, 3), symbol: 'a', password: "aaabbbd".to_string() },
           Password { parameters: (2, 9), symbol: 'b', password: "ccddbb".to_string() },
        ])
    }

    #[test]
    fn test_verify_password_by_occurance() {
        let correct_password_1 = Password::new('e', "abcdeefg", (2, 3));
        let correct_password_2 = Password::new('e', "abcdeeeeeeeefg", (5, 9));
        let incorrect_password_1 = Password::new('a', "abbabbcc", (3, 5));
        let incorrect_password_2 = Password::new('a', "bbcaaaaaaaaaaaaabbbcc", (3, 12));

        assert_eq!(verify_password_by_occurance(&correct_password_1), true);
        assert_eq!(verify_password_by_occurance(&correct_password_2), true);
        assert_eq!(verify_password_by_occurance(&incorrect_password_1), false);
        assert_eq!(verify_password_by_occurance(&incorrect_password_2), false);
    }

    #[test]
    fn test_verify_password_by_position() {
        let correct_password_1 = Password::new('e', "abebbbbb", (1, 3));
        let correct_password_2 = Password::new('e', "ebabbbbb", (1, 3));
        let incorrect_password_1 = Password::new('e', "ebebbbbb", (1, 3));

        assert_eq!(verify_password_by_position(&correct_password_1), true);
        assert_eq!(verify_password_by_position(&correct_password_2), true);
        assert_eq!(verify_password_by_position(&incorrect_password_1), false);
    }

    #[test]
    fn test_part_one() {
        let input = read_lines("input/day2.txt");
        assert_eq!(part_one(input), 538);
    }

    #[test]
    fn test_part_two() {
        let input = read_lines("input/day2.txt");
        assert_eq!(part_two(input), 489);
    }
}
