
use std::io::{self, BufRead, Lines};
use std::fs::File;

fn readlines(filename: &str) -> Result<Vec<String>, io::Error> {
    let file: File = File::open(filename)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_,_>>()?;
    Ok(lines)
}

fn parse_range(s: &str) -> (u32, u32) {
    let str_nums: Vec<&str> = s.split("-").collect();
    let nums: Vec<u32> = str_nums.iter().map(|sn| sn.parse().unwrap()).collect();
    (nums[0], nums[1])
}

fn contains(outer: (u32, u32), inner: (u32, u32)) -> bool {
    let (l1,r1) = outer;
    let (l2, r2) = inner;
    l1 <= l2 && r1 >= r2
}

fn pair_includes_complete_containment(r1: (u32,u32), r2: (u32, u32)) -> bool {
    contains(r1, r2) || contains(r2, r1)
}

fn pair_overlaps_to_the_right(left: (u32,u32), right: (u32,u32)) -> bool {
    let (l1,r1) = left;
    let (l2, _r2) = right;
    r1 >= l2 && l1 <= l2
}

fn pair_overlaps_at_all(r1: (u32,u32), r2: (u32, u32)) -> bool {
    pair_overlaps_to_the_right(r1, r2) || pair_overlaps_to_the_right(r2, r1)
}

// ------------------------------------------------------------------------------
// TESTS
// ------------------------------------------------------------------------------
#[test]
fn test_containment() {
    assert!(!pair_includes_complete_containment((2,4), (6,8)));
    assert!(!pair_includes_complete_containment((2,3), (4,5)));
    assert!(!pair_includes_complete_containment((5,7), (7,9)));
    assert!(pair_includes_complete_containment((2,8), (3,7)));
    assert!(pair_includes_complete_containment((6,6), (4,6)));
    assert!(!pair_includes_complete_containment((2,6), (4,8)));
}
// --------------------------------------------------------------------------------

fn main() {
    let lines = readlines("input.txt").unwrap();
    let pairs: Vec<Vec<&str>> = lines
        .iter()
        .map(|l| l.split(",").collect())
        .collect();
    let parsed_pairs: Vec<Vec<(u32,u32)>> = pairs
        .iter()
        .map(
            |p|
            p.iter().map(|range| parse_range(range)).collect()
        )
        .collect();
    let fully_contained_pairs: Vec<&Vec<(u32,u32)>> = parsed_pairs
        .iter()
        .filter(|v| pair_includes_complete_containment(v[0], v[1]))
        .collect();

    print!("Part 1: ");
    println!("{}", fully_contained_pairs.len());

    let overlapping_pairs: Vec<&Vec<(u32,u32)>> = parsed_pairs
        .iter()
        .filter(|v| pair_overlaps_at_all(v[0], v[1]))
        .collect();

    print!("Part 2: ");
    println!("{}", overlapping_pairs.len());
    
    
}
