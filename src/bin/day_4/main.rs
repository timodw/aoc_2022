use std::fs;

fn get_range(range: &str) -> (i64, i64) {
    let mut split = range.split('-');
    let start: i64 = split.next().unwrap().parse().unwrap();
    let end: i64 = split.next().unwrap().parse().unwrap();
    (start, end)
}

fn get_total_overlap(pair: Vec<&str>) -> i64 {
    let mut pair_iter = pair.iter().map(|p| get_range(p));
    let first = pair_iter.next().unwrap();
    let second = pair_iter.next().unwrap();
    if (first.0 >= second.0 && first.1 <= second.1) || (second.0 >= first.0 && second.1 <= first.1) { 1 } else { 0 }
}

fn get_overlap(pair: Vec<&str>) -> i64 {
    let mut pair_iter = pair.iter().map(|p| get_range(p));
    let first = pair_iter.next().unwrap();
    let second = pair_iter.next().unwrap();
    if first.0 > second.1 || first.1 < second.0 { 0 } else { 1 }
}

fn main() {
    let input_file_path = "src/bin/day_4/full.input";
    let contents = fs::read_to_string(input_file_path).unwrap();
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(list: &str) -> i64 {
    list.split('\n').map(|pair| { pair.split(',') })
                    .map(|pair| { get_total_overlap(pair.collect())})
                    .sum()
}

fn part_2(list: &str) -> i64 {
    list.split('\n').map(|pair| { pair.split(',') })
                    .map(|pair| { get_overlap(pair.collect())})
                    .sum()
}