fn main() {
    let mut i = 20;
    loop {
        let mut valid = true;
        for j in 1..=20 {
            if i % j != 0 {
                valid = false;
                break;
            }
        }
        if valid { break };
        i += 1;
    }
    println!("{}", i);
}