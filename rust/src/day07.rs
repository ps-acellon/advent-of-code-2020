use crate::aoc;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct ContainedBag {
    number: usize,
    color: String,
}

#[derive(Debug)]
struct Bag {
    color: String,
    parents: Vec<String>,
    children: Vec<ContainedBag>,
}

impl Bag {
    fn new(color: &str) -> Self {
        Self {
            color: String::from(color),
            parents: vec![],
            children: vec![],
        }
    }
}

pub fn all() {
    part1();
    part2();
}

pub fn part1() {
    let rules = aoc::read_lines("src/day07.txt");
    let bag_collection = build_map(rules);
    let mut set = HashSet::new();
    get_parents("shiny gold", &bag_collection, &mut set);
    aoc::print_response(7, 1, set.len());
}

fn build_map(rules: Vec<String>) -> HashMap<String, Bag> {
    let mut bag_collection = HashMap::new();
    for rule in rules.iter() {
        lazy_static! {
            static ref OUTER: Regex = Regex::new(r"^([\w ]+) bags contain ([\w, ]+)").unwrap();
            static ref INNER: Regex = Regex::new(r"([0-9]+) ([a-z]+ [a-z]+)").unwrap();
        }
        let outer_cap = OUTER.captures(rule).unwrap();
        let outer_bag_color = &outer_cap[1];
        for inner_cap in INNER.captures_iter(&outer_cap[2]) {
            {
                let inner_bag_color = String::from(&inner_cap[2]);
                let inner_bag = bag_collection
                    .entry(inner_bag_color)
                    .or_insert(Bag::new(&inner_cap[2]));
                inner_bag.parents.push(String::from(outer_bag_color));
            }
            {
                let outer_bag = bag_collection
                    .entry(String::from(outer_bag_color))
                    .or_insert(Bag::new(outer_bag_color));
                outer_bag.children.push(ContainedBag {
                    color: String::from(&inner_cap[2]),
                    number: inner_cap[1].parse::<usize>().unwrap(),
                });
            }
        }
    }
    bag_collection
}

fn get_parents(key: &str, bags: &HashMap<String, Bag>, set: &mut HashSet<String>) {
    let node = bags.get(key).unwrap();
    for parent in node.parents.iter() {
        get_parents(parent, bags, set);
        set.insert((&parent).to_string());
    }
}

pub fn part2() {
    let rules = aoc::read_lines("src/day07.txt");
    let bag_collection = build_map(rules);
    let count = recursive_count("shiny gold", 1, &bag_collection) - 1;
    aoc::print_response(7, 2, count);
}

fn recursive_count(key: &str, multiplier: usize, bags: &HashMap<String, Bag>) -> usize {
    let mut count = multiplier;
    let node = bags.get(key).unwrap();
    for child in node.children.iter() {
        count += multiplier * recursive_count(&child.color, child.number, bags);
    }
    count
}
