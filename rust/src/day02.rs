use crate::aoc;
use regex::Regex;

const REGEX_PATTERN: &str = r"^(\d+)-(\d+) ([a-z]{1}): ([a-z]+)$";

pub fn part1() {
    let re = Regex::new(REGEX_PATTERN).unwrap();
    let good_password_count: u32 = aoc::read_lines("src/input-day02.txt")
        .into_iter()
        .map(|line| {
            let cap = re.captures(&line).unwrap();
            let min: usize = cap[1].parse().unwrap();
            let max: usize = cap[2].parse().unwrap();
            let matches: Vec<_> = (&cap[4]).matches(&cap[3]).collect();
            (min <= matches.len() && matches.len() <= max) as u32
        })
        .sum();
    aoc::print_response(2, 1, &good_password_count);
}

pub fn part2() {
    let re = Regex::new(REGEX_PATTERN).unwrap();
    let good_password_count: u32 = aoc::read_lines("src/input-day02.txt")
        .into_iter()
        .map(|line| {
            let cap = re.captures(&line).unwrap();
            let first_idx = cap[1].parse::<usize>().unwrap() - 1;
            let second_idx = cap[2].parse::<usize>().unwrap() - 1;
            let c = &cap[3];
            let matches: Vec<_> = (&cap[4]).match_indices(c).collect();            
            (matches.contains(&(first_idx, c)) ^ matches.contains(&(second_idx, c))) as u32
        })
        .sum();
    aoc::print_response(2, 2, &good_password_count);
}
