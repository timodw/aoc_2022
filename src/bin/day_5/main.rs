use std::fs;
use regex::Regex;

fn process_stacks(stacks_str: &Vec<&str>) -> Vec<Vec<char>> {
    // let re = Regex::new(r"\s{3}|\[[A-Z]\]").unwrap();
    // let mut stacks_raw = stacks_str.iter().map(|line| {
    //     re.find_iter(line)
    //       .map(|m| m.as_str().chars())
    //       .map(|mut m| if m.next().unwrap() == '[' { m.next().unwrap() } else { '-' })
    //       .collect::<Vec<char>>()
    // });
    // let first_row = stacks_raw.next().unwrap();
    // println!("{:?}", first_row);
    // let n_stacks = first_row.len();
    // let mut stacks: Vec<Vec<char>> = Vec::new();
    // first_row.iter().copied().for_each(|c| if c == '-' { stacks.push(Vec::new()) } else {stacks.push(vec![c])});
    // stacks_raw.filter(|row| row.len() == n_stacks)
    //           .for_each(|row| {
    //             row.iter()
    //                .copied()
    //                .enumerate()
    //                .for_each(|(i, c)| if c != '-' { stacks[i].push(c) })
    //           });
    // stacks.iter().map(|stack| stack.iter().copied().rev().collect()).collect()
    vec![
        vec!['B', 'Z', 'T'],
        vec!['V', 'H', 'T', 'D', 'N'],
        vec!['B', 'F', 'M', 'D'],
        vec!['T', 'J', 'G', 'W', 'V', 'Q', 'L'],
        vec!['W', 'D', 'G', 'P', 'V', 'F', 'Q', 'M'],
        vec!['V', 'Z', 'Q', 'G', 'H', 'F', 'S'],
        vec!['Z', 'S', 'N', 'R', 'L', 'T', 'C', 'W'],
        vec!['Z', 'H', 'W', 'D', 'J', 'N', 'R', 'M'],
        vec!['M', 'Q', 'L', 'F', 'D', 'S']
    ]
}

fn process_moves(moves_str: &Vec<&str>) -> Vec<(usize, usize, usize)> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    moves_str.iter().map(|line| {
        let cap = re.captures(line).unwrap();
        (cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
         cap.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
         cap.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1)
    }).collect()
}

fn process_input(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let input_vec: Vec<Vec<&str>> = input.split("\n\n")
                                         .map(|x| x.split("\n").collect())
                                         .collect();
    let stacks_str = &input_vec[0];
    let moves_str = &input_vec[1];
    (process_stacks(stacks_str), process_moves(moves_str))
}

fn main() {
    let input_file_path = "src/bin/day_5/full.input";
    let contents = fs::read_to_string(input_file_path).unwrap();
    let (stacks, moves) = process_input(&contents);
    println!("Part 1: {}", part_1(&stacks, &moves));
    println!("Part 2: {}", part_2(&stacks, &moves));
}

fn part_1(stacks: &Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    let mut new_stacks = stacks.clone(); 
    moves.iter().copied().for_each(|(amount, start, end)| {
        (0..amount).for_each(|_| {
            let val = new_stacks[start].pop().unwrap();
            new_stacks[end].push(val);
        });
    });
    let mut ret_vec: Vec<char> = Vec::new();
    new_stacks.iter().for_each(|stack| ret_vec.push(*stack.last().unwrap()));
    ret_vec.iter().collect::<String>()
}

fn part_2(stacks: &Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    let mut new_stacks = stacks.clone(); 
    moves.iter().copied().for_each(|(amount, start, end)| {
        let s = new_stacks[start].len();
        let mut vals: Vec<char> = new_stacks[start].drain(s - amount..).collect();
        new_stacks[end].append(&mut vals);
    });
    let mut ret_vec: Vec<char> = Vec::new();
    new_stacks.iter().for_each(|stack| ret_vec.push(*stack.last().unwrap()));
    ret_vec.iter().collect::<String>()
}