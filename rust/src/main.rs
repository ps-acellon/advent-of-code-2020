#[macro_use]
extern crate lazy_static;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

fn main() {
    println!("----------------------------------");
    println!("| Day    | Part   || Answer \t |");
    println!("|================================|");
    day01::part1();
    day01::part2();
    println!("|--------------------------------|");
    day02::part1();
    day02::part2();
    println!("|--------------------------------|");
    day03::part1();
    day03::part2();
    println!("|--------------------------------|");
    day04::part1();
    day04::part2();
    println!("|--------------------------------|");
    day05::part1();
    day05::part2();
    println!("|--------------------------------|");
    day06::part1();
    day06::part2();
    println!("----------------------------------");
}

mod aoc {
    use std::fmt::Debug;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    pub fn read_lines<P>(filename: P) -> Vec<String>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename).expect("Unable to open file.");
        io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap())
            .collect()
    }

    pub fn print_response<T: Debug>(day: u32, part: u32, response: T) {
        println!(
            "| Day {day:0>2} | Part {part} || {response:#?} \t |",
            day = day,
            part = part,
            response = response
        );
    }
}
