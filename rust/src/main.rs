#[macro_use]
extern crate lazy_static;
use std::env;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("----------------------------------");
    println!("| Day    | Part   || Answer \t |");
    println!("|================================|");

    if args.len() == 1 {
        day01::all();
        println!("|--------------------------------|");
        day02::all();
        println!("|--------------------------------|");
        day03::all();
        println!("|--------------------------------|");
        day04::all();
        println!("|--------------------------------|");
        day05::all();
        println!("|--------------------------------|");
        day06::all();
        println!("|--------------------------------|");
        day07::all();
        println!("|--------------------------------|");
        day08::all();
        println!("|--------------------------------|");
        day09::all();
    } else {
        match &args[1]
            .parse::<usize>()
            .expect("Command line arguments must be int between 1 and 25")
        {
            1 => day01::all(),
            2 => day02::all(),
            3 => day03::all(),
            4 => day04::all(),
            5 => day05::all(),
            6 => day06::all(),
            7 => day07::all(),
            8 => day08::all(),
            9 => day09::all(),
            10..=25 => aoc::print_response(0, 0, "hold ur horses!!"),
            _ => panic!("Command line arg must be int between 1 and 25"),
        };
    }
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
