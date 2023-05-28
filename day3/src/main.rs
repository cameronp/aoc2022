use std::io::{self, BufRead};
use std::fs::File;

fn readlines(filename: &str) -> Result<Vec<String>, io::Error> {
    let file: File = File::open(filename)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_,_>>()?;
    Ok(lines)
}

fn chunk<T: std::clone::Clone>(v: Vec<T>, size: usize) -> Vec<Vec<T>> {
    let mut chunked: Vec<Vec<T>> = Vec::new();
    let len = v.len();

    for i in (0..len).step_by(size) {
        let end = std::cmp::min(i+size, len);
        chunked.push(v[i..end].to_vec());
    }
    chunked
}


fn split_in_half(s: &str) -> (&str, &str) {
    let new_s = s.clone();
    let half = new_s.len() / 2;
    (&new_s[0..half], &new_s[half..new_s.len()])
}

fn priority(c: char) -> u8 {
    if c.is_lowercase() {
        (c as u8) - 96 
    } else {
        (c as u8) - 38
    }
}

fn detect_common_item(comp1: &str, comp2: &str) -> char {
    for c in comp1.chars() {
        if comp2.contains(c) { return c }
    }
    panic!();
}

fn detect_common_item_in_three(comp1: &str, comp2: &str, comp3:&str) -> char {
    for c in comp1.chars() {
        if comp2.contains(c) && comp3.contains(c) { return c }
    }
    panic!();
}

fn detect_id(elf_group: Vec<String>) -> char {
    detect_common_item_in_three(&elf_group[0], &elf_group[1], &elf_group[2])
}

fn detect_dupe_item(backpack: &str) -> char {
    let (a,b) = split_in_half(backpack);
    detect_common_item(a, b)
}

fn backpack_priority(backpack: &str) -> u8 {
    let item = detect_dupe_item(backpack);
    priority(item)
}

// --------------------------------------------------------------------------
// TESTS
// --------------------------------------------------------------------------

#[test]
fn test_split_backpack() {
    assert_eq!(split_in_half("vJrwpWtwJgWrhcsFMMfFFhFp"), ("vJrwpWtwJgWr", "hcsFMMfFFhFp"));
}

#[test]
fn test_priorities() {
    assert_eq!(priority('c'), 3);
    assert_eq!(priority('m'), 13);
    assert_eq!(priority('D'), 30);
    assert_eq!(priority('Z'), 52);
}

#[test]
fn test_detect_common_item() {
    let common = detect_common_item("vJrwpWtwJgWr", "hcsFMMfFFhFp");
    assert_eq!(common, 'p');

    let common2 = detect_common_item("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL");
    assert_eq!(common2, 'L');

    let common3 = detect_common_item("PmmdzqPrV", "vPwwTWBwg");
    assert_eq!(common3, 'P');
}

#[test]
fn test_split_and_detect() {
    let cs4 = split_in_half("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
    let c4 = detect_common_item(cs4.0, cs4.1);
    assert_eq!(c4, 'v');

    let cs5 = split_in_half("ttgJtRGJQctTZtZT");
    let c5 = detect_common_item(cs5.0, cs5.1);
    assert_eq!(c5, 't');
}

#[test]
fn test_detect_dupe_item() {
    let c = detect_dupe_item("CrZsJsPPZsGzwwsLwLmpwMDw");
    assert_eq!(c, 's');
}

//-----------------------------------------------------------------------------

fn main() {
    let backpacks = readlines("input.txt").unwrap();
    let commons: Vec<char> = backpacks
        .iter()
        .map(|b| detect_dupe_item(b))
        .collect();
    
    let priorities: Vec<u8> = commons
        .iter()
        .map(|c| priority(*c))
        .collect();

    print!("Part 1: ");
    println!("{}", priorities.iter().map(|&n| n as u32).sum::<u32>());

    let elf_groups = chunk(backpacks, 3);
    let ids: Vec<char> = elf_groups
        .iter()
        .map(|g| detect_id(g.to_vec()))
        .collect();

    let part_two_priorities: Vec<u8> = ids
        .iter()
        .map(|c| priority(*c))
        .collect();

    print!("Part 2: ");
    println!("{}", part_two_priorities.iter().map(|&n| n as u32).sum::<u32>());

}
