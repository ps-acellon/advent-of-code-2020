use crate::aoc;

pub fn all() {
    part1();
    part2();
}

pub fn part1() {
    let numbers: Vec<_> = aoc::read_lines("src/day09.txt")
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    aoc::print_response(9, 1, find_invalid_number(25, &numbers));
}

fn find_invalid_number(window: usize, numbers: &Vec<usize>) -> usize {
    let mut output = 0;
    for i in 0..numbers.len() {
        let mut sums = vec![];
        for j in i..i + window {
            for k in j..i + window {
                sums.push(numbers[j] + numbers[k]);
            }
        }
        if !sums.contains(&numbers[i + window]) {
            output = numbers[i + window];
            break;
        }
    }
    output
}

pub fn part2() {
    let numbers: Vec<_> = aoc::read_lines("src/day09.txt")
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    let invalid = find_invalid_number(25, &numbers);
    for i in 0..numbers.len() {
        let mut j = i + 1;
        let mut sum: usize = numbers[i..j].iter().sum();
        while sum < invalid {
            j += 1;
            sum = numbers[i..j].iter().sum();
        }
        if sum == invalid {
            let encryption_weakness =
                numbers[i..j].iter().min().unwrap() + numbers[i..j].iter().max().unwrap();
            aoc::print_response(9, 2, encryption_weakness);
            break;
        }
    }
}
