use crate::aoc;
use std::collections::HashMap;

pub fn all() {
    part1();
    part2();
}

pub fn part1() {
    let mut joltages: Vec<_> = aoc::read_lines("src/day10.txt")
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    joltages.sort();
    joltages.push(joltages[joltages.len() - 1] + 3); // built-in adapter

    let mut ones = 0;
    let mut threes = 0;
    for i in 0..joltages.len() {
        let prev = match i {
            0 => 0,
            _ => joltages[i - 1],
        };
        match joltages[i] - prev {
            3 => threes += 1,
            1 => ones += 1,
            _ => (),
        };
    }
    aoc::print_response(10, 1, ones * threes)
}

pub fn part2() {
    let mut joltages: Vec<_> = aoc::read_lines("src/day10.txt")
        .iter()
        .map(|line| line.parse::<isize>().unwrap())
        .collect();
    joltages.sort();

    // let mut cache = HashMap::new();
    // let number_of_paths = number_of_paths(&mut cache, &joltages, 1) + number_of_paths(&mut cache, &joltages, 2) + number_of_paths(&mut cache, &joltages, 3);
    let number_of_paths = count_paths_dp(&joltages);
    aoc::print_response(10, 2, number_of_paths);
}

#[allow(unused)]
fn number_of_paths(cache: &mut HashMap<usize, usize>, joltages: &Vec<usize>, val: usize) -> usize {
    if !joltages.contains(&val) {
        return 0;
    };
    if &val == joltages.last().unwrap() {
        return 1;
    };

    if !cache.contains_key(&val) {
        let paths_to_here = number_of_paths(cache, joltages, val + 1)
            + number_of_paths(cache, joltages, val + 2)
            + number_of_paths(cache, joltages, val + 3);
        cache.insert(val, paths_to_here);
    }
    *cache.get(&val).unwrap()
}

/*
 * Looking at other people's solutions online, I found one that uses 'dynamic
 * programming' to solve part 2 much more simply. The following is based off
 * that.
 */

fn count_paths_dp(nums: &[isize]) -> usize {
    let mut paths_to_each_point = HashMap::new();
    paths_to_each_point.insert(0, 1);

    for &num in nums {
        let paths = paths_to_each_point.get(&(num - 1)).unwrap_or(&0)
            + paths_to_each_point.get(&(num - 2)).unwrap_or(&0)
            + paths_to_each_point.get(&(num - 3)).unwrap_or(&0);
        paths_to_each_point.insert(num, paths);
    }

    paths_to_each_point[nums.last().unwrap()]
}
