use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let actions : Vec<(String,i64)> = reader
        .lines()
        .map(|line| match line.unwrap().splitn(2,' ').collect_tuple(){
            Some((action, number)) => (action.to_string(), number.trim().parse::<i64>().unwrap()),
            None => panic!("invalid input."),
        })
        .collect();

    let mut position : (i64,i64) = (0,0);

    for (action, number)  in actions {
        let (x,y) = position;
        position = match action.as_str() {
            "forward" => (x+number,y),
            "down" => (x,y+number),
            "up" => (x,y-number),
            _ => panic!("Invalid action {}.", action),
        }
    }

    let (x,y) = position;

    println!("position ({},{}).",x,y);
    println!("The result of multiplying is {}.",x*y);

}
