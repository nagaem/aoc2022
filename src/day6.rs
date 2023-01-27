use std::fs;
use std::collections::HashSet;

fn all_unique(chars: &str, num_unique: usize) -> bool {
    let set: HashSet<char> = chars.chars().collect();
    return set.len() == num_unique;
}

fn get_start_of_marker(signal: &String, marker_size: usize) -> isize {
    // Get num chars processed to get start of packet marker\
    for (i, c) in signal.chars().enumerate() {
        if all_unique(&signal[i..i+marker_size], marker_size) {
            return (i + marker_size).try_into().unwrap();
        }
    }
    return -1;
}

pub fn day6() {
    let signal: String = fs::read_to_string("./input_files/day6.txt").unwrap().parse().unwrap();
    let start_of_packet = get_start_of_marker(&signal, 4);
    println!("{start_of_packet} characters must be processed before the first start-of-packet marker is detected.");
    let start_of_message = get_start_of_marker(&signal, 14);
    println!("{start_of_message} characters must be processed before the first start-of-message marker is detected.");
}