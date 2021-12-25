use std::fs;


pub fn main() {
    let input = fs::read_to_string("data/8.txt")
        .expect("Error reading file");

    let mut value = 0;

    for line in input.lines(){
        let line2 = line.split("|").collect::<Vec<&str>>();
        let output = line2.get(1).unwrap().split(" ").collect::<Vec<&str>>();

        for item in output {
            let size = item.len();
            let sizes = [2, 3, 4, 7];
            if sizes.contains(&size) {
                value +=1;
            }
        }
    }

    print!("{}", value);
}