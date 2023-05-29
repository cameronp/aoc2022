use std::io::{self, BufRead, Lines};
use std::fs::File;
use regex::Regex;


fn readlines(filename: &str) -> Result<Vec<String>, io::Error> {
    let file: File = File::open(filename)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_,_>>()?;
    Ok(lines)
}

#[derive(Clone, Copy, Debug)]
struct Move {
    from: usize,
    to: usize,
    count: usize
}

impl Move {
    pub fn parse(m: &str) -> Option<Move> {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        if let Some(captures) = re.captures(m) {
            Some(
                Move {
                count: captures[1].parse().unwrap(),
                from: captures[2].parse().unwrap(),
                to: captures[3].parse().unwrap()
                })
        } else {
            None
        }
    }

}


#[derive(Clone, Debug)]
struct Shipyard<T> {
    bays: Vec<Vec<T>>,
    count: usize
}

impl<T> Shipyard<T> {
    pub fn new(count: usize) -> Shipyard<T> {
        let mut bays = Vec::new();

        // We allocate an extra one, so that we can 1-index the bays.
        for _ in 0..count+1 {
            bays.push(Vec::new())
        }
        Shipyard {
            bays: bays,
            count: count
        }
    }

    pub fn init_bay(&mut self, bay: usize, start: Vec<T>) -> () {
        for c in start {
            self.bays[bay].push(c);
        }
    }

    pub fn peek_bay(&self, bay: usize) -> Option<&T> {
        self.bays[bay].last()
    }

    pub fn move_crate(&mut self, from: usize, to: usize) -> Option<()> {
        let c = self.bays[from].pop()?;
        self.bays[to].push(c);
        Some(())
    }

    pub fn execute_move(&mut self, cmd: Move) -> () {
        for _ in (0..cmd.count) {
            self.move_crate(cmd.from, cmd.to);
        }
    }

}

//-----------------------------------------------------
// TESTS
//-----------------------------------------------------

#[test]
fn test_move() {
    let m = Move::parse("move 2 from 24 to 23").unwrap();
    assert_eq!(m.count, 2);
    assert_eq!(m.from, 24);
    assert_eq!(m.to, 23);
}

#[test]
fn test_setup() {
    let mut sy: Shipyard<char> = Shipyard::new(3);
    sy.init_bay(1, "ZN".chars().collect());
    sy.init_bay(2, "MCD".chars().collect());
    sy.init_bay(3, "P".chars().collect());
    let m1 = Move::parse("move 1 from 2 to 1").unwrap();
    let m2 = Move::parse("move 3 from 1 to 3").unwrap();
    let m3 = Move::parse("move 2 from 2 to 1").unwrap();
    let m4 = Move::parse("move 1 from 1 to 2").unwrap();

    sy.execute_move(m1);
    sy.execute_move(m2);
    sy.execute_move(m3);
    sy.execute_move(m4);
    

    let mut res: Vec<char> = Vec::new();
    res.push(*sy.peek_bay(1).unwrap());
    res.push(*sy.peek_bay(2).unwrap());
    res.push(*sy.peek_bay(3).unwrap());

    let key: String = res.into_iter().collect();
    assert_eq!(key, "CMZ");
}
    
    
fn load_shipyard() -> Shipyard<char> {
    let lines: Vec<String> = readlines("start.txt").unwrap();
    let mut sy:Shipyard<char> = Shipyard::new(9);
    
    for i in (0..lines.len()) {
        sy.init_bay(i+1, lines[i].chars().collect());
    }
    println!("#{:?}", sy);
    sy
}    

fn generate_key(sy: &Shipyard<char>) -> String {
    let mut res: Vec<char> = Vec::new();

    for i in (1..sy.count+1) {
        res.push(*sy.peek_bay(i).unwrap());
    }
    res.into_iter().collect()
}


fn main() {
    let mut sy: Shipyard<char> = load_shipyard();
    let lines = readlines("input.txt").unwrap();
    let commands: Vec<Move> = lines
        .iter()
        .map(|l| Move::parse(l).unwrap())
        .collect();
    for cmd in commands {
        sy.execute_move(cmd);
    }

    let key: String = generate_key(&sy);

    print!("Part 1: ");
    println!("{:?}", key);


        

}
