use std::fs;

pub fn main() {
    let input = fs::read_to_string("data/11.txt")
        .expect("Error reading file");

    let mut fish: Vec<usize> = String::from("0,0").split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let mut fishByDate: Vec<usize> = vec![0; 9];
    let days = 256;

    for line in input.lines() {
        fish = line.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    }

    for f in fish {
        fishByDate[f] += 1;
    }

    for i in 0..days{
        print!("Starting day: {}\n", i);

        let mut newFish = fishByDate[0];

        for level in 0..fishByDate.len()-1 {
            fishByDate[level] = fishByDate[level+1];
        }

        fishByDate[6] += newFish;
        fishByDate[8] = newFish;

    }

    let sum: usize = fishByDate.iter().sum();
    print!("{}", sum);
}
