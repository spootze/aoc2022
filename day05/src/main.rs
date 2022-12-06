type Instruction = (usize, usize, usize);
type Instructions = Vec<Instruction>;
type Stack = Vec<char>;
type Stacks = Vec<Stack>;

fn main() {
    let input = include_str!("../input.txt");
    let (stacks, instructions) = parse_inputs(input);
    let end_state_part1 = apply_instructions(&stacks, &instructions, true);
    let top_crates_part1 = render_stacks(end_state_part1);
    println!("Part 1: {top_crates_part1}");

    let end_state_part2 = apply_instructions(&stacks, &instructions, false);
    let top_crates_part2 = render_stacks(end_state_part2);
    println!("Part 2: {top_crates_part2}");
}

fn parse_inputs(input: &str) -> (Stacks, Instructions) {
    let (start_state, instructions_raw) = input.split_once("\n\n").unwrap();
    let stacks = parse_stacks(start_state);
    let instructions = parse_instructions(instructions_raw);
    (stacks, instructions)
}

fn parse_stacks(start_state: &str) -> Stacks {
    let n = get_number_of_stacks(start_state);
    (0..n).map(|i| parse_stack(start_state, i)).collect()
}

fn get_number_of_stacks(start_state: &str) -> usize {
    start_state
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

fn parse_stack(start_state: &str, i: usize) -> Stack {
    let offset = get_stack_offset(i);
    start_state
        .lines()
        .map(|l| l.chars().nth(offset).unwrap())
        .filter(|c| c.is_alphabetic())
        .rev()
        .collect()
}

fn parse_instructions(instructions: &str) -> Instructions {
    instructions.lines().map(parse_instruction).collect()
}

fn parse_instruction(instruction: &str) -> Instruction {
    (
        parse_number(instruction, 1),
        parse_number(instruction, 3) - 1,
        parse_number(instruction, 5) - 1,
    )
}

fn parse_number(instruction: &str, i: usize) -> usize {
    instruction
        .split_whitespace()
        .nth(i)
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

fn get_stack_offset(i: usize) -> usize {
    i * 4 + 1
}

fn apply_instructions(stacks: &Stacks, instructions: &Instructions, reverse: bool) -> Stacks {
    let mut out = stacks.clone();
    for (n, from, to) in instructions.iter() {
        let origin_size = out[*from].len();
        let mut popped = out[*from].split_off(origin_size - *n);
        // simulate repeated pop & push
        if reverse {
            popped.reverse();
        }
        out[*to].append(&mut popped);
    }
    out
}

fn render_stacks(stacks: Stacks) -> String {
    let top_crates = stacks.iter().map(|s| s.last().unwrap_or(&' '));
    String::from_iter(top_crates)
}

#[test]
fn test_compute_end_state_wo_reverse() {
    let input = include_str!("../test.txt");
    let (stacks, instructions) = parse_inputs(input);
    let end_state = apply_instructions(&stacks, &instructions, true);
    let top_crates = render_stacks(end_state);
    assert_eq!(top_crates, "CMZ");
}

#[test]
fn test_compute_end_state_w_reverse() {
    let input = include_str!("../test.txt");
    let (stacks, instructions) = parse_inputs(input);
    let end_state = apply_instructions(&stacks, &instructions, false);
    let top_crates = render_stacks(end_state);
    assert_eq!(top_crates, "MCD");
}
