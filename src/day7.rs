#![allow(dead_code)]

use std::str::FromStr;
use regex::Regex;
use std::fmt;

#[derive(Debug, PartialEq)]
struct Rule {
    color: String,
    amount: usize,
}

impl FromStr for Rule {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Rule, Self::Err> {
        let re = Regex::new(r"(\d+) ([a-z|\s]+) (bag|bags)$").unwrap();
        let matches = re.captures(&input).unwrap();

        let color = matches.get(2)
            .ok_or("bagrule -> can't find color")?
            .as_str()
            .into();

        let amount = matches.get(1)
            .ok_or("bagrule -> can't find amount")?
            .as_str()
            .parse()
            .map_err(|_| "bagrule -> can't parse amount")?;

        Ok(Rule { color, amount })
    }
}

#[derive(Debug, PartialEq)]
struct Bag {
    color: String,
    rules: Vec<Rule>,
}

impl FromStr for Bag {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Bag, Self::Err> {
        let re = Regex::new(r"^([a-z\s]+) bags contain ([a-z\d\s,]+)\.$").unwrap();
        let matches = re.captures(&input).unwrap();

        let color: String = matches.get(1)
            .ok_or("bag -> can't find color")?
            .as_str()
            .into();

        let rules_string = matches.get(2)
            .ok_or("bag -> can't find rules")?
            .as_str();

        let rules: Vec<Rule>  = match rules_string {
            "no other bags" => vec![],
            string => {
                let mut v = vec![];
                for rule in string.split(",") {
                    v.push(rule.parse()?);
                }
                v
            }
        };

        Ok(Bag { color, rules })
    }
}

impl fmt::Display for Bag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.color)
    }
}

fn find_bags_for(bags: &Vec<Bag>, color: &str) -> usize {
    let mut result: Vec<&Bag> = bags
        .iter()
        .filter(|b| b.rules.iter().find(|r| r.color == color).is_some())
        .collect();

    let mut next_bags: Vec<&Bag> = result.clone();

    loop {
        let mut new_bags: Vec<&Bag> = vec![];

        for bag in next_bags.iter() {
            let mut results: Vec<&Bag> = bags
                .iter()
                .filter(|b| b.rules.iter().find(|r| r.color == bag.color).is_some())
                .collect();

            new_bags.append(&mut results);
        }

        new_bags.sort_by(|a, b| a.color.partial_cmp(&b.color).unwrap());
        new_bags.dedup_by(|a, b| &a.color == &b.color);

        if new_bags.len() == 0 {
            break;
        }

        next_bags = new_bags.clone();

        result.append(&mut new_bags);
    }

    result.sort_by(|a, b| a.color.partial_cmp(&b.color).unwrap());
    result.dedup_by(|a, b| &a.color == &b.color);
    result.len()
}

fn find_total_bags_inside(bags: &Vec<Bag>, colors: Vec<(&str, usize)>) -> usize {
    colors.iter().fold(0, |acc, (color, times)| {
        let bag = bags.iter().find(|&b| &b.color == color).unwrap();

        let bag_colors: Vec<(&str, usize)> = bag.rules.iter().map(|r| (r.color.as_str(), r.amount)).collect();

        acc + times + (times * find_total_bags_inside(bags, bag_colors))
    })
}


#[cfg(test)]
mod tests {
    use crate::input::{read_lines};
    use super::*;

    #[test]
    fn test_parse_rule() {
        let rule = "2 dark lavender bags".parse::<Rule>();
        assert_eq!(rule, Ok(Rule { color: "dark lavender".to_string(), amount: 2 }));

        let rule = "13 vibrant magenta bag".parse::<Rule>();
        assert_eq!(rule, Ok(Rule { color: "vibrant magenta".to_string(), amount: 13 }));
    }

    #[test]
    fn test_parse_bag() {
        let bag = "dotted salmon bags contain 2 dark lavender bags, 1 muted red bag, 1 vibrant magenta bag.".parse::<Bag>();
        assert_eq!(
            bag,
            Ok(Bag {
                color: "dotted salmon".to_string(),
                rules: vec![
                    Rule { color: "dark lavender".to_string(), amount: 2 },
                    Rule { color: "muted red".to_string(), amount: 1 },
                    Rule { color: "vibrant magenta".to_string(), amount: 1 },
                ],
            })
        );
    }

    #[test]
    fn test_find_bags_for() {
        let input: Vec<Bag> = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".parse().unwrap(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".parse().unwrap(),
            "bright white bags contain 1 shiny gold bag.".parse().unwrap(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".parse().unwrap(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".parse().unwrap(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".parse().unwrap(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".parse().unwrap(),
            "faded blue bags contain no other bags.".parse().unwrap(),
            "dotted black bags contain no other bags.".parse().unwrap(),
        ];

        assert_eq!(find_bags_for(&input, "shiny gold"), 4);
    }

    #[test]
    fn test_find_total_bags_inside() {
        let input: Vec<Bag> = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".parse().unwrap(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".parse().unwrap(),
            "bright white bags contain 1 shiny gold bag.".parse().unwrap(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".parse().unwrap(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".parse().unwrap(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".parse().unwrap(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".parse().unwrap(),
            "faded blue bags contain no other bags.".parse().unwrap(),
            "dotted black bags contain no other bags.".parse().unwrap(),
        ];

        assert_eq!(find_total_bags_inside(&input, vec![("shiny gold", 1)]) - 1, 32);
    }

    #[test]
    fn test_part_one() {
        let bags: Vec<Bag> = read_lines("input/day7.txt").iter().map(|l| l.parse().unwrap()).collect();
        assert_eq!(find_bags_for(&bags, "shiny gold"), 161);
    }

    #[test]
    fn test_part_two() {
        let bags: Vec<Bag> = read_lines("input/day7.txt").iter().map(|l| l.parse().unwrap()).collect();
        assert_eq!(find_total_bags_inside(&bags, vec![("shiny gold", 1)]) - 1, 0);
    }
}
