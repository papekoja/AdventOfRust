use std::collections::BinaryHeap;
use std::fs;

pub fn part2() {
    let path = "src/input.txt";
    let contents = fs::read_to_string(path).unwrap();
    let mut temp = 0;
    let mut heap =  BinaryHeap::new();
    contents.split('\n').for_each(|x| {
        if x == "" {
            heap.push(temp);
            temp = 0;
        } else {
            let n = x.parse::<i32>().unwrap();
            temp = temp + n;
        }
    });
    let mut sum = 0;
    for _ in 0..3 {
        sum += heap.pop().unwrap();
    }
    println!("{}", sum);
}