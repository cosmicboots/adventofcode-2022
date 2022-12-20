use std::{error::Error, fs::read_to_string};

#[derive(Debug)]
enum Instruction {
    Addx(isize),
    Noop,
}

impl From<&str> for Instruction {
    fn from(instr: &str) -> Self {
        match instr.trim().split_whitespace().collect::<Vec<&str>>()[..] {
            ["noop"] => Instruction::Noop,
            ["addx", x] => Instruction::Addx(x.to_owned().parse::<isize>().unwrap()),
            _ => panic!("Invalid instruction"),
        }
    }
}

trait VecExt<T> {
    fn from(_: T) -> Self;
}

impl VecExt<&str> for Vec<Instruction> {
    fn from(instrs: &str) -> Self {
        instrs
            .trim()
            .split("\n")
            .map(|x| Instruction::from(x))
            .collect()
    }
}

#[derive(Debug)]
struct Cpu {
    clock: usize,
    xreg: isize,
    search_cycles: Vec<usize>,
    sum_sig_strength: isize,
}

impl Cpu {
    fn new(search: &[usize]) -> Self {
        let mut search_cycles = search.to_vec();
        search_cycles.sort();
        Self {
            clock: 1,
            xreg: 1,
            search_cycles,
            sum_sig_strength: 0,
        }
    }

    fn exec(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Addx(x) => {
                self.increment(2);
                self.xreg += x;
            }
            Instruction::Noop => self.increment(1),
        }
    }

    fn increment(&mut self, count: usize) {
        for i in self.clock..(self.clock + count) {
            if self.search_cycles.contains(&i) {
                //println!("Current cycle: {}", self.clock);
                self.sum_sig_strength += i as isize * self.xreg;
            }
            self.clock += 1;
            //println!("{:?}", self);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from day10!");

    //let file = read_to_string("src/bin/day10/example.txt")?;
    let file = read_to_string("src/bin/day10/input.txt")?;

    let instrs: Vec<Instruction> = VecExt::<&str>::from(&file.as_str());

    let mut cpu = Cpu::new(&[20, 60, 100, 140, 180, 220]);

    for instr in instrs {
        //println!("Executing: {:?}", instr);
        cpu.exec(&instr);
    }

    println!("CPU after exec: {:?}", cpu);

    Ok(())
}
