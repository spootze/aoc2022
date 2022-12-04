use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let rucksacks = include_str!("../input.txt");
    let value = compute_rucksack_values(rucksacks);
    println!("Part 1: {value}");
    let identifier_value = compute_identifier_values(rucksacks);
    println!("Part 2: {identifier_value}");
}

fn compute_rucksack_values(rucksacks: &str) -> u32 {
    rucksacks
        .split_terminator("\n")
        .map(parse_compartments)
        .map(|(l, r)| find_common_items(l, r))
        .map(|is| is.into_iter().map(get_item_value).sum::<u32>())
        .sum()
}

fn compute_identifier_values(rucksacks: &str) -> u32 {
    rucksacks
        .split_terminator("\n")
        .map(to_hash_set)
        .tuples()
        .map(|(l, c, r)| find_common_items(l, find_common_items(c, r)))
        .map(|is| is.into_iter().map(get_item_value).sum::<u32>())
        .sum()
}

fn parse_compartments(rucksack: &str) -> (HashSet<char>, HashSet<char>) {
    let n = rucksack.len();
    assert!(
        n % 2 == 0,
        "Uneven number of items ({n}) in rucksack {rucksack}"
    );
    let (l, r) = rucksack.split_at(n / 2);
    (to_hash_set(l), to_hash_set(r))
}

fn to_hash_set(items: &str) -> HashSet<char> {
    HashSet::from_iter(items.chars())
}

fn find_common_items(left: HashSet<char>, right: HashSet<char>) -> HashSet<char> {
    let mut _right = right.clone();
    left.iter().filter_map(|v| _right.take(v)).collect()
}

fn get_item_value(item: char) -> u32 {
    match item as u32 {
        97..=122 => 1 + item as u32 - 97,
        65..=90 => 27 + item as u32 - 65,
        _ => panic!("invalid item {item}"),
    }
}

#[test]
fn test_compute_rucksack_values() {
    let rucksacks = include_str!("../test.txt");
    let score = compute_rucksack_values(rucksacks);
    assert_eq!(score, 157)
}

#[test]
fn test_compute_rucksack_identifier_values() {
    let rucksacks = include_str!("../test.txt");
    let score = compute_identifier_values(rucksacks);
    assert_eq!(score, 70)
}
