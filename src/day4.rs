#![allow(dead_code)]

use std::str::FromStr;
use std::collections::HashMap;
use regex::Regex;

struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<usize>,
}

fn parse_num(map: &HashMap<&str, &str>, field: &str) -> Result<usize, &'static str> {
    map
        .get(field)
        .ok_or("failed finding field")
        .and_then(|&s| s.parse::<usize>().map_err(|_| "failed parsing field to usize"))
}

fn parse_string(map: &HashMap<&str, &str>, field: &str) -> Result<String, &'static str> {
    map
        .get(field)
        .map(|&s| s.into())
        .ok_or("failed parsing field to string")
}

fn validate_year(year: usize, min: usize, max: usize) -> Result<usize, &'static str> {
    if year >= min && year <= max {
        Ok(year)
    } else {
        Err("invalid year")
    }
}

fn validate_height(s: String) -> Result<String, &'static str> {
    let re = Regex::new(r"(\d+)([a-z]+)").unwrap();

    if !re.is_match(&s) {
        return Err("bad height")
    }

    let matches = re.captures(&s).unwrap();
    let num: usize = matches.get(1).unwrap().as_str().parse::<usize>().map_err(|_| "cant parse height")?;
    let unit = matches.get(2).unwrap().as_str();

    match unit {
        "cm" => if num >= 150 && num <= 193 { Ok(s) } else { Err("invalid length") },
        "in" => if num >= 59 && num <= 76 { Ok(s) } else { Err("invalid length") },
        _ => Err("bad unit"),
    }
}

fn validate_hair_color(s: String) -> Result<String, &'static str> {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    if re.is_match(&s.trim()) { Ok(s) } else { Err("invalid hair color") }
}

fn validate_eye_color(s: String) -> Result<String, &'static str> {
    match s.as_str().trim() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Ok(s),
        _ => Err("invalid eye color"),
    }
}

fn validate_pid(s: String) -> Result<String, &'static str> {
    let re = Regex::new(r"^\d{9}$").unwrap();
    if re.is_match(&s.trim()) { Ok(s) } else { Err("invalid pid") }
}

impl FromStr for Passport {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Passport, Self::Err> {
        let fields: HashMap<&str, &str> = input
            .split("\n")
            .map(|line| line.split(" "))
            .flatten()
            .filter_map(|line| {
                let parts = line.split(":").collect::<Vec<_>>();
                let key = parts.get(0)?;
                let field = parts.get(1)?;
                Some((key.to_owned(), field.to_owned()))
            })
            .collect();

        if fields.len() < 7 {
            return Err("not enough fields");
        }

        let passport = Passport {
            byr: parse_num(&fields, "byr").and_then(|s| validate_year(s, 1920, 2002))?,
            iyr: parse_num(&fields, "iyr").and_then(|s| validate_year(s, 2010, 2020))?,
            eyr: parse_num(&fields, "eyr").and_then(|s| validate_year(s, 2020, 2030))?,
            hgt: parse_string(&fields, "hgt").and_then(|s| validate_height(s))?,
            hcl: parse_string(&fields, "hcl").and_then(|s| validate_hair_color(s))?,
            ecl: parse_string(&fields, "ecl").and_then(|s| validate_eye_color(s))?,
            pid: parse_string(&fields, "pid").and_then(|s| validate_pid(s))?,
            cid: parse_num(&fields, "cid").ok(),
        };

        Ok(passport)
    }
}

fn validate(entries: &Vec<&str>) -> Result<usize, &'static str> {
    let mut passports: Vec<Passport> = vec![];

    for entry in entries.iter() {
        match entry.parse() {
            Ok(p) => passports.push(p),
            _ => (),
        }
    }

    Ok(passports.len())
}

#[cfg(test)]
mod tests {
    use crate::input::{read_file};
    use super::*;

    #[test]
    fn test_passport_from_string() {
        let p1 = "ecl:gry pid:860033327 eyr:2020
            hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm".parse::<Passport>();
        let p2 = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929".parse::<Passport>();
        let p3 = "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn
            pid:760753108 byr:1931 hgt:179cm".parse::<Passport>();
        let p4 = "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011
            ecl:brn hgt:59in".parse::<Passport>();
        let p5 = "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170
            pid:186cm iyr:2018 byr:1926".parse::<Passport>();

        assert_eq!(p1.is_ok(), true);
        assert_eq!(p2.is_ok(), false);
        assert_eq!(p3.is_ok(), true);
        assert_eq!(p4.is_ok(), false);
        assert_eq!(p5.is_ok(), false);
    }

    #[test]
    fn validations() {
        assert_eq!(validate_year(2002, 1920, 2002).is_ok(), true);
        assert_eq!(validate_year(2003, 1920, 2002).is_ok(), false);
        assert_eq!(validate_year(1920, 1920, 2002).is_ok(), true);
        assert_eq!(validate_year(1919, 1920, 2002).is_ok(), false);

        assert_eq!(validate_height("60in".into()).is_ok(), true);
        assert_eq!(validate_height("190cm".into()).is_ok(), true);
        assert_eq!(validate_height("150cm".into()).is_ok(), true);
        assert_eq!(validate_height("190in".into()).is_ok(), false);

        assert_eq!(validate_hair_color("#123abc".into()).is_ok(), true);
        assert_eq!(validate_hair_color("#aabb99".into()).is_ok(), true);
        assert_eq!(validate_hair_color("#123abz".into()).is_ok(), false);

        assert_eq!(validate_eye_color("brn".into()).is_ok(), true);
        assert_eq!(validate_eye_color("grn".into()).is_ok(), true);
        assert_eq!(validate_eye_color("wat".into()).is_ok(), false);

        assert_eq!(validate_pid("000000001".into()).is_ok(), true);
        assert_eq!(validate_pid("0123456789".into()).is_ok(), false);
    }

    #[test]
    fn test_product() {
        let input = read_file("input/day4.txt");
        let entries: Vec<&str> = input
            .split("\n\n")
            .collect();

        assert_eq!(validate(&entries), Ok(184));
    }
}
