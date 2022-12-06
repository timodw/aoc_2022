use std::fs;
use std::collections::HashSet;

fn get_marker_index(chars: Vec<char>, marker_size: usize) -> i64 {
    let (i, _) = chars.as_slice().windows(marker_size).enumerate().skip_while(|(i, w)| {
        let mut set = HashSet::new();
        w.iter().for_each(|c| { set.insert(c); });
        w.len() != set.len()
    }).next().unwrap();
    (i + marker_size) as i64
}

fn main() {
    let input_file_path = "src/bin/day_6/full.input";
    let contents = fs::read_to_string(input_file_path).unwrap();
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(letters: &str) -> i64 {
    let chars: Vec<char> = letters.chars().collect();
    get_marker_index(chars, 4)
}

fn part_2(letters: &str) -> i64 {
    let chars: Vec<char> = letters.chars().collect();
    get_marker_index(chars, 14)
}