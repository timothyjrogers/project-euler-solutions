fn main() {
    let mut max_n: u64 = 2;
    let mut max_seq_len = 0;
    for i in 2..=1_000_000 {
        let mut cur: u64 = i;
        let mut seq_len = 0;
        while cur != 1 {
            seq_len = seq_len + 1;
            if cur % 2 == 0 {
                cur = cur / 2;
            } else {
                cur = 3*cur + 1;
            }
        }
        if seq_len > max_seq_len { 
            max_seq_len = seq_len as u64;
            max_n = i;
        }
    }
    println!("{}", max_n);
}