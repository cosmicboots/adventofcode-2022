use std::{error::Error, fmt::Display, fs::File, io::Read, ops::Range};

#[derive(Debug, Clone)]
struct Assignment(Range<usize>);

#[derive(Debug, Clone)]
struct AssignmentPair {
    left: Assignment,
    right: Assignment,
}

impl From<&str> for AssignmentPair {
    fn from(pair: &str) -> Self {
        let ranges = pair
            .trim()
            .split(",")
            .map(|e| {
                let raw: Vec<&str> = e.trim().split("-").collect();
                let start = raw[0].parse::<usize>().unwrap();
                let end = raw[1].parse::<usize>().unwrap();
                start..(end + 1)
            })
            .collect::<Vec<Range<usize>>>();
        Self {
            left: Assignment(ranges[0].clone()),
            right: Assignment(ranges[1].clone()),
        }
    }
}

impl ToString for Assignment {
    fn to_string(&self) -> String {
        self.0
            .clone()
            .into_iter()
            .map(|e| e.to_string() + ",")
            .collect::<Vec<String>>()
            .join("")
    }
}

impl AssignmentPair {
    fn fully_contained(&self) -> bool {
        let left = self.left.to_string();
        let right = self.right.to_string();
        if left.contains(&right) {
            let x = left.find(&right).unwrap();
            println!("{}\n{:>w$}", left, right, w = x + right.len());
        } else if right.contains(&left) {
            let x = right.find(&left).unwrap();
            println!("{:>w$}\n{}", left, right, w = x + left.len());
        }
        (left.contains(&right) || right.contains(&left)) // && left != right
    }
}

impl Display for AssignmentPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{},{}-{}",
            self.left.0.start,
            self.left.0.end - 1,
            self.right.0.start,
            self.right.0.end - 1
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from day04!");

    let mut file = File::open("src/bin/day04/input.txt")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let pairs = text
        .trim()
        .split("\n")
        .map(|e| AssignmentPair::from(e))
        .collect::<Vec<AssignmentPair>>();

    let total1: usize = pairs
        .clone()
        .into_iter()
        .map(|e| {
            if e.left.0.start <= e.right.0.start && e.left.0.end >= e.right.0.end {
                return 1;
            }
            if e.left.0.start >= e.right.0.start && e.left.0.end <= e.right.0.end {
                return 1;
            }
            return 0;
        })
        .sum();

    println!("Total part 1: {}", total1);

    let total2: usize = pairs
        .clone()
        .into_iter()
        .map(|e| {
            if e.left.0.start >= e.right.0.end || e.left.0.end <= e.right.0.start {
                0
            } else {
                1
            }
        }).sum();

    println!("Total part 2: {}", total2);

    Ok(())
}
