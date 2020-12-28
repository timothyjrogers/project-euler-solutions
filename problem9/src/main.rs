fn main() {
    for c in 3..=1000 {
        for b in 2..c {
            for a in 1..b {
                if a*a + b*b == c*c && a + b + c == 1000 {
                    println!("{}", a*b*c);
                    return;
                }
            }
        }
    }
}