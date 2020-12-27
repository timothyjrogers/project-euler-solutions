fn main() {
    let mut sum = 2;
    let mut t0: u64 = 1;
    let mut t1: u64 = 2;
    while t1 <= 4_000_000 {
        let mut t2 = t0 + t1;
        t0 = t1;
        t1 = t2;
        if t1 % 2 == 0 { sum += t1 };
    }
    println!("{}", sum);
}