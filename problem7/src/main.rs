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
    let mut count = 1;
    let mut i: u64 = 3;
    while count < 10_001 {
        if i % 2 == 0 { continue };
        if is_prime(i) {
            count += 1;
        }
        i += 2;
    }
    println!("{}", i-2);
}