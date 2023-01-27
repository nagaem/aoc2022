use crate::utils;

fn get_ranges(line: &str) -> Vec<[u32; 2]> {
    let mut ranges: Vec<[u32; 2]> = Vec::new();
    for range in line.split(",") {
        let r: Vec<u32> = range.split("-").map(|v| {v.parse::<u32>().unwrap()}).collect();
        ranges.push([r[0], r[1]]);
    }
    return ranges;
}

fn is_contained(ranges: &Vec<[u32; 2]>) -> bool {
    if (ranges[0][0] >= ranges[1][0]) && (ranges[0][1] <= ranges[1][1]) {
        return true;
    }
    
    if (ranges[0][0] <= ranges[1][0]) && (ranges[0][1] >= ranges[1][1]) {
        return true;
    }
    
    return false;
}

fn has_overlap(ranges: &Vec<[u32; 2]>) -> bool {
    // this could probably be done with bitmaps or sets, but... it's late, and I don't wanna
    let range1 = ranges[0][0]..ranges[0][1] + 1;
    let range2 = ranges[1][0]..ranges[1][1] + 1;

    if range1.contains(&ranges[1][0]) || range1.contains(&ranges[1][1]) {
        return true;
    }

    if range2.contains(&ranges[0][0]) || range2.contains(&ranges[0][1]) {
        return true;
    }

    return false;
}

fn part1() {
    let assignment_pairs = utils::read_lines("./input_files/day4.txt").unwrap();
    let containments: Vec<Vec<[u32; 2]>> = assignment_pairs
        .map(|l| {l.unwrap()})
        .map(|line| {get_ranges(&line)})
        .filter(|ranges| {is_contained(ranges)})
        .collect();

    println!("Part 1: There are {} assignment pairs where one range fully contains the other.", containments.len())
}

fn part2() {
    let assignment_pairs = utils::read_lines("./input_files/day4.txt").unwrap();
    let overlaps: Vec<Vec<[u32; 2]>> = assignment_pairs
        .map(|l| {l.unwrap()})
        .map(|line| {get_ranges(&line)})
        .filter(|ranges| {has_overlap(ranges)})
        .collect();

        println!("Part 2: There are {} assignment pairs that overlap.", overlaps.len())

}

pub fn day4() {
    part1();
    part2();
}