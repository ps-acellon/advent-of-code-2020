use crate::aoc;

const TREE: char = '#';

pub fn all() {
    part1();
    part2();
}

pub fn part1() {
    let slope = (3, 1);
    let lines = aoc::read_lines("src/day03.txt");
    let length = lines.first().unwrap().len();

    let collisions: u32 = count_collisions(&lines, &length, slope.0, slope.1);
    aoc::print_response(3, 1, &collisions);
}

pub fn part2() {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let lines = aoc::read_lines("src/day03.txt");
    let length = lines.first().unwrap().len();
    let collisions: u32 = slopes
        .into_iter()
        .map(|(dx, dy)| count_collisions(&lines, &length, dx, dy))
        .product();
    aoc::print_response(3, 2, &collisions);
}

fn count_collisions(lines: &Vec<String>, line_length: &usize, dx: usize, dy: usize) -> u32 {
    lines
        .iter()
        .step_by(dy)
        .enumerate()
        .map(|(idx, line)| (line.chars().nth((idx * dx) % line_length).unwrap() == TREE) as u32)
        .sum()
}
