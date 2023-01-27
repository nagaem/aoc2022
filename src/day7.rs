use crate::utils;
use trees::{tr, Tree};

fn build_tree() -> Tree<u32> {
    let mut tree = Tree::new(0);
    if let Ok(mut lines) = utils::read_lines("./input_files/day5.txt") {
        let mut line = lines.next().unwrap();
        if line.unwrap().contains("cd ..") {
            return tree;
        } else if line.unwrap().contains("cd ")  {
            // TODO this won't work b/c it needs to start from this point in the file
            // TODO so I think this'll have to be parsed out first, then we pass pointer to where we are in the file?
            tree.push_back(build_tree())
            // move to next line
        } else if line.unwrap().contains("dir") {
            // ignore for now, go to next line
        } else {
            // parse size, filename from line, append to tree
            let (size, name): (&str, &str) = line.unwrap().splitn(2, " ").collect();
        }
    }
    return tree;
}

pub fn day() {
    let tree = build_tree();
}