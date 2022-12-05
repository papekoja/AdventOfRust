use std::fs;

fn main() {
    let path = "src/input.txt";
    let contents = fs::read_to_string(path).unwrap();
    let mut max = 0;
    let mut temp = 0;
    contents.split('\n').for_each(|x| {
        if x == "" {
            if temp > max{
              max = temp.clone();
            }
            temp = 0;
        } else {
            let n = x.parse::<i32>().unwrap();
            temp = temp + n;
        }
    });
    println!("{:?}", max);
}