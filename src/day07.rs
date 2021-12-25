use std::fs;

pub fn main() {
    let input = fs::read_to_string("data/7.txt")
        .expect("Error reading file");

    let pos: Vec<usize> = input.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    let len = pos.len();
    let mut sum:usize = 0;
    for i in pos {
        sum += i;
    }
    let mut avg = sum / len;
    avg -= 1;
    
    let mut min = isize::MAX;
    for a in 0..2000{
        let mut value:isize = 0;

        let pos2: Vec<usize> = input.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        for v in pos2 {
            let n = (a - v as isize).abs();
            value += ((n*n) + n) / 2;
        }

        if value < min {
            min = value;
        }
    }

    print!("{}", min);
}