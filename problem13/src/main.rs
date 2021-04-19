use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let fname = "data.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut nums: Vec<Vec<u32>> = vec![];
    for v in vec {
        let mut nvec: Vec<u32> = vec![];
        for c in v.chars() {
            nvec.push(c.to_string().parse::<u32>().unwrap());
        }
        nums.push(nvec);
    }
    
    let mut sum: Vec<u32> = vec![0; 50];
    while !nums.is_empty() {
        let mut num = nums.pop().unwrap();
        let mut carry = 0;

        if sum.len() > num.len() {
            for _i in 0..sum.len()-num.len() {
                num.insert(0, 0);
            }
        }

        let size = sum.len();
        for i in 1..=size {
            let p_sum = sum[size-i] + num[size-i] + carry;
            if p_sum < 10 {
                sum[size-i] = p_sum;
                carry = 0;
            } else {
                sum[size-i] = p_sum % 10;
                carry = p_sum / 10;
            }
        }
        if carry > 0 {
            sum.insert(0, carry);
        }
    }
    println!("{}", sum.iter().map(|x| x.to_string()).collect::<Vec<String>>()[0..10].join(""));
}