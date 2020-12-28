fn main() {
    let mut sum_of_squares: u64 = 0;
    let mut sum_to_square: u64 = 0;
    for i in 1..=100 {
        sum_to_square += i;
        sum_of_squares += i*i;
    }
    let sum_squared: u64 = sum_to_square * sum_to_square;
    println!("{}", sum_squared - sum_of_squares);
}