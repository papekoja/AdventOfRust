use std::fs;

pub fn part1() {
    let path = "src/strategy.txt";
    let contents = fs::read_to_string(path).unwrap();
    let data: Vec<Vec<&str>>  = contents
        .split('\n')
        .map(|x| x.split(' ').collect())
        .collect();
    let mut total_score = 0;
    for x in data {
        total_score += calc_score(x[0], x[1]);
    }
    println!("{}", total_score);
}

fn calc_score(a: &str, x: &str) -> i32 {
    let score = calc_value(x) + calc_result(a, x);
    println!("{}", calc_result(a,x));
    score
}

fn calc_value(x: &str) -> i32 {
    match x {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    }
}

fn calc_result(a: &str, x: &str) -> i32 {
    if (a == "A" && x == "Z") || (a == "B" && x == "X") || (a == "C" && x == "Y") {
        0
    } else if (a == "A" && x == "X") || (a == "B" && x == "Y") || (a == "C" && x == "Z") {
        3
    } else if (a == "A" && x == "Y") || (a == "B" && x == "Z") || (a == "C" && x == "X") {
        6
    } else {
        0
    }
}