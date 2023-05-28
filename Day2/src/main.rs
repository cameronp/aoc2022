use std::io::{self, BufRead};
use std::fs::File;
use std::str::Split;

fn readlines(filename: &str) -> Result<Vec<String>, io::Error> {
    let file: File = File::open(filename)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_,_>>()?;
    Ok(lines)
}

fn atomify(mut split: Split<&str>) -> Option<(&'static str, &'static str)>
{
    let first = match split.next() {
        Some("A") => "A",
        Some("B") => "B",
        Some("C") => "C",
        _ => panic!()
    };
    let second  = match split.next() {
        Some("X") => "X",
        Some("Y") => "Y",
        Some("Z") => "Z",
        _ => panic!()
    };
    Some((first, second))
}

fn line_to_tuple(line: String) -> (&'static str, &'static str) {
    let sides = line.split(" ");
    let atomified = atomify(sides);
    atomified.unwrap()
}

fn lines_to_tuples(lines: Vec<String>) -> Vec<(&'static str, &'static str)> {
    let newlines = lines.clone();


    newlines.iter().map(|l| line_to_tuple(l.to_string())).collect()
}

#[derive(PartialEq, Debug)]
enum GameResult {
    Win,
    Lose,
    Draw
}

fn game_result(round: (&str, &str)) -> GameResult {
    match round {
        ("A", "Y") => GameResult::Win,
        ("A", "Z") => GameResult::Lose,
        ("B", "X") => GameResult::Lose,
        ("B", "Z") => GameResult::Win,
        ("C", "X") => GameResult::Win,
        ("C", "Y") => GameResult::Lose,
        _ => GameResult::Draw
    }
}

fn base_score(round: (&str, &str)) -> i32 {
    match round {
        (_x, "X") => 1,
        (_x, "Y") => 2,
        (_x, "Z") => 3,
        _ => panic!()
    }
}

fn win_score(round: (&str, &str)) -> i32 {
    match game_result(round) {
        GameResult::Win => 6,
        GameResult::Draw => 3,
        GameResult::Lose => 0
    }
}

fn total_score(round: (&str, &str)) -> i32 {
    base_score(round) + win_score(round)
}


fn main() {
    let lines: Vec<String> = readlines("input.txt").unwrap();
    
    let pairs: Vec<(&str, &str)> = lines_to_tuples(lines);

    let scores: Vec<i32> = pairs
        .iter()
        .map(|round| total_score(*round))
        .collect();

    print!("Part 1: ");
    println!("{:?}",scores.iter().sum::<i32>());


}

// -----------------------------------------------------------------------------------------------
// TESTS
// -----------------------------------------------------------------------------------------------

#[test]
fn test_win_logic() {
    // paper beats rock
    assert_eq!(game_result(("A", "Y")), GameResult::Win);
    
    // rock beats scissors
    assert_eq!(game_result(("A", "Z")), GameResult::Lose);

    // scissors beat paper
    assert_eq!(game_result(("B", "Z")), GameResult::Win);

    // paper beats rock
    assert_eq!(game_result(("B", "X")), GameResult::Lose);

    // scissors beat paper
    assert_eq!(game_result(("C", "Y")), GameResult::Lose);

    // rock beats scissors
    assert_eq!(game_result(("C", "X")), GameResult::Win);

    // same is a draw
    assert_eq!(game_result(("A", "X")), GameResult::Draw);
    assert_eq!(game_result(("B", "Y")), GameResult::Draw);
    assert_eq!(game_result(("C", "Z")), GameResult::Draw);


}
