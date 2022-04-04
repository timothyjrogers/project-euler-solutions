use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

const FNAME: &str = "p022_names.txt";


fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let mut lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    lines.sort();

    let mut res = 0;
    let alpha_scores: HashMap<char, u32> = HashMap::from([('A', 1), ('B', 2), ('C', 3), ('D', 4), ('E', 5), ('F', 6), ('G', 7), ('H', 8), ('I', 9), ('J', 10), ('K', 11), ('L', 12), ('M', 13), ('N', 14), ('O', 15), ('P', 16), ('Q', 17), ('R', 18), ('S', 19), ('T', 20), ('U', 21), ('V', 22), ('W', 23), ('X', 24), ('Y', 25), ('Z', 26)]);
    for (index, line) in lines.iter().enumerate() {
        let mut val = 0;
        for c in line.chars() {
            match &alpha_scores.get(&c) {
                Some(x) => val += *x,
                None => ()
            }
        }
        res += val * (index as u32 + 1);
    }
    println!("{}", res);
}