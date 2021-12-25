extern crate ndarray;
use ndarray::Array2;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("data/9.txt")
        .expect("Error reading file");
    
        //let mut state = [[0usize; 100]; 100];
        let mut state = Array2::<u32>::zeros((100, 100));
        
        let mut lowPoints: Vec<u32> = Vec::new();

        for (i, line) in input.lines().enumerate() {
            for (j, num) in line.chars().enumerate() {
                state[[i,j]] = num.to_digit(10).unwrap();
            }
        }

        for i in 0..100 {
            for j in 0..100 {
                
                let point = state[[i,j]];
                //print!("Checking: {}, {}: {}\n",i,j,point);
                let mut lowPoint = true;

                //up
                if i != 0 {
                    if state[[i-1,j]] <= point {
                        lowPoint = false;
                    }
                }

                //down
                if i != 99 {
                    if state[[i+1,j]] <= point {
                        lowPoint = false;
                    }
                }

                //right
                if j != 99 {
                    if state[[i,j+1]] <= point {
                        lowPoint = false;
                    }
                }

                //left
                if j != 0 {
                    if state[[i,j-1]] <= point {
                        lowPoint = false;
                    }
                }

                if lowPoint {
                    //print!("{},{} is a lowpoint \n", i,j);
                    lowPoints.push(point);
                }
            }
        }

    let mut sum: u32 = 0;
    for p in lowPoints {
        sum += p;
        sum += 1;
    }
    print!("{}", sum);
}
