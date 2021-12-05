use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let numbers : Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().trim().parse::<i64>().unwrap())
        .collect();

    let mut counter = 0;

    for i  in 0..(numbers.len() - 3) {
        if numbers[i] - numbers[i+3] < 0{
            counter += 1;
        }
    }

    println!("result {}", counter)

}
