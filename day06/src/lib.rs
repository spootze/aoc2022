use std::collections::HashSet;

pub fn find_start(buffer: &str, window_size: usize) -> usize {
    let (i, _) = buffer
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .find(|(_, cs)| is_unique(cs))
        .unwrap();

    i + window_size
}

// as per https://stackoverflow.com/a/46767732
fn is_unique(vals: &[u8]) -> bool {
    let mut uniq = HashSet::new();
    vals.iter().all(move |x| uniq.insert(x))
}

#[test]
fn test_find_start() {
    assert_eq!(find_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
    assert_eq!(find_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    assert_eq!(find_start("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    assert_eq!(find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    assert_eq!(find_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);

    assert_eq!(find_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    assert_eq!(find_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    assert_eq!(find_start("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    assert_eq!(find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    assert_eq!(find_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
}
