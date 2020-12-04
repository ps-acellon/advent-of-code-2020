use crate::aoc;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

pub fn part1() {
    let mut batch = String::new();
    let mut file = File::open("src/input-day04.txt").unwrap();
    file.read_to_string(&mut batch).unwrap();

    let key_value_re = Regex::new(r"(byr|iyr|eyr|hgt|hcl|ecl|pid):(\S+)").unwrap();
    let valid_count: u32 = batch
        .split("\n\n")
        .map(|passport| (key_value_re.find_iter(&passport).collect::<Vec<_>>().len() == 7) as u32)
        .sum();
    aoc::print_response(4, 1, &valid_count);
}

pub fn part2() {
    let mut batch = String::new();
    let mut file = File::open("src/input-day04.txt").unwrap();
    file.read_to_string(&mut batch).unwrap();

    let key_value_re = Regex::new(r"(byr|iyr|eyr|hgt|hcl|ecl|pid):(\S+)").unwrap();
    let valid_count: u32 = batch
        .split("\n\n")
        .map(|passport| {
            let valid_kvp_count: u32 = key_value_re
                .captures_iter(&passport)
                .map(|cap| validate(&cap[1], &cap[2]) as u32)
                .sum();
            (valid_kvp_count == 7) as u32
        })
        .sum();
    aoc::print_response(4, 2, &valid_count);
}

fn validate(key: &str, value: &str) -> bool {
    return match key {
        "byr" => is_int_between(value, 1920, 2002),
        "iyr" => is_int_between(value, 2010, 2020),
        "eyr" => is_int_between(value, 2020, 2030),
        "hgt" => is_height_valid(value),
        "hcl" => {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            }
            RE.is_match(value)
        }
        "ecl" => {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            }
            RE.is_match(value)
        }
        "pid" => {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
            }
            RE.is_match(value)
        }
        _ => panic!("Unexpected matching error."),
    };
}

fn is_int_between(value: &str, min: u32, max: u32) -> bool {
    let year = value.parse::<u32>().unwrap();
    min <= year && year <= max
}

fn is_height_valid(value: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9]+)(cm|in)").unwrap();
    }

    let cap = RE.captures(value);
    match cap {
        Some(cap) => {
            return match &cap[2] {
                "in" => is_int_between(&cap[1], 59, 76),
                "cm" => is_int_between(&cap[1], 150, 193),
                _ => false,
            };
        }
        None => return false,
    };
}
