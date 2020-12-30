use std::collections::HashSet;

fn floored_sqrt(n: u64) -> u64 {
    let mut x: f64 = n as f64;
    let mut y: f64 = 1.0;
    let acc = 0.000001; //arbitrary high accuracy
    while x - y > acc {
        x = (x + y)/2.0;
        y = n as f64 / x;
    }
    return x as u64;
}

fn main() {
    let mut i: u64 = 1;
    let mut triangle: u64 = 1;
    loop {
        let mut factors: HashSet<u64> = HashSet::new();
        for num in 1..=floored_sqrt(triangle) {
            if triangle % num == 0 {
                factors.insert(num);
                factors.insert(triangle / num);
            }
        }
        if factors.len() > 500 {
            break;
        }
        i += 1;
        triangle += i;
    }
    println!("{}", triangle);
}