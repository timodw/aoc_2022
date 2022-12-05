use std::fs;
use std::collections::HashSet;

fn get_priority(c: u8) -> i64 {
    (if c <= b'Z' { c - b'A' + 27 } else { c - b'a' + 1}) as i64
}

fn get_priority_for_contents(char_contents: Vec::<u8>) -> i64 {
    let backpack_size = char_contents.len();
    let first_half: HashSet<u8> = char_contents[..backpack_size / 2].iter().copied().collect();
    let second_half: HashSet<u8> = char_contents[backpack_size / 2..].iter().copied().collect();
    let item = first_half.intersection(&second_half).copied().nth(0).unwrap();
    get_priority(item)
}

fn get_priority_of_badge(group_contents: Vec::<&str>) -> i64 {
    let first_set: HashSet<u8> = group_contents[0].as_bytes().iter().copied().collect();
    let second_set: HashSet<u8> = group_contents[1].as_bytes().iter().copied().collect();
    let third_set: HashSet<u8> = group_contents[2].as_bytes().iter().copied().collect();
    let group_char = first_set.intersection(&second_set.intersection(&third_set).copied().collect()).copied().nth(0).unwrap();
    get_priority(group_char)
}

fn main() {
    let input_file_path = "src/bin/day_3/full.input";
    let contents = fs::read_to_string(input_file_path).unwrap();
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(list: &str) -> i64 {
    list.split('\n').map(|contents| {
        let char_contents: Vec::<u8> = contents.as_bytes().to_vec();
        get_priority_for_contents(char_contents)
    }).sum()
}

fn part_2(list: &str) -> i64 {
    list.split('\n').collect::<Vec<&str>>().chunks(3).map(|chunk| {
        get_priority_of_badge(chunk.to_vec())
    }).sum()
}