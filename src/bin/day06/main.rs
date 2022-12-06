use std::{collections::HashSet, fs::read_to_string};

struct Marker {
    marker: [char; 4],
    start: [char; 14],
}

impl Marker {
    fn new() -> Self {
        Self {
            marker: [' '; 4],
            start: [' '; 14],
        }
    }

    fn add(&mut self, c: char) {
        self.marker.rotate_right(1);
        self.marker[0] = c;
        self.start.rotate_right(1);
        self.start[0] = c;
    }

    fn unique(&self) -> bool {
        self.marker
            .clone()
            .into_iter()
            .collect::<HashSet<char>>()
            .len()
            == self.marker.len()
            && !self.marker.contains(&' ')
    }

    fn unique_msg(&self) -> bool {
        self.start
            .clone()
            .into_iter()
            .collect::<HashSet<char>>()
            .len()
            == self.start.len()
            && !self.start.contains(&' ')
    }
}

fn main() {
    println!("Hello from day06!");

    let text = read_to_string("src/bin/day06/input.txt")
        .unwrap()
        .trim()
        .to_owned();

    let mut marker = Marker::new();

    let mut found_mark: bool = false;
    let mut found_msg: bool = false;

    for (i, c) in text.bytes().into_iter().enumerate() {
        marker.add(c as char);
        if marker.unique() && !found_mark {
            println!("Marker at: {}", i + 1);
            found_mark = true;
        }
        if marker.unique_msg() && !found_msg {
            println!("Message start at: {}", i + 1);
            found_msg = true;
        }
        if found_msg && found_mark {
            break;
        }
    }
}
