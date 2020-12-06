use crate::aoc;

pub fn part1() {
    let lines = aoc::read_lines("src/input-day05.txt");
    let max_id = bsp_to_seat_id(lines).into_iter().max().unwrap();
    aoc::print_response(5, 1, &max_id);
}

// O(xlogx) I think
// pub fn part2() {
//     let lines = aoc::read_lines("src/input-day05.txt");
//     let mut seat_ids = bsp_to_seat_id(lines);
//     seat_ids.sort();
//     for id in seat_ids[0]..seat_ids[seat_ids.len() - 1] {
//         if !seat_ids.contains(&id) {
//             aoc::print_response(5, 2, &id);
//         }
//     }
// }

// O(x) potentially - but also maybe O(xlogx)
pub fn part2() {
    let lines = aoc::read_lines("src/input-day05.txt");
    let seat_ids = bsp_to_seat_id(lines);
    let min = seat_ids.iter().min().unwrap();
    let max = seat_ids.iter().max().unwrap();
    for id in *min..*max {
        if !seat_ids.contains(&id) {
            aoc::print_response(5, 2, &id);
        }
    }
}

fn bsp_to_seat_id(lines: Vec<String>) -> Vec<usize> {
    lines.into_iter().map(|line| {
        let binary_string: String = line.chars().map(|c| match c {
            'F' => '0',
            'B' => '1',
            'L' => '0',
            'R' => '1',
            _ => panic!("unexpected char")
        }).collect();
        usize::from_str_radix(&binary_string, 2).unwrap()
    }).collect()
}