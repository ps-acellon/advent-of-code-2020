use crate::aoc;

const CARDINALS: [char; 4] = ['E', 'S', 'W', 'N'];

pub fn all() {
    part1();
    part2();
}

pub fn part1() {
    let data = include_str!("./day12.txt").lines();
    let mut heading = 'E';
    let mut position = (0, 0);
    for line in data {
        navigate(&mut heading, &mut position, line);
        // println!("{}, {}", position.0, position.1);
    }
    aoc::print_response(12, 1, position.0.abs() + position.1.abs());
}

pub fn part2() {
    let data = include_str!("./day11.txt");
    // aoc::print_response(12, 2, data);
}

fn navigate(heading: &mut char, position: &mut (isize, isize), instruction: &str) {
    let action = instruction.chars().nth(0).unwrap();
    let value = instruction
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<isize>()
        .unwrap();

    match action {
        'L' => {
            let pos = CARDINALS.iter().position(|c| c == heading).unwrap() as isize;
            let new_pos = ((pos + 4 - (value / 90)) % 4) as usize;
            *heading = CARDINALS[new_pos];
        }
        'R' => {
            let pos = CARDINALS.iter().position(|c| c == heading).unwrap();
            let new_pos = (pos + (value as usize / 90)) % 4;
            *heading = CARDINALS[new_pos];
        }
        'N' => position.1 += value,
        'E' => position.0 += value,
        'S' => position.1 -= value,
        'W' => position.0 -= value,
        'F' => match heading {
            'N' => position.1 += value,
            'S' => position.1 -= value,
            'E' => position.0 += value,
            'W' => position.0 -= value,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
}

fn navigate_waypoint(waypoint: &mut (isize, isize), position: &mut (isize, isize), instruction: &str) {
    let action = instruction.chars().nth(0).unwrap();
    let value = instruction
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<isize>()
        .unwrap();

    match action {
        'L' => (),
        'R' => (),
        'N' => waypoint.1 += value,
        'E' => waypoint.0 += value,
        'S' => waypoint.1 -= value,
        'W' => waypoint.0 -= value,
        'F' => *position = (position.0 + value * waypoint.0, position.1 + value * waypoint.1),
        _ => unreachable!(),
    };
}

fn rotate(current: &(isize, isize), degrees: isize) -> (isize, isize) {

    (0, 0)
}