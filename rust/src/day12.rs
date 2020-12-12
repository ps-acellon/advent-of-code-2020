use crate::aoc;

pub fn all() {
    part1();
    part2();
}

pub fn part1() {
    let data = include_str!("./day11.txt");
    aoc::print_response(12, 1, data);
}

pub fn part2() {
    let data = include_str!("./day11.txt");
    aoc::print_response(12, 2, data);
}
