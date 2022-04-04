use std::collections::{HashMap, HashSet};

fn factorize(n: u32) -> Vec<u32> {
    let mut factors: Vec<u32> = vec![];
    let max = (n as f32).sqrt().trunc() as u32;
    for i in 1..=max {
        if n % i == 0 {
            factors.push(i);
            if i > 1 && i != max {
                factors.push(n/i);
            }
        }
    }
    return factors;
}

fn sum_vec(v: Vec<u32>) -> u32 {
    let mut res = 0;
    for n in v {
        res += n;
    }
    return res;
}

fn main() {
    let mut summed_factors: HashMap<u32,u32> = HashMap::new();
    for i in 1..10000 {
        let sum_of_factors = sum_vec(factorize(i));
        summed_factors.insert(i, sum_of_factors);
    }
    let mut amicable_numbers: HashSet<u32> = HashSet::new();
    for (k, v) in &summed_factors {
        let d_of_b = sum_vec(factorize(*v));
        if *k == d_of_b && *k != *v {
            amicable_numbers.insert(*k);
            if *v < 10000 { amicable_numbers.insert(*v); }
        }
    }
    let mut res = 0;
    for n in &amicable_numbers {
        res += *n;
    }
    println!("{}", res);
}
