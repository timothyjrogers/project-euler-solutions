use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

const FNAME: &str = "triangle.txt";


fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let mut lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    let mut rows: Vec<Vec<u32>> = vec![];
    for line in lines {
        rows.push(line.split(' ').map(|x| x.parse::<u32>().unwrap()).collect());
    }

    for i in 0..rows.len()-1 {
        let index = rows.len() - 2 - i;
        let next = index + 1;
        for pos in 0..rows[index].len() {
            let child = std::cmp::max(rows[next][pos], rows[next][pos+1]);
            rows[index][pos] = rows[index][pos] + child;
        }
    }
    println!("{}", rows[0][0]);
}