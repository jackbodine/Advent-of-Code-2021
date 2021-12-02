use std::fs;
use std::vec::Vec;

pub fn main() {
    let input = fs::read_to_string("data/1.txt")
    .expect("Error reading file");

    let mut count:u32 = 0;
    let mut total:i32 = 0;
    let mut vec = Vec::new();


    for line in input.lines() {
        vec.push(line);
    }

    total = vec.get(0).unwrap().parse::<i32>().unwrap() + vec.get(1).unwrap().parse::<i32>().unwrap() + vec.get(2).unwrap().parse::<i32>().unwrap();

    for i in 3..vec.len() {
        let temp = total;
        
        total = total - vec.get(i-3).unwrap().parse::<i32>().unwrap();
        total = total + vec.get(i).unwrap().parse::<i32>().unwrap();

        if total > temp {
            count += 1;
        }
    }

    print!("{}",count);
}