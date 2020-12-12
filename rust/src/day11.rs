use crate::aoc;

const NEIGHBOR_DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
];

pub fn all() {
    part1();
    part2();
}

pub fn part1() {
    let mut seats: Vec<Vec<char>> = include_str!("./day11.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut count = usize::MIN;
    let mut prev = usize::MAX;

    while count != prev {
        seats = update_seats(&seats, 4, &find_adjacent_neighbor);
        prev = count;
        count = seats
            .iter()
            .flatten()
            .fold(0, |acc, &c| acc + (c == '#') as usize);
    }
    aoc::print_response(11, 1, count);
}

pub fn part2() {
    let mut seats: Vec<Vec<char>> = include_str!("./day11.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut count = usize::MIN;
    let mut prev = usize::MAX;

    while count != prev {
        seats = update_seats(&seats, 5, &find_visible_neighbor);
        prev = count;
        count = seats
            .iter()
            .flatten()
            .fold(0, |acc, &c| acc + (c == '#') as usize);
    }
    aoc::print_response(11, 2, count);
}

fn update_seats(
    seats: &Vec<Vec<char>>,
    occupied_limit: usize,
    neighbor_fn: &dyn Fn((usize, usize), (isize, isize), &Vec<Vec<char>>) -> Option<char>,
) -> Vec<Vec<char>> {
    let mut new_seats = seats.clone();

    for j in 0..seats.len() {
        for i in 0..seats[0].len() {
            let mut valid = vec![];
            for delta in &NEIGHBOR_DIRECTIONS {
                if let Some(neighbor) = neighbor_fn((i, j), *delta, seats) {
                    valid.push(neighbor);
                }
            }
            let neighbor_count = valid.into_iter().filter(|&c| c == '#').count();

            new_seats[j][i] = apply_rule(seats[j][i], neighbor_count, occupied_limit);
        }
    }
    new_seats
}

fn apply_rule(seat: char, occupied_neighbors: usize, limit: usize) -> char {
    match seat {
        '.' => seat,
        '#' | 'L' => match occupied_neighbors {
            num if num == 0 => '#',
            num if num >= limit => 'L',
            _ => seat,
        },
        _ => unreachable!(),
    }
}

fn find_adjacent_neighbor(
    start: (usize, usize),
    delta: (isize, isize),
    seats: &Vec<Vec<char>>,
) -> Option<char> {
    let index = (start.0 as isize + delta.0, start.1 as isize + delta.1);
    if 0 <= index.0
        && index.0 < seats[0].len() as isize
        && 0 <= index.1
        && index.1 < seats.len() as isize
    {
        return Some(seats[index.1 as usize][index.0 as usize]);
    }
    None
}

fn find_visible_neighbor(
    start: (usize, usize),
    delta: (isize, isize),
    seats: &Vec<Vec<char>>,
) -> Option<char> {
    let mut index = (start.0 as isize + delta.0, start.1 as isize + delta.1);

    while 0 <= index.0
        && index.0 < seats[0].len() as isize
        && 0 <= index.1
        && index.1 < seats.len() as isize
    {
        if seats[index.1 as usize][index.0 as usize] != '.' {
            return Some(seats[index.1 as usize][index.0 as usize]);
        }
        index = (index.0 + delta.0, index.1 + delta.1);
    }
    None
}

#[allow(unused)]
fn print_seats(seats: &Vec<Vec<char>>) {
    for line in seats {
        println!("{}", line.iter().collect::<String>());
    }
    println!("\n");
}
