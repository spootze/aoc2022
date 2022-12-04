use std::cmp;

type Assignment = (u32, u32, u32, u32);
type Predicate = fn(Assignment) -> bool;

fn main() {
    let assignments = include_str!("../input.txt");
    let full_overlaps = count_overlaps(assignments, fully_overlaps);
    println!("Part 1: {full_overlaps} assignments");
    let partial_overlaps = count_overlaps(assignments, partially_overlaps);
    println!("Part 2: {partial_overlaps} assignments");
}

fn count_overlaps(assignments: &str, predicate: Predicate) -> usize {
    assignments
        .lines()
        .map(parse_assignment)
        .filter(|&a| predicate(a))
        .count()
}

fn parse_assignment(assignment: &str) -> Assignment {
    let (x, y) = assignment.split_once(",").unwrap();
    let (x_l, x_r) = x.split_once("-").unwrap();
    let (y_l, y_r) = y.split_once("-").unwrap();
    (
        x_l.parse::<u32>().unwrap(),
        x_r.parse::<u32>().unwrap(),
        y_l.parse::<u32>().unwrap(),
        y_r.parse::<u32>().unwrap(),
    )
}

fn fully_overlaps(assignment: Assignment) -> bool {
    let (x_l, x_r, y_l, y_r) = assignment;
    (x_l <= y_l && y_r <= x_r) || (y_l <= x_l && x_r <= y_r)
}

fn partially_overlaps(assignment: Assignment) -> bool {
    let (x_l, x_r, y_l, y_r) = assignment;
    cmp::max(x_l, y_l) <= cmp::min(x_r, y_r)
}

#[test]
fn test_count_full_overlaps() {
    let assignments = include_str!("../test.txt");
    let n = count_overlaps(assignments, fully_overlaps);
    assert_eq!(n, 2);
}

#[test]
fn test_count_partially_overlaps() {
    let assignments = include_str!("../test.txt");
    let n = count_overlaps(assignments, partially_overlaps);
    assert_eq!(n, 4);
}

#[test]
fn test_fully_overlaps() {
    assert_eq!(fully_overlaps((2, 4, 6, 8)), false);
    assert_eq!(fully_overlaps((2, 3, 4, 5)), false);
    assert_eq!(fully_overlaps((5, 7, 7, 9)), false);
    assert_eq!(fully_overlaps((2, 8, 3, 7)), true);
    assert_eq!(fully_overlaps((6, 6, 4, 6)), true);
    assert_eq!(fully_overlaps((2, 6, 4, 8)), false);
}

#[test]
fn test_partially_overlaps() {
    assert_eq!(partially_overlaps((2, 4, 6, 8)), false);
    assert_eq!(partially_overlaps((2, 3, 4, 5)), false);
    assert_eq!(partially_overlaps((5, 7, 7, 9)), true);
    assert_eq!(partially_overlaps((2, 8, 3, 7)), true);
    assert_eq!(partially_overlaps((6, 6, 4, 6)), true);
    assert_eq!(partially_overlaps((2, 6, 4, 8)), true);
}
