use std::fs;
use std::collections::HashSet;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");

    part1(&contents);
}


fn part1(contents: &String) {
    // let mut iter = contents.iter().peekable()
    // for char in iter {
    // }

    let mut slice: &str;
    let end_idx = contents.len() - 4;
    for start_idx in 0..end_idx {
        slice = &contents[start_idx..(start_idx+4)];
        let set = slice.chars().collect::<HashSet<char>>();
        if set.len() == 4 {
            println!("Part1 - Start-of-packet marker found at offset {}", start_idx);
            return;
        }
    }
}


