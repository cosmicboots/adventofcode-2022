use std::fs::read_to_string;

struct Stacks(Vec<Vec<char>>);

impl From<&str> for Stacks {
    fn from(raw_stacks: &str) -> Self {
        // Example input:
        //     [D]
        // [N] [C]
        // [Z] [M] [P]
        //  1   2   3
        let lines: Vec<&str> = raw_stacks.split("\n").collect();

        let cols: usize = lines[lines.len() - 1]
            .split_ascii_whitespace()
            .map(|e| e.trim().parse::<usize>().unwrap())
            .last()
            .unwrap();

        let mut stacks: Vec<Vec<char>> = vec![vec![]; cols];

        for line in lines[..lines.len() - 1].iter() {
            for col in 0..cols {
                let i: usize = col * 4 + 1;
                if i < line.len() {
                    let c = line.as_bytes()[col * 4 + 1] as char;
                    if c != ' ' {
                        stacks[col].push(c);
                    }
                }
            }
        }

        Self(stacks)
    }
}

fn main() {
    println!("Hello from day05!");

    let text = read_to_string("src/bin/day05/example.txt").unwrap();

    let (stacks_raw, moves) = text.split_once("\n\n").unwrap();

    let stacks = Stacks::from(stacks_raw);
    println!("Stacks: {:?}", stacks.0)
}
