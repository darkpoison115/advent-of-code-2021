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

    let mut last_element = numbers[0];
    let mut counter = 0;

    for number in numbers {
        if last_element < number{
            counter += 1;
        }

        last_element = number;
    }

    println!("result {}", counter)

}
