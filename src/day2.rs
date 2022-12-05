use std::collections::HashMap;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use phf::{phf_map};
mod utils;


static SHAPE_SCORES: phf::Map<&'static str, u8> = phf_map! {
    "rock" => 1, 
    "paper" => 2,
    "scissors" => 3,
};

static OPP_MOVES: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "rock",
    "B" => "paper",
    "C" => "scissors",
};

const WINS: [(&'static str, &'static str); 3] = [("rock", "paper"), ("paper", "scissors"), ("scissors", "rock")];
const LOSSES: [(&'static str, &'static str); 3] = [("rock", "scissors"), ("paper", "rock"), ("scissors", "paper")];

fn score_round_part_1(opponent: &str, me: &str) -> u8 {
    let mut outcome_score: u8 = 0;
    static MY_MOVES: phf::Map<&'static str, &'static str> = phf_map! {
        "X" => "rock",
        "Y" => "paper",
        "Z" => "scissors",
    };
    
    let opp_move = OPP_MOVES.get(opponent).unwrap();
    let my_move = MY_MOVES.get(me).unwrap();

    if WINS.contains(&(opp_move, my_move)) {
        outcome_score = 6;
    } else if LOSSES.contains(&(opp_move, my_move)) {
        // outcome score stays 0
    } else {
        outcome_score = 3;
    }

    let shape_score = SHAPE_SCORES.get(my_move).unwrap();
    return outcome_score + shape_score;
}

fn score_round_part_2(opponent: &str, me: &str) -> u8 {
    let win_map = HashMap::from(WINS);
    let loss_map = HashMap::from(LOSSES);

    let mut score: u8 = 0;
    if let Some(opp_move) = OPP_MOVES.get(opponent) {
        match me {
            "X" => {
                score = *SHAPE_SCORES.get(loss_map.get(opp_move).unwrap()).unwrap();
            },
            "Y" => {
                score = SHAPE_SCORES.get(opp_move).unwrap() + 3;
            },
            "Z" => {
                score = SHAPE_SCORES.get(win_map.get(opp_move).unwrap()).unwrap() + 6;
        
            },
            &_ => {},
        }
    };
    return score;
}

pub fn day2() {

    if let Ok(lines) = utils::read_lines("./input_files/day2.txt") {
        let parsed_lines = lines
            .filter(|l| {l.is_ok()})
            .map(|l| {l.expect("aaah").split(" ").map(str::to_owned).collect::<Vec<_>>()})
            .collect::<Vec<_>>();

        let part_1_score = parsed_lines.iter().map(|moves: &Vec<_>| {score_round_part_1(&moves[0], &moves[1])})
            .fold(0, |acc, el| { acc as u32 + el as u32 });

        println!("Part 1 score is {part_1_score}");

        let part_2_score = parsed_lines.iter().map(|moves: &Vec<_>| {score_round_part_2(&moves[0], &moves[1])}).fold(0, |acc, el| { acc as u32 + el as u32 });
        println!("Part 2 score is {part_2_score}");
    }

}
