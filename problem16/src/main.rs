use num::bigint::BigUint;

fn main() {
    let mut n = BigUint::from(2 as u32);
    n = n.pow(1000);
    let digits: Vec<_> = n.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let mut sum = 0;
    for digit in digits {
        sum = sum + digit;
    }
    println!("{}", sum);
}