use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("/home/prashant/advent_of_code/day_2/inputs/input").unwrap();
    let buf = BufReader::new(f);
    let input_lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();
    challenge1(&input_lines);
    // challenge2(&input_lines);

}

fn challenge1(lines: &Vec<String>) {

    for x in lines{
        println!("Challenge1:: {:?}", x);
    }

}
