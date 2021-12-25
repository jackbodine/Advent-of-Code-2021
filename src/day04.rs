extern crate ndarray;
use ndarray::Array3;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("data/4.txt")
        .expect("Error reading file");


    let mut boards = [[[0isize; 100]; 50]; 100];
    //let mut boards = Array3::<usize>::zeros((5, 5,1000));
    let mut calls;


    let mut boardNum = 0;
    let mut row = 0;
    for line in input.lines() {

        if line.contains(',') {
            calls = line.split(',');
            continue;
        }

        if row == 6{
            print!("NEW BOARD!\n");
            boardNum += 1;
            row = 0;
            continue;
        }
        
        let line2 = line.split(' ');
        let mut counter:isize = 0;
        for (i, c) in line2.enumerate() {

             print!("row: {}, col: {}, board: {}\n", row, counter, boardNum);
            if (c.chars().nth(0) != Some(' ')) && (c.chars().nth(0) != None){
                print!("Parsing: {}\n", c);
                let col = counter as usize;
                //rint!("{}, col);
                boards[boardNum][col][row] = c.parse().unwrap();
            } else {
                counter -= 1;
            }
            counter += 1;
        }
        row += 1;

    }

    for i in 1..6 {
        for j in 0..5 {
            print!("{} ", boards[i][j][0]);
        }
        print!("\n");
    }

    let value = 0;
    print!("{}", value);
}