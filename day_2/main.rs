use std::fs;
use std::collections::HashMap;

fn get_own_move(opponent_hand: char, outcome: char) -> char {
    match opponent_hand {
        'A' => {
            match outcome {
                'X' => 'Z',
                'Y' => 'X',
                _ => 'Y'
            }
        },
        'B' => outcome,
        _ => {
            match outcome {
                'X' => 'Y',
                'Y' => 'Z',
                _ => 'X'
            }
        }
    }
}

fn get_round_score(opponent_hand: char, own_hand: char) -> i64 {
    let hand_scores = HashMap::from([
        ('X', 1),
        ('Y', 2),
        ('Z', 3)
    ]);

    match opponent_hand {
        'A' => {
            match own_hand {
                'Y' => hand_scores[&own_hand] + 6,
                'Z' => hand_scores[&own_hand],
                _ => hand_scores[&own_hand] + 3
            }
        },
        'B' => {
            match own_hand {
                'Z' => hand_scores[&own_hand] + 6,
                'X' => hand_scores[&own_hand],
                _ => hand_scores[&own_hand] + 3
            }
        },
        'C' => {
            match own_hand {
                'X' => hand_scores[&own_hand] + 6,
                'Y' => hand_scores[&own_hand],
                _ => hand_scores[&own_hand] + 3
            }
        },
        _ => -1
    }
}

fn main() {
    let input_file_path = "full.input";
    let contents = fs::read_to_string(input_file_path).unwrap();
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}

fn part_1(list: &str) -> i64 {
    list.split('\n').map(|round| {
        let round_moves: Vec::<char> = round.split(' ').map(|s| {
            s.chars().nth(0).unwrap()
        }).collect();
        get_round_score(round_moves[0], round_moves[1])
    }).sum::<i64>()
}

fn part_2(list: &str) -> i64 {
    list.split('\n').map(|round| {
        let round_moves: Vec::<char> = round.split(' ').map(|s| {
            s.chars().nth(0).unwrap()
        }).collect();
        let own_move = get_own_move(round_moves[0], round_moves[1]);
        get_round_score(round_moves[0], own_move)
    }).sum::<i64>()
}