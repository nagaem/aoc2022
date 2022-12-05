use crate::utils;
use std::collections::HashSet;

// TODO so defining fns that return iterators in rust is a pain in the ass...
// fn open_rucksacks() -> impl Iterator<Item = str> + '_ {
//     let rucksacks = utils::read_lines("./input_files/day3_test.txt").unwrap();
//     return rucksacks.map(|r| { r.unwrap() });
// }

fn part1() {
    let rucksacks = utils::read_lines("./input_files/day3_test.txt").unwrap();
    for rucksack in rucksacks.map(|r| { r.unwrap() }) {
        let split = rucksack.len() / 2;
        let comp1: HashSet<char> = rucksack[0..split].chars().collect();
        let comp2: HashSet<char> = rucksack[split..rucksack.len()].chars().collect();
        println!("{:?}, {:?}", comp1, comp2);

        println!("{:?}", comp1.intersection(&comp2));
    }
}

pub fn day3() {
    part1();
}