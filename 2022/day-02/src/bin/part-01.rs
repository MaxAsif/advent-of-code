use std::fs;
use day_02::process_part1;

fn main(){
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&file));
}