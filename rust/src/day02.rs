use crate::aoc::read_lines;
use regex::Regex;

pub fn part1() {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]{1}): ([a-z]+)$").unwrap();
    let good_password_count: u32 = read_lines("src/input-day02.txt")
        .into_iter()
        .map(|line| {
            let cap = re.captures(&line).unwrap();
            let min: usize = cap[1].parse().unwrap();
            let max: usize = cap[2].parse().unwrap();
            let c = &cap[3];
            let pw = &cap[4];
            let matches = pw.matches(c).collect::<Vec<&str>>().len();
            if min <= matches && matches <= max {
                1
            } else {
                0
            }
        })
        .sum();
    println!("| Day 02 | Part 1 || {} \t |", &good_password_count);
}

pub fn part2() {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]{1}): ([a-z]+)$").unwrap();
    let good_password_count: u32 = read_lines("src/input-day02.txt")
        .into_iter()
        .map(|line| {
            let cap = re.captures(&line).unwrap();
            let first_idx = cap[1].parse::<usize>().unwrap() - 1;
            let second_idx = cap[2].parse::<usize>().unwrap() - 1;
            let c = &cap[3];
            let pw = &cap[4];
            let matches: Vec<_> = pw.match_indices(c).collect();

            if matches.contains(&(first_idx, c)) ^ matches.contains(&(second_idx, c)) {
                1
            } else {
                0
            }
        })
        .sum();
    println!("| Day 02 | Part 2 || {} \t |", &good_password_count);
}
