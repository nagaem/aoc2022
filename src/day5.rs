use crate::utils;
use regex::Regex;

fn make_crate_stacks(crate_lines: Vec<String>, num_crates: u32) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    // TODO initialize num_crates empty stacks
    for _ in 0..num_crates {
        stacks.push(Vec::new());
    }

    let re = Regex::new(r"\[").unwrap();
    for line in crate_lines.iter().rev() {
        let line_chars: Vec<char> = line.chars().collect();
        let caps_iter = re.captures_iter(&line);
        for caps in caps_iter {
            for cap in caps.iter() {
                let crate_start = cap.unwrap().start();
                stacks[crate_start / 4].push(line_chars[crate_start + 1]);
            }
        }
        
    }
    
    return stacks;
}

fn crane() {
    let mut crate_lines: Vec<String> = Vec::new();
    if let Ok(mut lines) = utils::read_lines("./input_files/day5.txt") {
        let mut line = lines.next().unwrap();
        // crate lines
        while line.as_ref().unwrap().contains("[") {
            let crate_line = line.unwrap();
            crate_lines.push(String::from(crate_line));
            line = lines.next().unwrap();
        }
        println!("Crates: {:?}", crate_lines);
        let mut num_crates = String::from(line.unwrap());
        num_crates.retain(|c| {c != ' '});
        let total_crates: u32 = num_crates.pop().unwrap().to_digit(10).unwrap();

        let mut crate_stacks = make_crate_stacks(crate_lines, total_crates);
        let mut part_2_crate_stacks = crate_stacks.clone();
        println!("Crate stacks: {:?}", crate_stacks);

        // skip blank line
        _ = lines.next().unwrap();
        line = lines.next().unwrap();

        let mut move_crates_3000 = |num_crates: usize, from_stack: usize, to_stack: usize| {
            for _ in 0..num_crates {
                let fs = &mut crate_stacks[from_stack - 1];
                let crate_contents = fs.pop().unwrap();
                crate_stacks[to_stack - 1].push(crate_contents);
            }
        };

        let mut move_crates_3001 = |num_crates: usize, from_stack: usize, to_stack: usize| {
            let fs = &mut part_2_crate_stacks[from_stack - 1];
            let mut moved_crates: Vec<char> = Vec::new();
            for _ in 0..num_crates {
                moved_crates.push(fs.pop().unwrap());
            }
            moved_crates.reverse();
            part_2_crate_stacks[to_stack - 1].append(&mut moved_crates);
        };

        while line.as_ref().unwrap().contains("move") {
            let instructions: Vec<&str> = line.as_ref().unwrap().splitn(6, " ").collect();
            move_crates_3000(instructions[1].parse().unwrap(), instructions[3].parse().unwrap(), instructions[5].parse().unwrap());
            move_crates_3001(instructions[1].parse().unwrap(), instructions[3].parse().unwrap(), instructions[5].parse().unwrap());
            if let Some(next_line) = lines.next() {
                line = next_line;
            } else {
                break;
            }
        }

        println!("CrateMover 3000 after moving: {:?}", crate_stacks);
        let message: Vec<char> = crate_stacks.iter().map(|s| {let mut stack = s.clone(); stack.pop().unwrap()}).collect();
        println!("CrateMover 3000 message: {:?}", message);

        println!("CrateMover 3001 after moving: {:?}", part_2_crate_stacks);
        let message_p2: Vec<char> = part_2_crate_stacks.iter().map(|s| {let mut stack = s.clone(); stack.pop().unwrap()}).collect();
        println!("CrateMover 3001 message: {:?}", message_p2);

    }
}

pub fn day5() {
    crane()
}