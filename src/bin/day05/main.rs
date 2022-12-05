use std::fs::read_to_string;

#[derive(Clone)]
struct Stacks(Vec<Vec<char>>);

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Move {
    fn from(raw_move: &str) -> Self {
        // move 1 from 2 to 1
        let s: Vec<&str> = raw_move.split_ascii_whitespace().collect();
        Self {
            count: s[1].parse::<usize>().unwrap(),
            from: s[3].parse::<usize>().unwrap(),
            to: s[5].parse::<usize>().unwrap(),
        }
    }
}

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

        for line in lines[..lines.len() - 1].iter().rev() {
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

impl Stacks {
    fn make_stack_moves(&mut self, moves: &Vec<Move>) {
        for moveq in moves {
            for _ in 0..moveq.count {
                let val = self.0[moveq.from - 1].pop().unwrap();
                self.0[moveq.to - 1].push(val);
            }
        }
    }

    fn make_moves(&mut self, moves: &Vec<Move>) {
        for moveq in moves {
            let f = moveq.from - 1;
            let t = moveq.to - 1;
            let crates = self.0[f][self.0[f].len() - moveq.count..].to_vec();
            let truncate = self.0[f].len()-crates.len();
            self.0[f].truncate(truncate);
            self.0[t].extend_from_slice(&crates);
        }
    }
}

fn main() {
    println!("Hello from day05!");

    let text = read_to_string("src/bin/day05/input.txt").unwrap();

    let (stacks_raw, moves) = text.split_once("\n\n").unwrap();

    let moves = moves
        .trim()
        .split("\n")
        .map(|e| Move::from(e.trim()))
        .collect::<Vec<Move>>();

    let mut stacks = Stacks::from(stacks_raw);
    let mut stacks2 = stacks.clone();
    println!("Starting Stacks: {:?}", stacks.0);
    stacks.make_stack_moves(&moves);
    stacks2.make_moves(&moves);
    println!("Ending Stacks (part1): {:?}", stacks.0);
    print!("Top of stacks: ");
    for stack in stacks.0 {
        print!("{}", stack.last().unwrap());
    }
    println!("\nPart2:");
    println!("Ending Stacks (part2): {:?}", stacks2.0);
    print!("Top of stacks: ");
    for stack in stacks2.0 {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
