fn main() {
    let input = include_str!("../input.txt");
    let score = score_strategies(input);
    println!("Part 1: {score}");
    let score2 = score_strategies_from_outcome(input);
    println!("Part 2: {score2}")
}

fn score_strategies(strategies: &str) -> i32 {
    strategies
        .lines()
        .map(parse_line)
        .map(|(l, r)| score_strategy(l, r))
        .sum()
}

fn score_strategies_from_outcome(strategies: &str) -> i32 {
    strategies
        .lines()
        .map(parse_line)
        .map(|(l, r)| (l, encode_outcome(r)))
        .map(|(l, r)| (l, determine_my_strategy(l, r)))
        .map(|(l, r)| score_strategy(l, r))
        .sum()
}

fn parse_line(line: &str) -> (char, char) {
    let (l, r) = line.split_once(' ').unwrap();
    (l.chars().next().unwrap(), r.chars().next().unwrap())
}

fn score_strategy(their_shape: char, my_shape: char) -> i32 {
    let outcome = determine_outcome(their_shape, my_shape);
    score_outcome(outcome) + score_shape(my_shape)
}

fn score_outcome(outcome: Outcome) -> i32 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Loss => 0,
    }
}

fn determine_outcome(their_shape: char, my_shape: char) -> Outcome {
    match (their_shape, my_shape) {
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => Outcome::Draw,
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => Outcome::Win,
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => Outcome::Loss,
        _ => panic!("Invalid shapes {their_shape}, {my_shape}"),
    }
}

fn score_shape(shape: char) -> i32 {
    match shape {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => panic!("Invalid shapes {shape}"),
    }
}

fn encode_outcome(outcome_str: char) -> Outcome {
    match outcome_str {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("Invalid desired outcome {outcome_str}"),
    }
}

fn determine_my_strategy(their_shape: char, desired_outcome: Outcome) -> char {
    match (their_shape, desired_outcome) {
        ('A', Outcome::Draw) | ('B', Outcome::Loss) | ('C', Outcome::Win) => 'X',
        ('A', Outcome::Win) | ('B', Outcome::Draw) | ('C', Outcome::Loss) => 'Y',
        ('A', Outcome::Loss) | ('B', Outcome::Win) | ('C', Outcome::Draw) => 'Z',
        _ => panic!("Invalid shape and outcome {their_shape}"),
    }
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

#[test]
fn test_determine_outcome() {
    assert_eq!(determine_outcome('A', 'Y'), Outcome::Win);
}

#[test]
fn test_score_strategies() {
    let test_input = include_str!("../test.txt");
    let score = score_strategies(test_input);
    assert_eq!(score, 15);
}

#[test]
fn test_score_strategies_from_outcome() {
    let test_input = include_str!("../test.txt");
    let score = score_strategies_from_outcome(test_input);
    assert_eq!(score, 12);
}
