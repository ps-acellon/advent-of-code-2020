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
            println!("| Day 01 | Part 1 || {} \t |", expense * summand);
            return
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
                        "| Day 01 | Part 2 || {} \t |",
                        first_entry * second_entry * third_entry
                    );
                    return
                }
            }
        }
    }
}
