use std::{
    fs::File,
    io::Read,
};

#[derive(Debug)]
struct Rucksack {
    compart1: Vec<usize>,
    compart2: Vec<usize>,
}

impl From<&str> for Rucksack {
    fn from(items: &str) -> Self {
        let len = items.len();
        let mut items = items.to_owned();
        let (lhs, rhs) = items.split_at_mut(len / 2);
        let compart1 = lhs
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    return c as usize - 'A' as usize + 27;
                } else {
                    return c as usize - 'a' as usize + 1;
                }
            })
            .collect::<Vec<usize>>();
        let compart2 = rhs
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    return c as usize - 'A' as usize + 27;
                } else {
                    return c as usize - 'a' as usize + 1;
                }
            })
            .collect::<Vec<usize>>();
        assert_eq!(compart1.len(), compart2.len());
        Self { compart1, compart2 }
    }
}

impl Rucksack {
    fn arrange(&self) -> usize {
        for x in &self.compart1 {
            for y in &self.compart2 {
                if x == y {
                    return x.clone();
                }
            }
        }
        0
    }

    fn get_contents(&self) -> Vec<usize> {
        let mut ret = self.compart1.clone();
        ret.extend_from_slice(&self.compart2);
        assert_eq!(ret.len(), self.compart1.len() + self.compart2.len());
        ret
    }
}

fn intersect(a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    //println!("calc inter:\n\t{:?}:{}\n\t{:?}:{}", a, a.len(), b, b.len());
    let mut ret = vec![];
    for x in &a {
        for y in &b {
            if *x == *y {
                ret.push(x.clone());
            }
        }
    }
    ret
}

struct Group {
    sacks: Vec<Rucksack>,
    badge: usize,
}

impl From<&str> for Group {
    fn from(group: &str) -> Self {
        let sacks = group
            .trim()
            .split("\n")
            .map(|e| Rucksack::from(e))
            .collect::<Vec<Rucksack>>();
        assert_eq!(sacks.len(), 3);
        let tmp = intersect(sacks[0].get_contents(), sacks[1].get_contents());
        let int = intersect(tmp, sacks[2].get_contents());
        let badge = int[0];
        Self { sacks, badge }
    }
}

fn main() {
    println!("Hello from day03!");

    let mut file = File::open("src/bin/day03/input.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    let mut lines = text.trim().split("\n").collect::<Vec<&str>>();

    let mut groups = vec![];
    let mut sum = 0;
    while lines.len() != 0 {
        let mut raw_group = vec![];
        raw_group.push(lines.pop().unwrap());
        raw_group.push(lines.pop().unwrap());
        raw_group.push(lines.pop().unwrap());
        let group = Group::from(raw_group.join("\n").as_str());
        group.sacks.iter().for_each(|e| sum += e.arrange());
        groups.push(group);
    }
    let badges = groups.iter().map(|e| e.badge).sum::<usize>();

    println!("Badges {}", badges);
    println!("Total {}", sum);
}
