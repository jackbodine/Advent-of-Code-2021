use std::fs;

pub fn main() {
    

}

pub fn part1() {
    let input = fs::read_to_string("data/3.txt")
        .expect("Error reading file");

    //let mut v: Vec<i32> = Vec::new();
    let mut v: [i8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,];

    for line in input.lines() {
        for (i, char) in line.chars().enumerate() {
            let t = v.get(i).unwrap();
            let mut n = *t;

            if char == '0' {
                n -= 1;
            } else {
                n += 1;
            }

            v[i] = n;
        }
    }

    let mut s:String = String::from("");
    let mut s2:String = String::from("");

    println!("Hello");
    for i in v {
        if i > 0 {
            s.push('1');
            s2.push('0');
        } else {
            s.push('0');
            s2.push('1');
        }
    }


    print!("{}\n", s);
    print!("{}", s2);
}