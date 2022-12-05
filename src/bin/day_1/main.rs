use std::fs;

fn main() {
    let input_file_path = "src/bin/day_1/full.input";
    let contents = fs::read_to_string(input_file_path).unwrap();
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(list: &str) -> i64 {
    list.split("\n\n").map(|elf_list| {
        elf_list.split("\n")
        .map(|x| {
            x.parse::<i64>().unwrap()
        })
        .sum::<i64>()
    })
    .max()
    .unwrap()
}

fn part_2(list: &str) -> i64 {
    let mut x: Vec<i64> = list.split("\n\n").map(|elf_list| {
        elf_list.split("\n")
        .map(|x| {
            x.parse::<i64>().unwrap()
        })
        .sum::<i64>()
    })
    .collect::<Vec<i64>>();
    x.sort_by(|a, b| b.cmp(a));
    let top_three: &[i64] = &x[..3];
    top_three.iter().sum::<i64>()
}