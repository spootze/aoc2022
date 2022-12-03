fn main() {
    let input = include_str!("../input.txt");
    let max_calories = compute_calories_of_elves(input, 1);
    println!("Part 1: {max_calories} calories");
    let max_3_calories = compute_calories_of_elves(input, 3);
    println!("Part 2: {max_3_calories} calories");
}

fn compute_calories_of_elves(input: &str, n: usize) -> u32 {
    let mut calories: Vec<u32> = input.split("\n\n").map(compute_calories_of_elf).collect();
    calories.sort_by(|a, b| b.cmp(a));
    return calories[..n].iter().sum();
}

fn compute_calories_of_elf(input: &str) -> u32 {
    return input
        .split_terminator("\n")
        .map(|l| l.parse::<u32>().unwrap())
        .sum();
}

#[test]
fn test_compute_calories_of_elves() {
    let test_input = include_str!("../test.txt");
    let max_calories = compute_calories_of_elves(test_input, 1);
    assert_eq!(max_calories, 24000)
}

#[test]
fn test_compute_calories_of_top_3_elves() {
    let test_input = include_str!("../test.txt");
    let max_calories = compute_calories_of_elves(test_input, 3);
    assert_eq!(max_calories, 45000)
}
