use crate::utils;
use std::collections::HashSet;

fn get_rucksack_char(rucksack: &str) -> char {
    let split = rucksack.len() / 2;
    let comp1: HashSet<char> = rucksack[0..split].chars().collect();
    let comp2: HashSet<char> = rucksack[split..rucksack.len()].chars().collect();

    return comp1.intersection(&comp2).nth(0).unwrap().to_owned();
}

fn get_char_priority(c: &char) -> u32 {
    if c.is_uppercase() {
        return c.to_owned() as u32 - 38;
    } else if c.is_lowercase() {
        return c.to_owned() as u32 - 96;
    } else {
        panic!("aaaaaa");
    }
}

fn part1() {
    let rucksacks = utils::read_lines("./input_files/day3.txt").unwrap();
    let total_priorities: u32 = rucksacks
        .map(|r| { r.unwrap() })
        .map(|r| {get_rucksack_char(&r)})
        .map(|c| {get_char_priority(&c)})
        .sum();

    println!("Part 1 total priorities: {total_priorities}");
}

fn part2() {
    // TODO there's gotta be a better way
    let rucksacks: Vec<String> = utils::read_lines("./input_files/day3.txt").unwrap().map(|l| {l.ok().unwrap()}).collect();
    let mut i = 0;
    let mut priorities: u32 = 0;
    while i + 2 < rucksacks.len() {
        let bag1: HashSet<char> = rucksacks[i].chars().collect();
        let bag2: HashSet<char> = rucksacks[i+1].chars().collect();
        let bag3: HashSet<char> = rucksacks[i+2].chars().collect();

        let common1: HashSet<char> = bag1.intersection(&bag2).map(|c| {c.to_owned()}).collect();
        let common = common1.intersection(&bag3).nth(0).unwrap();
        priorities = priorities + get_char_priority(common);
        i = i + 3;
    }
    println!("Part 2 total priorities: {priorities}");

}

pub fn day3() {
    part1();
    part2();
}