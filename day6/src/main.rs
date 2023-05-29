use std::fs::{self};
const INPUT_FILE: &str = "input.txt";
const  SOP_LEN: usize = 4;
const  SOM_LEN: usize = 14;

fn uniq(cs: &[char]) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for c in cs {
        if !result.contains(c) {
            result.push(*c);
        }
    }
    result
}

fn contains_dupes( cs: &[char]) -> bool {
    !(uniq(cs) == cs)
}

fn detect_sop(cs: Vec<char>) -> usize {
    for i in 0..cs.len()-SOP_LEN {
        if !contains_dupes(&cs[i..i+SOP_LEN]) {
            return i + SOP_LEN
        }
    }
    0
}

fn detect_som(cs: Vec<char>) -> usize {
    for i in 0..cs.len()-SOM_LEN {
        if !contains_dupes(&cs[i..i+SOM_LEN]) {
            return i + SOM_LEN
        }
    }
    0
}



fn main() {
    let signal: String = fs::read_to_string(INPUT_FILE).expect("Doh.");

    let part1: usize = detect_sop(signal.chars().collect());
    let part2: usize = detect_som(signal.chars().collect());

    print!("Part 1: ");
    println!("{:?}", part1);

    print!("Part 2: ");
    println!("{:?}", part2);

}

