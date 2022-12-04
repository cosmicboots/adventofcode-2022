use std::{
    collections::HashSet, error::Error, fmt::Display, fs::File, io::Read, ops::RangeInclusive,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Assignment(RangeInclusive<usize>);

#[derive(Debug, Clone, PartialEq, Eq)]
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
                start..=end
            })
            .collect::<Vec<RangeInclusive<usize>>>();
        Self {
            left: Assignment(ranges[0].clone()),
            right: Assignment(ranges[1].clone()),
        }
    }
}

impl AssignmentPair {
    fn fully_contained(&self) -> bool {
        let left = self.left.0.clone().into_iter().collect::<HashSet<usize>>();
        let right = self.right.0.clone().into_iter().collect::<HashSet<usize>>();
        left.is_subset(&right) || right.is_subset(&left)
    }

    fn overlap(&self) -> bool {
        let left = self.left.0.clone().into_iter().collect::<HashSet<usize>>();
        let right = self.right.0.clone().into_iter().collect::<HashSet<usize>>();
        !left.is_disjoint(&right)
    }
}

impl Display for AssignmentPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{},{}-{}",
            self.left.0.start(),
            self.left.0.end(),
            self.right.0.start(),
            self.right.0.end()
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

    let mut matched_part1: Vec<AssignmentPair> = vec![];

    let total1: usize = pairs
        .clone()
        .into_iter()
        .map(|e| {
            if e.left.0.start() <= e.right.0.start() && e.left.0.end() >= e.right.0.end() {
                matched_part1.push(e);
                return 1;
            }
            if e.left.0.start() >= e.right.0.start() && e.left.0.end() <= e.right.0.end() {
                matched_part1.push(e);
                return 1;
            }
            return 0;
        })
        .sum();

    println!("Total part 1: {}", total1);

    let total1b: usize = pairs
        .clone()
        .into_iter()
        .map(|e| if e.fully_contained() { 1 } else { 0 })
        .sum();

    println!("Total part 1b: {}", total1b);

    let total2: usize = pairs
        .clone()
        .into_iter()
        .map(|e| {
            if e.left.0.start() > e.right.0.end() || e.left.0.end() < e.right.0.start() {
                0
            } else {
                1
            }
        })
        .sum();

    println!("Total part 2: {}", total2);

    let total2b: usize = pairs
        .clone()
        .into_iter()
        .map(|e| if e.overlap() { 1 } else { 0 })
        .sum();

    println!("Total part 2b: {}", total2b);

    Ok(())
}
