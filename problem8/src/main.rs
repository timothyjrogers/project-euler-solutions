use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let fname = "data.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut digits: Vec<u64> = vec![];
    for v in vec {
        for c in v.chars() {
            digits.push(c.to_string().parse::<u64>().unwrap());
        }
    }
    let mut max: u64 = 0;
    for i in 0..digits.len()-13 {
        let mut res: u64 = 1;
        for j in 0..13 {
            res = res * digits[i+j];
        }
        if res > max { max = res };
    }
    println!("{}", max);
}