use day06::find_start;

fn main() {
    let buffer = include_str!("../input.txt");
    let start_packet = find_start(buffer, 4);
    println!("Part 1: {start_packet}");
    let start_msg = find_start(buffer, 14);
    println!("Part 2: {start_msg}");
}
