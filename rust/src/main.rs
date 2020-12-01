fn main() {
    day01::part1();
    day01::part2();
}

mod day01 {
    use crate::aoc::read_lines;
    const SUM: i32 = 2020;

    pub fn part1() {
        let expenses: Vec<i32> = read_lines("src/input-day01.txt")
            .into_iter()
            .map(|line| line.parse::<i32>().unwrap())
            .collect();
        for expense in expenses.iter() {
            let summand = SUM - expense;
            if expenses.contains(&summand) {
                println!("Day 1, part 1: {}", expense * summand);
                return;
            }
        }
    }

    pub fn part2() {
        let mut expenses: Vec<i32> = read_lines("src/input-day01.txt")
            .into_iter()
            .map(|line| line.parse::<i32>().unwrap())
            .collect();
        expenses.sort();
        for (first_idx, first_entry) in expenses.iter().enumerate() {
            for second_idx in (first_idx + 1)..expenses.len() {
                for third_idx in (first_idx + 2)..expenses.len() {
                    let second_entry = expenses[second_idx];
                    let third_entry = expenses[expenses.len() - third_idx];
                    if first_entry + second_entry + third_entry == SUM {
                        println!(
                            "Day 1, part 2: {}",
                            first_entry * second_entry * third_entry
                        );
                        return;
                    }
                }
            }
        }
    }
}

mod aoc {
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
}
