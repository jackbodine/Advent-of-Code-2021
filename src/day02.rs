use std::fs;
use std::vec::Vec;

pub fn main() {
    let input = fs::read_to_string("data/2.txt")
        .expect("Error reading file");

    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;

    for line in input.lines() {
        let v2:u32 = line.chars().last().unwrap().to_digit(10).unwrap();
        let v:i32 = v2 as i32;
        
        if line.contains("forward"){
            position += v;
            depth += aim * v;

        } else if line.contains("down"){
            aim += v;

        } else if line.contains("up"){
            aim -= v;
        }
    }

    let value = depth * position;
    print!("{}", value);
}