use std::fs;

pub fn main() {
    let input = fs::read_to_string("data/10-2.txt")
        .expect("Error reading file");

    let mut scores = Vec::new();

    for line in input.lines(){
        let mut vec = Vec::new();
        let open_chars = ['(', '[', '{', '<'];

        for c in line.chars() {
            if open_chars.contains(&c){
                vec.push(c);
            } else {
                vec.pop();
            }
        }

        let mut score:usize = 0;

        while vec.len() > 0 {
            let ch = vec.pop().unwrap();
            score = score * 5;

            if ch == '(' {
                score += 1;
            } else if ch == '[' {
                score += 2;
            } else if ch == '{' {
                score += 3;
            } else if ch == '<' {
                score += 4;
            }
        }

        scores.push(score);
    }

    scores.sort();

    let middle = scores.len() / 2;
    let value = *scores.get(middle).unwrap();

    print!("{}", value);
}