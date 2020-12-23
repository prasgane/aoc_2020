use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("../inputs/input").unwrap();
    let buf = BufReader::new(f);
    let input_lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();
    challenge1(&input_lines);
    challenge2(&input_lines);

}

fn challenge1(lines: &Vec<String>) {

    for (position1, value1) in lines.iter().enumerate() {
        for (_, value2) in lines.iter().skip(position1+1).enumerate(){
            let val1 = value1.parse::<i32>().unwrap();
            let val2 = value2.parse::<i32>().unwrap();

            if  val1 + val2 == 2020 {
                println!("Challenge1::The numbers were {:?}", (val1, val2));
                println!("Challenge1::Product is {:?}", val1*val2);
            }
        }
    }
}

fn challenge2(lines: &Vec<String>) {

    for (position1, value1) in lines.iter().enumerate() {
        for (position2, value2) in lines.iter().skip(position1+1).enumerate(){
            for (_, value3) in lines.iter().skip(position2+2).enumerate() {
                let val1 = value1.parse::<i32>().unwrap();
                let val2 = value2.parse::<i32>().unwrap();
                let val3 = value3.parse::<i32>().unwrap();

                if val1 + val2 + val3 == 2020 {
                    println!("Challenge2::The numbers were {:?}", (val1, val2, val3));
                    println!("Challenge2::Product is {:?}", val1 * val2 * val3);
                }
            }
        }
    }

}
