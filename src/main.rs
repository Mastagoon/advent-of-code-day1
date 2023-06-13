use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
    read_lines("data.txt")
}

fn read_lines(filename: &str) {
    let mut result = Vec::new();

    let mut sum_arr: Vec<i32> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        if line.len() > 0 {
            println!("continue");
            let num: i32 = line.parse().unwrap_or(0);
            sum_arr.push(num);
        } else {
            println!("break");
            let sum: i32 = sum_arr.clone().into_iter().sum();
            result.push(sum);
            sum_arr = Vec::new();
        }
    }
    result.sort();
    result.reverse();
    println!("{:?}", result[0] + result[1] + result[2]);
}
