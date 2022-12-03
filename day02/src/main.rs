fn main() {
    let input = include_str!("../input.txt");
    let score = score_strategies(input);
    println!("Part 1: {score}");
    let score2 = score_strategies_from_outcome(input);
    println!("Part 2: {score2}")
}

fn score_strategies(strategies: &str) -> i32 {
    strategies
        .split_terminator("\n")
        .map(parse_line)
        .map(|(l, r)| score_strategy(l, r))
        .sum()
}

fn score_strategies_from_outcome(strategies: &str) -> i32 {
    strategies
        .split_terminator("\n")
        .map(parse_line)
        .map(|(l, r)| (l, encode_outcome(r)))
        .map(|(l, r)| (l, determine_my_strategy(l, r)))
        .map(|(l, r)| score_strategy(l, r))
        .sum()
}

fn parse_line(line: &str) -> (char, char) {
    (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap())
}

fn score_strategy(their_shape: char, my_shape: char) -> i32 {
    let outcome = determine_outcome(their_shape, my_shape);
    score_outcome(outcome) + score_shape(my_shape)
}

fn score_outcome(outcome: Outcome) -> i32 {
    match outcome {
        Outcome::WIN => 6,
        Outcome::DRAW => 3,
        Outcome::LOSS => 0,
    }
}

fn determine_outcome(their_shape: char, my_shape: char) -> Outcome {
    match (their_shape, my_shape) {
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => Outcome::DRAW,
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => Outcome::WIN,
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => Outcome::LOSS,
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
        'X' => Outcome::LOSS,
        'Y' => Outcome::DRAW,
        'Z' => Outcome::WIN,
        _ => panic!("Invalid desired outcome {outcome_str}"),
    }
}

fn determine_my_strategy(their_shape: char, desired_outcome: Outcome) -> char {
    match (their_shape, desired_outcome) {
        ('A', Outcome::DRAW) | ('B', Outcome::LOSS) | ('C', Outcome::WIN) => 'X',
        ('A', Outcome::WIN) | ('B', Outcome::DRAW) | ('C', Outcome::LOSS) => 'Y',
        ('A', Outcome::LOSS) | ('B', Outcome::WIN) | ('C', Outcome::DRAW) => 'Z',
        _ => panic!("Invalid shape and outcome {their_shape}"),
    }
}

#[derive(PartialEq, Debug)]
enum Outcome {
    WIN,
    LOSS,
    DRAW,
}

#[test]
fn test_determine_outcome() {
    assert_eq!(determine_outcome('A', 'Y'), Outcome::WIN);
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
