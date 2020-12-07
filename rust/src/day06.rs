use crate::aoc;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

pub fn part1() {
    let input = get_input();
    let count: usize = input
        .split("\n\n")
        .map(|group| {
            let mut uniques = HashSet::new();
            for c in group.chars().filter(|&c| c != '\n') {
                uniques.insert(c);
            }
            uniques.len()
        })
        .sum();
    aoc::print_response(6, 1, &count);
}

pub fn part2() {
    let input = get_input();
    let groups = input.split("\n\n");
    let count: usize = groups
        .map(|group| {
            let mut answers: Vec<char> = group.lines().next().unwrap().chars().collect();
            for line in group.lines() {
                answers.retain(|&c| line.find(c) != None);
            }
            answers.len()
        })
        .sum();
    aoc::print_response(6, 2, &count)
}

fn get_input() -> String {
    let mut text = String::new();
    let mut file = File::open("src/day06.txt").unwrap();
    file.read_to_string(&mut text).unwrap();
    text
}
