use std::fs;

extern crate ndarray;
use ndarray::Array2;

pub fn main() {
    let input = fs::read_to_string("data/5.txt")
        .expect("Error reading file");

    let mut a2 = Array2::<usize>::zeros((1000, 1000));
    
    for line in input.lines() {

        let cords = line.split(" ").collect::<Vec<&str>>();
        let start_cords = cords.get(0).unwrap().split(",").collect::<Vec<&str>>();
        let end_cords = cords.get(2).unwrap().split(",").collect::<Vec<&str>>();

        let x_start:usize = start_cords.get(0).unwrap().parse().unwrap();
        let y_start:usize = start_cords.get(1).unwrap().parse().unwrap();

        let x_end:usize = end_cords.get(0).unwrap().parse().unwrap();
        let y_end:usize = end_cords.get(1).unwrap().parse().unwrap();

        print!("Starting: ({},{})\n", x_start, y_start);
        print!("Ending: ({},{})\n", x_end, y_end);

        let mut x_current = x_start;
        let mut y_current = y_start;

        if y_start == y_end {
            while x_current != x_end {
                a2[[x_current, y_current]] += 1;

                if x_current < x_end {
                    x_current += 1;
                } else {
                    x_current -= 1;
                }

                if x_current == x_end {
                    a2[[x_current, y_current]] += 1;
                }
            }
        } else if x_start == x_end {

            while y_current != y_end {
                a2[[x_current, y_current]] += 1;

                if y_current < y_end {
                    y_current += 1;
                } else {
                    y_current -= 1;
                }

                if y_current == y_end {
                    a2[[x_current, y_current]] += 1;
                }
            }

        } else {
            while y_current != y_end {
                a2[[x_current, y_current]] += 1;

                if y_current < y_end {
                    y_current += 1;
                } else {
                    y_current -= 1;
                }

                if x_current < x_end {
                    x_current += 1;
                } else {
                    x_current -= 1;
                }

                if y_current == y_end {
                    a2[[x_current, y_current]] += 1;
                }
            }
        }
    }

    let mut value = 0;
    for row in a2.outer_iter() {
        for item in row {
            //print!("{} ", item);
            if item >= &2 {
                value += 1;
            }
        }
        //print!("\n");
    }

    print!("{}", value);
}