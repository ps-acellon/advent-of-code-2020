pub mod day01;
pub mod day02;

fn main() {
    println!("----------------------------------");
    println!("| Day    | Part   || Answer \t |");
    println!("|================================|");
    day01::part1();
    day01::part2();
    println!("|--------------------------------|");
    day02::part1();
    day02::part2();
    println!("----------------------------------");
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
