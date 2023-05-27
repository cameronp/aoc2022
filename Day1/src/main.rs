use std::io::{self, BufRead};
use std::fs::File;

fn readlines(filename: &str) -> Result<Vec<String>, io::Error> {
    let file: File = File::open(filename)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_,_>>()?;
    Ok(lines)
}

fn strings_to_ints(v: Vec<String>) -> Vec<i32> {
    v.iter().map(|s| s.parse().expect("not a valid integer")).collect()
}

fn main() {
    let lines: Vec<String> = readlines("input.txt").unwrap();

    let elf_inventories: Vec<Vec<String>> = lines.split(|s| s.is_empty())
        .map(|group| group.to_vec())
        .collect();

    let elf_inventories_parsed: Vec<Vec<i32>> = elf_inventories
        .iter()
        .map(|e| strings_to_ints(e.to_vec()))
        .collect();

    let sums: Vec<i32> = elf_inventories_parsed
        .iter()
        .map(|elf| elf.iter().sum())
        .collect();

    let mut sorted: Vec<i32> = sums.clone();
    sorted.sort();

    let top_three: Vec<i32> = sorted[sorted.len() - 3..].to_vec();
   
    print!("Part 1: ");
    println!("{:?}", sums.iter().max().unwrap());

    print!("Part 2: ");
    println!("{:?}", top_three.iter().sum::<i32>());

}
