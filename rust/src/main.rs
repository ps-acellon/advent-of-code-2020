fn main() {
    day01::part1();
    day01::part2();
}

pub mod day01 {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    pub fn part2() {
        let lines = read_lines("src/input-day01.txt");
        let expenses = get_sorted_expenses(lines);
        for (idx, expense) in expenses.iter().enumerate() {
            for front_offset in (idx + 1)..expenses.len() {
                for offset in (idx + 1)..expenses.len() {
                    let front_summand = expenses[front_offset];
                    let summand = expenses[expenses.len() - offset];
                    if expense + front_summand + summand == 2020 {
                        println!("Day 1, part 1 - {}", expense * front_summand * summand);
                        return;
                    }
                }
            }
        }
    }

    pub fn part1() {
        let lines = read_lines("src/input-day01.txt");
        let expenses = get_sorted_expenses(lines);
        for (idx, expense) in expenses.iter().enumerate() {
            for offset in (idx + 1)..expenses.len() {
                let summand = expenses[expenses.len() - offset];
                if expense + summand == 2020 {
                    println!("Day 1, part 2 - {}", expense * summand);
                    return;
                }
            }
        }
    }

    fn get_sorted_expenses(lines: io::Lines<io::BufReader<File>>) -> Vec<i32> {
        let mut expenses: Vec<i32> = lines
            .into_iter()
            .map(|line| line.unwrap())
            .map(|line| line.parse::<i32>().unwrap())
            .collect();
        expenses.sort();
        expenses
    }

    fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename).expect("Unable to open file.");
        io::BufReader::new(file).lines()
    }
}
