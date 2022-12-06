use std::{fs, collections::{HashMap}};

pub fn part1() {
    let path = "src/signal.txt";
    let content = fs::read_to_string(path).unwrap();
    let l: Vec<char> = content.chars().collect();
    let mut n = 0;
    for x in 0..l.len()-4 {
        if unique(&content[x..x+4]) {
            break;
        }
        n += 1;
    }
    print!("{}", n+4);
}

fn unique(s: &str) -> bool {
    for x in s.chars() {
        if s.matches(x).count() > 1 {
            return false;
        }
    }
    println!("{s}");
    true
}