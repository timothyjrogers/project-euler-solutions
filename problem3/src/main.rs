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

fn is_prime(n: u64) -> bool {
    for i in 2..=floored_sqrt(n) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let source: u64 = 600851475143;
    let mut max: u64 = 0;
    let mut factors: Vec<u64> = vec![];
    for i in 2..floored_sqrt(source) {
        if source % i == 0 {
            factors.push(i);
        }
    }
    for factor in factors {
        if is_prime(factor) && factor > max {
            max = factor;
        }
    }
    println!("{}", max);
}