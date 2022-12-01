use std::{fs::File, io::Read};

#[derive(Debug)]
struct SnackBag {
    snacks: Vec<usize>,
    total: usize,
}

impl From<&str> for SnackBag {
    fn from(snacks: &str) -> Self {
        let snacks: Vec<usize> = snacks
            .trim()
            .split("\n")
            .map(|e| e.to_owned().parse::<usize>().unwrap())
            .collect();
        let total = snacks.iter().sum();
        Self { snacks, total }
    }
}

fn main() {
    println!("Hello from day01!");

    let mut file = File::open("./src/bin/day01/input1.txt").unwrap();

    let mut text = String::new();

    file.read_to_string(&mut text).unwrap();

    let mut snack_bags = text
        .split("\n\n")
        .map(|e| SnackBag::from(e))
        .collect::<Vec<SnackBag>>();

    snack_bags.sort_by_key(|e| e.total);
    let total = snack_bags.iter().map(|e| e.total).sum::<usize>();
    let max = snack_bags.iter().max_by_key(|e| e.total);
    let top3 = &snack_bags[snack_bags.len()-3..];
    let top3_max = top3.iter().map(|e| e.total).sum::<usize>();

    println!(
        "Total: {:?}\n\nMax: {:?}\n\nTop 3 max: {:?}",
        total, max, top3_max
    );
}
