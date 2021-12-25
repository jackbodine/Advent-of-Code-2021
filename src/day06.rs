use std::fs;

pub fn main() {
    let input = fs::read_to_string("data/6.txt")
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

    //let value = fishByDate.sum();   //Needs to count
    let sum: usize = fishByDate.iter().sum();
    print!("{}", sum);
}

fn part1() {
    let input = fs::read_to_string("data/6.txt")
        .expect("Error reading file");

        let mut fish: Vec<usize> = String::from("0,0").split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        let days = 256;
    
        for line in input.lines() {
            fish = line.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        }
    
        for i in 0..days{
            print!("Starting day: {}\n", i);
            for j in 0..fish.len(){
                if fish[j] == 0 {
                    fish[j] = 6;
                    fish.push(8);
                } else {
                    fish[j] -= 1;
                }
            }
        }

    let value = fish.len();
    print!("{}", value);
}